# AI 一对一教学应用 - 设计规格说明

> 文档版本：v1.0
> 创建日期：2026-03-30
> 应用名称：智学伴侣 (StudyMate)

---

## 1. 项目概述

### 1.1 项目目标

开发一款基于 Tauri 的本地 AI 一对一教学应用，让学生根据自己想学习的内容、目标、花费时长和喜欢的教学风格，获得个性化的 AI 教学服务。学习记录同步到码云仓库，实现数据备份和多设备访问。

### 1.2 核心功能

- **授权系统**：密钥激活 + 机器绑定 + 有效期验证
- **配置引导**：Git 安装配置、码云账户、本地工作空间、AI 服务商设置
- **咨询师 Agent**：结构化引导收集学习需求，生成课程计划
- **教师 Agent**：生成 HTML 课件、解答疑问、生成练习题、AI 分析答案
- **学习界面**：三栏布局（课程大纲 | 课件展示 | 聊天答疑）
- **码云同步**：每课程独立仓库，全文件同步

### 1.3 技术栈

| 层级 | 技术 | 版本/备注 |
|------|------|----------|
| 桌面框架 | Tauri | v2.x |
| 前端框架 | React + TypeScript | React 18+ |
| 状态管理 | Zustand | - |
| UI 组件 | shadcn/ui + Tailwind CSS | 温暖风格定制 |
| 后端语言 | Rust | - |
| 本地数据库 | SQLite | 通过 rusqlite |
| AI 服务 | Claude / OpenAI / 通义千问 / DeepSeek / 智谱 GLM / MiniMax / Kimi (Moonshot) | 用户可选 |
| 远程仓库 | 码云 (Gitee) | Git 同步 |

---

## 2. 系统架构

### 2.1 整体架构图

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri 应用 (桌面端)                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
│  │   前端层    │  │  Tauri层    │  │   本地层    │          │
│  │   (React)   │  │   (Rust)    │  │  (SQLite)   │          │
│  └─────────────┘  └─────────────┘  └─────────────┘          │
│        │                │                │                  │
│        └────────────────┼────────────────┘                  │
│                         │                                    │
│              ┌──────────┴──────────┐                        │
│              │   本地工作空间      │                        │
│              │  (用户指定目录)     │                        │
│              │  ├─ course-xxx/    │                        │
│              │  │  ├─ plan.json   │                        │
│              │  │  ├─ records.json │                        │
│              │  │  ├─ lessons/    │                        │
│              │  │  └─ exercises/  │                        │
│              └─────────────────────┘                        │
└─────────────────────────────────────────────────────────────┘
                         │
              Git 同步    │   API 调用
                         │
        ┌────────────────┴────────────────┐
        │                                  │
   ┌────┴────┐                    ┌────────┴────────┐
   │  码云   │                    │   AI 服务 API   │
   │ (远程)  │                    │ Claude/OpenAI/  │
   │ 仓库    │                    │ Qwen/DeepSeek/  │
   └─────────┘                    │ GLM            │
                                   └────────────────┘
```

### 2.2 核心模块职责

| 模块 | 职责 | 技术实现 |
|------|------|---------|
| **授权模块** | 密钥验证、机器码绑定、有效期校验 | Rust（加密/解密） |
| **配置模块** | Git 设置、AI 密钥、教学风格管理 | React + Tauri 配置存储 |
| **码云集成** | 仓库创建/克隆、同步推送/拉取 | Rust（调用 git 命令） |
| **咨询师 Agent** | 结构化引导收集需求、生成课程计划 | React 聊天组件 + AI API |
| **教师 Agent** | 课件生成、答疑、练习题生成与分析 | React 聊天组件 + AI API |
| **学习模块** | 课程列表、HTML课件渲染、练习交互 | React（三栏布局） |
| **数据同步** | SQLite → JSON 导出 → Git 同步 | Rust 定时任务 |

---

## 3. 授权系统

### 3.1 密钥格式（Ed25519 签名）

```
密钥结构: Base64(JSON数据) | Base64(Ed25519签名)

JSON数据:
{
  "machine_hash": "a3f2b1c9...",   // 用户机器码的SHA256哈希
  "expire_at": "2025-12-31"       // 有效期截止日期
}

签名: Ed25519 签名，使用管理员私钥签名 JSON 数据
```

**密钥生成流程：**
1. 收集用户机器码（SHA256 哈希）
2. 构建 JSON 数据（包含 machine_hash 和 expire_at）
3. 使用 Ed25519 私钥对 JSON 进行签名
4. 组合：Base64(JSON) | Base64(签名)

**密钥验证流程：**
1. 分离 JSON 和签名
2. 使用公钥验证签名
3. 解析 JSON 获取 machine_hash
4. 比对当前机器码
5. 检查 expire_at 是否过期

### 3.2 机器码获取

采用多个硬件特征组合：CPU ID + 硬盘序列号 + MAC 地址 → SHA256 哈希

### 3.3 签名密钥管理

| 角色 | 密钥 | 用途 |
|------|------|------|
| 管理员 | 私钥（Base64） | 生成许可证签名 |
| 应用 | 公钥（Base64） | 验证许可证签名 |

**重要：**
- 私钥由管理员妥善保管，用于生成用户许可证
- 公钥内置于应用中，用于验证许可证
- 首次使用需要在管理员模式初始化签名密钥

### 3.4 授权流程

```
启动应用
    ↓
检查本地授权状态（加密存储的授权文件）
    ↓
┌─ 已授权且未过期 → 进入应用
│
└─ 未授权或已过期 → 显示授权界面
                      ↓
                 用户输入密钥
                      ↓
                 应用解密密钥
                      ↓
                 校验机器码是否匹配
                      ↓
                 校验有效期是否未过期
                      ↓
                 ┌─ 通过 → 保存授权状态 → 进入应用
                 │
                 └─ 失败 → 提示错误原因 → 重新输入
```

---

## 4. 配置系统

### 4.1 首次启动引导流程

**步骤1: 检查 Git 安装**
- 已安装 → 继续
- 未安装 → 提示安装，提供下载链接

**步骤2: 配置 Git 用户信息**
- 输入 Git 用户名
- 输入 Git 邮箱
- 应用调用 `git config --global` 存储

**步骤3: 码云账户配置**
- 提示注册码云账号（提供链接）
- 输入码云用户名
- 输入码云密码/个人访问令牌
- 应用加密存储凭证

**步骤4: 设置工作空间目录**
- 用户选择本地目录
- 应用验证目录有效性并存储路径

**步骤5: AI 服务配置**
- 选择 AI 服务商
- 输入 API 密钥
- 选择模型（可选）
- 应用验证 API 有效性

**步骤6: 完成**
- 显示"配置完成"，引导进入首页

---

## 5. 码云集成

### 5.1 支持的 AI 服务商

| 服务商 | API 地址 | 模型选项 |
|--------|---------|---------|
| Anthropic (Claude) | api.anthropic.com | claude-3-opus, claude-3-sonnet, claude-3-haiku |
| OpenAI (ChatGPT) | api.openai.com | gpt-4, gpt-4-turbo, gpt-3.5-turbo |
| 阿里云 (通义千问) | dashscope.aliyuncs.com | qwen-turbo, qwen-plus, qwen-max, qwen-long |
| DeepSeek | api.deepseek.com | deepseek-chat, deepseek-coder |
| 智谱 AI (GLM) | open.bigmodel.cn | glm-4, glm-4-flash, glm-3-turbo |
| MiniMax | api.minimax.chat | abab6-chat, abab5.5-chat |
| Kimi (Moonshot AI) | api.moonshot.cn | moonshot-v1-8k, moonshot-v1-32k, moonshot-v1-128k |

### 5.2 创建课程仓库流程

```
1. 调用码云 API 创建远程仓库
   - 仓库名: {课程名}-course
   - 仓库描述: AI一对一教学课程记录

2. 本地克隆仓库到工作空间

3. 初始化仓库结构
   - plan.json（课程计划）
   - records.json（学习记录）
   - lessons/ 目录
   - exercises/ 目录

4. 首次提交并推送
```

### 5.3 仓库目录结构

```
{课程名}-course/
├── plan.json              # 课程计划
├── records.json           # 学习记录
├── README.md              # 课程简介
├── lessons/               # 课件目录
│   ├── lesson-001.html
│   └── ...
├── exercises/             # 练习目录
│   ├── exercise-001.html
│   ├── exercise-001-result.json  # AI分析结果
│   └── ...
└── notes/                 # 用户笔记目录（可选）
```

### 5.4 同步机制

**同步触发点：**
- 完成一节课学习后
- 完成练习并获得分析结果后
- 退出应用前
- 用户手动点击"同步"按钮

**同步流程：**
```
1. 从 SQLite 导出关键数据 → 写入 records.json
2. git add 所有变更文件
3. git commit -m "学习进度更新"
4. git push origin main
```

**数据导出策略：** SQLite 关键数据导出 JSON 同步到仓库（避免二进制 SQLite 文件入 Git）

---

## 6. Agent 设计

### 6.1 咨询师 Agent

**交互方式：** 结构化引导（固定顺序提问）

**对话流程：**

| 步骤 | 内容 | 用户输入 |
|------|------|---------|
| 欢迎 | "你好！我来帮你制定学习计划。请问你想学习什么内容？" | 学习内容 |
| 追问 | "你是想从零开始，还是有一定基础？" | 基础情况 |
| 目标 | "你希望达到什么目标？"（选项：A入门/B能写程序/C能开发项目） | 学习目标 |
| 时长 | "你计划花多长时间学习？" | 时长 |
| 风格 | "请选择教学风格"（展示6种风格卡片） | 风格选择 |
| 确认 | 总结计划，用户确认 | 确认 |
| 生成 | 创建仓库，同步课程计划 | 完成 |

**教学风格（内置6种）：**
1. 严谨学术型 - 结构严谨、概念清晰
2. 实战应用型 - 案例驱动、边学边做
3. 轻松故事型 - 故事/比喻、语言轻松
4. 循序渐进型 - 小步前进、充分练习
5. 启发探索型 - 提问引导、独立思考
6. 快速高效型 - 精炼要点、直奔目标

**用户自定义风格：** 支持添加自定义教学风格

### 6.2 教师 Agent

**交互方式：** 聊天对话式（会话上下文保持）

**教师职责：**

| 场景 | 触发方式 | 教师行为 |
|------|---------|---------|
| 进入课程 | 用户点击课程卡片 | 生成开场欢迎语，引导开始第一课 |
| 学习课件 | 用户点击"开始学习" | 根据课程大纲生成 HTML 课件 |
| 答疑解惑 | 用户在聊天区提问 | 解答问题，结合当前课程上下文 |
| 生成练习 | 用户点击"生成练习题" | 根据当前课程内容生成 HTML 练习题 |
| 分析答案 | 用户提交练习答案 | 分析评分、错题解析、薄弱点建议 |

**开场语示例：**
```
"你好，小明同学！欢迎来到《Python 编程入门》课程。

 本课程共12章36节课，预计学习时长3个月。
 我们采用实战应用型的教学风格，边学边练。

 现在让我们从第1章「Python 基础语法」开始吧！
 你准备好了吗？"
```

### 6.3 练习题交互

- 练习题以 HTML 内嵌表单展示（选择题、填空题、代码编辑框）
- 用户在 HTML 内直接作答，点击提交
- AI 分析返回：总评分、逐题解析、薄弱知识点、复习建议
- 分析结果保存到 exercises/exercise-xxx-result.json

---

## 7. 界面设计

### 7.1 界面风格

**视觉风格：** 温暖、柔和、亲切

**配色方案：**

| 用途 | 颜色 | 色值 |
|------|------|------|
| 主色调（暖绿） | --color-primary | #588157 |
| 暖橙 | --color-secondary | #d4a373 |
| 暖黄 | --color-accent | #e9c46a |
| 浅米色背景 | --color-bg-primary | #fefae0 |
| 米色背景 | --color-bg-secondary | #f5ebe0 |
| 卡片背景 | --color-bg-card | #ffffff |
| 文字主色 | --color-text-primary | #333333 |
| 文字辅助 | --color-text-secondary | #666666 |

### 7.2 首页布局

左侧固定导航 + 右侧课程列表：

```
┌────────────────────────────────────────────────────────────┐
│  ┌──────────┐  ┌─────────────────────────────────────────┐│
│  │ 📚 课程  │  │  我的课程                              ││
│  │ ──────── │  │  ────────────────────────────────       ││
│  │ 💬 咨询师│  │  ┌───────────┐  ┌───────────┐          ││
│  │ ──────── │  │  │Python入门 │  │React开发   │          ││
│  │ ⚙️ 设置  │  │  │进度: 25%  │  │进度: 10%   │          ││
│  │          │  │  └───────────┘  └───────────┘          ││
│  └──────────┘  └─────────────────────────────────────────┘│
└────────────────────────────────────────────────────────────┘
```

### 7.3 学习界面布局（三栏）

```
┌─────────────────────────────────────────────────────────────────┐
│  顶部栏：课程名称 | 当前章节 | 进度 | 返回按钮                    │
├────────────┬──────────────────────────────┬─────────────────────┤
│            │                              │                     │
│  课程大纲  │         课件展示区           │    教师对话区       │
│            │                              │                     │
│  ✓ 1.1    │  第1课：变量与数据类型        │  💬 教师            │
│  ● 1.2 ←  │  ───────────────────────     │  ────────────       │
│  ○ 1.3    │                              │  老师: 有问题       │
│            │  Python中的变量不需要...     │  随时问我！         │
│  ○ 2.1    │                              │                     │
│            │  x = 10  # 整数              │  我: 变量名         │
│            │                              │  有什么规则？      │
│            │  ───────────────────────     │                     │
│            │  [生成练习题] [向老师提问]    │  [输入框...]       │
│            │                              │  [发送]            │
├────────────┴──────────────────────────────┴─────────────────────┤
│  底部栏：[上一课]                        [下一课]                │
└─────────────────────────────────────────────────────────────────┘
```

### 7.4 练习题界面

- 在课件区展示 HTML 练习题
- 内嵌选择题、填空题、代码编辑框
- 提交后显示 AI 分析结果（评分、解析、建议）

---

## 8. 数据模型

### 8.1 SQLite 数据表

**courses（课程表）**

| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT | 课程唯一ID（UUID） |
| name | TEXT | 课程名称 |
| gitee_repo_url | TEXT | 码云仓库地址 |
| local_path | TEXT | 本地仓库路径 |
| target_level | TEXT | 学习目标 |
| duration | TEXT | 学习时长 |
| teaching_style | TEXT | 教学风格 |
| created_at | DATETIME | 创建时间 |
| status | INTEGER | 状态（进行中/已完成/暂停） |

**chapters（章节表）**

| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT | 章节ID |
| course_id | TEXT | 所属课程ID（外键） |
| chapter_index | INTEGER | 章节序号 |
| name | TEXT | 章节名称 |

**lessons（课时表）**

| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT | 课时ID |
| chapter_id | TEXT | 所属章节ID（外键） |
| lesson_index | INTEGER | 课时序号 |
| name | TEXT | 课时名称 |
| duration | TEXT | 预计时长 |
| status | INTEGER | 状态（未开始/进行中/已完成） |
| completed_at | DATETIME | 完成时间 |
| lesson_file | TEXT | 课件HTML文件路径 |

**lesson_exercises（练习记录表）**

| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT | 练习ID |
| lesson_id | TEXT | 所属课时ID（外键） |
| exercise_file | TEXT | 练习题HTML文件路径 |
| score | INTEGER | 得分 |
| submitted_at | DATETIME | 提交时间 |
| result_file | TEXT | 分析结果JSON文件路径 |

**chat_messages（问答记录表）**

| 字段 | 类型 | 说明 |
|------|------|------|
| id | TEXT | 消息ID |
| course_id | TEXT | 所属课程ID（外键） |
| lesson_id | TEXT | 所属课时ID（可选） |
| agent_type | TEXT | Agent类型（consultant/teacher） |
| role | TEXT | 角色（user/assistant） |
| content | TEXT | 消息内容 |
| created_at | DATETIME | 时间 |

**user_config（用户配置表）**

| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| gitee_username | TEXT | 码云用户名 |
| gitee_token | TEXT | 码云访问令牌（加密） |
| workspace_path | TEXT | 本地工作空间路径 |
| ai_provider | TEXT | AI服务商 |
| ai_api_key | TEXT | API密钥（加密） |
| ai_model | TEXT | 选择的模型 |
| git_username | TEXT | Git用户名 |
| git_email | TEXT | Git邮箱 |

### 8.2 JSON 同步文件

**plan.json（课程计划）**

```json
{
  "courseId": "c-uuid-001",
  "courseName": "Python 编程入门",
  "createdAt": "2026-03-30T10:00:00Z",
  "targetLevel": "能够独立编写简单程序",
  "duration": "3个月",
  "teachingStyle": "实战应用型",
  "chapters": [
    {
      "chapterId": "ch-001",
      "chapterIndex": 1,
      "chapterName": "Python 基础语法",
      "lessons": [
        { "lessonId": "l-001", "lessonIndex": 1, "lessonName": "变量与数据类型", "duration": "30分钟" }
      ]
    }
  ]
}
```

**records.json（学习记录）**

```json
{
  "courseId": "c-uuid-001",
  "updatedAt": "2026-03-30T15:30:00Z",
  "progress": { "currentChapter": 1, "currentLesson": 3, "totalCompleted": 5, "totalLessons": 36, "percentage": 14 },
  "lessonStatus": [
    { "lessonId": "l-001", "status": "completed", "completedAt": "2026-03-28T10:00:00Z" }
  ],
  "exerciseResults": [
    { "lessonId": "l-001", "score": 85, "submittedAt": "2026-03-28T10:30:00Z", "weakPoints": ["变量命名规范"] }
  ],
  "chatHistory": []
}
```

---

## 9. 项目目录结构

```
study-ai-app/
├── src-tauri/                        # Tauri Rust 后端
│   ├── src/
│   │   ├── main.rs                   # 入口
│   │   ├── lib.rs                    # 库导出
│   │   ├── commands/                 # Tauri 命令模块
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs               # 授权相关
│   │   │   ├── config.rs             # 配置相关
│   │   │   ├── git.rs                # Git操作
│   │   │   ├── gitee.rs              # 码云API
│   │   │   ├── database.rs           # 数据库操作
│   │   │   └── sync.rs               # 同步
│   │   ├── services/                 # 业务服务
│   │   │   ├── mod.rs
│   │   │   ├── machine_id.rs         # 机器码生成
│   │   │   ├── crypto.rs             # 加密解密
│   │   │   ├── license.rs            # 授权验证
│   │   │   └── git_ops.rs            # Git操作封装
│   │   ├── models/                   # 数据模型
│   │   │   └── mod.rs
│   │   └── db/                       # 数据库
│   │       ├── mod.rs
│   │       ├── schema.rs
│   │       └── operations.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                              # React 前端
│   ├── main.tsx
│   ├── App.tsx
│   ├── components/
│   │   ├── common/                   # 通用组件
│   │   ├── auth/                     # 授权模块
│   │   ├── setup/                    # 引导配置模块
│   │   ├── home/                     # 首页模块
│   │   ├── consultant/               # 咨询师模块
│   │   ├── learning/                 # 学习模块
│   │   └── settings/                 # 设置模块
│   ├── hooks/
│   ├── services/
│   ├── stores/
│   ├── types/
│   ├── utils/
│   └── styles/
│
├── docs/
│   └── specs/                        # 设计文档
│
├── package.json
├── vite.config.ts
├── tsconfig.json
├── CLAUDE.md                         # 项目说明
└── README.md
```

---

## 10. Tauri 命令列表

### 10.1 授权相关命令

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `get_machine_id_command` | - | `String` | 获取机器码 |
| `get_machine_hash_command` | - | `String` | 获取机器码哈希（SHA256） |
| `validate_license_command` | `license_key: String` | `LicenseResult` | 验证授权密钥 |
| `get_license_status_command` | - | `LicenseResult` | 获取当前授权状态 |
| `is_admin_password_set_command` | - | `bool` | 检查管理员密码是否已设置 |
| `set_admin_password_command` | `password: String` | `()` | 设置管理员密码（首次） |
| `verify_admin_password_command` | `password: String` | `bool` | 验证管理员密码 |
| `change_admin_password_command` | `old_password: String, new_password: String` | `()` | 修改管理员密码 |
| `generate_license_key_command` | `expire_date: String, machine_hash: Option<String>` | `String` | 生成授权密钥 |
| `generate_signing_key_pair_command` | - | `SigningKeyInfo` | 生成签名密钥对 |
| `set_signing_key_command` | `signing_key: String` | `()` | 设置签名私钥（首次） |
| `is_signing_key_set_command` | - | `bool` | 检查签名密钥是否已设置 |

### 10.2 课程数据库命令

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `create_course` | `course: object` | `()` | 创建课程 |
| `get_all_courses` | - | `Vec<Course>` | 获取所有课程 |
| `get_course_by_id` | `id: String` | `Course` | 获取课程详情 |
| `update_course` | `course: object` | `()` | 更新课程 |
| `delete_course` | `id: String` | `()` | 删除课程 |

### 10.3 章节和课时命令

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `create_chapter` | `chapter: object` | `()` | 创建章节 |
| `get_chapters_by_course` | `course_id: String` | `Vec<Chapter>` | 获取课程的所有章节 |
| `update_chapter` | `chapter: object` | `()` | 更新章节 |
| `delete_chapter` | `id: String` | `()` | 删除章节 |
| `create_lesson` | `lesson: object` | `()` | 创建课时 |
| `get_lessons_by_chapter` | `chapter_id: String` | `Vec<Lesson>` | 获取章节的所有课时 |
| `get_lesson_by_id` | `id: String` | `Lesson` | 获取课时详情 |
| `update_lesson_status` | `id: String, status: i32` | `()` | 更新课时状态 |
| `update_lesson` | `lesson: object` | `()` | 更新课时 |
| `delete_lesson` | `id: String` | `()` | 删除课时 |

### 10.4 练习题命令

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `create_exercise` | `exercise: object` | `()` | 创建练习 |
| `get_exercises_by_lesson` | `lesson_id: String` | `Vec<Exercise>` | 获取课时的所有练习 |
| `update_exercise_score` | `id: String, score: i32, result_file: Option<String>` | `()` | 更新练习得分 |
| `delete_exercise` | `id: String` | `()` | 删除练习 |

### 10.5 聊天消息命令

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `create_chat_message` | `message: object` | `()` | 创建聊天消息 |
| `get_chat_messages_by_course` | `course_id: String` | `Vec<ChatMessage>` | 获取课程的所有聊天消息 |
| `get_chat_messages_by_lesson` | `lesson_id: String` | `Vec<ChatMessage>` | 获取课时的所有聊天消息 |
| `delete_chat_message` | `id: String` | `()` | 删除聊天消息 |
| `clear_chat_messages_by_course` | `course_id: String` | `()` | 清除课程的所有聊天消息 |

### 10.6 用户配置命令

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `get_user_config` | - | `UserConfig` | 获取用户配置 |
| `update_user_config` | `config: object` | `()` | 更新用户配置 |

### 10.7 Git 相关命令

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `check_git_installed` | - | `bool` | 检查 Git 是否已安装 |
| `check_git_status` | - | `GitStatus` | 获取 Git 状态（安装情况和版本） |
| `get_git_config` | - | `GitConfig` | 获取 Git 配置 |
| `set_git_username` | `username: String` | `GitResult` | 设置 Git 用户名 |
| `set_git_email` | `email: String` | `GitResult` | 设置 Git 邮箱 |
| `git_init` | `path: String` | `GitResult` | 初始化 Git 仓库 |
| `git_clone` | `url: String, path: String` | `GitResult` | 克隆仓库 |
| `git_commit` | `path: String, message: String` | `GitResult` | 提交更改 |
| `git_push` | `path: String` | `GitResult` | 推送到远程 |
| `git_pull` | `path: String` | `GitResult` | 从远程拉取 |
| `git_has_changes` | `path: String` | `GitResult` | 检查是否有未提交的更改 |

### 10.8 码云相关命令

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `verify_gitee_account` | `username: String, token: String` | `GiteeAccountResult` | 验证码云账户 |
| `create_gitee_repo` | `name: String, description: String` | `GiteeRepo` | 创建码云仓库 |
| `check_gitee_repo_exists` | `owner: String, repo: String` | `bool` | 检查仓库是否存在 |

### 10.9 同步相关命令

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `sync_course_to_git_command` | `course_id: String` | `SyncResult` | 同步课程数据到 Git 仓库 |
| `create_course_repository_command` | `course_id: String` | `SyncResult` | 创建课程仓库（本地 + 码云） |

### 10.10 AI 相关命令

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `ai_chat_command` | 见 `AIChatParams` | `AIResult` | AI 聊天 |
| `ai_generate_lesson_command` | 见 `AIGenerateLessonParams` | `AIResult` | 生成课件 |
| `ai_generate_exercise_command` | 见 `AIGenerateExerciseParams` | `AIResult` | 生成练习题 |
| `ai_analyze_answers_command` | 见 `AIAnalyzeAnswersParams` | `AIResult` | 分析答案 |
| `ai_verify_key_command` | 见 `AIVerifyKeyParams` | `bool` | 验证 API 密钥 |
| `ai_generate_structured_exercise_command` | 见 `AIGenerateStructuredExerciseParams` | `AIStructuredExerciseResult` | 生成结构化练习题 |

---

## 11. 验收标准

### 10.1 功能验收

| 功能 | 验收条件 |
|------|---------|
| 授权系统 | 输入有效密钥且机器码匹配可激活；过期/无效密钥拒绝 |
| 配置引导 | 首次启动完整执行6步引导，配置信息正确保存 |
| Git 配置 | 可检测 Git 是否安装；可配置用户名邮箱 |
| 码云集成 | 可创建仓库、克隆、提交、推送、拉取 |
| 咨询师 Agent | 7步引导流程完整；生成 plan.json 正确 |
| 教师 Agent | 可生成 HTML 课件；可答疑；可生成练习题 |
| 学习界面 | 三栏布局正常；课程大纲可点击跳转 |
| 练习题 | HTML 表单可交互；提交后显示 AI 分析结果 |
| 数据同步 | 关键数据 JSON 同步到码云仓库 |

### 10.2 视觉验收

| 验收项 | 标准 |
|-------|------|
| 配色 | 主色调为暖绿(#588157)、暖橙(#d4a373)、浅米色(#fefae0) |
| 圆角 | 卡片、按钮使用 8px 圆角 |
| 阴影 | 卡片有柔和阴影 |
| 字体 | 清晰可读，无错位 |

---

## 12. 后续计划

### Phase 1: 核心框架搭建
- Tauri 项目初始化
- React + TypeScript + Tailwind 环境配置
- 基础页面框架和路由

### Phase 2: 授权与配置
- 授权系统实现
- 6步引导配置流程

### Phase 3: 码云集成
- Git 操作封装
- 码云 API 对接
- 数据同步机制

### Phase 4: Agent 实现
- 咨询师 Agent（结构化引导）
- 教师 Agent（课件生成、答疑、练习）

### Phase 5: 学习界面
- 三栏布局实现
- 课件渲染
- 练习题交互

---

*文档结束*
