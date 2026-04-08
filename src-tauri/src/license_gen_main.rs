//! 密钥生成 CLI 工具
//!
//! 用于为用户生成激活密钥
//!
//! 用法:
//!   cargo run --bin license-gen -- <过期日期> <机器码>
//!
//! 示例:
//!   cargo run --bin license-gen -- 2027-12-31 a1b2c3d4e5f6...

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::Serialize;

// 复用 crypto 模块的加密函数
use study_ai_app_lib::services::crypto::{sign_data, signing_key_from_base64};

#[derive(Serialize)]
struct LicenseData {
    machine_hash: String,
    expire_at: String,
    signature: String,
}

const DEFAULT_SIGNATURE: &str = "StudyMate-2026-Dev";

/// 默认内置私钥 Base64（用于开发/演示）
/// 实际生产中应更换此密钥
const DEFAULT_SIGNING_KEY_BASE64: &str = "KqQpTHq3P9cL8RJvF3dK2wZ5yH7gM9xT4nB6jL8vU0=";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let expire_date = &args[1];
    let machine_hash = &args[2];

    // 验证日期格式
    if chrono::NaiveDate::parse_from_str(expire_date, "%Y-%m-%d").is_err() {
        eprintln!("错误: 日期格式无效，请使用 YYYY-MM-DD 格式，例如 2027-12-31");
        std::process::exit(1);
    }

    // 验证机器码格式（64位十六进制）
    if machine_hash.len() != 64 || !machine_hash.chars().all(|c| c.is_ascii_hexdigit()) {
        eprintln!("错误: 机器码格式无效，应为64位十六进制字符");
        std::process::exit(1);
    }

    // 生成密钥
    match generate_license_key(machine_hash, expire_date) {
        Ok(key) => {
            println!("\n=== 生成的激活密钥 ===\n");
            println!("{}", key);
            println!("\n======================\n");
            println!("过期日期: {}", expire_date);
            println!("机器码:   {}...", &machine_hash[..16]);
        }
        Err(e) => {
            eprintln!("错误: 生成密钥失败 - {}", e);
            std::process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    println!("智学伴侣 - 密钥生成工具\n");
    println!("用法:");
    println!("  {} <过期日期> <机器码>", program);
    println!("\n参数:");
    println!("  过期日期   授权到期日期，格式 YYYY-MM-DD");
    println!("  机器码     用户的64位机器码（十六进制）");
    println!("\n示例:");
    println!("  {} 2027-12-31 a1b2c3d4e5f6...", program);
    println!("\n提示:");
    println!("  用户可在应用的激活界面获取其机器码");
}

fn generate_license_key(machine_hash: &str, expire_date: &str) -> Result<String, String> {
    let license_data = LicenseData {
        machine_hash: machine_hash.to_string(),
        expire_at: expire_date.to_string(),
        signature: DEFAULT_SIGNATURE.to_string(),
    };

    let json = serde_json::to_vec(&license_data)
        .map_err(|e| format!("序列化失败: {}", e))?;

    // 使用 Ed25519 签名
    let signing_key = signing_key_from_base64(DEFAULT_SIGNING_KEY_BASE64)
        .map_err(|e| format!("密钥加载失败: {}", e))?;

    let signed = sign_data(&json, &signing_key)
        .map_err(|e| format!("签名失败: {}", e))?;

    Ok(signed)
}
