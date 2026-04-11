# 简化授权系统实现计划

> **面向 AI 代理的工作者：** 必需子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 逐任务实现此计划。步骤使用复选框（`- [ ]`）语法来跟踪进度。

**目标：** 简化授权系统，移除管理员密码，改用私钥文件检测来控制授权码生成界面

**架构：** 内置默认 Ed25519 公私钥对，公钥硬编码在代码中用于验签，私钥放在 localData 目录用于授权码生成

**技术栈：** Rust + Tauri + Ed25519

---

## 文件清单

**新建：**
- `src-tauri/src/services/default_keys.rs` - 内置默认公私钥对
- `signing_key.pem` - 项目根目录的默认私钥文件（备份）

**修改：**
- `src-tauri/src/services/crypto.rs` - 添加内置默认公钥
- `src-tauri/src/services/license.rs` - 移除管理员密码逻辑，改用私钥文件检测
- `src-tauri/src/commands/auth.rs` - 移除管理员密码相关命令
- `src-tauri/src/services/mod.rs` - 导出 default_keys 模块

---

## 任务 1：生成默认密钥对

**文件：**
- 创建：`src-tauri/src/services/default_keys.rs`
- 创建：`signing_key.pem`

- [ ] **步骤 1：创建 `src-tauri/src/services/default_keys.rs`**

```rust
//! 默认密钥对
//!
//! 内置应用默认的 Ed25519 公私钥对

use super::crypto::{SigningKey, VerifyingKey, signing_key_to_base64, verify_key_to_base64};

/// 默认 Ed25519 签名密钥（Base64 编码）
/// 私钥用于生成授权码，应妥善保管
pub const DEFAULT_SIGNING_KEY: &str = "生成的默认私钥Base64";

/// 默认 Ed25519 验签公钥（Base64 编码）
/// 此公钥硬编码在代码中，用于验证授权码签名
pub const DEFAULT_VERIFY_KEY: &str = "生成的默认公钥Base64";

/// 获取默认签名密钥
pub fn get_default_signing_key() -> Result<SigningKey, super::crypto::CryptoError> {
    super::crypto::signing_key_from_base64(DEFAULT_SIGNING_KEY)
}

/// 获取默认验签公钥
pub fn get_default_verify_key() -> Result<VerifyingKey, super::crypto::CryptoError> {
    super::crypto::verify_key_from_base64(DEFAULT_VERIFY_KEY)
}
```

- [ ] **步骤 2：生成默认密钥对**

在终端运行以下命令生成默认密钥对：

```bash
cd /d/wgq_ai/study-ai-app/src-tauri
cargo run --bin license-gen -- generate-keypair
```

或者临时创建一个 Rust 程序来生成密钥对并打印 Base64 输出。

- [ ] **步骤 3：更新 `default_keys.rs` 中的密钥常量**

用生成的密钥替换 `DEFAULT_SIGNING_KEY` 和 `DEFAULT_VERIFY_KEY` 的值。

- [ ] **步骤 4：创建 `signing_key.pem` 文件**

将私钥以 PEM 格式保存到项目根目录：

```
-----BEGIN ED25519 SIGNING KEY-----
<私钥Base64>
-----END ED25519 SIGNING KEY-----
```

- [ ] **步骤 5：在 `src-tauri/src/services/mod.rs` 中导出**

```rust
pub mod default_keys;
```

- [ ] **步骤 6：验证编译**

```bash
cd D:\wgq_ai\study-ai-app\src-tauri && cargo check
```

- [ ] **步骤 7：Commit**

```bash
git add src-tauri/src/services/default_keys.rs signing_key.pem src-tauri/src/services/mod.rs
git commit -m "feat: add default Ed25519 keypair for license signing"
```

---

## 任务 2：修改 license.rs 移除管理员密码逻辑

**文件：**
- 修改：`src-tauri/src/services/license.rs`

- [ ] **步骤 1：阅读现有 license.rs**

了解当前的管理员密码相关函数：
- `is_admin_password_set()`
- `set_admin_password()`
- `verify_admin_password()`
- `change_admin_password()`
- `get_admin_password_file_path()`
- `DEFAULT_ADMIN_PASSWORD`

- [ ] **步骤 2：移除管理员密码相关函数和常量**

删除：
- `DEFAULT_ADMIN_PASSWORD` 常量
- `is_admin_password_set()` 函数
- `set_admin_password()` 函数
- `verify_admin_password()` 函数
- `change_admin_password()` 函数
- `get_admin_password_file_path()` 函数

- [ ] **步骤 3：修改 `get_verify_key()` 使用内置公钥**

将 `get_verify_key()` 改为使用 `default_keys::get_default_verify_key()`：

```rust
/// 获取当前有效的验签公钥
fn get_verify_key() -> Result<super::crypto::VerifyingKey, LicenseError> {
    default_keys::get_default_verify_key()
        .map_err(|_| LicenseError::NotFound)
}
```

- [ ] **步骤 4：修改 `get_signing_key()` 使用私钥文件检测**

```rust
/// 获取签名私钥（从 localData 目录或项目根目录）
pub fn get_signing_key() -> Result<Option<super::crypto::SigningKey>, LicenseError> {
    // 优先检查 localData 目录
    let local_data_key = get_local_data_dir().join("signing_key.pem");
    if local_data_key.exists() {
        let content = fs::read_to_string(&local_data_key)
            .map_err(|e| LicenseError::ReadError(e.to_string()))?;
        let key = super::crypto::signing_key_from_base64(content.trim())
            .map_err(|_| LicenseError::InvalidFormat)?;
        return Ok(Some(key));
    }

    // 检查项目根目录（仅开发时使用）
    let project_key = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("signing_key.pem");
    if project_key.exists() {
        let content = fs::read_to_string(&project_key)
            .map_err(|e| LicenseError::ReadError(e.to_string()))?;
        let key = super::crypto::signing_key_from_base64(content.trim())
            .map_err(|_| LicenseError::InvalidFormat)?;
        return Ok(Some(key));
    }

    Ok(None)
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
```

- [ ] **步骤 5：更新 `set_signing_key()` 使用 localData 目录**

修改私钥保存位置为 `localData` 目录：

```rust
/// 设置签名私钥（保存到 localData 目录）
pub fn set_signing_key(signing_key_base64: &str) -> Result<(), LicenseError> {
    let local_data_dir = get_local_data_dir();
    if !local_data_dir.exists() {
        fs::create_dir_all(&local_data_dir)
            .map_err(|e| LicenseError::WriteError(e.to_string()))?;
    }

    let key_file = local_data_dir.join("signing_key.pem");

    // 如果已存在，不覆盖（保持一致性）
    if key_file.exists() {
        return Err(LicenseError::WriteError("签名密钥已设置，无法重复设置".to_string()));
    }

    // 验证密钥格式
    let signing_key = super::crypto::signing_key_from_base64(signing_key_base64.trim())
        .map_err(|_| LicenseError::InvalidFormat)?;

    // 保存密钥
    fs::write(&key_file, super::crypto::signing_key_to_base64(&signing_key))
        .map_err(|e| LicenseError::WriteError(e.to_string()))?;

    Ok(())
}
```

- [ ] **步骤 6：更新 `is_signing_key_set()` 使用 localData 目录**

```rust
/// 检查签名密钥是否已设置
pub fn is_signing_key_set() -> bool {
    let local_data_dir = get_local_data_dir();
    let local_data_key = local_data_dir.join("signing_key.pem");
    if local_data_key.exists() {
        return true;
    }

    // 也检查项目根目录（仅开发时使用）
    let project_key = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("signing_key.pem");
    project_key.exists()
}
```

- [ ] **步骤 7：验证编译**

```bash
cd D:\wgq_ai\study-ai-app\src-tauri && cargo check
```

- [ ] **步骤 8：Commit**

```bash
git add src-tauri/src/services/license.rs
git commit -m "feat: remove admin password, use private key file detection"
```

---

## 任务 3：修改 auth.rs 移除管理员密码命令

**文件：**
- 修改：`src-tauri/src/commands/auth.rs`

- [ ] **步骤 1：阅读现有 auth.rs**

了解当前的管理员密码相关命令：
- `is_admin_password_set_command`
- `set_admin_password_command`
- `verify_admin_password_command`
- `change_admin_password_command`

- [ ] **步骤 2：移除管理员密码相关导入和命令**

从导入中移除：
```rust
is_admin_password_set, set_admin_password, verify_admin_password, change_admin_password,
```

删除命令函数：
- `is_admin_password_set_command`
- `set_admin_password_command`
- `verify_admin_password_command`
- `change_admin_password_command`

- [ ] **步骤 3：验证编译**

```bash
cd D:\wgq_ai\study-ai-app\src-tauri && cargo check
```

- [ ] **步骤 4：Commit**

```bash
git add src-tauri/src/commands/auth.rs
git commit -m "feat: remove admin password commands from auth"
```

---

## 验收标准

- [ ] 默认密钥对已生成并保存到 `signing_key.pem`
- [ ] `crypto.rs` 和 `license.rs` 中使用内置公钥验签
- [ ] 管理员密码相关函数和命令已移除
- [ ] 私钥文件检测使用 `localData` 目录
- [ ] cargo check 通过
- [ ] 授权码生成和验证功能正常

---

## 备注

1. **私钥文件优先级**：`localData` > 项目根目录（开发时使用）

2. **向后兼容**：已有的 `license.dat` 文件可继续使用

3. **前端 Ctrl+Shift+G**：需要在前端添加快捷键检测，调用 `is_signing_key_set_command` 判断是否显示授权码生成界面
