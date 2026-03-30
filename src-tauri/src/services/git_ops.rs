//! Git 操作服务
//!
//! 封装 Git 命令行操作，提供仓库管理功能

use std::process::Command;
use thiserror::Error;

/// Git 操作错误
#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git 未安装或不在 PATH 中")]
    NotInstalled,
    #[error("Git 命令执行失败: {0}")]
    CommandFailed(String),
    #[error("仓库操作失败: {0}")]
    RepoError(String),
    #[error("Git 配置错误: {0}")]
    ConfigError(String),
}

/// 检查 Git 是否已安装
pub fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// 获取 Git 版本
pub fn get_git_version() -> Result<String, GitError> {
    let output = Command::new("git")
        .arg("--version")
        .output()
        .map_err(|_| GitError::NotInstalled)?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(GitError::NotInstalled)
    }
}

/// 配置 Git 用户名
pub fn set_git_config_username(username: &str) -> Result<(), GitError> {
    let output = Command::new("git")
        .args(["config", "--global", "user.name", username])
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::ConfigError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 配置 Git 邮箱
pub fn set_git_config_email(email: &str) -> Result<(), GitError> {
    let output = Command::new("git")
        .args(["config", "--global", "user.email", email])
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::ConfigError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 获取 Git 用户名
pub fn get_git_config_username() -> Result<Option<String>, GitError> {
    let output = Command::new("git")
        .args(["config", "--global", "--get", "user.name"])
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if name.is_empty() {
            Ok(None)
        } else {
            Ok(Some(name))
        }
    } else {
        // 配置不存在不是错误
        Ok(None)
    }
}

/// 获取 Git 邮箱
pub fn get_git_config_email() -> Result<Option<String>, GitError> {
    let output = Command::new("git")
        .args(["config", "--global", "--get", "user.email"])
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        let email = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if email.is_empty() {
            Ok(None)
        } else {
            Ok(Some(email))
        }
    } else {
        Ok(None)
    }
}

/// 初始化本地 Git 仓库
pub fn init_repo(path: &str) -> Result<(), GitError> {
    let output = Command::new("git")
        .args(["init"])
        .current_dir(path)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 克隆远程仓库
pub fn clone_repo(url: &str, path: &str) -> Result<(), GitError> {
    let output = Command::new("git")
        .args(["clone", url, path])
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 添加文件到暂存区
pub fn add_files(path: &str, files: &[&str]) -> Result<(), GitError> {
    let mut cmd = Command::new("git");
    cmd.args(["add"]).current_dir(path);

    for file in files {
        cmd.arg(file);
    }

    let output = cmd
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 添加所有文件到暂存区
pub fn add_all(path: &str) -> Result<(), GitError> {
    let output = Command::new("git")
        .args(["add", "-A"])
        .current_dir(path)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 提交更改
pub fn commit(path: &str, message: &str) -> Result<(), GitError> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(path)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 推送更改到远程仓库
pub fn push(path: &str, remote: &str, branch: &str) -> Result<(), GitError> {
    let output = Command::new("git")
        .args(["push", remote, branch])
        .current_dir(path)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 拉取远程仓库更改
pub fn pull(path: &str, remote: &str, branch: &str) -> Result<(), GitError> {
    let output = Command::new("git")
        .args(["pull", remote, branch])
        .current_dir(path)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 获取当前分支名称
pub fn get_current_branch(path: &str) -> Result<String, GitError> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(path)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 检查是否有未提交的更改
pub fn has_changes(path: &str) -> Result<bool, GitError> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(path)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(!output_str.is_empty())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 检查远程仓库是否已配置
pub fn has_remote(path: &str, remote_name: &str) -> Result<bool, GitError> {
    let output = Command::new("git")
        .args(["remote", "show", remote_name])
        .current_dir(path)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    Ok(output.status.success())
}

/// 添加远程仓库
pub fn add_remote(path: &str, name: &str, url: &str) -> Result<(), GitError> {
    let output = Command::new("git")
        .args(["remote", "add", name, url])
        .current_dir(path)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 设置默认分支
pub fn set_default_branch(path: &str, branch: &str) -> Result<(), GitError> {
    let output = Command::new("git")
        .args(["branch", "-M", branch])
        .current_dir(path)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}
