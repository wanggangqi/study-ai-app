//! Tauri 命令模块
//!
//! 提供与前端交互的 Tauri 命令

pub mod auth;
pub mod database;
pub mod git;
pub mod gitee;
pub mod sync;
pub mod ai;

pub use auth::{
    get_machine_id_command,
    get_machine_hash_command,
    validate_license_command,
    get_license_status_command,
    LicenseResult,
};

pub use database::{
    DbState,
    create_course,
    get_all_courses,
    get_course_by_id,
    update_course,
    delete_course,
    create_chapter,
    get_chapters_by_course,
    update_chapter,
    delete_chapter,
    create_lesson,
    get_lessons_by_chapter,
    get_lesson_by_id,
    update_lesson_status,
    update_lesson,
    delete_lesson,
    create_exercise,
    get_exercises_by_lesson,
    update_exercise_score,
    delete_exercise,
    create_chat_message,
    get_chat_messages_by_course,
    get_chat_messages_by_lesson,
    delete_chat_message,
    clear_chat_messages_by_course,
    get_user_config,
    update_user_config,
};

pub use git::{
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
    GitStatus,
    GitConfig,
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
    verify_gitee_account,
    create_gitee_repo,
    check_gitee_repo_exists,
    GiteeRepo,
    GiteeError,
    GiteeAccountResult,
    GiteeRepoResult,
};

pub use ai::{
    ai_chat_command,
    ai_generate_lesson_command,
    ai_generate_exercise_command,
    ai_analyze_answers_command,
    ai_verify_key_command,
    AIChatParams,
    ChatMessageParams,
    AIGenerateLessonParams,
    AIGenerateExerciseParams,
    AIAnalyzeAnswersParams,
    AIVerifyKeyParams,
    AIResult,
    AIAnalyzeResult,
};
