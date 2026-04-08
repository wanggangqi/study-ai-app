//! 授权验证服务
//!
//! 解析密钥格式、验证机器码、验证有效期，使用 Ed25519 非对称签名

use chrono::{Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

use super::crypto::{decrypt_data, encrypt_data, sign_data, verify_signature as crypto_verify_signature,
    signing_key_from_base64, signing_key_to_base64, verify_key_from_base64};
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

/// 获取签名密钥文件路径
fn get_signing_key_file_path() -> PathBuf {
    let app_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("StudyMate");

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).ok();
    }

    app_dir.join("signing_key.blob")
}

/// 获取当前有效的验签公钥
fn get_verify_key() -> Result<super::crypto::VerifyingKey, LicenseError> {
    let key_file = get_signing_key_file_path();
    if !key_file.exists() {
        return Err(LicenseError::NotFound);
    }

    let signing_key_base64 = fs::read_to_string(&key_file)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;
    let signing_key = signing_key_from_base64(signing_key_base64.trim())
        .map_err(|_| LicenseError::InvalidFormat)?;
    Ok(signing_key.verifying_key())
}

/// 获取当前有效的签名私钥
pub fn get_signing_key() -> Result<Option<super::crypto::SigningKey>, LicenseError> {
    let key_file = get_signing_key_file_path();
    if !key_file.exists() {
        return Ok(None);
    }

    let signing_key_base64 = fs::read_to_string(&key_file)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;
    let signing_key = signing_key_from_base64(signing_key_base64.trim())
        .map_err(|_| LicenseError::InvalidFormat)?;
    Ok(Some(signing_key))
}

/// 设置签名私钥（仅首次设置）
pub fn set_signing_key(signing_key_base64: &str) -> Result<(), LicenseError> {
    let key_file = get_signing_key_file_path();

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

/// 检查签名密钥是否已设置
pub fn is_signing_key_set() -> bool {
    get_signing_key_file_path().exists()
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

/// 默认管理员密码（首次使用）
/// 实际应用中应在首次设置后从配置文件读取
const DEFAULT_ADMIN_PASSWORD: &str = "Admin@2026";

/// 检查是否已设置管理员密码
pub fn is_admin_password_set() -> bool {
    get_admin_password_file_path().exists()
}

/// 获取管理员密码文件路径
fn get_admin_password_file_path() -> PathBuf {
    let app_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("StudyMate");

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).ok();
    }

    app_dir.join("admin.pass")
}

/// 设置管理员密码（首次设置）
pub fn set_admin_password(password: &str) -> Result<(), LicenseError> {
    let file_path = get_admin_password_file_path();

    if file_path.exists() {
        return Err(LicenseError::WriteError("管理员密码已设置，无法重复设置".to_string()));
    }

    // 使用 SHA256 哈希存储密码
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hash = hex::encode(hasher.finalize());

    fs::write(&file_path, hash)
        .map_err(|e| LicenseError::WriteError(e.to_string()))?;

    Ok(())
}

/// 验证管理员密码
pub fn verify_admin_password(password: &str) -> Result<bool, LicenseError> {
    let file_path = get_admin_password_file_path();

    // 如果没有设置过密码，使用默认密码验证
    if !file_path.exists() {
        return Ok(password == DEFAULT_ADMIN_PASSWORD);
    }

    let stored_hash = fs::read_to_string(&file_path)
        .map_err(|e| LicenseError::ReadError(e.to_string()))?;

    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let input_hash = hex::encode(hasher.finalize());

    Ok(stored_hash.trim() == input_hash)
}

/// 修改管理员密码
pub fn change_admin_password(old_password: &str, new_password: &str) -> Result<(), LicenseError> {
    // 先验证旧密码
    if !verify_admin_password(old_password)? {
        return Err(LicenseError::InvalidSignature); // 用这个错误表示密码错误
    }

    // 使用新密码覆盖
    let mut hasher = Sha256::new();
    hasher.update(new_password.as_bytes());
    let hash = hex::encode(hasher.finalize());

    fs::write(get_admin_password_file_path(), hash)
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
