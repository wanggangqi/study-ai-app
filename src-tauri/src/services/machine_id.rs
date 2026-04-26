//! 机器码生成服务
//!
//! 获取硬件特征信息（CPU ID、硬盘序列号、MAC地址）并生成唯一机器码
//! 使用缓存机制避免重复调用 PowerShell 命令

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use sha2::{Sha256, Digest};
use std::process::Command;
use std::sync::OnceLock;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MachineIdError {
    #[error("Failed to execute command: {0}")]
    CommandError(#[from] std::io::Error),

    #[error("Failed to parse output: {0}")]
    ParseError(String),

    #[error("Failed to get hardware info: {0}")]
    HardwareError(String),
}

/// 缓存的组合硬件信息
static CACHED_HARDWARE_INFO: OnceLock<String> = OnceLock::new();

/// 获取机器码（带缓存）
///
/// 组合 CPU ID、硬盘序列号、MAC 地址，计算 SHA256 哈希
/// 首次调用会执行 PowerShell 命令，后续调用直接返回缓存值
pub fn get_machine_id() -> Result<String, MachineIdError> {
    let combined = get_cached_hardware_info()?;
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();
    Ok(BASE64.encode(result))
}

/// 获取机器码的原始哈希值（带缓存，用于密钥验证）
///
/// 首次调用会执行 PowerShell 命令，后续调用直接返回缓存值
pub fn get_machine_hash() -> Result<String, MachineIdError> {
    let combined = get_cached_hardware_info()?;
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// 获取缓存的硬件信息（首次调用会计算，后续直接返回）
fn get_cached_hardware_info() -> Result<&'static str, MachineIdError> {
    // 如果已缓存，直接返回
    if let Some(cached) = CACHED_HARDWARE_INFO.get() {
        return Ok(cached);
    }

    // 首次调用时计算
    let cpu_id = get_cpu_id()?;
    let disk_serial = get_disk_serial()?;
    let mac_address = get_mac_address()?;
    let combined = format!("{}|{}|{}", cpu_id, disk_serial, mac_address);

    // 存入缓存（忽略已存在的情况）
    let _ = CACHED_HARDWARE_INFO.set(combined);

    // 返回缓存中的值
    Ok(CACHED_HARDWARE_INFO.get().unwrap())
}

/// 获取 CPU ID
#[cfg(windows)]
fn get_cpu_id() -> Result<String, MachineIdError> {
    // 使用 PowerShell 获取 CPU ID
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "(Get-CimInstance -ClassName Win32_Processor).ProcessorId"
        ])
        .output()?;

    if !output.status.success() {
        return Err(MachineIdError::HardwareError("Failed to get CPU ID".to_string()));
    }

    let cpu_id = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    if cpu_id.is_empty() {
        // 备选方案：使用 CPU 名称
        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "(Get-CimInstance -ClassName Win32_Processor).Name"
            ])
            .output()?;

        return Ok(String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string());
    }

    Ok(cpu_id)
}

/// 获取硬盘序列号
#[cfg(windows)]
fn get_disk_serial() -> Result<String, MachineIdError> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "(Get-CimInstance -ClassName Win32_DiskDrive).SerialNumber"
        ])
        .output()?;

    if !output.status.success() {
        return Err(MachineIdError::HardwareError("Failed to get disk serial".to_string()));
    }

    let serial = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    Ok(serial)
}

/// 获取 MAC 地址
#[cfg(windows)]
fn get_mac_address() -> Result<String, MachineIdError> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "(Get-NetAdapter | Where-Object { $_.Status -eq 'Up' -and $_.MacAddress -ne $null } | Select-Object -First 1).MacAddress"
        ])
        .output()?;

    if !output.status.success() {
        return Err(MachineIdError::HardwareError("Failed to get MAC address".to_string()));
    }

    let mac = String::from_utf8_lossy(&output.stdout)
        .trim()
        .replace("-", "")
        .replace(":", "");

    if mac.is_empty() {
        return Err(MachineIdError::HardwareError("No active network adapter found".to_string()));
    }

    Ok(mac)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_machine_id() {
        let result = get_machine_id();
        assert!(result.is_ok());
        let id = result.unwrap();
        assert!(!id.is_empty());
        println!("Machine ID: {}", id);
    }

    #[test]
    fn test_get_machine_hash() {
        let result = get_machine_hash();
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash.len(), 64); // SHA256 produces 64 hex characters
        println!("Machine Hash: {}", hash);
    }
}
