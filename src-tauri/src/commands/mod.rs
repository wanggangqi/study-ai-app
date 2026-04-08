//! Tauri 命令模块
//!
//! 提供与前端交互的 Tauri 命令

pub mod auth;
pub mod config;
pub mod database;
pub mod git;
pub mod gitee;
pub mod sync;
pub mod ai;

pub use ai::{
    ai_chat_command,
    ai_generate_lesson_command,
    ai_generate_exercise_command,
    ai_analyze_answers_command,
    ai_verify_key_command,
    ai_generate_structured_exercise_command,
    AIChatParams,
    ChatMessageParams,
    AIGenerateLessonParams,
    AIGenerateExerciseParams,
    AIAnalyzeAnswersParams,
    AIVerifyKeyParams,
    AIGenerateStructuredExerciseParams,
    AIStructuredExerciseResult,
    AIResult,
    AIAnalyzeResult,
};

pub use auth::{
    get_machine_id_command,
    get_machine_hash_command,
    validate_license_command,
    get_license_status_command,
    generate_license_key_command,
    is_admin_password_set_command,
    set_admin_password_command,
    verify_admin_password_command,
    change_admin_password_command,
    generate_signing_key_pair_command,
    set_signing_key_command,
    is_signing_key_set_command,
    SigningKeyInfo,
    LicenseResult,
};

pub use database::{
    DbState,
    create_course_command,
    get_all_courses_command,
    get_course_by_id_command,
    update_course_command,
    delete_course_command,
    create_chapter_command,
    get_chapters_by_course_command,
    update_chapter_command,
    delete_chapter_command,
    create_lesson_command,
    get_lessons_by_chapter_command,
    get_lesson_by_id_command,
    update_lesson_status_command,
    update_lesson_command,
    delete_lesson_command,
    create_exercise_command,
    get_exercises_by_lesson_command,
    update_exercise_score_command,
    delete_exercise_command,
    create_chat_message_command,
    get_chat_messages_by_course_command,
    get_chat_messages_by_lesson_command,
    delete_chat_message_command,
    clear_chat_messages_by_course_command,
    get_user_config_command,
    update_user_config_command,
};

pub use git::{
    check_git_status_command,
    check_git_installed_command,
    git_init_command,
    git_clone_command,
    git_commit_command,
    git_push_command,
    git_pull_command,
    git_has_changes_command,
    set_git_username_command,
    set_git_email_command,
    GitStatus,
    GitResult,
};

pub use sync::{
    export_course_plan,
    export_learning_records,
    sync_course_to_git,
    create_course_repository,
    SyncResult,
    SyncError,
    CoursePlan,
    ChapterPlan,
    LessonPlan,
    LearningRecords,
};

pub use gitee::{
    verify_gitee_account_command,
    create_gitee_repo_command,
    check_gitee_repo_exists_command,
    GiteeRepo,
    GiteeError,
    GiteeAccountResult,
    GiteeRepoResult,
};

pub use config::{
    get_config_command,
    set_config_command,
    update_config_command,
};
