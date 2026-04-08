//! 数据库命令模块
//!
//! 提供与前端交互的 Tauri 数据库命令

use std::sync::Mutex;
use tauri::State;
use crate::db::{
    Database, DbError,
    schema::{AgentType, LessonStatus, MessageRole},
    operations::{
        self,
        Course, Chapter, Lesson, LessonExercise, ChatMessage, UserConfig,
    },
};

/// 数据库连接状态
pub struct DbState(pub Mutex<Database>);

/// 通用错误响应
#[derive(Debug, serde::Serialize)]
pub struct DbCommandError {
    pub code: String,
    pub message: String,
}

impl From<DbError> for DbCommandError {
    fn from(err: DbError) -> Self {
        DbCommandError {
            code: match &err {
                DbError::NotFound(_) => "NOT_FOUND".to_string(),
                DbError::AlreadyExists(_) => "ALREADY_EXISTS".to_string(),
                DbError::NotInitialized => "NOT_INITIALIZED".to_string(),
                _ => "INTERNAL_ERROR".to_string(),
            },
            message: err.to_string(),
        }
    }
}

type DbResult<T> = Result<T, DbCommandError>;

// ==================== Course Commands ====================

/// 创建课程
#[tauri::command]
pub fn create_course_command(
    state: State<DbState>,
    name: String,
    target_level: Option<String>,
    duration: Option<String>,
    teaching_style: Option<String>,
) -> DbResult<Course> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::create_course(&conn, name, target_level, duration, teaching_style)
        .map_err(|e| DbCommandError::from(e))
}

/// 获取所有课程
#[tauri::command]
pub fn get_all_courses_command(state: State<DbState>) -> DbResult<Vec<Course>> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::get_all_courses(&conn).map_err(|e| DbCommandError::from(e))
}

/// 根据 ID 获取课程
#[tauri::command]
pub fn get_course_by_id_command(state: State<DbState>, id: String) -> DbResult<Course> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::get_course_by_id(&conn, &id).map_err(|e| DbCommandError::from(e))
}

/// 更新课程
#[tauri::command]
pub fn update_course_command(
    state: State<DbState>,
    id: String,
    name: Option<String>,
    gitee_repo_url: Option<String>,
    local_path: Option<String>,
    target_level: Option<String>,
    duration: Option<String>,
    teaching_style: Option<String>,
    status: Option<i32>,
) -> DbResult<Course> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::update_course(&conn, &id, name, gitee_repo_url, local_path, target_level, duration, teaching_style, status)
        .map_err(|e| DbCommandError::from(e))
}

/// 删除课程
#[tauri::command]
pub fn delete_course_command(state: State<DbState>, id: String) -> DbResult<()> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::delete_course(&conn, &id).map_err(|e| DbCommandError::from(e))
}

// ==================== Chapter Commands ====================

/// 创建章节
#[tauri::command]
pub fn create_chapter_command(
    state: State<DbState>,
    course_id: String,
    chapter_index: i32,
    name: String,
) -> DbResult<Chapter> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::create_chapter(&conn, course_id, chapter_index, name)
        .map_err(|e| DbCommandError::from(e))
}

/// 获取课程的所有章节
#[tauri::command]
pub fn get_chapters_by_course_command(state: State<DbState>, course_id: String) -> DbResult<Vec<Chapter>> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::get_chapters_by_course(&conn, &course_id).map_err(|e| DbCommandError::from(e))
}

/// 更新章节
#[tauri::command]
pub fn update_chapter_command(
    state: State<DbState>,
    id: String,
    name: Option<String>,
    chapter_index: Option<i32>,
) -> DbResult<Chapter> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::update_chapter(&conn, &id, name, chapter_index)
        .map_err(|e| DbCommandError::from(e))
}

/// 删除章节
#[tauri::command]
pub fn delete_chapter_command(state: State<DbState>, id: String) -> DbResult<()> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::delete_chapter(&conn, &id).map_err(|e| DbCommandError::from(e))
}

// ==================== Lesson Commands ====================

/// 创建课时
#[tauri::command]
pub fn create_lesson_command(
    state: State<DbState>,
    chapter_id: String,
    lesson_index: i32,
    name: String,
    duration: Option<String>,
) -> DbResult<Lesson> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::create_lesson(&conn, chapter_id, lesson_index, name, duration)
        .map_err(|e| DbCommandError::from(e))
}

/// 获取章节的所有课时
#[tauri::command]
pub fn get_lessons_by_chapter_command(state: State<DbState>, chapter_id: String) -> DbResult<Vec<Lesson>> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::get_lessons_by_chapter(&conn, &chapter_id).map_err(|e| DbCommandError::from(e))
}

/// 根据 ID 获取课时
#[tauri::command]
pub fn get_lesson_by_id_command(state: State<DbState>, id: String) -> DbResult<Lesson> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::get_lesson_by_id(&conn, &id).map_err(|e| DbCommandError::from(e))
}

/// 更新课时状态
#[tauri::command]
pub fn update_lesson_status_command(
    state: State<DbState>,
    id: String,
    status: i32,
) -> DbResult<Lesson> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    let lesson_status = LessonStatus::from_i32(status);
    operations::update_lesson_status(&conn, &id, lesson_status)
        .map_err(|e| DbCommandError::from(e))
}

/// 更新课时
#[tauri::command]
pub fn update_lesson_command(
    state: State<DbState>,
    id: String,
    name: Option<String>,
    duration: Option<String>,
    lesson_file: Option<String>,
    status: Option<i32>,
) -> DbResult<Lesson> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::update_lesson(&conn, &id, name, duration, lesson_file, status)
        .map_err(|e| DbCommandError::from(e))
}

/// 删除课时
#[tauri::command]
pub fn delete_lesson_command(state: State<DbState>, id: String) -> DbResult<()> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::delete_lesson(&conn, &id).map_err(|e| DbCommandError::from(e))
}

// ==================== Exercise Commands ====================

/// 创建练习记录
#[tauri::command]
pub fn create_exercise_command(
    state: State<DbState>,
    lesson_id: String,
    exercise_file: Option<String>,
) -> DbResult<LessonExercise> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::create_exercise(&conn, lesson_id, exercise_file)
        .map_err(|e| DbCommandError::from(e))
}

/// 获取课时的所有练习
#[tauri::command]
pub fn get_exercises_by_lesson_command(state: State<DbState>, lesson_id: String) -> DbResult<Vec<LessonExercise>> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::get_exercises_by_lesson(&conn, &lesson_id).map_err(|e| DbCommandError::from(e))
}

/// 更新练习成绩
#[tauri::command]
pub fn update_exercise_score_command(
    state: State<DbState>,
    id: String,
    score: i32,
    result_file: Option<String>,
) -> DbResult<LessonExercise> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::update_exercise_score(&conn, &id, score, result_file)
        .map_err(|e| DbCommandError::from(e))
}

/// 删除练习记录
#[tauri::command]
pub fn delete_exercise_command(state: State<DbState>, id: String) -> DbResult<()> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::delete_exercise(&conn, &id).map_err(|e| DbCommandError::from(e))
}

// ==================== Chat Message Commands ====================

/// 创建聊天消息
#[tauri::command]
pub fn create_chat_message_command(
    state: State<DbState>,
    course_id: String,
    lesson_id: Option<String>,
    agent_type: String,
    role: String,
    content: String,
) -> DbResult<ChatMessage> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    let agent = AgentType::from_str(&agent_type);
    let msg_role = MessageRole::from_str(&role);
    operations::create_chat_message(&conn, course_id, lesson_id, agent, msg_role, content)
        .map_err(|e| DbCommandError::from(e))
}

/// 获取课程的所有聊天消息
#[tauri::command]
pub fn get_chat_messages_by_course_command(state: State<DbState>, course_id: String) -> DbResult<Vec<ChatMessage>> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::get_chat_messages_by_course(&conn, &course_id).map_err(|e| DbCommandError::from(e))
}

/// 获取课时的聊天消息
#[tauri::command]
pub fn get_chat_messages_by_lesson_command(state: State<DbState>, lesson_id: String) -> DbResult<Vec<ChatMessage>> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::get_chat_messages_by_lesson(&conn, &lesson_id).map_err(|e| DbCommandError::from(e))
}

/// 删除聊天消息
#[tauri::command]
pub fn delete_chat_message_command(state: State<DbState>, id: String) -> DbResult<()> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::delete_chat_message(&conn, &id).map_err(|e| DbCommandError::from(e))
}

/// 清空课程的所有聊天消息
#[tauri::command]
pub fn clear_chat_messages_by_course_command(state: State<DbState>, course_id: String) -> DbResult<()> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::clear_chat_messages_by_course(&conn, &course_id).map_err(|e| DbCommandError::from(e))
}

// ==================== User Config Commands ====================

/// 获取用户配置
#[tauri::command]
pub fn get_user_config_command(state: State<DbState>) -> DbResult<UserConfig> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::get_or_create_user_config(&conn).map_err(|e| DbCommandError::from(e))
}

/// 更新用户配置
#[tauri::command]
pub fn update_user_config_command(
    state: State<DbState>,
    gitee_username: Option<String>,
    gitee_token: Option<String>,
    workspace_path: Option<String>,
    ai_provider: Option<String>,
    ai_api_key: Option<String>,
    ai_model: Option<String>,
    git_username: Option<String>,
    git_email: Option<String>,
) -> DbResult<UserConfig> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();
    operations::update_user_config(&conn, gitee_username, gitee_token, workspace_path, ai_provider, ai_api_key, ai_model, git_username, git_email)
        .map_err(|e| DbCommandError::from(e))
}
