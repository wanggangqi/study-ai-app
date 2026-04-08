//! Git 操作命令
//!
//! 提供 Git 相关的 Tauri 命令

use crate::services::git_ops::{
    is_git_installed, get_git_version, set_git_config_username, set_git_config_email,
    get_git_config_username, get_git_config_email, init_repo, clone_repo,
    add_all, commit, push, pull, has_changes, GitError,
};
use serde::{Deserialize, Serialize};

/// Git 状态结果
#[derive(Debug, Serialize, Deserialize)]
pub struct GitStatus {
    pub installed: bool,
    pub version: Option<String>,
}

/// Git 配置结果
#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    pub username: Option<String>,
    pub email: Option<String>,
}

/// Git 操作结果
#[derive(Debug, Serialize, Deserialize)]
pub struct GitResult {
    pub success: bool,
    pub message: String,
}

/// 检查 Git 安装状态
#[tauri::command]
pub fn check_git_status() -> GitStatus {
    let installed = is_git_installed();
    let version = if installed {
        get_git_version().ok()
    } else {
        None
    };

    GitStatus { installed, version }
}

/// 检查 Git 是否已安装
#[tauri::command]
pub fn check_git_installed() -> bool {
    is_git_installed()
}

/// 获取 Git 配置
#[tauri::command]
pub fn get_git_config() -> GitConfig {
    GitConfig {
        username: get_git_config_username().ok().flatten(),
        email: get_git_config_email().ok().flatten(),
    }
}

/// 设置 Git 用户名
#[tauri::command]
pub fn set_git_username(username: String) -> GitResult {
    match set_git_config_username(&username) {
        Ok(_) => GitResult {
            success: true,
            message: "用户名设置成功".to_string(),
        },
        Err(e) => GitResult {
            success: false,
            message: e.to_string(),
        },
    }
}

/// 设置 Git 邮箱
#[tauri::command]
pub fn set_git_email(email: String) -> GitResult {
    match set_git_config_email(&email) {
        Ok(_) => GitResult {
            success: true,
            message: "邮箱设置成功".to_string(),
        },
        Err(e) => GitResult {
            success: false,
            message: e.to_string(),
        },
    }
}

/// 初始化本地仓库
#[tauri::command]
pub fn git_init(path: String) -> GitResult {
    match init_repo(&path) {
        Ok(_) => GitResult {
            success: true,
            message: "仓库初始化成功".to_string(),
        },
        Err(e) => GitResult {
            success: false,
            message: e.to_string(),
        },
    }
}

/// 克隆远程仓库
#[tauri::command]
pub fn git_clone(url: String, path: String) -> GitResult {
    match clone_repo(&url, &path) {
        Ok(_) => GitResult {
            success: true,
            message: "仓库克隆成功".to_string(),
        },
        Err(e) => GitResult {
            success: false,
            message: e.to_string(),
        },
    }
}

/// Git 提交
#[tauri::command]
pub fn git_commit(path: String, message: String) -> GitResult {
    // 先添加所有文件
    if let Err(e) = add_all(&path) {
        return GitResult {
            success: false,
            message: format!("添加文件失败: {}", e),
        };
    }

    match commit(&path, &message) {
        Ok(_) => GitResult {
            success: true,
            message: "提交成功".to_string(),
        },
        Err(e) => GitResult {
            success: false,
            message: e.to_string(),
        },
    }
}

/// Git 推送
#[tauri::command]
pub fn git_push(path: String) -> GitResult {
    match push(&path, "origin", "main") {
        Ok(_) => GitResult {
            success: true,
            message: "推送成功".to_string(),
        },
        Err(e) => GitResult {
            success: false,
            message: e.to_string(),
        },
    }
}

/// Git 拉取
#[tauri::command]
pub fn git_pull(path: String) -> GitResult {
    match pull(&path, "origin", "main") {
        Ok(_) => GitResult {
            success: true,
            message: "拉取成功".to_string(),
        },
        Err(e) => GitResult {
            success: false,
            message: e.to_string(),
        },
    }
}

/// 检查仓库是否有变更
#[tauri::command]
pub fn git_has_changes(path: String) -> GitResult {
    match has_changes(&path) {
        Ok(has) => GitResult {
            success: true,
            message: if has { "有变更" } else { "无变更" }.to_string(),
        },
        Err(e) => GitResult {
            success: false,
            message: e.to_string(),
        },
    }
}