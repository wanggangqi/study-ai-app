# 简化授权系统设计

> 文档版本：v1.0
> 创建日期：2026-04-08
> 应用名称：智学伴侣 (StudyMate)

---

## 1. 概述

简化授权系统，移除管理员密码机制，改用私钥文件检测来控制授权码生成界面的访问权限。

### 1.1 设计目标

- 简化管理员授权流程
- 移除管理员密码相关代码
- 数据文件统一存储在 `localData` 目录

### 1.2 默认密钥对

应用内置默认 Ed25519 公私钥对：

| 类型 | 位置 | 说明 |
|------|------|------|
| 公钥 | 代码中（硬编码） | 用于验证授权码签名 |
| 私钥 | 项目根目录 `signing_key.pem` | 管理员备份原始私钥 |
| 私钥副本 | `localData\signing_key.pem` | 管理员实际使用的私钥 |

---

## 2. 数据存储结构

### 2.1 localData 目录

```
C:\Users\{用户名}\AppData\Local\com.studymate.app\
└─ localData\
   ├─ config.json          # 用户配置（AI密钥、码云Token等）
   ├─ license.dat          # 授权状态（加密）
   ├─ machine_key.dat     # 机器码验证密钥
   ├─ signing_key.pem    # 管理员私钥（可选，需手动放置）
   └─ study_mate.db       # SQLite 数据库
```

### 2.2 项目根目录

```
D:\wgq_ai\study-ai-app\
└─ signing_key.pem       # 默认私钥备份（版本控制）
```

---

## 3. 授权流程

### 3.1 授权码生成流程（管理员）

1. 管理员将私钥文件复制到 `localData\signing_key.pem`
2. 管理员在应用中按 `Ctrl+Shift+G`
3. 应用检测到 `localData\signing_key.pem` 存在
4. 进入授权码生成界面
5. 输入目标机器码哈希（或获取当前机器码）
6. 输入到期日期
7. 生成授权码
8. 将授权码提供给用户

### 3.2 用户激活流程

1. 用户安装应用
2. 应用检测到未授权
3. 显示授权界面
4. 用户输入授权码
5. 应用使用内置公钥验证签名
6. 验证通过 → 保存授权状态 → 进入应用

### 3.3 无私钥时的处理

- 按 `Ctrl+Shift+G` 时检测 `localData\signing_key.pem` 是否存在
- 不存在 → 提示"未找到私钥文件，无法生成授权码"或直接忽略
- 不影响普通用户的正常激活流程

---

## 4. 密钥管理

### 4.1 内置公钥

在 `src-tauri/src/services/crypto.rs` 中硬编码默认公钥：

```rust
// 默认 Ed25519 公钥（Base64 编码）
const DEFAULT_VERIFY_KEY: &str = "默认公钥Base64字符串";
```

### 4.2 私钥位置优先级

1. 首先检查 `localData\signing_key.pem`（管理员实际使用）
2. 如不存在，检查项目根目录 `signing_key.pem`（仅开发时使用）
3. 都不存在 → 无法生成授权码

### 4.3 私钥文件格式

```
-----BEGIN ED25519 SIGNING KEY-----
Base64编码的私钥内容
-----END ED25519 SIGNING KEY-----
```

---

## 5. 移除的功能

### 5.1 移除管理员密码

| 移除项 | 说明 |
|--------|------|
| `admin.pass` 文件 | 不再需要 |
| `set_admin_password` 命令 | 不再需要 |
| `verify_admin_password` 命令 | 不再需要 |
| `change_admin_password` 命令 | 不再需要 |
| `is_admin_password_set` 命令 | 不再需要 |
| 默认管理员密码 `Admin@2026` | 不再需要 |

### 5.2 简化后的 auth.rs 命令

| 保留的命令 | 说明 |
|-----------|------|
| `get_machine_id_command` | 获取机器码 |
| `get_machine_hash_command` | 获取机器码哈希 |
| `validate_license_command` | 验证授权码 |
| `get_license_status_command` | 获取授权状态 |
| `generate_license_key_command` | 生成授权码 |
| `generate_signing_key_pair_command` | 生成新的密钥对（保留，但不使用） |
| `is_signing_key_set_command` | 保留，检测私钥文件是否存在 |

| 移除的命令 | 说明 |
|-----------|------|
| `set_admin_password_command` | 移除 |
| `verify_admin_password_command` | 移除 |
| `change_admin_password_command` | 移除 |

---

## 6. 文件变更清单

### 6.1 新增文件

| 文件 | 说明 |
|------|------|
| `src-tauri/src/services/default_keys.rs` | 内置默认公私钥对 |
| `docs/superpowers/specs/2026-04-08-simplified-auth-design.md` | 本设计文档 |

### 6.2 修改文件

| 文件 | 修改内容 |
|------|---------|
| `src-tauri/src/services/crypto.rs` | 添加内置默认公钥，移除管理员密码相关逻辑 |
| `src-tauri/src/services/license.rs` | 改用私钥文件检测，移除管理员密码逻辑 |
| `src-tauri/src/commands/auth.rs` | 移除管理员密码相关命令 |
| `src-tauri/src/db/schema.rs` | 可选：将 `user_config` 表重命名为 `app_config` |

### 6.3 删除文件

| 文件 | 说明 |
|------|------|
| 无 | 旧私钥文件无需删除，会被新逻辑覆盖 |

---

## 7. 验证检查点

- [ ] `Ctrl+Shift+G` 在无私钥文件时无反应或提示
- [ ] `Ctrl+Shift+G` 在有私钥文件时进入授权码生成界面
- [ ] 生成的授权码可被正常激活
- [ ] 移除管理员密码后不影响其他功能
- [ ] 数据库和配置正常读写到 `localData` 目录

---

## 8. 向后兼容

- 已有的 `license.dat` 文件可继续使用
- 已有的 `admin.pass` 文件会被忽略（不再创建）
- 现有授权码生成流程的用户体验不变

---

*文档结束*
