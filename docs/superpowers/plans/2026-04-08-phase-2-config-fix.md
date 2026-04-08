# Phase 2 配置流程修复计划

> **面向 AI 代理的工作者：** 必需子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 逐任务实现此计划。

**目标：** 修复 Phase 2 引导配置流程中的 3 个问题

**架构：** 保持现有 5 步流程，修复配置保存和验证逻辑

**技术栈：** React + TypeScript + Tauri Rust

---

## 文件结构

```
修改文件：
- src/stores/configStore.ts          # 添加 teachingStyle 字段
- src/components/setup/StyleSelectStep.tsx   # 保存风格选择
- src/components/setup/GitSetupStep.tsx      # 调用 git config 命令
- src/components/setup/GiteeSetupStep.tsx     # 验证码云账户
- src/components/setup/AISetupStep.tsx        # 验证 API 密钥
- src-tauri/src/commands/config.rs   # 添加 git 用户名/邮箱命令
```

---

## 任务 1：修复教学风格保存

**文件：**
- 修改：`src/stores/configStore.ts`
- 修改：`src/types/index.ts`
- 修改：`src/components/setup/StyleSelectStep.tsx`

- [ ] **步骤 1：添加 teachingStyle 到类型定义**

```typescript
// src/types/index.ts
export interface UserConfig {
  // ... existing fields
  teachingStyle?: string;  // 新增
}
```

- [ ] **步骤 2：在 configStore 中添加 teachingStyle**

```typescript
// src/stores/configStore.ts
const defaultConfig: UserConfig = {
  // ... existing fields
  teachingStyle: '',
};
```

- [ ] **步骤 3：修改 StyleSelectStep 保存风格**

```typescript
// src/components/setup/StyleSelectStep.tsx
import { useConfigStore } from '../../stores/configStore';

// handleSubmit 修改为：
const handleSubmit = async () => {
  if (!selectedStyle) return;
  setConfig({ teachingStyle: selectedStyle });
  await saveConfig();
  onNext();
};
```

---

## 任务 2：Git 用户信息调用 git config 命令

**文件：**
- 修改：`src-tauri/src/commands/config.rs`
- 修改：`src-tauri/src/services/config.rs`
- 修改：`src/components/setup/GitSetupStep.tsx`

- [ ] **步骤 1：在 config.rs 添加 git 用户名/邮箱字段和持久化**

```rust
// src-tauri/src/services/config.rs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    // ... existing fields
    /// Git 用户名
    pub git_username: Option<String>,
    /// Git 邮箱
    pub git_email: Option<String>,
}
```

- [ ] **步骤 2：修改 GitSetupStep 调用 git config 命令**

```typescript
// src/components/setup/GitSetupStep.tsx
const handleSubmit = async () => {
  if (!username.trim() || !email.trim()) return;

  try {
    // 调用 Rust 命令设置 git 全局配置
    await invoke('set_git_username_command', { username });
    await invoke('set_git_email_command', { email });
    // 同时保存到 configStore
    setConfig({ gitUsername: username, gitEmail: email });
    await saveConfig();
    onNext();
  } catch (error) {
    console.error('Failed to save git config:', error);
  }
};
```

---

## 任务 3：验证码云账户和 AI API 密钥

**文件：**
- 修改：`src/components/setup/GiteeSetupStep.tsx`
- 修改：`src/components/setup/AISetupStep.tsx`

- [ ] **步骤 1：修改 GiteeSetupStep 验证 token 并保存用户名**

```typescript
// src/components/setup/GiteeSetupStep.tsx
import { useConfigStore } from '../../stores/configStore';

export const GiteeSetupStep: React.FC<SetupStepProps> = ({ onNext, onBack }) => {
  const [username, setUsername] = useState('');  // 新增
  const [token, setToken] = useState('');
  // ...

  const handleSubmit = async () => {
    if (!token.trim() || !username.trim()) return;

    setIsValidating(true);
    setError('');

    try {
      // 验证码云账户
      await invoke('verify_gitee_account_command', { username, token });
      setConfig({ giteeUsername: username, giteeToken: token });
      await saveConfig();
      onNext();
    } catch (err) {
      setError('验证码云账户失败，请检查用户名和令牌');
    } finally {
      setIsValidating(false);
    }
  };
};
```

- [ ] **步骤 2：修改 AISetupStep 验证 API 密钥**

```typescript
// src/components/setup/AISetupStep.tsx
const handleSubmit = async () => {
  if (!selectedProvider || !apiKey.trim() || !selectedModel) return;

  setIsValidating(true);
  setError('');

  try {
    // 验证 API 密钥
    const isValid = await invoke<boolean>('ai_verify_key_command', {
      provider: selectedProvider,
      apiKey: apiKey,
      model: selectedModel,
    });

    if (!isValid) {
      setError('API 密钥验证失败，请检查配置');
      setIsValidating(false);
      return;
    }

    setConfig({
      aiProvider: selectedProvider as any,
      aiApiKey: apiKey,
      aiModel: selectedModel,
    });
    await saveConfig();
    onNext();
  } catch (err) {
    setError('配置保存失败');
  } finally {
    setIsValidating(false);
  }
};
```

---

## 任务 4：更新 configStore 处理 giteeUsername

**文件：**
- 修改：`src/stores/configStore.ts`
- 修改：`src/types/index.ts`

- [ ] **步骤 1：添加 giteeUsername 到 UserConfig 类型**

```typescript
// src/types/index.ts
export interface UserConfig {
  // ...
  giteeUsername?: string;  // 新增
}
```

- [ ] **步骤 2：更新 configStore loadConfig**

```typescript
// src/stores/configStore.ts
loadConfig: async () => {
  // ...
  set({
    // ... existing fields
    giteeUsername: config.gitee_username || '',  // 新增
    // ...
  });
},

saveConfig: async () => {
  // ...
  await invoke('set_config_command', {
    config: {
      // ... existing fields
      gitee_username: state.giteeUsername || null,  // 新增
    },
  });
},
```

---

## 执行说明

### 任务执行顺序
1. 任务 1（风格保存）→ 任务 4（类型更新）→ 任务 2（Git）→ 任务 3（Gitee/AI 验证）

### 验证方式
1. 运行 `npm run tauri dev`
2. 观察控制台是否有错误
3. 完成引导流程，检查配置是否正确保存

### 提交信息
```
fix: 完善 Phase 2 引导配置流程

- 添加教学风格保存功能
- GitSetupStep 调用 git config 命令设置全局配置
- GiteeSetupStep 验证码云账户并保存用户名
- AISetupStep 验证 API 密钥有效性
```
