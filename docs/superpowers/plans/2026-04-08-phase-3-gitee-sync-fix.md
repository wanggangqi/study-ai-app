# Phase 3 补充计划：码云同步功能完善

> **面向 AI 代理的工作者：** 必需子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 逐任务实现此计划。步骤使用复选框（`- [ ]`）语法来跟踪进度。

**目标：** 将后端同步函数暴露为 Tauri 命令，使前端能够创建课程仓库并同步学习数据到码云

**架构：** 在 `commands/sync.rs` 中添加 `#[tauri::command]` 包装函数，重命名原有函数为 `*_impl` 后缀，在 `src/services/tauri.ts` 中添加前端调用接口

**技术栈：** Tauri v2 + React + Rust

---

## 背景

Phase 3 码云集成核心功能（Git 操作、码云 API）已实现，但关键问题：
- `sync_course_to_git` 和 `create_course_repository` 是内部函数，未暴露为 Tauri 命令
- 前端无法调用同步功能

---

## 文件清单

**修改：**
- `src-tauri/src/commands/sync.rs` - 添加命令包装函数，重命名原函数
- `src-tauri/src/commands/mod.rs` - 更新导出
- `src-tauri/src/lib.rs` - 注册新的 Tauri 命令
- `src/services/tauri.ts` - 添加前端同步接口
- `src/pages/Consultant.tsx` - 集成课程创建后同步
- `docs/superpowers/specs/2026-03-30-ai-one-on-one-teaching-design.md` - 更新规格文档

---

## 任务 1：暴露同步命令到 Tauri

**文件：**
- 修改：`src-tauri/src/commands/sync.rs`

- [ ] **步骤 1：重命名现有函数为内部实现函数**

将约第 329 行的 `pub fn sync_course_to_git` 改名为 `pub(crate) fn sync_course_to_git_impl`

将约第 409 行的 `pub async fn create_course_repository` 改名为 `pub(crate) async fn create_course_repository_impl`

- [ ] **步骤 2：添加必要的导入**

在 `sync.rs` 顶部添加（如果尚未导入）：

```rust
use crate::db::Database;
use crate::commands::database::DbState;
```

- [ ] **步骤 3：添加 `sync_course_to_git_command` Tauri 命令**

在 `sync.rs` 文件末尾添加：

```rust
/// 同步课程数据到 Git 仓库
#[tauri::command]
pub fn sync_course_to_git_command(
    db: std::sync::Mutex<Database>,
    course_id: String,
) -> Result<SyncResult, String> {
    let db_state = DbState(db);
    crate::commands::sync::sync_course_to_git_impl(&db_state, &course_id)
        .map_err(|e| e.to_string())
}
```

- [ ] **步骤 4：添加 `create_course_repository_command` Tauri 命令**

在 `sync.rs` 文件末尾添加：

```rust
/// 创建课程仓库（本地 + 码云）
#[tauri::command]
pub async fn create_course_repository_command(
    db: std::sync::Mutex<Database>,
    course_id: String,
) -> Result<SyncResult, String> {
    let db_state = DbState(db);
    crate::commands::sync::create_course_repository_impl(&db_state, &course_id)
        .await
        .map_err(|e| e.to_string())
}
```

- [ ] **步骤 5：更新 `commands/mod.rs` 导出**

找到 `pub use sync::` 部分，更新为：

```rust
pub use sync::{
    // 移除旧的内部函数导出（如果之前有）
    // sync_course_to_git,
    // create_course_repository,
    // 添加新的命令
    sync_course_to_git_command,
    create_course_repository_command,
    SyncResult,
    SyncError,
    CoursePlan,
    ChapterPlan,
    LessonPlan,
    LearningRecords,
};
```

- [ ] **步骤 6：在 `lib.rs` 中注册新命令**

在 `invoke_handler` 的 `generate_handler![]` 宏中添加：

```rust
// 同步相关命令
sync_course_to_git_command,
create_course_repository_command,
```

- [ ] **步骤 7：验证 Rust 编译**

```bash
cd D:\wgq_ai\study-ai-app\src-tauri && cargo check
```

预期：无编译错误。如果有错误，根据错误信息调整导入或函数签名

- [ ] **步骤 8：Commit**

```bash
git add src-tauri/src/commands/sync.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "fix: expose sync commands as Tauri commands for frontend"
```

---

## 任务 2：前端同步接口

**文件：**
- 修改：`src/services/tauri.ts`

- [ ] **步骤 1：在 `tauriService` 对象中添加同步方法**

在 `updateExerciseScore` 方法后添加：

```typescript
// 码云同步相关
async syncCourseToGitee(courseId: string): Promise<{ success: boolean; message: string; repoUrl?: string }> {
  return invoke('sync_course_to_git_command', { courseId });
},

async createCourseRepository(courseId: string): Promise<{ success: boolean; message: string; repoUrl?: string }> {
  return invoke('create_course_repository_command', { courseId });
},
```

- [ ] **步骤 2：验证 TypeScript 编译**

```bash
cd D:\wgq_ai\study-ai-app && npx tsc --noEmit
```

预期：无 TypeScript 错误

- [ ] **步骤 3：Commit**

```bash
git add src/services/tauri.ts
git commit -m "feat: add gitee sync interfaces to frontend"
```

---

## 任务 3：课程创建流程集成

**文件：**
- 修改：`src/pages/Consultant.tsx` 或相关组件

- [ ] **步骤 1：找到课程创建逻辑**

在咨询师 Agent 完成课程计划创建后，找到调用 `tauriService.saveCourse` 或类似方法的位置

- [ ] **步骤 2：在课程创建成功后调用码云仓库创建**

在课程创建成功的回调中添加：

```typescript
import { tauriService } from '../services/tauri';

// 课程创建成功后
const handleCourseCreated = async (course: Course) => {
  try {
    // 创建码云仓库并同步
    const result = await tauriService.createCourseRepository(course.id);
    if (result.success && result.repoUrl) {
      console.log('课程仓库创建成功:', result.repoUrl);
    } else if (!result.success) {
      console.warn('码云同步失败:', result.message);
      // 可选：显示警告提示，允许用户稍后重试
    }
  } catch (error) {
    console.error('码云同步失败:', error);
    // 错误不影响课程创建，允许用户稍后手动同步
  }
};
```

- [ ] **步骤 3：Commit**

```bash
git add src/pages/Consultant.tsx
git commit -m "feat: integrate gitee repo creation in course creation flow"
```

---

## 任务 4：规格文档更新

**文件：**
- 修改：`docs/superpowers/specs/2026-03-30-ai-one-on-one-teaching-design.md`

- [ ] **步骤 1：在 10.7 或新增 10.9 节添加同步命令文档**

在设计文档 Tauri 命令列表末尾添加：

```markdown
### 10.9 同步相关命令

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `sync_course_to_git_command` | `course_id: String` | `SyncResult` | 同步课程数据到 Git 仓库 |
| `create_course_repository_command` | `course_id: String` | `SyncResult` | 创建课程仓库（本地 + 码云） |
```

- [ ] **步骤 2：Commit**

```bash
git add docs/superpowers/specs/2026-03-30-ai-one-on-one-teaching-design.md
git commit -m "docs: add sync commands to Tauri command spec"
```

---

## 验收标准

- [ ] `sync_course_to_git_command` 可从前端调用（通过 invoke）
- [ ] `create_course_repository_command` 可从前端调用（通过 invoke）
- [ ] 创建课程后自动创建码云仓库
- [ ] 可手动触发同步学习进度到码云
- [ ] Rust 编译通过（cargo check）
- [ ] TypeScript 编译通过（tsc --noEmit）
- [ ] 规格文档已更新

---

## 备注

1. **DbState 传递方式**：审查发现原计划直接传递 `State<DbState>` 有类型不匹配问题。正确方式是通过 `std::sync::Mutex<Database>` 中间类型转换

2. **create_course_repository 失败处理**：当前实现如果创建远程仓库后推送失败，会导致不一致状态。这是已知的简化，可在后续迭代中改进

3. **gitee_repo_url 更新**：命令内部已处理更新 `course.gitee_repo_url`，无需额外步骤
