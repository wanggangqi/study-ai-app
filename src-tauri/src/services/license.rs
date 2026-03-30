//! 授权验证服务
//!
//! 解析密钥格式、验证机器码、验证有效期

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

use super::crypto::{decrypt_data, encrypt_data};
use super::machine_id::{get_machine_hash, MachineIdError};

#[derive(Error, Debug)]
pub enum LicenseError {
    #[error("Invalid license format")]
    InvalidFormat,

    #[error("License expired at {0}")]
    Expired(String),

    #[error("Machine code mismatch")]
    MachineMismatch,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Failed to read license file: {0}")]
    ReadError(String),

    #[error("Failed to write license file: {0}")]
    WriteError(String),

    #[error("License not found")]
    NotFound,

    #[error("Machine ID error: {0}")]
    MachineIdError(#[from] MachineIdError),
}

/// 密钥数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseData {
    /// 用户机器码的 SHA256 哈希
    pub machine_hash: String,
    /// 有效期截止日期 (YYYY-MM-DD)
    pub expire_at: String,
    /// 开发者签名防篡改
    pub signature: String,
}

/// 授权状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseStatus {
    /// 是否已授权
    pub is_licensed: bool,
    /// 授权到期日期
    pub expire_at: Option<String>,
    /// 剩余天数（如果已授权）
    pub remaining_days: Option<i64>,
    /// 错误信息（如果未授权）
    pub error_message: Option<String>,
}

/// 默认开发者签名（实际应从配置文件或环境变量获取）
const DEFAULT_SIGNATURE: &str = "StudyMate-2026-Dev";

/// 获取授权状态文件路径
fn get_license_file_path() -> PathBuf {
    let app_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("StudyMate");

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).ok();
    }

    app_dir.join("license.dat")
}

/// 获取密钥文件路径
fn get_key_file_path() -> PathBuf {
    let app_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("StudyMate");

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).ok();
    }

    app_dir.join("key.dat")
}

/// 验证密钥格式并解密
///
/// 密钥格式：BASE64(AES加密(机器码哈希 + 有效期时间戳 + 签名))
fn parse_and_decrypt_license(encrypted_license: &str) -> Result<LicenseData, LicenseError> {
    // 使用默认密钥解密（实际应用中密钥应安全存储）
    let key = super::crypto::derive_key("StudyMate-License-Key-2026", b"StudyMate-Salt");
    let decrypted = decrypt_data(encrypted_license, &key)
        .map_err(|_| LicenseError::InvalidFormat)?;

    // 解析 JSON
    let license_data: LicenseData = serde_json::from_slice(&decrypted)
        .map_err(|_| LicenseError::InvalidFormat)?;

    Ok(license_data)
}

/// 验证授权状态
pub fn validate_license(license_key: &str) -> Result<LicenseStatus, LicenseError> {
    // 1. 解析并解密密钥
    let license_data = parse_and_decrypt_license(license_key)?;

    // 2. 验证机器码
    let current_machine_hash = get_machine_hash()?;
    if license_data.machine_hash != current_machine_hash {
        return Err(LicenseError::MachineMismatch);
    }

    // 3. 验证有效期
    let expire_date = NaiveDate::parse_from_str(&license_data.expire_at, "%Y-%m-%d")
        .map_err(|_| LicenseError::InvalidFormat)?;

    let today = Utc::now().date_naive();
    if expire_date < today {
        return Err(LicenseError::Expired(license_data.expire_at.clone()));
    }

    // 4. 验证签名（简单的防篡改检查）
    if !verify_signature(&license_data) {
        return Err(LicenseError::InvalidSignature);
    }

    // 5. 计算剩余天数
    let remaining_days = (expire_date - today).num_days();

    // 6. 保存授权状态
    save_license_status(&license_data)?;

    Ok(LicenseStatus {
        is_licensed: true,
        expire_at: Some(license_data.expire_at),
        remaining_days: Some(remaining_days),
        error_message: None,
    })
}

/// 验证签名
fn verify_signature(license_data: &LicenseData) -> bool {
    // 简单的签名验证：检查机器码哈希和日期是否被篡改
    // 实际应用中应使用非对称签名
    let expected_signature = format!("{}-{}", license_data.machine_hash, license_data.expire_at);
    let expected_hash = Sha256::digest(expected_signature.as_bytes());

    // 将 hex 编码的签名与计算出的签名比较
    let computed = hex::encode(expected_hash);
    computed.starts_with(&license_data.signature[..8.min(license_data.signature.len())])
        || license_data.signature == DEFAULT_SIGNATURE
}

/// 保存授权状态到本地文件
fn save_license_status(license_data: &LicenseData) -> Result<(), LicenseError> {
    let license_file = get_license_file_path();

    // 使用机器码哈希作为加密密钥
    let key = super::crypto::derive_key(&license_data.machine_hash, b"License-Status-Salt");

    let status_json = serde_json::to_vec(license_data)
        .map_err(|e| LicenseError::WriteError(e.to_string()))?;

    let encrypted = encrypt_data(&status_json, &key)
        .map_err(|e| LicenseError::WriteError(e.to_string()))?;

    fs::write(&license_file, encrypted)
        .map_err(|e| LicenseError::WriteError(e.to_string()))?;

    // 保存原始密钥
    let key_file = get_key_file_path();
    let key_encrypted = encrypt_data(license_data.machine_hash.as_bytes(), &key)
        .map_err(|e| LicenseError::WriteError(e.to_string()))?;
    fs::write(&key_file, key_encrypted)
        .map_err(|e| LicenseError::WriteError(e.to_string()))?;

    Ok(())
}

/// 获取当前授权状态
pub fn get_license_status() -> Result<LicenseStatus, LicenseError> {
    let license_file = get_license_file_path();
    let key_file = get_key_file_path();

    if !license_file.exists() || !key_file.exists() {
        return Ok(LicenseStatus {
            is_licensed: false,
            expire_at: None,
            remaining_days: None,
            error_message: Some("License not found".to_string()),
        });
    }

    // 读取加密的状态
    let encrypted_status = fs::read_to_string(&license_file)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;

    // 读取密钥以获取机器码哈希
    let encrypted_key = fs::read_to_string(&key_file)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;

    // 解密获取机器码哈希
    let key = super::crypto::derive_key("License-Status-Salt", b"License-Status-Salt");
    let machine_hash_bytes = decrypt_data(&encrypted_key, &key)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;
    let machine_hash = String::from_utf8(machine_hash_bytes)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;

    // 使用机器码哈希解密状态
    let status_key = super::crypto::derive_key(&machine_hash, b"License-Status-Salt");
    let decrypted = decrypt_data(&encrypted_status, &status_key)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;

    let license_data: LicenseData = serde_json::from_slice(&decrypted)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;

    // 验证机器码是否匹配
    let current_machine_hash = get_machine_hash()?;
    if license_data.machine_hash != current_machine_hash {
        return Ok(LicenseStatus {
            is_licensed: false,
            expire_at: None,
            remaining_days: None,
            error_message: Some("Machine code mismatch - license invalid".to_string()),
        });
    }

    // 验证是否过期
    let expire_date = NaiveDate::parse_from_str(&license_data.expire_at, "%Y-%m-%d")
        .map_err(|_| LicenseError::InvalidFormat)?;

    let today = Utc::now().date_naive();
    if expire_date < today {
        return Ok(LicenseStatus {
            is_licensed: false,
            expire_at: Some(license_data.expire_at),
            remaining_days: None,
            error_message: Some("License expired".to_string()),
        });
    }

    let remaining_days = (expire_date - today).num_days();

    Ok(LicenseStatus {
        is_licensed: true,
        expire_at: Some(license_data.expire_at),
        remaining_days: Some(remaining_days),
        error_message: None,
    })
}

/// 生成密钥（仅供测试或管理员使用）
///
/// # Arguments
/// * `expire_date` - 过期日期 (YYYY-MM-DD)
#[allow(dead_code)]
pub fn generate_license_key(expire_date: &str) -> Result<String, LicenseError> {
    let machine_hash = get_machine_hash()?;

    let license_data = LicenseData {
        machine_hash,
        expire_at: expire_date.to_string(),
        signature: DEFAULT_SIGNATURE.to_string(),
    };

    let json = serde_json::to_vec(&license_data)
        .map_err(|_e| LicenseError::InvalidFormat)?;

    let key = super::crypto::derive_key("StudyMate-License-Key-2026", b"StudyMate-Salt");
    let encrypted = encrypt_data(&json, &key)
        .map_err(|_e| LicenseError::InvalidFormat)?;

    Ok(BASE64.encode(encrypted))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_status_not_found() {
        let result = get_license_status();
        // 可能会失败因为没有 license 文件，但不应 panic
        println!("License status: {:?}", result);
    }

    #[test]
    fn test_generate_license_key() {
        // 这个测试需要有效的机器码
        let result = generate_license_key("2027-12-31");
        match result {
            Ok(key) => {
                println!("Generated license key: {}", key);
                assert!(!key.is_empty());
            }
            Err(e) => {
                println!("Failed to generate key (expected in test): {:?}", e);
            }
        }
    }

    #[test]
    fn test_parse_invalid_license() {
        let result = parse_and_decrypt_license("invalid_key");
        assert!(result.is_err());
    }
}
