//! 默认密钥对
//!
//! 内置应用默认的 Ed25519 公钥，用于验证授权码签名

use super::crypto::{VerifyingKey, verify_key_from_base64};

/// 默认 Ed25519 验签公钥（Base64 编码）
/// 此公钥硬编码在代码中，用于验证授权码签名
pub const DEFAULT_VERIFY_KEY: &str = "FdUymUNfZ7UZViZiaDEp017pZp8T2A/pYlznueFi6/Q=";

/// 获取默认验签公钥
pub fn get_default_verify_key() -> Result<VerifyingKey, super::crypto::CryptoError> {
    verify_key_from_base64(DEFAULT_VERIFY_KEY)
}
