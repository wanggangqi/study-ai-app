//! 数据库 CRUD 操作
//!
//! 提供所有数据表的增删改查操作

use std::sync::MutexGuard;
use rusqlite::{params, Row, Connection};
use uuid::Uuid;
use chrono::Utc;

use super::{
    DbError,
    schema::{CourseStatus, LessonStatus, AgentType, MessageRole},
};

/// ==================== Course 操作 ====================

/// Course 数据结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub gitee_repo_url: Option<String>,
    pub local_path: Option<String>,
    pub target_level: Option<String>,
    pub duration: Option<String>,
    pub teaching_style: Option<String>,
    pub created_at: String,
    pub status: i32,
}

impl Course {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Course {
            id: row.get(0)?,
            name: row.get(1)?,
            gitee_repo_url: row.get(2)?,
            local_path: row.get(3)?,
            target_level: row.get(4)?,
            duration: row.get(5)?,
            teaching_style: row.get(6)?,
            created_at: row.get(7)?,
            status: row.get(8)?,
        })
    }
}

/// 创建新课程
pub fn create_course(
    conn: &MutexGuard<Connection>,
    name: String,
    target_level: Option<String>,
    duration: Option<String>,
    teaching_style: Option<String>,
) -> Result<Course, DbError> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();
    let status = CourseStatus::InProgress.to_i32();

    conn.execute(
        "INSERT INTO courses (id, name, gitee_repo_url, local_path, target_level, duration, teaching_style, created_at, status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![id, name, None::<String>, None::<String>, target_level, duration, teaching_style, created_at, status],
    )?;

    Ok(Course {
        id,
        name,
        gitee_repo_url: None,
        local_path: None,
        target_level,
        duration,
        teaching_style,
        created_at,
        status,
    })
}

/// 获取所有课程
pub fn get_all_courses(conn: &MutexGuard<Connection>) -> Result<Vec<Course>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, gitee_repo_url, local_path, target_level, duration, teaching_style, created_at, status
         FROM courses ORDER BY created_at DESC"
    )?;

    let courses = stmt.query_map([], |row| Course::from_row(row))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(courses)
}

/// 根据 ID 获取课程
pub fn get_course_by_id(conn: &MutexGuard<Connection>, id: &str) -> Result<Course, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, gitee_repo_url, local_path, target_level, duration, teaching_style, created_at, status
         FROM courses WHERE id = ?1"
    )?;

    let course = stmt.query_row([id], |row| Course::from_row(row))
        .map_err(|_| DbError::NotFound(format!("Course with id '{}' not found", id)))?;

    Ok(course)
}

/// 更新课程
pub fn update_course(
    conn: &MutexGuard<Connection>,
    id: &str,
    name: Option<String>,
    gitee_repo_url: Option<String>,
    local_path: Option<String>,
    target_level: Option<String>,
    duration: Option<String>,
    teaching_style: Option<String>,
    status: Option<i32>,
) -> Result<Course, DbError> {
    let mut updates = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(v) = name {
        updates.push("name = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = gitee_repo_url {
        updates.push("gitee_repo_url = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = local_path {
        updates.push("local_path = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = target_level {
        updates.push("target_level = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = duration {
        updates.push("duration = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = teaching_style {
        updates.push("teaching_style = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = status {
        updates.push("status = ?");
        params_vec.push(Box::new(v));
    }

    if updates.is_empty() {
        return get_course_by_id(conn, id);
    }

    params_vec.push(Box::new(id.to_string()));
    let sql = format!("UPDATE courses SET {} WHERE id = ?", updates.join(", "));
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

    conn.execute(&sql, params_refs.as_slice())?;

    get_course_by_id(conn, id)
}

/// 删除课程
pub fn delete_course(conn: &MutexGuard<Connection>, id: &str) -> Result<(), DbError> {
    let rows_affected = conn.execute("DELETE FROM courses WHERE id = ?1", [id])?;

    if rows_affected == 0 {
        return Err(DbError::NotFound(format!("Course with id '{}' not found", id)));
    }

    Ok(())
}

/// ==================== Chapter 操作 ====================

/// Chapter 数据结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    pub id: String,
    pub course_id: String,
    pub chapter_index: i32,
    pub name: String,
}

impl Chapter {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Chapter {
            id: row.get(0)?,
            course_id: row.get(1)?,
            chapter_index: row.get(2)?,
            name: row.get(3)?,
        })
    }
}

/// 创建章节
pub fn create_chapter(
    conn: &MutexGuard<Connection>,
    course_id: String,
    chapter_index: i32,
    name: String,
) -> Result<Chapter, DbError> {
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO chapters (id, course_id, chapter_index, name) VALUES (?1, ?2, ?3, ?4)",
        params![id, course_id, chapter_index, name],
    )?;

    Ok(Chapter {
        id,
        course_id,
        chapter_index,
        name,
    })
}

/// 获取课程的所有章节
pub fn get_chapters_by_course(conn: &MutexGuard<Connection>, course_id: &str) -> Result<Vec<Chapter>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, course_id, chapter_index, name FROM chapters WHERE course_id = ?1 ORDER BY chapter_index"
    )?;

    let chapters = stmt.query_map([course_id], |row| Chapter::from_row(row))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(chapters)
}

/// 更新章节
pub fn update_chapter(
    conn: &MutexGuard<Connection>,
    id: &str,
    name: Option<String>,
    chapter_index: Option<i32>,
) -> Result<Chapter, DbError> {
    let mut updates = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(v) = name {
        updates.push("name = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = chapter_index {
        updates.push("chapter_index = ?");
        params_vec.push(Box::new(v));
    }

    if updates.is_empty() {
        let mut stmt = conn.prepare("SELECT id, course_id, chapter_index, name FROM chapters WHERE id = ?1")?;
        return stmt.query_row([id], |row| Chapter::from_row(row))
            .map_err(|_| DbError::NotFound(format!("Chapter with id '{}' not found", id)));
    }

    params_vec.push(Box::new(id.to_string()));
    let sql = format!("UPDATE chapters SET {} WHERE id = ?", updates.join(", "));
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

    conn.execute(&sql, params_refs.as_slice())?;

    let mut stmt = conn.prepare("SELECT id, course_id, chapter_index, name FROM chapters WHERE id = ?1")?;
    stmt.query_row([id], |row| Chapter::from_row(row))
        .map_err(|_| DbError::NotFound(format!("Chapter with id '{}' not found", id)))
}

/// 删除章节
pub fn delete_chapter(conn: &MutexGuard<Connection>, id: &str) -> Result<(), DbError> {
    let rows_affected = conn.execute("DELETE FROM chapters WHERE id = ?1", [id])?;

    if rows_affected == 0 {
        return Err(DbError::NotFound(format!("Chapter with id '{}' not found", id)));
    }

    Ok(())
}

/// ==================== Lesson 操作 ====================

/// Lesson 数据结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Lesson {
    pub id: String,
    pub chapter_id: String,
    pub lesson_index: i32,
    pub name: String,
    pub duration: Option<String>,
    pub status: i32,
    pub completed_at: Option<String>,
    pub lesson_file: Option<String>,
}

impl Lesson {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Lesson {
            id: row.get(0)?,
            chapter_id: row.get(1)?,
            lesson_index: row.get(2)?,
            name: row.get(3)?,
            duration: row.get(4)?,
            status: row.get(5)?,
            completed_at: row.get(6)?,
            lesson_file: row.get(7)?,
        })
    }
}

/// 创建课时
pub fn create_lesson(
    conn: &MutexGuard<Connection>,
    chapter_id: String,
    lesson_index: i32,
    name: String,
    duration: Option<String>,
) -> Result<Lesson, DbError> {
    let id = Uuid::new_v4().to_string();
    let status = LessonStatus::NotStarted.to_i32();

    conn.execute(
        "INSERT INTO lessons (id, chapter_id, lesson_index, name, duration, status, completed_at, lesson_file)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![id, chapter_id, lesson_index, name, duration, status, None::<String>, None::<String>],
    )?;

    Ok(Lesson {
        id,
        chapter_id,
        lesson_index,
        name,
        duration,
        status,
        completed_at: None,
        lesson_file: None,
    })
}

/// 获取章节的所有课时
pub fn get_lessons_by_chapter(conn: &MutexGuard<Connection>, chapter_id: &str) -> Result<Vec<Lesson>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, chapter_id, lesson_index, name, duration, status, completed_at, lesson_file
         FROM lessons WHERE chapter_id = ?1 ORDER BY lesson_index"
    )?;

    let lessons = stmt.query_map([chapter_id], |row| Lesson::from_row(row))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(lessons)
}

/// 根据 ID 获取课时
pub fn get_lesson_by_id(conn: &MutexGuard<Connection>, id: &str) -> Result<Lesson, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, chapter_id, lesson_index, name, duration, status, completed_at, lesson_file
         FROM lessons WHERE id = ?1"
    )?;

    let lesson = stmt.query_row([id], |row| Lesson::from_row(row))
        .map_err(|_| DbError::NotFound(format!("Lesson with id '{}' not found", id)))?;

    Ok(lesson)
}

/// 更新课时状态
pub fn update_lesson_status(
    conn: &MutexGuard<Connection>,
    id: &str,
    status: LessonStatus,
) -> Result<Lesson, DbError> {
    let completed_at = if status == LessonStatus::Completed {
        Some(Utc::now().to_rfc3339())
    } else {
        None
    };

    conn.execute(
        "UPDATE lessons SET status = ?1, completed_at = ?2 WHERE id = ?3",
        params![status.to_i32(), completed_at, id],
    )?;

    get_lesson_by_id(conn, id)
}

/// 更新课时（包含课件文件路径）
pub fn update_lesson(
    conn: &MutexGuard<Connection>,
    id: &str,
    name: Option<String>,
    duration: Option<String>,
    lesson_file: Option<String>,
    status: Option<i32>,
) -> Result<Lesson, DbError> {
    let mut updates = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(v) = name {
        updates.push("name = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = duration {
        updates.push("duration = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = lesson_file {
        updates.push("lesson_file = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = status {
        updates.push("status = ?");
        params_vec.push(Box::new(v));
        if v == LessonStatus::Completed.to_i32() {
            updates.push("completed_at = ?");
            params_vec.push(Box::new(Utc::now().to_rfc3339()));
        }
    }

    if updates.is_empty() {
        return get_lesson_by_id(conn, id);
    }

    params_vec.push(Box::new(id.to_string()));
    let sql = format!("UPDATE lessons SET {} WHERE id = ?", updates.join(", "));
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

    conn.execute(&sql, params_refs.as_slice())?;

    get_lesson_by_id(conn, id)
}

/// 删除课时
pub fn delete_lesson(conn: &MutexGuard<Connection>, id: &str) -> Result<(), DbError> {
    let rows_affected = conn.execute("DELETE FROM lessons WHERE id = ?1", [id])?;

    if rows_affected == 0 {
        return Err(DbError::NotFound(format!("Lesson with id '{}' not found", id)));
    }

    Ok(())
}

/// ==================== Lesson Exercise 操作 ====================

/// LessonExercise 数据结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LessonExercise {
    pub id: String,
    pub lesson_id: String,
    pub exercise_file: Option<String>,
    pub score: Option<i32>,
    pub submitted_at: Option<String>,
    pub result_file: Option<String>,
}

impl LessonExercise {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(LessonExercise {
            id: row.get(0)?,
            lesson_id: row.get(1)?,
            exercise_file: row.get(2)?,
            score: row.get(3)?,
            submitted_at: row.get(4)?,
            result_file: row.get(5)?,
        })
    }
}

/// 创建练习记录
pub fn create_exercise(
    conn: &MutexGuard<Connection>,
    lesson_id: String,
    exercise_file: Option<String>,
) -> Result<LessonExercise, DbError> {
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO lesson_exercises (id, lesson_id, exercise_file, score, submitted_at, result_file)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, lesson_id, exercise_file, None::<i32>, None::<String>, None::<String>],
    )?;

    Ok(LessonExercise {
        id,
        lesson_id,
        exercise_file,
        score: None,
        submitted_at: None,
        result_file: None,
    })
}

/// 获取课时的所有练习
pub fn get_exercises_by_lesson(conn: &MutexGuard<Connection>, lesson_id: &str) -> Result<Vec<LessonExercise>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, lesson_id, exercise_file, score, submitted_at, result_file
         FROM lesson_exercises WHERE lesson_id = ?1 ORDER BY submitted_at DESC"
    )?;

    let exercises = stmt.query_map([lesson_id], |row| LessonExercise::from_row(row))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(exercises)
}

/// 更新练习成绩
pub fn update_exercise_score(
    conn: &MutexGuard<Connection>,
    id: &str,
    score: i32,
    result_file: Option<String>,
) -> Result<LessonExercise, DbError> {
    let submitted_at = Utc::now().to_rfc3339();

    conn.execute(
        "UPDATE lesson_exercises SET score = ?1, submitted_at = ?2, result_file = ?3 WHERE id = ?4",
        params![score, submitted_at, result_file, id],
    )?;

    let mut stmt = conn.prepare(
        "SELECT id, lesson_id, exercise_file, score, submitted_at, result_file FROM lesson_exercises WHERE id = ?1"
    )?;

    stmt.query_row([id], |row| LessonExercise::from_row(row))
        .map_err(|_| DbError::NotFound(format!("Exercise with id '{}' not found", id)))
}

/// 删除练习记录
pub fn delete_exercise(conn: &MutexGuard<Connection>, id: &str) -> Result<(), DbError> {
    let rows_affected = conn.execute("DELETE FROM lesson_exercises WHERE id = ?1", [id])?;

    if rows_affected == 0 {
        return Err(DbError::NotFound(format!("Exercise with id '{}' not found", id)));
    }

    Ok(())
}

/// ==================== Chat Message 操作 ====================

/// ChatMessage 数据结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub course_id: String,
    pub lesson_id: Option<String>,
    pub agent_type: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

impl ChatMessage {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(ChatMessage {
            id: row.get(0)?,
            course_id: row.get(1)?,
            lesson_id: row.get(2)?,
            agent_type: row.get(3)?,
            role: row.get(4)?,
            content: row.get(5)?,
            created_at: row.get(6)?,
        })
    }
}

/// 创建聊天消息
pub fn create_chat_message(
    conn: &MutexGuard<Connection>,
    course_id: String,
    lesson_id: Option<String>,
    agent_type: AgentType,
    role: MessageRole,
    content: String,
) -> Result<ChatMessage, DbError> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO chat_messages (id, course_id, lesson_id, agent_type, role, content, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id, course_id, lesson_id, agent_type.to_str(), role.to_str(), content, created_at],
    )?;

    Ok(ChatMessage {
        id,
        course_id,
        lesson_id,
        agent_type: agent_type.to_str().to_string(),
        role: role.to_str().to_string(),
        content,
        created_at,
    })
}

/// 获取课程的所有聊天消息
pub fn get_chat_messages_by_course(conn: &MutexGuard<Connection>, course_id: &str) -> Result<Vec<ChatMessage>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, course_id, lesson_id, agent_type, role, content, created_at
         FROM chat_messages WHERE course_id = ?1 ORDER BY created_at ASC"
    )?;

    let messages = stmt.query_map([course_id], |row| ChatMessage::from_row(row))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(messages)
}

/// 获取课时的聊天消息
pub fn get_chat_messages_by_lesson(conn: &MutexGuard<Connection>, lesson_id: &str) -> Result<Vec<ChatMessage>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, course_id, lesson_id, agent_type, role, content, created_at
         FROM chat_messages WHERE lesson_id = ?1 ORDER BY created_at ASC"
    )?;

    let messages = stmt.query_map([lesson_id], |row| ChatMessage::from_row(row))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(messages)
}

/// 删除聊天消息
pub fn delete_chat_message(conn: &MutexGuard<Connection>, id: &str) -> Result<(), DbError> {
    let rows_affected = conn.execute("DELETE FROM chat_messages WHERE id = ?1", [id])?;

    if rows_affected == 0 {
        return Err(DbError::NotFound(format!("Chat message with id '{}' not found", id)));
    }

    Ok(())
}

/// 清空课程的所有聊天消息
pub fn clear_chat_messages_by_course(conn: &MutexGuard<Connection>, course_id: &str) -> Result<(), DbError> {
    conn.execute("DELETE FROM chat_messages WHERE course_id = ?1", [course_id])?;
    Ok(())
}

/// ==================== User Config 操作 ====================

/// UserConfig 数据结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserConfig {
    pub id: i32,
    pub gitee_username: Option<String>,
    pub gitee_token: Option<String>,
    pub workspace_path: Option<String>,
    pub ai_provider: Option<String>,
    pub ai_api_key: Option<String>,
    pub ai_model: Option<String>,
    pub git_username: Option<String>,
    pub git_email: Option<String>,
}

impl UserConfig {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(UserConfig {
            id: row.get(0)?,
            gitee_username: row.get(1)?,
            gitee_token: row.get(2)?,
            workspace_path: row.get(3)?,
            ai_provider: row.get(4)?,
            ai_api_key: row.get(5)?,
            ai_model: row.get(6)?,
            git_username: row.get(7)?,
            git_email: row.get(8)?,
        })
    }
}

/// 获取用户配置（只有一条记录，id = 1）
pub fn get_user_config(conn: &MutexGuard<Connection>) -> Result<UserConfig, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, gitee_username, gitee_token, workspace_path, ai_provider, ai_api_key, ai_model, git_username, git_email
         FROM user_config WHERE id = 1"
    )?;

    let config = stmt.query_row([], |row| UserConfig::from_row(row))
        .map_err(|_| DbError::NotFound("User config not found".to_string()))?;

    Ok(config)
}

/// 获取或创建用户配置
pub fn get_or_create_user_config(conn: &MutexGuard<Connection>) -> Result<UserConfig, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, gitee_username, gitee_token, workspace_path, ai_provider, ai_api_key, ai_model, git_username, git_email
         FROM user_config WHERE id = 1"
    )?;

    match stmt.query_row([], |row| UserConfig::from_row(row)) {
        Ok(config) => Ok(config),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            // 创建默认配置
            conn.execute(
                "INSERT INTO user_config (id) VALUES (1)",
                [],
            )?;
            get_user_config(conn)
        }
        Err(e) => Err(DbError::SqliteError(e)),
    }
}

/// 更新用户配置
pub fn update_user_config(
    conn: &MutexGuard<Connection>,
    gitee_username: Option<String>,
    gitee_token: Option<String>,
    workspace_path: Option<String>,
    ai_provider: Option<String>,
    ai_api_key: Option<String>,
    ai_model: Option<String>,
    git_username: Option<String>,
    git_email: Option<String>,
) -> Result<UserConfig, DbError> {
    // 确保配置记录存在
    get_or_create_user_config(conn)?;

    let mut updates = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(v) = gitee_username {
        updates.push("gitee_username = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = gitee_token {
        updates.push("gitee_token = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = workspace_path {
        updates.push("workspace_path = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = ai_provider {
        updates.push("ai_provider = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = ai_api_key {
        updates.push("ai_api_key = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = ai_model {
        updates.push("ai_model = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = git_username {
        updates.push("git_username = ?");
        params_vec.push(Box::new(v));
    }
    if let Some(v) = git_email {
        updates.push("git_email = ?");
        params_vec.push(Box::new(v));
    }

    if !updates.is_empty() {
        let sql = format!("UPDATE user_config SET {} WHERE id = 1", updates.join(", "));
        let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        conn.execute(&sql, params_refs.as_slice())?;
    }

    get_user_config(conn)
}
