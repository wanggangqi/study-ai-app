//! 授权命令模块
//!
//! 提供 Tauri 授权相关命令

use serde::{Deserialize, Serialize};
use crate::services::{
    machine_id::{get_machine_id, get_machine_hash},
    license::{validate_license, get_license_status, LicenseError,
        get_signing_key},
    crypto::{generate_keypair, signing_key_to_base64, verify_key_to_base64},
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

/// 为指定机器码生成授权密钥
///
/// # Arguments
/// * `expire_date` - 过期日期，格式：YYYY-MM-DD
/// * `machine_hash` - 目标机器的机器码哈希（可选，为空则使用当前机器）
///
/// # Returns
/// 生成的授权密钥
#[tauri::command]
pub fn generate_license_key_command(expire_date: String, machine_hash: Option<String>) -> Result<String, String> {
    use crate::services::machine_id::get_machine_hash;

    // 如果提供了机器码哈希，直接使用；否则获取当前机器的
    let hash = match machine_hash {
        Some(h) => h,
        None => get_machine_hash().map_err(|e| e.to_string())?,
    };

    // 构建临时的 LicenseData
    use crate::services::crypto::sign_data;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct LicenseData {
        machine_hash: String,
        expire_at: String,
    }

    let license_data = LicenseData {
        machine_hash: hash,
        expire_at: expire_date,
    };

    let json = serde_json::to_vec(&license_data)
        .map_err(|e| format!("序列化失败：{}", e))?;

    // 使用 Ed25519 签名
    let signing_key = get_signing_key()
        .map_err(|e| format!("获取签名密钥失败：{}", e))?
        .ok_or_else(|| "未找到签名密钥".to_string())?;

    let signed = sign_data(&json, &signing_key)
        .map_err(|e| format!("签名失败：{}", e))?;

    Ok(signed)
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

/// 签名密钥信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SigningKeyInfo {
    /// 私钥（Base64格式，管理员需要妥善保管）
    pub signing_key: String,
    /// 公钥（Base64格式，用于验证签名）
    pub verify_key: String,
}

/// 生成新的签名密钥对
///
/// 返回私钥和公钥，私钥由管理员保管，公钥用于验证
#[tauri::command]
pub fn generate_signing_key_pair_command() -> Result<SigningKeyInfo, String> {
    let (signing_key, verify_key) = generate_keypair();

    Ok(SigningKeyInfo {
        signing_key: signing_key_to_base64(&signing_key),
        verify_key: verify_key_to_base64(&verify_key),
    })
}

/// 设置签名私钥（首次设置后不可更改）
#[tauri::command]
pub fn set_signing_key_command(signing_key: String) -> Result<(), String> {
    crate::services::license::set_signing_key(&signing_key).map_err(|e| e.to_string())
}

/// 检查签名密钥是否已设置
#[tauri::command]
pub fn is_signing_key_set_command() -> Result<bool, String> {
    Ok(crate::services::license::is_signing_key_set())
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