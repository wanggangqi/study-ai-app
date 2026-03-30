//! 业务服务模块
//!
//! 提供授权系统、加密等核心业务服务

pub mod machine_id;
pub mod crypto;
pub mod license;
pub mod git_ops;

pub use machine_id::{get_machine_id, get_machine_hash, MachineIdError};
pub use crypto::{encrypt_data, decrypt_data, CryptoError};
pub use license::{validate_license, get_license_status, LicenseStatus, LicenseError};
pub use git_ops::{
    is_git_installed,
    get_git_version,
    set_git_config_username,
    set_git_config_email,
    get_git_config_username,
    get_git_config_email,
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
    GitError,
};
