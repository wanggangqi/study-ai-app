//! 授权验证服务
//!
//! 解析密钥格式、验证机器码、验证有效期，使用 Ed25519 非对称签名

use chrono::{Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

use super::crypto::{decrypt_data, encrypt_data, sign_data, verify_signature as crypto_verify_signature,
    signing_key_from_base64, signing_key_to_base64};
use super::default_keys;
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


/// 获取授权状态文件路径
fn get_license_file_path() -> PathBuf {
    get_local_data_dir().join("license.dat")
}

/// 获取密钥文件路径
fn get_key_file_path() -> PathBuf {
    get_local_data_dir().join("key.dat")
}

/// 获取 localData 目录路径
fn get_local_data_dir() -> std::path::PathBuf {
    std::env::var("LOCALAPPDATA")
        .map(|p| std::path::Path::new(&p).join("com.studymate.app").join("localData"))
        .unwrap_or_else(|_| {
            dirs::data_local_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("StudyMate")
        })
}

/// 获取当前有效的验签公钥
fn get_verify_key() -> Result<super::crypto::VerifyingKey, LicenseError> {
    default_keys::get_default_verify_key()
        .map_err(|_| LicenseError::NotFound)
}

/// 获取当前有效的签名私钥（从 localData 目录或项目根目录）
pub fn get_signing_key() -> Result<Option<super::crypto::SigningKey>, LicenseError> {
    // 优先检查 localData 目录
    let local_data_key = get_local_data_dir().join("signing_key.pem");
    if local_data_key.exists() {
        let content = fs::read_to_string(&local_data_key)
            .map_err(|e| LicenseError::ReadError(e.to_string()))?;
        let key = signing_key_from_base64(content.trim())
            .map_err(|_| LicenseError::InvalidFormat)?;
        return Ok(Some(key));
    }

    // 检查项目根目录（默认私钥文件）
    let project_key = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("signing_key.pem");
    if project_key.exists() {
        let content = fs::read_to_string(&project_key)
            .map_err(|e| LicenseError::ReadError(e.to_string()))?;
        let key = signing_key_from_base64(content.trim())
            .map_err(|_| LicenseError::InvalidFormat)?;
        return Ok(Some(key));
    }

    Ok(None)
}

/// 设置签名私钥（保存到 localData 目录）
pub fn set_signing_key(signing_key_base64: &str) -> Result<(), LicenseError> {
    let local_data_dir = get_local_data_dir();
    if !local_data_dir.exists() {
        fs::create_dir_all(&local_data_dir)
            .map_err(|e| LicenseError::WriteError(e.to_string()))?;
    }

    let key_file = local_data_dir.join("signing_key.pem");

    // 如果已存在，不覆盖
    if key_file.exists() {
        return Err(LicenseError::WriteError("签名密钥已设置，无法重复设置".to_string()));
    }

    // 验证密钥格式
    let signing_key = signing_key_from_base64(signing_key_base64.trim())
        .map_err(|_| LicenseError::InvalidFormat)?;

    // 保存密钥
    fs::write(&key_file, signing_key_to_base64(&signing_key))
        .map_err(|e| LicenseError::WriteError(e.to_string()))?;

    Ok(())
}

/// 检查签名密钥文件是否已设置
pub fn is_signing_key_set() -> bool {
    // 优先检查 localData 目录
    let local_data_key = get_local_data_dir().join("signing_key.pem");
    if local_data_key.exists() {
        return true;
    }

    // 检查项目根目录（仅开发时使用）
    let project_key = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("signing_key.pem");
    project_key.exists()
}

/// 验证密钥格式并验签
///
/// 密钥格式：Base64(JSON) | Base64(Ed25519签名)
fn parse_and_verify_license(license_key: &str) -> Result<LicenseData, LicenseError> {
    let verify_key = get_verify_key()?;
    let decrypted = crypto_verify_signature(license_key, &verify_key)
        .map_err(|_| LicenseError::InvalidFormat)?;

    // 解析 JSON
    let license_data: LicenseData = serde_json::from_slice(&decrypted)
        .map_err(|_| LicenseError::InvalidFormat)?;

    Ok(license_data)
}

/// 验证授权状态
pub fn validate_license(license_key: &str) -> Result<LicenseStatus, LicenseError> {
    // 1. 解析并验签密钥
    let license_data = parse_and_verify_license(license_key)?;

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

    // 4. 计算剩余天数
    let remaining_days = (expire_date - today).num_days();

    // 5. 保存授权状态
    save_license_status(&license_data)?;

    Ok(LicenseStatus {
        is_licensed: true,
        expire_at: Some(license_data.expire_at),
        remaining_days: Some(remaining_days),
        error_message: None,
    })
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

    // 保存机器码哈希（使用固定密钥加密，以便后续解密）
    let key_file = get_key_file_path();
    // 使用固定的派生密钥来加密 machine_hash
    let fixed_key = super::crypto::derive_key("License-Status-Salt", b"License-Status-Salt");
    let key_encrypted = encrypt_data(license_data.machine_hash.as_bytes(), &fixed_key)
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

    // 读取密钥文件（使用固定密钥加密的机器码哈希）
    let encrypted_key = fs::read_to_string(&key_file)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;

    // 解密获取机器码哈希（使用与保存时相同的密钥）
    let key_salt = super::crypto::derive_key("License-Status-Salt", b"License-Status-Salt");
    let machine_hash_bytes = decrypt_data(&encrypted_key, &key_salt)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;
    let machine_hash = String::from_utf8(machine_hash_bytes)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;

    // 使用机器码哈希解密状态（与保存时使用的密钥相同）
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
    };

    let json = serde_json::to_vec(&license_data)
        .map_err(|_e| LicenseError::InvalidFormat)?;

    // 使用 Ed25519 签名
    let signing_key = get_signing_key()?
        .ok_or_else(|| LicenseError::InvalidFormat)?;

    let signed = sign_data(&json, &signing_key)
        .map_err(|_e| LicenseError::InvalidFormat)?;

    Ok(signed)
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
        let result = parse_and_verify_license("invalid_key");
        assert!(result.is_err());
    }
}
