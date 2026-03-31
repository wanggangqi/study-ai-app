//! 智学伴侣 (StudyMate) - Rust 后端库
//!
//! AI 一对一教学应用的 Tauri 后端入口

use std::sync::Mutex;
use tauri::Manager;

pub mod commands;
pub mod db;
pub mod services;

use commands::{
    // 授权相关命令
    get_machine_id_command,
    get_machine_hash_command,
    validate_license_command,
    get_license_status_command,
    // 数据库状态
    database::DbState,
    // Course 操作
    create_course,
    get_all_courses,
    get_course_by_id,
    update_course,
    delete_course,
    // Chapter 操作
    create_chapter,
    get_chapters_by_course,
    update_chapter,
    delete_chapter,
    // Lesson 操作
    create_lesson,
    get_lessons_by_chapter,
    get_lesson_by_id,
    update_lesson_status,
    update_lesson,
    delete_lesson,
    // Exercise 操作
    create_exercise,
    get_exercises_by_lesson,
    update_exercise_score,
    delete_exercise,
    // ChatMessage 操作
    create_chat_message,
    get_chat_messages_by_course,
    get_chat_messages_by_lesson,
    delete_chat_message,
    clear_chat_messages_by_course,
    // UserConfig 操作
    get_user_config,
    update_user_config,
    // Git 命令
    check_git_status,
    get_git_config,
    set_git_username,
    set_git_email,
    git_init,
    git_clone,
    git_commit,
    git_push,
    git_pull,
    git_has_changes,
    // Gitee 命令
    verify_gitee_account,
    create_gitee_repo,
    check_gitee_repo_exists,
    // AI 命令
    ai_chat_command,
    ai_generate_lesson_command,
    ai_generate_exercise_command,
    ai_analyze_answers_command,
    ai_verify_key_command,
};
use db::Database;

/// 通用 greeting 命令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好，{}！欢迎使用智学伴侣！", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化数据库
    let db_path = Database::get_default_path();
    let database = match Database::new(db_path.clone()) {
        Ok(db) => {
            println!("数据库初始化成功: {:?}", db_path);
            db
        }
        Err(e) => {
            eprintln!("数据库初始化失败: {}", e);
            panic!("无法启动应用：数据库初始化失败");
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(DbState(Mutex::new(database)))
        .invoke_handler(tauri::generate_handler![
            // 通用命令
            greet,
            // 授权相关命令
            get_machine_id_command,
            get_machine_hash_command,
            validate_license_command,
            get_license_status_command,
            // 数据库相关命令 - Course
            create_course,
            get_all_courses,
            get_course_by_id,
            update_course,
            delete_course,
            // 数据库相关命令 - Chapter
            create_chapter,
            get_chapters_by_course,
            update_chapter,
            delete_chapter,
            // 数据库相关命令 - Lesson
            create_lesson,
            get_lessons_by_chapter,
            get_lesson_by_id,
            update_lesson_status,
            update_lesson,
            delete_lesson,
            // 数据库相关命令 - Exercise
            create_exercise,
            get_exercises_by_lesson,
            update_exercise_score,
            delete_exercise,
            // 数据库相关命令 - ChatMessage
            create_chat_message,
            get_chat_messages_by_course,
            get_chat_messages_by_lesson,
            delete_chat_message,
            clear_chat_messages_by_course,
            // 数据库相关命令 - UserConfig
            get_user_config,
            update_user_config,
            // Git 相关命令
            check_git_status,
            get_git_config,
            set_git_username,
            set_git_email,
            git_init,
            git_clone,
            git_commit,
            git_push,
            git_pull,
            git_has_changes,
            // Gitee 相关命令
            verify_gitee_account,
            create_gitee_repo,
            check_gitee_repo_exists,
            // AI 相关命令
            ai_chat_command,
            ai_generate_lesson_command,
            ai_generate_exercise_command,
            ai_analyze_answers_command,
            ai_verify_key_command,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("智学伴侣 - AI 一对一教学").ok();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("启动应用时发生错误");
}