//! 授权命令模块
//!
//! 提供 Tauri 授权相关命令

use serde::{Deserialize, Serialize};
use crate::services::{
    machine_id::{get_machine_id, get_machine_hash},
    license::{validate_license, get_license_status, LicenseError},
};

/// 授权结果（简化版，供前端使用）
#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseResult {
    pub is_licensed: bool,
    pub expire_at: Option<String>,
    pub remaining_days: Option<i64>,
    pub error_message: Option<String>,
}

/// 获取当前机器码
///
/// 返回当前计算机的唯一机器码（SHA256 哈希）
#[tauri::command]
pub fn get_machine_id_command() -> Result<String, String> {
    get_machine_id().map_err(|e| e.to_string())
}

/// 获取机器码哈希（十六进制格式）
#[tauri::command]
pub fn get_machine_hash_command() -> Result<String, String> {
    get_machine_hash().map_err(|e| e.to_string())
}

/// 验证授权密钥
///
/// # Arguments
/// * `license_key` - 用户输入的授权密钥
///
/// # Returns
/// 授权结果，包含授权状态信息
#[tauri::command]
pub fn validate_license_command(license_key: String) -> Result<LicenseResult, String> {
    match validate_license(&license_key) {
        Ok(status) => Ok(LicenseResult {
            is_licensed: true,
            expire_at: status.expire_at,
            remaining_days: status.remaining_days,
            error_message: None,
        }),
        Err(e) => {
            let message = match &e {
                LicenseError::MachineMismatch => "机器码不匹配，该密钥不适用于当前计算机".to_string(),
                LicenseError::Expired(date) => format!("授权已过期，到期日期：{}", date),
                LicenseError::InvalidFormat => "密钥格式无效".to_string(),
                LicenseError::InvalidSignature => "密钥签名验证失败".to_string(),
                LicenseError::NotFound => "未找到授权文件".to_string(),
                _ => format!("验证失败：{}", e),
            };
            Ok(LicenseResult {
                is_licensed: false,
                expire_at: None,
                remaining_days: None,
                error_message: Some(message),
            })
        }
    }
}

/// 获取当前授权状态
#[tauri::command]
pub fn get_license_status_command() -> Result<LicenseResult, String> {
    match get_license_status() {
        Ok(status) => Ok(LicenseResult {
            is_licensed: status.is_licensed,
            expire_at: status.expire_at,
            remaining_days: status.remaining_days,
            error_message: status.error_message,
        }),
        Err(e) => Ok(LicenseResult {
            is_licensed: false,
            expire_at: None,
            remaining_days: None,
            error_message: Some(format!("获取授权状态失败：{}", e)),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_machine_id() {
        let result = get_machine_id_command();
        assert!(result.is_ok());
        println!("Machine ID: {}", result.unwrap());
    }

    #[test]
    fn test_get_license_status() {
        let result = get_license_status_command();
        assert!(result.is_ok());
        let status = result.unwrap();
        println!("License status: is_licensed={}, message={:?}", status.is_licensed, status.error_message);
    }
}