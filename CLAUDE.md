# 智学伴侣 (StudyMate) - AI 一对一教学应用

## 项目概述

基于 Tauri 的本地 AI 一对一教学应用，让学生根据自己想学习的内容、目标、花费时长和喜欢的教学风格，获得个性化的 AI 教学服务。

## 技术栈

- **桌面框架**: Tauri v2.x
- **前端**: React 18 + TypeScript + Vite
- **状态管理**: Zustand
- **UI组件**: shadcn/ui + Tailwind CSS（温暖风格）
- **后端**: Rust
- **本地数据库**: SQLite (rusqlite)
- **AI服务**: Claude / OpenAI / 通义千问 / DeepSeek / 智谱GLM / MiniMax / Kimi (Moonshot)
- **远程仓库**: 码云 (Gitee)

## 核心功能模块

| 模块 | 说明 |
|------|------|
| 授权系统 | 密钥激活 + 机器绑定 + 有效期验证 |
| 配置引导 | Git安装、码云账户、本地工作空间、AI服务商设置 |
| 咨询师Agent | 结构化引导收集需求，生成课程计划JSON |
| 教师Agent | 生成HTML课件、答疑、练习题、AI分析答案 |
| 学习界面 | 三栏布局（课程大纲 \| 课件展示 \| 聊天答疑） |
| 码云同步 | 每课程独立仓库，全文件同步 |

## 设计文档

详细设计规格说明: `docs/superpowers/specs/2026-03-30-ai-one-on-one-teaching-design.md`

## 项目结构

```
study-ai-app/
├── src-tauri/          # Rust后端
│   ├── src/commands/   # Tauri命令
│   ├── src/services/   # 业务服务
│   └── src/db/         # 数据库
├── src/                # React前端
│   ├── components/     # 组件
│   ├── hooks/          # 自定义hooks
│   ├── stores/         # 状态管理
│   └── types/          # 类型定义
└── docs/               # 文档
```

## 快速开始

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 构建
npm run tauri build
```

## 注意事项

- 前端代码在 `src/`，后端代码在 `src-tauri/src/`
- 组件开发使用 React + TypeScript
- 后端命令在 `src-tauri/src/commands/` 目录下添加
- 数据库操作在 `src-tauri/src/db/` 目录下实现
