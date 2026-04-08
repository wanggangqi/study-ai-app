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
    is_admin_password_set_command,
    set_admin_password_command,
    verify_admin_password_command,
    change_admin_password_command,
    generate_license_key_command,
    generate_signing_key_pair_command,
    set_signing_key_command,
    is_signing_key_set_command,
    // 配置命令
    get_config_command,
    set_config_command,
    update_config_command,
    // 数据库状态
    database::DbState,
    // Course 操作
    create_course_command,
    get_all_courses_command,
    get_course_by_id_command,
    update_course_command,
    delete_course_command,
    // Chapter 操作
    create_chapter_command,
    get_chapters_by_course_command,
    update_chapter_command,
    delete_chapter_command,
    // Lesson 操作
    create_lesson_command,
    get_lessons_by_chapter_command,
    get_lesson_by_id_command,
    update_lesson_status_command,
    update_lesson_command,
    delete_lesson_command,
    // Exercise 操作
    create_exercise_command,
    get_exercises_by_lesson_command,
    update_exercise_score_command,
    delete_exercise_command,
    // ChatMessage 操作
    create_chat_message_command,
    get_chat_messages_by_course_command,
    get_chat_messages_by_lesson_command,
    delete_chat_message_command,
    clear_chat_messages_by_course_command,
    // UserConfig 操作
    get_user_config_command,
    update_user_config_command,
    // Git 命令
    check_git_installed_command,
    check_git_status_command,
    git_init_command,
    git_clone_command,
    git_commit_command,
    git_push_command,
    git_pull_command,
    git_has_changes_command,
    // Gitee 命令
    verify_gitee_account_command,
    create_gitee_repo_command,
    check_gitee_repo_exists_command,
    // AI 命令
    ai_chat_command,
    ai_generate_lesson_command,
    ai_generate_exercise_command,
    ai_analyze_answers_command,
    ai_verify_key_command,
    ai_generate_structured_exercise_command,
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
        .plugin(tauri_plugin_dialog::init())
        .manage(DbState(Mutex::new(database)))
        .invoke_handler(tauri::generate_handler![
            // 通用命令
            greet,
            // 授权相关命令
            get_machine_id_command,
            get_machine_hash_command,
            validate_license_command,
            get_license_status_command,
            is_admin_password_set_command,
            set_admin_password_command,
            verify_admin_password_command,
            change_admin_password_command,
            generate_license_key_command,
            generate_signing_key_pair_command,
            set_signing_key_command,
            is_signing_key_set_command,
            // 配置命令
            get_config_command,
            set_config_command,
            update_config_command,
            // 数据库相关命令 - Course
            create_course_command,
            get_all_courses_command,
            get_course_by_id_command,
            update_course_command,
            delete_course_command,
            // 数据库相关命令 - Chapter
            create_chapter_command,
            get_chapters_by_course_command,
            update_chapter_command,
            delete_chapter_command,
            // 数据库相关命令 - Lesson
            create_lesson_command,
            get_lessons_by_chapter_command,
            get_lesson_by_id_command,
            update_lesson_status_command,
            update_lesson_command,
            delete_lesson_command,
            // 数据库相关命令 - Exercise
            create_exercise_command,
            get_exercises_by_lesson_command,
            update_exercise_score_command,
            delete_exercise_command,
            // 数据库相关命令 - ChatMessage
            create_chat_message_command,
            get_chat_messages_by_course_command,
            get_chat_messages_by_lesson_command,
            delete_chat_message_command,
            clear_chat_messages_by_course_command,
            // 数据库相关命令 - UserConfig
            get_user_config_command,
            update_user_config_command,
            // Git 相关命令
            check_git_installed_command,
            check_git_status_command,
            git_init_command,
            git_clone_command,
            git_commit_command,
            git_push_command,
            git_pull_command,
            git_has_changes_command,
            // Gitee 相关命令
            verify_gitee_account_command,
            create_gitee_repo_command,
            check_gitee_repo_exists_command,
            // AI 相关命令
            ai_chat_command,
            ai_generate_lesson_command,
            ai_generate_exercise_command,
            ai_analyze_answers_command,
            ai_verify_key_command,
            ai_generate_structured_exercise_command,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("智学伴侣 - AI 一对一教学").ok();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("启动应用时发生错误");
}