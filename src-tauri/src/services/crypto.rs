//! AES 加密/解密服务
//!
//! 使用 AES-256-GCM 进行数据加密和解密

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed: {0}")]
    EncryptionError(String),

    #[error("Decryption failed: {0}")]
    DecryptionError(String),

    #[error("Invalid key length")]
    InvalidKeyLength,

    #[error("Invalid data format")]
    InvalidDataFormat,

    #[error("Signing failed: {0}")]
    SigningError(String),

    #[error("Verification failed: {0}")]
    VerificationError(String),

    #[error("Invalid signature format")]
    InvalidSignatureFormat,
}

/// AES-256-GCM 加密
///
/// # Arguments
/// * `data` - 待加密的原始数据
/// * `key` - 32字节的 AES-256 密钥
///
/// # Returns
/// Base64 编码的加密数据（包含随机 nonce）
pub fn encrypt_data(data: &[u8], key: &[u8]) -> Result<String, CryptoError> {
    if key.len() != 32 {
        return Err(CryptoError::InvalidKeyLength);
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;

    // 生成随机 12 字节 nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 加密数据
    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;

    // 将 nonce + 密文合并
    let mut combined = nonce_bytes.to_vec();
    combined.extend(ciphertext);

    // Base64 编码
    Ok(BASE64.encode(combined))
}

/// AES-256-GCM 解密
///
/// # Arguments
/// * `encrypted_data` - Base64 编码的加密数据
/// * `key` - 32字节的 AES-256 密钥
///
/// # Returns
/// 解密后的原始数据
pub fn decrypt_data(encrypted_data: &str, key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != 32 {
        return Err(CryptoError::InvalidKeyLength);
    }

    // Base64 解码
    let combined = BASE64
        .decode(encrypted_data)
        .map_err(|_| CryptoError::InvalidDataFormat)?;

    if combined.len() < 12 {
        return Err(CryptoError::InvalidDataFormat);
    }

    // 分离 nonce 和密文
    let nonce = Nonce::from_slice(&combined[..12]);
    let ciphertext = &combined[12..];

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| CryptoError::DecryptionError(e.to_string()))?;

    // 解密数据
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::DecryptionError(e.to_string()))
}

/// 从密码派生 AES-256 密钥
///
/// 使用简单的 HKDF-like 方式从密码派生 32 字节密钥
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    use sha2::{Sha256, Digest};

    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    let result = hasher.finalize();

    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

/// 生成随机密钥
pub fn generate_random_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

/// 生成随机盐
pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    salt
}

// ============================================================================
// Ed25519 签名相关
// ============================================================================

pub use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

/// Ed25519 签名数据
///
/// 返回格式：Base64(JSON) | Base64(签名)
pub fn sign_data(data: &[u8], signing_key: &SigningKey) -> Result<String, CryptoError> {
    let signature = signing_key.sign(data);

    // 将数据和签名组合
    let combined = format!("{}|{}", BASE64.encode(data), BASE64.encode(signature.to_bytes()));

    Ok(combined)
}

/// Ed25519 验签
///
/// 输入格式：Base64(JSON) | Base64(签名)
pub fn verify_signature(combined_data: &str, verify_key: &VerifyingKey) -> Result<Vec<u8>, CryptoError> {
    let parts: Vec<&str> = combined_data.split('|').collect();
    if parts.len() != 2 {
        return Err(CryptoError::InvalidSignatureFormat);
    }

    let data = BASE64.decode(parts[0])
        .map_err(|_| CryptoError::InvalidDataFormat)?;

    let signature_bytes = BASE64.decode(parts[1])
        .map_err(|_| CryptoError::InvalidDataFormat)?;

    if signature_bytes.len() != 64 {
        return Err(CryptoError::InvalidSignatureFormat);
    }

    // 将 Vec<u8> 转换为 [u8; 64]
    let mut signature_arr = [0u8; 64];
    signature_arr.copy_from_slice(&signature_bytes);
    let signature = Signature::from_bytes(&signature_arr);

    verify_key.verify_strict(&data, &signature)
        .map_err(|_| CryptoError::VerificationError("Signature verification failed".to_string()))?;

    Ok(data)
}

/// 生成新的 Ed25519 密钥对
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verify_key = signing_key.verifying_key();
    (signing_key, verify_key)
}

/// 将签名密钥序列化为 Base64 字符串
pub fn signing_key_to_base64(key: &SigningKey) -> String {
    BASE64.encode(key.as_bytes())
}

/// 将验签密钥序列化为 Base64 字符串
pub fn verify_key_to_base64(key: &VerifyingKey) -> String {
    BASE64.encode(key.as_bytes())
}

/// 从 Base64 字符串反序列化为签名密钥
pub fn signing_key_from_base64(s: &str) -> Result<SigningKey, CryptoError> {
    let bytes = BASE64.decode(s)
        .map_err(|_| CryptoError::InvalidDataFormat)?;

    if bytes.len() != 32 {
        return Err(CryptoError::InvalidKeyLength);
    }

    // 将 Vec<u8> 转换为 [u8; 32]
    let mut key_arr = [0u8; 32];
    key_arr.copy_from_slice(&bytes);
    Ok(SigningKey::from_bytes(&key_arr))
}

/// 从 Base64 字符串反序列化为验签密钥
pub fn verify_key_from_base64(s: &str) -> Result<VerifyingKey, CryptoError> {
    let bytes = BASE64.decode(s)
        .map_err(|_| CryptoError::InvalidDataFormat)?;

    if bytes.len() != 32 {
        return Err(CryptoError::InvalidKeyLength);
    }

    // 将 Vec<u8> 转换为 [u8; 32]
    let mut key_arr = [0u8; 32];
    key_arr.copy_from_slice(&bytes);
    VerifyingKey::from_bytes(&key_arr)
        .map_err(|_| CryptoError::InvalidKeyLength)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = generate_random_key();
        let data = b"Hello, World! This is a test message.";

        let encrypted = encrypt_data(data, &key).unwrap();
        assert!(!encrypted.is_empty());

        let decrypted = decrypt_data(&encrypted, &key).unwrap();
        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_encrypt_decrypt_with_password() {
        let password = "test_password_123";
        let salt = generate_salt();
        let key = derive_key(password, &salt);

        let data = b"Secret message";

        let encrypted = encrypt_data(data, &key).unwrap();
        let decrypted = decrypt_data(&encrypted, &key).unwrap();

        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_invalid_key_length() {
        let short_key = [0u8; 16]; // 16 bytes instead of 32
        let data = b"test";

        let result = encrypt_data(data, &short_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_encrypted_data() {
        let key = generate_random_key();
        let result = decrypt_data("invalid_base64_data!!!", &key);
        assert!(result.is_err());
    }
}
