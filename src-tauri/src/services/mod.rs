//! 业务服务模块
//!
//! 提供授权系统、加密等核心业务服务

pub mod machine_id;
pub mod crypto;
pub mod default_keys;
pub mod license;
pub mod git_ops;
pub mod ai;
pub mod config;

pub use machine_id::{get_machine_id, get_machine_hash, MachineIdError};
pub use crypto::{
    encrypt_data, decrypt_data, CryptoError,
    sign_data, verify_signature as crypto_verify_signature, generate_keypair,
    signing_key_to_base64, verify_key_to_base64,
    signing_key_from_base64, verify_key_from_base64,
};
pub use default_keys::{get_default_signing_key, get_default_verify_key};
pub use license::{
    validate_license, get_license_status, LicenseStatus, LicenseError,
    is_admin_password_set, set_admin_password, verify_admin_password, change_admin_password,
    get_signing_key,
};
pub use config::{AppConfig, load_config, save_config, update_config, ConfigError};
pub use git_ops::{
    is_git_installed,
    get_git_version,
    init_repo,
    clone_repo,
    add_files,
    add_all,
    commit,
    push,
    pull,
    get_current_branch,
    has_changes,
    has_remote,
    add_remote,
    set_default_branch,
    set_repo_git_config,
    GitError,
};
pub use ai::{
    AIProvider,
    AIConfig,
    AIError,
    ChatMessage,
    AnalyzeResult,
    ExerciseOption,
    StructuredExercise,
    chat,
    generate_lesson,
    generate_exercise,
    analyze_answers,
    verify_api_key,
    generate_structured_exercise,
};
