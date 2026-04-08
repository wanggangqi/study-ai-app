//! 配置命令模块
//!
//! 提供应用配置相关的 Tauri 命令

use crate::services::config::{AppConfig, load_config, save_config, update_config};

/// 加载应用配置
#[tauri::command]
pub fn get_config_command() -> Result<AppConfig, String> {
    load_config().map_err(|e| e.to_string())
}

/// 保存应用配置
#[tauri::command]
pub fn set_config_command(config: AppConfig) -> Result<(), String> {
    save_config(&config).map_err(|e| e.to_string())
}

/// 更新应用配置（部分更新）
#[tauri::command]
pub fn update_config_command(config: AppConfig) -> Result<AppConfig, String> {
    update_config(config).map_err(|e| e.to_string())
}
