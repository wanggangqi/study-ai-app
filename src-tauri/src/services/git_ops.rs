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

/// 常见 Git 安装路径 (Windows)
const GIT_PATHS: &[&str] = &[
    r"C:\Program Files\Git\cmd\git.exe",
    r"C:\Program Files (x86)\Git\cmd\git.exe",
    r"C:\Git\cmd\git.exe",
];

/// 查找 Git 可执行文件路径
fn find_git_executable() -> Option<String> {
    for git_path in GIT_PATHS {
        if std::path::Path::new(git_path).exists() {
            return Some(git_path.to_string());
        }
    }
    None
}

/// 执行 Git 命令（内部使用）
fn run_git_command_internal(args: &[&str], path: Option<&str>) -> Result<std::process::Output, GitError> {
    // 先尝试直接调用 git
    let mut cmd = Command::new("git");
    cmd.args(args);
    if let Some(p) = path {
        cmd.current_dir(p);
    }

    if let Ok(output) = cmd.output() {
        if output.status.success() {
            return Ok(output);
        }
    }

    // 尝试使用完整路径
    if let Some(git_path) = find_git_executable() {
        let mut cmd = Command::new(&git_path);
        cmd.args(args);
        if let Some(p) = path {
            cmd.current_dir(p);
        }
        if let Ok(output) = cmd.output() {
            if output.status.success() {
                return Ok(output);
            }
        }
    }

    Err(GitError::NotInstalled)
}

/// 检查 Git 是否已安装
pub fn is_git_installed() -> bool {
    // 先尝试直接调用 git
    if Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
    {
        return true;
    }

    // 尝试使用完整路径
    if let Some(git_path) = find_git_executable() {
        if Command::new(&git_path)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
        {
            return true;
        }
    }

    false
}

/// 获取 Git 版本
pub fn get_git_version() -> Result<String, GitError> {
    let output = run_git_command_internal(&["--version"], None)?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// 初始化本地 Git 仓库
pub fn init_repo(path: &str) -> Result<(), GitError> {
    let output = run_git_command_internal(&["init"], Some(path))?;
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
    let output = run_git_command_internal(&["clone", url, path], None)?;
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
    let mut args = vec!["add"];
    for file in files {
        args.push(file);
    }
    let output = run_git_command_internal(&args, Some(path))?;
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
    let output = run_git_command_internal(&["add", "-A"], Some(path))?;
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
    let output = run_git_command_internal(&["commit", "-m", message], Some(path))?;
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
    let output = run_git_command_internal(&["push", remote, branch], Some(path))?;
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
    let output = run_git_command_internal(&["pull", remote, branch], Some(path))?;
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
    let output = run_git_command_internal(&["rev-parse", "--abbrev-ref", "HEAD"], Some(path))?;
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
    let output = run_git_command_internal(&["status", "--porcelain"], Some(path))?;
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
    let output = run_git_command_internal(&["remote", "show", remote_name], Some(path))?;
    Ok(output.status.success())
}

/// 添加远程仓库
pub fn add_remote(path: &str, name: &str, url: &str) -> Result<(), GitError> {
    let output = run_git_command_internal(&["remote", "add", name, url], Some(path))?;
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
    let output = run_git_command_internal(&["branch", "-M", branch], Some(path))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(GitError::RepoError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ))
    }
}

/// 设置仓库级别的 Git 用户名和邮箱（在初始化仓库后调用）
pub fn set_repo_git_config(path: &str, username: &str, email: &str) -> Result<(), GitError> {
    // 设置用户名的
    let output = run_git_command_internal(&["config", "user.name", username], Some(path))?;
    if !output.status.success() {
        return Err(GitError::ConfigError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }

    // 设置邮箱
    let output = run_git_command_internal(&["config", "user.email", email], Some(path))?;
    if !output.status.success() {
        return Err(GitError::ConfigError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }

    Ok(())
}
