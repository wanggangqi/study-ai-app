//! 数据同步命令
//!
//! 提供课程数据导出和码云同步功能

use std::fs;
use std::path::PathBuf;
use thiserror::Error;

use crate::db::operations::{
    Lesson, UserConfig,
    get_course_by_id, get_chapters_by_course, get_lessons_by_chapter,
    get_exercises_by_lesson, get_chat_messages_by_course,
};
use crate::commands::database::DbState;
use crate::services::git_ops::{
    is_git_installed, init_repo, add_all, commit, push,
    has_remote, add_remote, set_default_branch, has_changes, GitError,
};
use super::gitee::{create_gitee_repo_internal, GiteeError};

/// 同步错误类型
#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Git 操作失败: {0}")]
    GitError(#[from] GitError),
    #[error("码云 API 错误: {0}")]
    GiteeError(#[from] GiteeError),
    #[error("数据库错误: {0}")]
    DbError(String),
    #[error("文件操作失败: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON 序列化失败: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("课程不存在: {0}")]
    CourseNotFound(String),
    #[error("工作空间未配置")]
    WorkspaceNotConfigured,
    #[error("码云账户未配置")]
    GiteeNotConfigured,
}

/// 课程计划 JSON 结构（对应 plan.json）
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CoursePlan {
    pub course_id: String,
    pub course_name: String,
    pub created_at: String,
    pub target_level: Option<String>,
    pub duration: Option<String>,
    pub teaching_style: Option<String>,
    pub chapters: Vec<ChapterPlan>,
}

/// 章节计划
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChapterPlan {
    pub chapter_id: String,
    pub chapter_index: i32,
    pub chapter_name: String,
    pub lessons: Vec<LessonPlan>,
}

/// 课时计划
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LessonPlan {
    pub lesson_id: String,
    pub lesson_index: i32,
    pub lesson_name: String,
    pub duration: Option<String>,
}

/// 学习记录 JSON 结构（对应 records.json）
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LearningRecords {
    pub course_id: String,
    pub updated_at: String,
    pub progress: Progress,
    pub lesson_status: Vec<LessonStatusRecord>,
    pub exercise_results: Vec<ExerciseResultRecord>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chat_history: Vec<ChatHistoryItem>,
}

/// 学习进度
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Progress {
    pub current_chapter: i32,
    pub current_lesson: i32,
    pub total_completed: i32,
    pub total_lessons: i32,
    pub percentage: i32,
}

/// 课时状态记录
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LessonStatusRecord {
    pub lesson_id: String,
    pub status: String,
    pub completed_at: Option<String>,
}

/// 练习结果记录
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExerciseResultRecord {
    pub lesson_id: String,
    pub score: Option<i32>,
    pub submitted_at: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub weak_points: Vec<String>,
}

/// 聊天历史项
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChatHistoryItem {
    pub lesson_id: Option<String>,
    pub agent_type: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

/// 同步结果
#[derive(Debug, serde::Serialize)]
pub struct SyncResult {
    pub success: bool,
    pub message: String,
    pub repo_url: Option<String>,
}

/// 导出课程计划到 JSON
pub fn export_course_plan(
    db: &DbState,
    course_id: &str,
) -> Result<CoursePlan, SyncError> {
    let database = db.0.lock().map_err(|e| SyncError::DbError(e.to_string()))?;
    let guard = database.get_connection();

    let course = get_course_by_id(&guard, course_id)
        .map_err(|e| SyncError::DbError(e.to_string()))?;

    let chapters = get_chapters_by_course(&guard, course_id)
        .map_err(|e| SyncError::DbError(e.to_string()))?;

    let mut chapter_plans = Vec::new();
    for chapter in chapters {
        let lessons = get_lessons_by_chapter(&guard, &chapter.id)
            .map_err(|e| SyncError::DbError(e.to_string()))?;

        let lesson_plans: Vec<LessonPlan> = lessons
            .into_iter()
            .map(|l| LessonPlan {
                lesson_id: l.id,
                lesson_index: l.lesson_index,
                lesson_name: l.name,
                duration: l.duration,
            })
            .collect();

        chapter_plans.push(ChapterPlan {
            chapter_id: chapter.id,
            chapter_index: chapter.chapter_index,
            chapter_name: chapter.name,
            lessons: lesson_plans,
        });
    }

    Ok(CoursePlan {
        course_id: course.id,
        course_name: course.name,
        created_at: course.created_at,
        target_level: course.target_level,
        duration: course.duration,
        teaching_style: course.teaching_style,
        chapters: chapter_plans,
    })
}

/// 导出学习记录到 JSON
pub fn export_learning_records(
    db: &DbState,
    course_id: &str,
) -> Result<LearningRecords, SyncError> {
    let database = db.0.lock().map_err(|e| SyncError::DbError(e.to_string()))?;
    let guard = database.get_connection();

    let course = get_course_by_id(&guard, course_id)
        .map_err(|e| SyncError::DbError(e.to_string()))?;

    let chapters = get_chapters_by_course(&guard, course_id)
        .map_err(|e| SyncError::DbError(e.to_string()))?;

    let mut all_lessons: Vec<Lesson> = Vec::new();
    let mut lesson_status_records = Vec::new();
    let mut total_lessons = 0;
    let mut completed_lessons = 0;
    let mut current_chapter = 0;
    let mut current_lesson = 0;

    for (idx, chapter) in chapters.iter().enumerate() {
        let lessons = get_lessons_by_chapter(&guard, &chapter.id)
            .map_err(|e| SyncError::DbError(e.to_string()))?;

        for lesson in lessons {
            total_lessons += 1;
            let status_str = match lesson.status {
                2 => "completed",
                1 => "in_progress",
                _ => "not_started",
            };

            if lesson.status == 2 {
                completed_lessons += 1;
            }

            // 找到第一个未完成的课时作为当前进度
            if current_chapter == 0 && lesson.status != 2 {
                current_chapter = chapter.chapter_index;
                current_lesson = lesson.lesson_index;
            }

            lesson_status_records.push(LessonStatusRecord {
                lesson_id: lesson.id.clone(),
                status: status_str.to_string(),
                completed_at: lesson.completed_at.clone(),
            });

            // 获取练习结果
            let exercises = get_exercises_by_lesson(&guard, &lesson.id)
                .map_err(|e| SyncError::DbError(e.to_string()))?;

            for ex in exercises {
                if let Some(score) = ex.score {
                    lesson_status_records.push(LessonStatusRecord {
                        lesson_id: lesson.id.clone(),
                        status: format!("exercise_score:{}", score),
                        completed_at: ex.submitted_at.clone(),
                    });
                }
            }

            all_lessons.push(lesson);
        }
    }

    // 计算百分比
    let percentage = if total_lessons > 0 {
        (completed_lessons * 100) / total_lessons
    } else {
        0
    };

    // 获取聊天历史
    let messages = get_chat_messages_by_course(&guard, course_id)
        .map_err(|e| SyncError::DbError(e.to_string()))?;

    let chat_history: Vec<ChatHistoryItem> = messages
        .into_iter()
        .map(|m| ChatHistoryItem {
            lesson_id: m.lesson_id,
            agent_type: m.agent_type,
            role: m.role,
            content: m.content,
            created_at: m.created_at,
        })
        .collect();

    Ok(LearningRecords {
        course_id: course.id,
        updated_at: chrono::Utc::now().to_rfc3339(),
        progress: Progress {
            current_chapter,
            current_lesson,
            total_completed: completed_lessons,
            total_lessons,
            percentage,
        },
        lesson_status: lesson_status_records,
        exercise_results: vec![], // 简化处理
        chat_history,
    })
}

/// 获取用户配置
pub fn get_user_config(db: &DbState) -> Result<UserConfig, SyncError> {
    let database = db.0.lock().map_err(|e| SyncError::DbError(e.to_string()))?;
    let guard = database.get_connection();
    crate::db::operations::get_user_config(&guard)
        .map_err(|e| SyncError::DbError(e.to_string()))
}

/// 创建课程仓库结构
pub fn create_course_repo_structure(base_path: &str, course_name: &str) -> Result<(), SyncError> {
    let repo_path = PathBuf::from(base_path).join(format!("{}-course", course_name));

    // 创建目录结构
    fs::create_dir_all(repo_path.join("lessons"))?;
    fs::create_dir_all(repo_path.join("exercises"))?;
    fs::create_dir_all(repo_path.join("notes"))?;

    // 创建 README.md
    let readme_content = format!(
        r#"# {} 课程

> 由智学伴侣 (StudyMate) AI 教学应用管理

## 课程简介

本课程使用智学伴侣应用进行学习管理。

## 目录结构

- `plan.json` - 课程计划
- `records.json` - 学习记录
- `lessons/` - 课件目录
- `exercises/` - 练习题目录
- `notes/` - 笔记目录

---
*此仓库由智学伴侣自动同步*
"#,
        course_name
    );

    fs::write(repo_path.join("README.md"), readme_content)?;

    Ok(())
}

/// 同步课程数据到本地 Git 仓库
pub fn sync_course_to_git(
    db: &DbState,
    course_id: &str,
) -> Result<SyncResult, SyncError> {
    // 检查 Git 是否安装
    if !is_git_installed() {
        return Ok(SyncResult {
            success: false,
            message: "Git 未安装或未配置".to_string(),
            repo_url: None,
        });
    }

    // 获取用户配置
    let config = get_user_config(db)?;

    let workspace_path = config.workspace_path
        .ok_or(SyncError::WorkspaceNotConfigured)?;

    // 获取课程信息
    let database = db.0.lock().map_err(|e| SyncError::DbError(e.to_string()))?;
    let guard = database.get_connection();
    let course = get_course_by_id(&guard, course_id)
        .map_err(|e| SyncError::DbError(e.to_string()))?;
    drop(guard);

    let repo_path = PathBuf::from(&workspace_path).join(format!("{}-course", course.name));

    // 导出计划和数据
    let plan = export_course_plan(db, course_id)?;
    let records = export_learning_records(db, course_id)?;

    // 写入 JSON 文件
    let plan_json = serde_json::to_string_pretty(&plan)?;
    let records_json = serde_json::to_string_pretty(&records)?;

    fs::write(repo_path.join("plan.json"), plan_json)?;
    fs::write(repo_path.join("records.json"), records_json)?;

    // Git 操作
    if !repo_path.exists() {
        fs::create_dir_all(&repo_path)?;
        init_repo(repo_path.to_str().unwrap())?;

        // 如果有远程仓库 URL，设置远程
        if let Some(repo_url) = &course.gitee_repo_url {
            if !has_remote(repo_path.to_str().unwrap(), "origin")? {
                add_remote(repo_path.to_str().unwrap(), "origin", repo_url)?;
            }
        }

        set_default_branch(repo_path.to_str().unwrap(), "main")?;

        // 初始提交
        add_all(repo_path.to_str().unwrap())?;
        commit(repo_path.to_str().unwrap(), "初始化课程仓库")?;
    } else {
        // 检查是否有变更
        if has_changes(repo_path.to_str().unwrap())? {
            add_all(repo_path.to_str().unwrap())?;
            commit(repo_path.to_str().unwrap(), "更新学习进度")?;
        }
    }

    // 推送（如果有远程仓库）
    if let Some(repo_url) = &course.gitee_repo_url {
        if has_remote(repo_path.to_str().unwrap(), "origin")? {
            push(repo_path.to_str().unwrap(), "origin", "main")?;
        }
    }

    Ok(SyncResult {
        success: true,
        message: "同步成功".to_string(),
        repo_url: course.gitee_repo_url,
    })
}

/// 创建课程仓库（本地 + 码云）
#[allow(dead_code)]
pub async fn create_course_repository(
    db: &DbState,
    course_id: &str,
) -> Result<SyncResult, SyncError> {
    // 检查 Git 是否安装
    if !is_git_installed() {
        return Ok(SyncResult {
            success: false,
            message: "Git 未安装或未配置".to_string(),
            repo_url: None,
        });
    }

    // 获取用户配置
    let config = get_user_config(db)?;

    let workspace_path = config.workspace_path
        .ok_or(SyncError::WorkspaceNotConfigured)?;
    let gitee_token = config.gitee_token
        .ok_or(SyncError::GiteeNotConfigured)?;
    let gitee_username = config.gitee_username
        .ok_or(SyncError::GiteeNotConfigured)?;

    // 获取课程信息
    let database = db.0.lock().map_err(|e| SyncError::DbError(e.to_string()))?;
    let guard = database.get_connection();
    let course = get_course_by_id(&guard, course_id)
        .map_err(|e| SyncError::DbError(e.to_string()))?;
    drop(guard);

    let repo_name = format!("{}-course", course.name);

    // 在码云创建远程仓库
    let gitee_repo = create_gitee_repo_internal(
        &gitee_token,
        &repo_name,
        &format!("智学伴侣 - {} 课程学习记录", course.name),
        true, // 私有仓库
    ).await?;

    // 创建本地仓库结构
    let base_path = PathBuf::from(&workspace_path);
    create_course_repo_structure(&workspace_path, &course.name)?;

    let repo_path = base_path.join(&repo_name);

    // 初始化 Git 仓库
    init_repo(repo_path.to_str().unwrap())?;

    // 添加远程仓库
    let remote_url = format!("https://gitee.com/{}/{}", gitee_username, repo_name);
    add_remote(repo_path.to_str().unwrap(), "origin", &remote_url)?;
    set_default_branch(repo_path.to_str().unwrap(), "main")?;

    // 导出计划
    let plan = export_course_plan(db, course_id)?;
    let plan_json = serde_json::to_string_pretty(&plan)?;
    fs::write(repo_path.join("plan.json"), plan_json)?;

    // 初始提交
    add_all(repo_path.to_str().unwrap())?;
    commit(repo_path.to_str().unwrap(), "初始化课程仓库")?;

    // 推送到远程
    push(repo_path.to_str().unwrap(), "origin", "main")?;

    Ok(SyncResult {
        success: true,
        message: "仓库创建成功".to_string(),
        repo_url: Some(gitee_repo.html_url),
    })
}
