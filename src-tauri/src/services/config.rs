//! 应用配置服务
//!
//! 管理应用配置文件的读写

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config: {0}")]
    ReadError(String),

    #[error("Failed to write config: {0}")]
    WriteError(String),

    #[error("Config not found")]
    NotFound,
}

/// 应用配置数据结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// 是否已完成初始设置
    pub setup_completed: bool,
    /// 码云用户名
    pub gitee_username: Option<String>,
    /// 码云访问令牌
    pub gitee_token: Option<String>,
    /// 本地工作空间路径
    pub workspace_path: Option<String>,
    /// AI 服务商
    pub ai_provider: Option<String>,
    /// AI API 密钥
    pub ai_api_key: Option<String>,
    /// AI 模型
    pub ai_model: Option<String>,
    /// Git 用户名
    pub git_username: Option<String>,
    /// Git 邮箱
    pub git_email: Option<String>,
    /// 教学风格
    pub teaching_style: Option<String>,
}

/// 获取配置文件路径
fn get_config_file_path() -> PathBuf {
    let app_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("StudyMate");

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).ok();
    }

    app_dir.join("config.json")
}

/// 加载配置文件
pub fn load_config() -> Result<AppConfig, ConfigError> {
    let file_path = get_config_file_path();

    if !file_path.exists() {
        return Ok(AppConfig::default());
    }

    let content = fs::read_to_string(&file_path)
        .map_err(|e| ConfigError::ReadError(e.to_string()))?;

    let config: AppConfig = serde_json::from_str(&content)
        .map_err(|e| ConfigError::ReadError(e.to_string()))?;

    Ok(config)
}

/// 保存配置文件
pub fn save_config(config: &AppConfig) -> Result<(), ConfigError> {
    let file_path = get_config_file_path();

    let content = serde_json::to_string_pretty(config)
        .map_err(|e| ConfigError::WriteError(e.to_string()))?;

    fs::write(&file_path, content)
        .map_err(|e| ConfigError::WriteError(e.to_string()))?;

    Ok(())
}

/// 更新配置（部分更新）
pub fn update_config(updates: AppConfig) -> Result<AppConfig, ConfigError> {
    let mut config = load_config()?;

    if let Some(v) = updates.gitee_username {
        config.gitee_username = Some(v);
    }
    if let Some(v) = updates.gitee_token {
        config.gitee_token = Some(v);
    }
    if let Some(v) = updates.workspace_path {
        config.workspace_path = Some(v);
    }
    if let Some(v) = updates.ai_provider {
        config.ai_provider = Some(v);
    }
    if let Some(v) = updates.ai_api_key {
        config.ai_api_key = Some(v);
    }
    if let Some(v) = updates.ai_model {
        config.ai_model = Some(v);
    }
    if let Some(v) = updates.git_username {
        config.git_username = Some(v);
    }
    if let Some(v) = updates.git_email {
        config.git_email = Some(v);
    }
    if let Some(v) = updates.teaching_style {
        config.teaching_style = Some(v);
    }
    config.setup_completed = updates.setup_completed;

    save_config(&config)?;
    Ok(config)
}
