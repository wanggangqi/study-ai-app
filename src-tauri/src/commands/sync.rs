//! 数据同步命令
//!
//! 提供课程数据导出和码云同步功能

use std::fs;
use std::path::PathBuf;
use thiserror::Error;

use crate::db::operations::{
    Lesson,
    get_course_by_id, get_chapters_by_course, get_lessons_by_chapter,
    get_exercises_by_lesson, get_chat_messages_by_course,
    get_lesson_by_id, update_lesson,
};
use crate::commands::database::DbState;
use crate::services::git_ops::{
    is_git_installed, init_repo, add_all, commit, push,
    has_remote, add_remote, set_default_branch, has_changes, GitError,
};
use crate::services::config::load_config;
use super::gitee::{create_gitee_repo_internal, GiteeError};

/// 将中文课程名转换为码云可用的仓库名
/// 中文转拼音首字母，保留英文字母和数字
fn to_gitee_repo_name(course_name: &str, course_id: &str) -> String {
    // 简化的中文到拼音首字母映射（常用字）
    let pinyin_map = [
        ("智", "z"), ("能", "n"), ("家", "j"), ("居", "j"),
        ("语", "y"), ("文", "w"), ("数", "s"), ("学", "x"),
        ("英", "y"), ("语", "y"), ("历", "l"), ("史", "s"),
        ("物", "w"), ("理", "l"), ("化", "h"), ("生", "s"),
        ("地", "d"), ("理", "l"), ("政", "z"), ("治", "z"),
        ("音", "y"), ("乐", "y"), ("美", "m"), ("术", "s"),
        ("体", "t"), ("育", "y"), ("编", "b"), ("程", "c"),
        ("前", "q"), ("端", "d"), ("后", "h"), ("端", "d"),
        ("全", "q"), ("栈", "z"), ("人", "r"), ("工", "g"),
        ("智", "z"), ("能", "n"), ("汽", "q"), ("车", "c"),
        ("机", "j"), ("器", "q"), ("学", "x"), ("习", "x"),
    ];

    let mut result = String::new();
    for ch in course_name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            result.push(ch);
        } else if ch.is_ascii_whitespace() {
            result.push('_');
        } else {
            // 尝试查找拼音映射
            let mut found = false;
            for (c, p) in &pinyin_map {
                if *c == ch.to_string() {
                    result.push_str(p);
                    found = true;
                    break;
                }
            }
            if !found {
                // 未找到映射，跳过该字符
            }
        }
    }

    // 确保非空且以字母或数字开头
    if result.is_empty() || !result.chars().next().unwrap().is_alphanumeric() {
        result = format!("course_{}", &course_id[..8.min(course_id.len())]);
    }

    // 限制长度并添加 course_id 后缀保证唯一性
    let suffix = &course_id[..8.min(course_id.len())];
    if result.len() > 180 {
        format!("{}_{}", &result[..175], suffix)
    } else {
        format!("{}_{}", result, suffix)
    }
}

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
    #[error("配置错误: {0}")]
    ConfigError(String),
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

/// 获取或创建用户配置，如果数据库中没有则从配置文件读取
pub fn get_config() -> Result<crate::services::config::AppConfig, SyncError> {
    load_config().map_err(|e| SyncError::ConfigError(e.to_string()))
}

/// 创建课程仓库结构
pub fn create_course_repo_structure(base_path: &str, repo_name: &str, course_name: &str) -> Result<(), SyncError> {
    let repo_path = PathBuf::from(base_path).join(repo_name);

    // 创建目录结构（添加 .gitkeep 确保空目录被 Git 追踪）
    let lessons_dir = repo_path.join("lessons");
    let exercises_dir = repo_path.join("exercises");
    let notes_dir = repo_path.join("notes");

    fs::create_dir_all(&lessons_dir)?;
    fs::create_dir_all(&exercises_dir)?;
    fs::create_dir_all(&notes_dir)?;

    // 在空目录中添加 .gitkeep 文件
    fs::write(lessons_dir.join(".gitkeep"), "")?;
    fs::write(exercises_dir.join(".gitkeep"), "")?;
    fs::write(notes_dir.join(".gitkeep"), "")?;

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

/// 同步课程数据到本地 Git 仓库（内部实现）
pub(crate) fn sync_course_to_git_impl(
    db: &DbState,
    course_id: &str,
) -> Result<SyncResult, SyncError> {
    eprintln!("[Sync] 开始同步课程数据，course_id={}", course_id);

    // 检查 Git 是否安装
    if !is_git_installed() {
        return Ok(SyncResult {
            success: false,
            message: "Git 未安装或未配置".to_string(),
            repo_url: None,
        });
    }

    // 获取用户配置
    let config = get_config()?;

    let workspace_path = config.workspace_path
        .ok_or(SyncError::WorkspaceNotConfigured)?;

    // 获取课程信息
    let (course, repo_name, chapters) = {
        let database = db.0.lock().map_err(|e| SyncError::DbError(e.to_string()))?;
        let guard = database.get_connection();
        let course = get_course_by_id(&guard, course_id)
            .map_err(|e| SyncError::DbError(e.to_string()))?;
        let repo_name = to_gitee_repo_name(&course.name, course_id);
        let chapters = get_chapters_by_course(&guard, course_id)
            .map_err(|e| SyncError::DbError(e.to_string()))?;
        (course, repo_name, chapters)
    };

    eprintln!("[Sync] 课程信息: name={}, id={}, repo_name={}", course.name, course.id, repo_name);

    let repo_path = PathBuf::from(&workspace_path).join(&repo_name);
    eprintln!("[Sync] 仓库路径: {}", repo_path.display());

    // 导出计划和数据
    eprintln!("[Sync] 正在导出课程计划和记录...");
    let plan = export_course_plan(db, course_id)?;
    let records = export_learning_records(db, course_id)?;

    // 写入 JSON 文件
    let plan_json = serde_json::to_string_pretty(&plan)?;
    let records_json = serde_json::to_string_pretty(&records)?;

    fs::write(repo_path.join("plan.json"), plan_json)?;
    fs::write(repo_path.join("records.json"), records_json)?;
    eprintln!("[Sync] plan.json 和 records.json 写入完成");

    // 同步课件文件
    eprintln!("[Sync] 正在同步课件文件...");
    {
        let database = db.0.lock().map_err(|e| SyncError::DbError(e.to_string()))?;
        let guard = database.get_connection();

        let lessons_dir = repo_path.join("lessons");
        fs::create_dir_all(&lessons_dir)?;

        for chapter in &chapters {
            let lessons = get_lessons_by_chapter(&guard, &chapter.id)
                .map_err(|e| SyncError::DbError(e.to_string()))?;

            for lesson in lessons {
                if let Some(lesson_file) = &lesson.lesson_file {
                    // 检查课件文件是否存在
                    let source_path = PathBuf::from(lesson_file);
                    if source_path.exists() {
                        // 复制到仓库的 lessons 目录
                        let dest_path = lessons_dir.join(format!("{}.html", lesson.id));
                        fs::copy(&source_path, &dest_path)?;
                        eprintln!("[Sync] 复制课件文件: {} -> {}", source_path.display(), dest_path.display());
                    }
                }
            }
        }
    }
    eprintln!("[Sync] 课件文件同步完成");

    // Git 操作
    if !repo_path.exists() {
        eprintln!("[Sync] 仓库不存在，正在创建...");
        fs::create_dir_all(&repo_path)?;
        init_repo(repo_path.to_str().unwrap())?;

        // 如果有远程仓库 URL，设置远程
        if let Some(repo_url) = &course.gitee_repo_url {
            eprintln!("[Sync] 添加远程仓库: {}", repo_url);
            if !has_remote(repo_path.to_str().unwrap(), "origin")? {
                add_remote(repo_path.to_str().unwrap(), "origin", repo_url)?;
            }
        }

        set_default_branch(repo_path.to_str().unwrap(), "main")?;

        // 初始提交
        eprintln!("[Sync] 执行初始提交...");
        add_all(repo_path.to_str().unwrap())?;
        commit(repo_path.to_str().unwrap(), "初始化课程仓库")?;
    } else {
        // 检查是否有变更
        eprintln!("[Sync] 检查仓库变更...");
        if has_changes(repo_path.to_str().unwrap())? {
            eprintln!("[Sync] 有变更，正在提交...");
            add_all(repo_path.to_str().unwrap())?;
            commit(repo_path.to_str().unwrap(), "更新学习进度")?;
        } else {
            eprintln!("[Sync] 无变更，跳过提交");
        }
    }

    // 推送（如果有远程仓库）
    if let Some(repo_url) = &course.gitee_repo_url {
        eprintln!("[Sync] 尝试推送，repo_url={}", repo_url);
        if has_remote(repo_path.to_str().unwrap(), "origin")? {
            eprintln!("[Sync] 正在推送到 origin/main...");
            push(repo_path.to_str().unwrap(), "origin", "main")?;
            eprintln!("[Sync] 推送完成");
        } else {
            eprintln!("[Sync] 无远程仓库，跳过推送");
        }
    }

    Ok(SyncResult {
        success: true,
        message: "同步成功".to_string(),
        repo_url: course.gitee_repo_url,
    })
}

/// 创建课程仓库（本地 + 码云）（内部实现）
pub(crate) async fn create_course_repository_impl(
    db: &DbState,
    course_id: &str,
) -> Result<SyncResult, SyncError> {
    eprintln!("[Sync] 开始创建课程仓库，course_id={}", course_id);

    // 检查 Git 是否安装
    if !is_git_installed() {
        eprintln!("[Sync] Git 未安装");
        return Ok(SyncResult {
            success: false,
            message: "Git 未安装或未配置".to_string(),
            repo_url: None,
        });
    }
    eprintln!("[Sync] Git 已安装");

    // 获取用户配置
    let config = get_config()?;
    eprintln!("[Sync] 配置加载成功: workspace_path={}, gitee_username={}, gitee_token长度={}",
        config.workspace_path.as_deref().unwrap_or("未配置"),
        config.gitee_username.as_deref().unwrap_or("未配置"),
        config.gitee_token.as_ref().map(|t| t.len()).unwrap_or(0));

    let workspace_path = config.workspace_path
        .ok_or(SyncError::WorkspaceNotConfigured)?;
    let gitee_token = config.gitee_token
        .ok_or(SyncError::GiteeNotConfigured)?;
    let gitee_username = config.gitee_username
        .ok_or(SyncError::GiteeNotConfigured)?;

    eprintln!("[Sync] 配置验证通过: workspace_path={}, gitee_username={}", workspace_path, gitee_username);

    // 获取课程信息
    let course = {
        let database = db.0.lock().map_err(|e| SyncError::DbError(e.to_string()))?;
        let guard = database.get_connection();
        get_course_by_id(&guard, course_id)
            .map_err(|e| SyncError::DbError(e.to_string()))?
    };
    eprintln!("[Sync] 课程信息: name={}, id={}", course.name, course.id);

    let repo_name = to_gitee_repo_name(&course.name, course_id);
    eprintln!("[Sync] 仓库名称: {}", repo_name);

    // 在码云创建远程仓库
    eprintln!("[Sync] 正在调用码云 API 创建仓库...");
    let gitee_repo = create_gitee_repo_internal(
        &gitee_token,
        &repo_name,
        &format!("智学伴侣 - {} 课程学习记录", course.name),
        true, // 私有仓库
    ).await
    .map_err(|e| {
        eprintln!("[Sync] 码云 API 调用失败: {}", e);
        SyncError::GiteeError(e)
    })?;
    eprintln!("[Sync] 码云仓库创建成功: html_url={}", gitee_repo.html_url);

    // 创建本地仓库结构
    eprintln!("[Sync] 正在创建本地仓库结构...");
    let base_path = PathBuf::from(&workspace_path);
    create_course_repo_structure(&workspace_path, &repo_name, &course.name)?;
    eprintln!("[Sync] 本地仓库结构创建完成");

    let repo_path = base_path.join(&repo_name);
    eprintln!("[Sync] 本地仓库路径: {}", repo_path.display());

    // 初始化 Git 仓库
    eprintln!("[Sync] 正在初始化 Git 仓库...");
    init_repo(repo_path.to_str().unwrap())?;
    eprintln!("[Sync] Git 仓库初始化完成");

    // 添加远程仓库
    let remote_url = format!("https://gitee.com/{}/{}", gitee_username, repo_name);
    eprintln!("[Sync] 远程仓库 URL: {}", remote_url);
    add_remote(repo_path.to_str().unwrap(), "origin", &remote_url)?;
    set_default_branch(repo_path.to_str().unwrap(), "main")?;

    // 导出计划和记录
    eprintln!("[Sync] 正在导出课程计划和记录...");
    let plan = export_course_plan(db, course_id)?;
    let records = export_learning_records(db, course_id)?;

    let plan_json = serde_json::to_string_pretty(&plan)?;
    let records_json = serde_json::to_string_pretty(&records)?;

    fs::write(repo_path.join("plan.json"), plan_json)?;
    fs::write(repo_path.join("records.json"), records_json)?;
    eprintln!("[Sync] plan.json 和 records.json 写入完成");

    // 初始提交
    eprintln!("[Sync] 正在执行 Git 提交...");
    add_all(repo_path.to_str().unwrap())?;
    commit(repo_path.to_str().unwrap(), "初始化课程仓库")?;
    eprintln!("[Sync] Git 提交完成");

    // 推送到远程
    eprintln!("[Sync] 正在推送到远程仓库...");
    push(repo_path.to_str().unwrap(), "origin", "main")?;
    eprintln!("[Sync] 推送完成");

    Ok(SyncResult {
        success: true,
        message: "仓库创建成功".to_string(),
        repo_url: Some(gitee_repo.html_url),
    })
}

/// 保存课件文件到工作空间并更新数据库
#[tauri::command]
pub fn save_lesson_file_command(
    db: State<'_, DbState>,
    course_id: String,
    lesson_id: String,
    content: String,
) -> Result<String, String> {
    save_lesson_file_impl(&*db, &course_id, &lesson_id, &content)
        .map_err(|e| e.to_string())
}

/// 保存课件文件的内部实现
pub(crate) fn save_lesson_file_impl(
    db: &DbState,
    course_id: &str,
    lesson_id: &str,
    content: &str,
) -> Result<String, SyncError> {
    // 获取用户配置
    let config = get_config()?;
    let workspace_path = config.workspace_path
        .ok_or(SyncError::WorkspaceNotConfigured)?;

    // 获取课程和课时信息
    let (course, lesson) = {
        let database = db.0.lock().map_err(|e| SyncError::DbError(e.to_string()))?;
        let guard = database.get_connection();

        let course = get_course_by_id(&guard, course_id)
            .map_err(|e| SyncError::DbError(e.to_string()))?;

        let lesson = get_lesson_by_id(&guard, lesson_id)
            .map_err(|e| SyncError::DbError(e.to_string()))?;

        (course, lesson)
    };

    // 生成仓库名称
    let repo_name = to_gitee_repo_name(&course.name, course_id);

    // 创建课件文件路径
    let lessons_dir = PathBuf::from(&workspace_path)
        .join(&repo_name)
        .join("lessons");

    // 确保目录存在
    fs::create_dir_all(&lessons_dir)?;

    // 保存课件 HTML 文件
    let file_name = format!("{}.html", lesson_id);
    let file_path = lessons_dir.join(&file_name);
    fs::write(&file_path, content)?;

    eprintln!("[Sync] 课件已保存到: {}", file_path.display());

    // 更新数据库中的 lesson_file 字段
    {
        let database = db.0.lock().map_err(|e| SyncError::DbError(e.to_string()))?;
        let guard = database.get_connection();

        update_lesson(&guard, lesson_id, None, None, Some(file_path.to_str().unwrap().to_string()), None)
            .map_err(|e| SyncError::DbError(e.to_string()))?;
    }

    Ok(file_path.to_str().unwrap().to_string())
}

// ==================== Tauri 命令 ====================

use tauri::State;

/// 同步课程数据到 Git 仓库
#[tauri::command]
pub fn sync_course_to_git_command(
    db: State<'_, DbState>,
    course_id: String,
) -> Result<SyncResult, String> {
    sync_course_to_git_impl(&*db, &course_id)
        .map_err(|e| e.to_string())
}

/// 创建课程仓库（本地 + 码云）
#[tauri::command]
pub async fn create_course_repository_command(
    db: State<'_, DbState>,
    course_id: String,
) -> Result<SyncResult, String> {
    create_course_repository_impl(&*db, &course_id)
        .await
        .map_err(|e| e.to_string())
}

// ==================== AI 课程大纲生成 ====================

use serde::{Deserialize, Serialize};

/// AI 生成课程大纲命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct AIGenerateCoursePlanParams {
    pub provider: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    pub model: Option<String>,
    #[serde(rename = "courseName")]
    pub course_name: String,
    #[serde(rename = "targetLevel")]
    pub target_level: String,
    pub duration: String,
    #[serde(rename = "teachingStyle")]
    pub teaching_style: String,
    #[serde(rename = "baseKnowledge")]
    pub base_knowledge: String,
}

/// AI 生成课程大纲命令
#[tauri::command]
pub async fn ai_generate_course_plan_command(
    params: AIGenerateCoursePlanParams,
) -> Result<crate::services::ai::CoursePlanOutline, String> {
    use crate::services::AIProvider;
    use crate::services::AIConfig;
    use crate::services::ai::generate_course_plan;

    let provider = match params.provider.to_lowercase().as_str() {
        "qwen" => AIProvider::Qwen,
        "deepseek" => AIProvider::DeepSeek,
        "glm" => AIProvider::Glm,
        "minimax" => AIProvider::MiniMax,
        "kimi" => AIProvider::Kimi,
        _ => return Err(format!("不支持的 AI 服务商: {}", params.provider)),
    };

    let config = AIConfig::new(provider, params.api_key);
    let config = if let Some(model) = params.model {
        config.with_model(model)
    } else {
        config
    };

    let result = generate_course_plan(
        &config,
        &params.course_name,
        &params.target_level,
        &params.duration,
        &params.teaching_style,
        &params.base_knowledge,
    ).await;

    match result {
        Ok(plan) => Ok(plan),
        Err(e) => Err(e.to_string()),
    }
}
