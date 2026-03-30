# Phase 1: 项目框架搭建 实现计划

> **面向 AI 代理的工作者：** 必需子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 逐任务实现此计划。步骤使用复选框（`- [ ]`）语法来跟踪进度。

**目标：** 初始化 Tauri + React + TypeScript 项目，搭建基础目录结构和路由框架

**架构：** 使用 Tauri v2 作为桌面框架，React 18 + TypeScript 作为前端框架，Vite 作为构建工具。应用采用 SPA 路由，组件按功能模块划分目录。

**技术栈：** Tauri v2 / React 18 / TypeScript / Vite / React Router / Zustand / Tailwind CSS / shadcn/ui

---

## 文件结构

```
study-ai-app/
├── src-tauri/                           # [新建] Tauri Rust 后端
│   ├── src/
│   │   ├── main.rs                      # [新建] 入口
│   │   ├── lib.rs                       # [新建] 库导出
│   │   ├── commands/                    # [新建] Tauri 命令
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── config.rs
│   │   │   ├── git.rs
│   │   │   ├── gitee.rs
│   │   │   ├── database.rs
│   │   │   └── sync.rs
│   │   ├── services/                    # [新建] 业务服务
│   │   │   ├── mod.rs
│   │   │   ├── machine_id.rs
│   │   │   ├── crypto.rs
│   │   │   ├── license.rs
│   │   │   └── git_ops.rs
│   │   ├── models/                      # [新建] 数据模型
│   │   │   └── mod.rs
│   │   └── db/                          # [新建] 数据库
│   │       ├── mod.rs
│   │       ├── schema.rs
│   │       └── operations.rs
│   ├── Cargo.toml                       # [新建] Rust 依赖
│   └── tauri.conf.json                  # [新建] Tauri 配置
│
├── src/                                 # [新建] React 前端
│   ├── main.tsx                         # [新建] 入口
│   ├── App.tsx                          # [新建] 根组件
│   ├── components/                      # [新建] 组件目录
│   │   ├── common/                      # [新建] 通用组件
│   │   │   ├── Button.tsx
│   │   │   ├── Input.tsx
│   │   │   ├── Card.tsx
│   │   │   ├── Modal.tsx
│   │   │   └── Sidebar.tsx
│   │   ├── auth/                        # [新建] 授权模块
│   │   │   ├── AuthPage.tsx
│   │   │   └── AuthInput.tsx
│   │   ├── setup/                       # [新建] 引导配置模块
│   │   │   ├── SetupWizard.tsx
│   │   │   ├── GitSetupStep.tsx
│   │   │   ├── GiteeSetupStep.tsx
│   │   │   ├── WorkspaceStep.tsx
│   │   │   ├── AISetupStep.tsx
│   │   │   └── StyleSelectStep.tsx
│   │   ├── home/                        # [新建] 首页模块
│   │   │   ├── HomePage.tsx
│   │   │   ├── CourseList.tsx
│   │   │   ├── CourseCard.tsx
│   │   │   └── Navigation.tsx
│   │   ├── consultant/                  # [新建] 咨询师模块
│   │   │   ├── ConsultantPage.tsx
│   │   │   ├── ChatPanel.tsx
│   │   │   ├── StepIndicator.tsx
│   │   │   └── PlanPreview.tsx
│   │   ├── learning/                    # [新建] 学习模块
│   │   │   ├── LearningPage.tsx
│   │   │   ├── CourseOutline.tsx
│   │   │   ├── LessonViewer.tsx
│   │   │   ├── TeacherChat.tsx
│   │   │   ├── ExercisePanel.tsx
│   │   │   └── AnalysisPanel.tsx
│   │   └── settings/                     # [新建] 设置模块
│   │       ├── SettingsPage.tsx
│   │       ├── GitSettings.tsx
│   │       ├── AISettings.tsx
│   │       └── StyleSettings.tsx
│   ├── hooks/                           # [新建] 自定义hooks
│   │   ├── useAuth.ts
│   │   ├── useConfig.ts
│   │   ├── useCourse.ts
│   │   ├── useChat.ts
│   │   └── useAI.ts
│   ├── services/                         # [新建] 前端服务
│   │   ├── tauri.ts
│   │   ├── ai.ts
│   │   └── storage.ts
│   ├── stores/                          # [新建] 状态管理
│   │   ├── authStore.ts
│   │   ├── configStore.ts
│   │   ├── courseStore.ts
│   │   └── chatStore.ts
│   ├── types/                           # [新建] 类型定义
│   │   ├── course.ts
│   │   ├── lesson.ts
│   │   ├── config.ts
│   │   └── ai.ts
│   ├── utils/                           # [新建] 工具函数
│   │   ├── format.ts
│   │   ├── validation.ts
│   │   └── encryption.ts
│   ├── styles/                          # [新建] 样式目录
│   │   ├── global.css
│   │   ├── variables.css
│   │   └── components.css
│   └── pages/                            # [新建] 页面组件
│       ├── Auth.tsx
│       ├── Setup.tsx
│       ├── Home.tsx
│       ├── Consultant.tsx
│       ├── Learning.tsx
│       └── Settings.tsx
│
├── package.json                         # [新建] 前端依赖
├── vite.config.ts                       # [新建] Vite 配置
├── tsconfig.json                        # [新建] TypeScript 配置
├── tailwind.config.js                   # [新建] Tailwind 配置
├── postcss.config.js                    # [新建] PostCSS 配置
└── index.html                           # [新建] HTML 入口
```

---

## 任务列表

### 任务 1：初始化 Tauri 项目

**文件：**
- 创建：项目根目录初始化

- [ ] **步骤 1：检查环境**

运行：`node --version && npm --version && rustc --version && cargo --version`
预期：显示 Node.js、npm、Rust、 Cargo 版本信息

- [ ] **步骤 2：创建 Tauri 项目**

运行：`npm create tauri-app@latest study-ai-app -- --template react-ts --manager npm`
注意：在项目已存在的情况下，需要手动创建文件结构

- [ ] **步骤 3：验证项目结构**

运行：`ls -la study-ai-app/src-tauri && ls -la study-ai-app/src`
预期：显示 Tauri 和 React 目录结构

- [ ] **步骤 4：安装前端依赖**

运行：`cd study-ai-app && npm install`
预期：package.json 中的依赖安装成功

- [ ] **步骤 5：验证开发模式启动**

运行：`cd study-ai-app && npm run tauri dev -- --no-watch`
超时：120000ms
预期：Tauri 窗口成功打开，显示默认页面

- [ ] **步骤 6：Commit**

```bash
git add -A
git commit -m "feat: 初始化 Tauri + React + TypeScript 项目"
```

---

### 任务 2：配置 Tailwind CSS 和 shadcn/ui

**文件：**
- 创建：tailwind.config.js, postcss.config.js
- 修改：src/styles/global.css

- [ ] **步骤 1：安装 Tailwind CSS 依赖**

运行：`cd study-ai-app && npm install -D tailwindcss postcss autoprefixer`
预期：依赖安装成功

- [ ] **步骤 2：初始化 Tailwind 配置**

运行：`npx tailwindcss init -p`
预期：生成 tailwind.config.js 和 postcss.config.js

- [ ] **步骤 3：配置 Tailwind**

修改 `study-ai-app/tailwind.config.js`：

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: '#588157',
        secondary: '#d4a373',
        accent: '#e9c46a',
        'bg-primary': '#fefae0',
        'bg-secondary': '#f5ebe0',
      },
      borderRadius: {
        'sm': '4px',
        'md': '8px',
        'lg': '12px',
      },
    },
  },
  plugins: [],
}
```

- [ ] **步骤 4：配置 PostCSS**

修改 `study-ai-app/postcss.config.js`：

```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

- [ ] **步骤 5：创建全局样式**

创建 `study-ai-app/src/styles/variables.css`：

```css
:root {
  --color-primary: #588157;
  --color-secondary: #d4a373;
  --color-accent: #e9c46a;
  --color-bg-primary: #fefae0;
  --color-bg-secondary: #f5ebe0;
  --color-bg-card: #ffffff;
  --color-text-primary: #333333;
  --color-text-secondary: #666666;
  --color-text-muted: #999999;
  --color-border: #e0dcd3;
  --radius-sm: 4px;
  --radius-md: 8px;
  --radius-lg: 12px;
  --shadow-sm: 0 1px 2px rgba(0,0,0,0.05);
  --shadow-md: 0 2px 4px rgba(0,0,0,0.1);
  --shadow-lg: 0 4px 8px rgba(0,0,0,0.15);
}
```

创建 `study-ai-app/src/styles/global.css`：

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@import './variables.css';

body {
  font-family: 'Source Han Sans', 'Microsoft YaHei', sans-serif;
  background-color: var(--color-bg-primary);
  color: var(--color-text-primary);
  margin: 0;
  padding: 0;
}

@layer components {
  .btn-primary {
    @apply bg-primary text-white px-4 py-2 rounded-md hover:opacity-90 transition-opacity;
  }

  .btn-secondary {
    @apply bg-secondary text-white px-4 py-2 rounded-md hover:opacity-90 transition-opacity;
  }

  .card {
    @apply bg-white rounded-lg shadow-md p-4;
  }

  .input {
    @apply border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary;
  }
}
```

- [ ] **步骤 6：安装 shadcn/ui 依赖**

运行：`cd study-ai-app && npm install -D @types/node`
预期：类型定义安装成功

- [ ] **步骤 7：初始化 shadcn/ui**

运行：`cd study-ai-app && npx shadcn@latest init -d`
预期：显示初始化配置提示，shadcn-ui 配置文件创建成功

- [ ] **步骤 8：添加基础组件**

运行：`cd study-ai-app && npx shadcn@latest add button card input label select`
预期：组件安装成功，创建以下文件：
- src/components/ui/button.tsx
- src/components/ui/card.tsx
- src/components/ui/input.tsx
- src/components/ui/label.tsx
- src/components/ui/select.tsx

- [ ] **步骤 9：Commit**

```bash
git add -A
git commit -m "feat: 配置 Tailwind CSS 和样式变量"
```

---

### 任务 2.5：创建 Hooks 和 Services 基础文件

**文件：**
- 创建：src/hooks/*.ts, src/services/*.ts

- [ ] **步骤 1：创建 hooks 目录结构**

创建 `study-ai-app/src/hooks/index.ts`：

```typescript
export { useAuth } from './useAuth';
export { useConfig } from './useConfig';
export { useCourse } from './useCourse';
export { useChat } from './useChat';
export { useAI } from './useAI';
```

创建 `study-ai-app/src/hooks/useAuth.ts`：

```typescript
import { useAuthStore } from '../stores/authStore';

export const useAuth = () => {
  const { isAuthorized, expireAt, setAuthorized, clearAuth, checkAuth } = useAuthStore();

  return {
    isAuthorized,
    expireAt,
    setAuthorized,
    clearAuth,
    checkAuth,
  };
};
```

创建 `study-ai-app/src/hooks/useConfig.ts`：

```typescript
import { useConfigStore } from '../stores/configStore';

export const useConfig = () => {
  const config = useConfigStore();
  return config;
};
```

创建 `study-ai-app/src/hooks/useCourse.ts`：

```typescript
import { useCourseStore } from '../stores/courseStore';

export const useCourse = () => {
  const {
    courses,
    currentCourse,
    currentChapter,
    currentLesson,
    setCourses,
    addCourse,
    selectCourse,
    selectChapter,
    selectLesson,
    updateLessonStatus,
  } = useCourseStore();

  return {
    courses,
    currentCourse,
    currentChapter,
    currentLesson,
    setCourses,
    addCourse,
    selectCourse,
    selectChapter,
    selectLesson,
    updateLessonStatus,
  };
};
```

创建 `study-ai-app/src/hooks/useChat.ts`：

```typescript
import { useChatStore } from '../stores/chatStore';

export const useChat = () => {
  const {
    consultantMessages,
    teacherMessages,
    addConsultantMessage,
    addTeacherMessage,
    clearConsultantMessages,
    clearTeacherMessages,
    setTeacherMessages,
  } = useChatStore();

  return {
    consultantMessages,
    teacherMessages,
    addConsultantMessage,
    addTeacherMessage,
    clearConsultantMessages,
    clearTeacherMessages,
    setTeacherMessages,
  };
};
```

创建 `study-ai-app/src/hooks/useAI.ts`：

```typescript
// AI API 调用 hook - 完整实现在 Phase 4
export const useAI = () => {
  const sendMessage = async (messages: any[]) => {
    // TODO: Phase 4 实现
    console.log('AI sendMessage called');
  };

  return { sendMessage };
};
```

- [ ] **步骤 2：创建 services 目录结构**

创建 `study-ai-app/src/services/index.ts`：

```typescript
export { tauriService } from './tauri';
export { aiService } from './ai';
export { storageService } from './storage';
```

创建 `study-ai-app/src/services/tauri.ts`：

```typescript
import { invoke } from '@tauri-apps/api/core';

// Tauri 命令调用封装
export const tauriService = {
  // 授权相关
  async validateLicense(key: string): Promise<boolean> {
    return invoke('validate_license', { key });
  },

  // 配置相关
  async getMachineId(): Promise<string> {
    return invoke('get_machine_id');
  },

  // Git 相关
  async checkGitInstalled(): Promise<boolean> {
    return invoke('check_git_installed');
  },

  async setGitConfig(username: string, email: string): Promise<void> {
    return invoke('set_git_config', { username, email });
  },

  // 码云相关
  async createGiteeRepo(name: string, description: string): Promise<string> {
    return invoke('create_gitee_repo', { name, description });
  },

  async cloneRepo(url: string, path: string): Promise<void> {
    return invoke('clone_repo', { url, path });
  },

  // 数据库相关
  async getCourses(): Promise<any[]> {
    return invoke('get_courses');
  },

  async saveCourse(course: any): Promise<void> {
    return invoke('save_course', { course });
  },
};
```

创建 `study-ai-app/src/services/ai.ts`：

```typescript
// AI 服务封装 - 完整实现在 Phase 4
export type AIProvider = 'claude' | 'openai' | 'qwen' | 'deepseek' | 'glm' | 'minimax' | 'kimi';

interface AIMessage {
  role: 'user' | 'assistant';
  content: string;
}

export const aiService = {
  async chat(provider: AIProvider, apiKey: string, model: string, messages: AIMessage[]): Promise<string> {
    // TODO: Phase 4 实现多服务商适配
    console.log('AI chat called', { provider, model, messages });
    return 'AI response placeholder';
  },

  async generateLesson(provider: AIProvider, apiKey: string, context: any): Promise<string> {
    // TODO: Phase 4 实现课件生成
    return '<html><body><h1>Placeholder Lesson</h1></body></html>';
  },

  async generateExercise(provider: AIProvider, apiKey: string, context: any): Promise<string> {
    // TODO: Phase 4 实现练习题生成
    return '<html><body><h1>Placeholder Exercise</h1></body></html>';
  },

  async analyzeExercise(provider: AIProvider, apiKey: string, exercise: any, answers: any): Promise<any> {
    // TODO: Phase 4 实现练习分析
    return { score: 100, feedback: 'Placeholder feedback' };
  },
};
```

创建 `study-ai-app/src/services/storage.ts`：

```typescript
// 本地存储服务
const STORAGE_KEYS = {
  AUTH_STATE: 'studymate_auth',
  CONFIG: 'studymate_config',
  COURSES: 'studymate_courses',
} as const;

export const storageService = {
  get<T>(key: string, defaultValue: T): T {
    const item = localStorage.getItem(key);
    return item ? JSON.parse(item) : defaultValue;
  },

  set<T>(key: string, value: T): void {
    localStorage.setItem(key, JSON.stringify(value));
  },

  remove(key: string): void {
    localStorage.removeItem(key);
  },

  clear(): void {
    Object.values(STORAGE_KEYS).forEach((key) => localStorage.removeItem(key));
  },
};
```

- [ ] **步骤 3：Commit**

```bash
git add -A
git commit -m "feat: 创建 hooks 和 services 基础文件"
```

---

### 任务 3：配置 Zustand 状态管理

**文件：**
- 创建：src/stores/authStore.ts, src/stores/configStore.ts, src/stores/courseStore.ts, src/stores/chatStore.ts

- [ ] **步骤 1：安装 Zustand**

运行：`cd study-ai-app && npm install zustand`
预期：Zustand 安装成功

- [ ] **步骤 2：创建类型定义**

创建 `study-ai-app/src/types/index.ts`：

```typescript
// AI 服务商类型
export type AIProvider = 'claude' | 'openai' | 'qwen' | 'deepseek' | 'glm' | 'minimax' | 'kimi';

// 授权状态
export interface AuthState {
  isAuthorized: boolean;
  expireAt: string | null;
  machineHash: string | null;
}

// 用户配置
export interface UserConfig {
  giteeUsername: string;
  giteeToken: string;
  workspacePath: string;
  aiProvider: AIProvider;
  aiApiKey: string;
  aiModel: string;
  gitUsername: string;
  gitEmail: string;
}

// 课程
export interface Course {
  id: string;
  name: string;
  giteeRepoUrl: string;
  localPath: string;
  targetLevel: string;
  duration: string;
  teachingStyle: string;
  createdAt: string;
  status: 'active' | 'completed' | 'paused';
  progress?: number; // 0-100，计算得出的学习进度
  totalLessons?: number; // 总课时数
  completedLessons?: number; // 已完成课时数
}

// 章节
export interface Chapter {
  id: string;
  courseId: string;
  chapterIndex: number;
  name: string;
}

// 课时
export interface Lesson {
  id: string;
  chapterId: string;
  lessonIndex: number;
  name: string;
  duration: string;
  status: 'not_started' | 'in_progress' | 'completed';
  completedAt?: string;
  lessonFile?: string;
}

// 消息
export interface ChatMessage {
  id: string;
  courseId: string;
  lessonId?: string;
  agentType: 'consultant' | 'teacher';
  role: 'user' | 'assistant';
  content: string;
  createdAt: string;
}

// 教学风格
export interface TeachingStyle {
  id: string;
  name: string;
  description: string;
  icon: string;
}
```

- [ ] **步骤 3：创建授权 Store**

创建 `study-ai-app/src/stores/authStore.ts`：

```typescript
import { create } from 'zustand';
import { AuthState } from '../types';

interface AuthStore extends AuthState {
  setAuthorized: (expireAt: string, machineHash: string) => void;
  clearAuth: () => void;
  checkAuth: () => boolean;
}

export const useAuthStore = create<AuthStore>((set, get) => ({
  isAuthorized: false,
  expireAt: null,
  machineHash: null,

  setAuthorized: (expireAt, machineHash) => {
    set({ isAuthorized: true, expireAt, machineHash });
  },

  clearAuth: () => {
    set({ isAuthorized: false, expireAt: null, machineHash: null });
  },

  checkAuth: () => {
    const state = get();
    if (!state.isAuthorized || !state.expireAt) return false;
    return new Date(state.expireAt) > new Date();
  },
}));
```

- [ ] **步骤 4：创建配置 Store**

创建 `study-ai-app/src/stores/configStore.ts`：

```typescript
import { create } from 'zustand';
import { UserConfig, AIProvider } from '../types';

interface ConfigStore extends UserConfig {
  isSetupComplete: boolean;
  setConfig: (config: Partial<UserConfig>) => void;
  setSetupComplete: (complete: boolean) => void;
  resetConfig: () => void;
}

const defaultConfig: UserConfig = {
  giteeUsername: '',
  giteeToken: '',
  workspacePath: '',
  aiProvider: 'claude',
  aiApiKey: '',
  aiModel: '',
  gitUsername: '',
  gitEmail: '',
};

export const useConfigStore = create<ConfigStore>((set) => ({
  ...defaultConfig,
  isSetupComplete: false,

  setConfig: (config) => set((state) => ({ ...state, ...config })),

  setSetupComplete: (complete) => set({ isSetupComplete: complete }),

  resetConfig: () => set({ ...defaultConfig, isSetupComplete: false }),
}));
```

- [ ] **步骤 5：创建课程 Store**

创建 `study-ai-app/src/stores/courseStore.ts`：

```typescript
import { create } from 'zustand';
import { Course, Chapter, Lesson } from '../types';

interface CourseStore {
  courses: Course[];
  currentCourse: Course | null;
  currentChapter: Chapter | null;
  currentLesson: Lesson | null;

  setCourses: (courses: Course[]) => void;
  addCourse: (course: Course) => void;
  selectCourse: (courseId: string) => void;
  selectChapter: (chapter: Chapter | null) => void;
  selectLesson: (lesson: Lesson | null) => void;
  updateLessonStatus: (lessonId: string, status: Lesson['status']) => void;
}

export const useCourseStore = create<CourseStore>((set, get) => ({
  courses: [],
  currentCourse: null,
  currentChapter: null,
  currentLesson: null,

  setCourses: (courses) => set({ courses }),

  addCourse: (course) => set((state) => ({ courses: [...state.courses, course] })),

  selectCourse: (courseId) => {
    const course = get().courses.find((c) => c.id === courseId);
    set({ currentCourse: course || null, currentChapter: null, currentLesson: null });
  },

  selectChapter: (chapter) => set({ currentChapter: chapter }),

  selectLesson: (lesson) => set({ currentLesson: lesson }),

  updateLessonStatus: (lessonId, status) => {
    // 注意：课程和章节数据分开存储，这里只更新当前课程上下文中的课时状态
    // 实际的数据库更新通过 Tauri 命令处理
    set((state) => ({
      currentLesson: state.currentLesson?.id === lessonId
        ? { ...state.currentLesson, status, completedAt: status === 'completed' ? new Date().toISOString() : undefined }
        : state.currentLesson,
    }));
  },
}));
```

- [ ] **步骤 6：创建聊天 Store**

创建 `study-ai-app/src/stores/chatStore.ts`：

```typescript
import { create } from 'zustand';
import { ChatMessage } from '../types';

interface ChatStore {
  consultantMessages: ChatMessage[];
  teacherMessages: ChatMessage[];

  addConsultantMessage: (message: Omit<ChatMessage, 'id' | 'createdAt'>) => void;
  addTeacherMessage: (message: Omit<ChatMessage, 'id' | 'createdAt'>) => void;
  clearConsultantMessages: () => void;
  clearTeacherMessages: () => void;
  setTeacherMessages: (messages: ChatMessage[]) => void;
}

export const useChatStore = create<ChatStore>((set) => ({
  consultantMessages: [],
  teacherMessages: [],

  addConsultantMessage: (message) =>
    set((state) => ({
      consultantMessages: [
        ...state.consultantMessages,
        {
          ...message,
          id: crypto.randomUUID(),
          createdAt: new Date().toISOString(),
        },
      ],
    })),

  addTeacherMessage: (message) =>
    set((state) => ({
      teacherMessages: [
        ...state.teacherMessages,
        {
          ...message,
          id: crypto.randomUUID(),
          createdAt: new Date().toISOString(),
        },
      ],
    })),

  clearConsultantMessages: () => set({ consultantMessages: [] }),
  clearTeacherMessages: () => set({ teacherMessages: [] }),
  setTeacherMessages: (messages) => set({ teacherMessages: messages }),
}));
```

- [ ] **步骤 7：Commit**

```bash
git add -A
git commit -m "feat: 配置 Zustand 状态管理"
```

---

### 任务 4：创建基础 UI 组件

**文件：**
- 创建：src/components/common/*.tsx
- 创建：src/pages/*.tsx

- [ ] **步骤 1：创建通用组件 - Button**

创建 `study-ai-app/src/components/common/Button.tsx`：

```tsx
import React from 'react';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'outline';
  size?: 'sm' | 'md' | 'lg';
}

export const Button: React.FC<ButtonProps> = ({
  children,
  variant = 'primary',
  size = 'md',
  className = '',
  ...props
}) => {
  const baseStyles = 'font-medium rounded-md transition-all duration-200 disabled:opacity-50';

  const variantStyles = {
    primary: 'bg-primary text-white hover:bg-primary/90',
    secondary: 'bg-secondary text-white hover:bg-secondary/90',
    outline: 'border-2 border-primary text-primary hover:bg-primary/10',
  };

  const sizeStyles = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg',
  };

  return (
    <button
      className={`${baseStyles} ${variantStyles[variant]} ${sizeStyles[size]} ${className}`}
      {...props}
    >
      {children}
    </button>
  );
};
```

- [ ] **步骤 2：创建通用组件 - Input**

创建 `study-ai-app/src/components/common/Input.tsx`：

```tsx
import React from 'react';

interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
}

export const Input: React.FC<InputProps> = ({
  label,
  error,
  className = '',
  ...props
}) => {
  return (
    <div className="flex flex-col gap-1">
      {label && (
        <label className="text-sm font-medium text-gray-700">{label}</label>
      )}
      <input
        className={`border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent ${
          error ? 'border-red-500' : ''
        } ${className}`}
        {...props}
      />
      {error && <span className="text-sm text-red-500">{error}</span>}
    </div>
  );
};
```

- [ ] **步骤 3：创建通用组件 - Card**

创建 `study-ai-app/src/components/common/Card.tsx`：

```tsx
import React from 'react';

interface CardProps {
  children: React.ReactNode;
  className?: string;
  onClick?: () => void;
}

export const Card: React.FC<CardProps> = ({ children, className = '', onClick }) => {
  return (
    <div
      className={`bg-white rounded-lg shadow-md p-4 ${
        onClick ? 'cursor-pointer hover:shadow-lg transition-shadow' : ''
      } ${className}`}
      onClick={onClick}
    >
      {children}
    </div>
  );
};
```

- [ ] **步骤 4：创建通用组件 - Sidebar**

创建 `study-ai-app/src/components/common/Sidebar.tsx`：

```tsx
import React from 'react';

interface NavItem {
  icon: string;
  label: string;
  path: string;
  badge?: string;
}

interface SidebarProps {
  items: NavItem[];
  activePath: string;
  onNavigate: (path: string) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ items, activePath, onNavigate }) => {
  return (
    <div className="w-48 bg-bg-secondary min-h-screen p-4 flex flex-col gap-2">
      {items.map((item) => (
        <button
          key={item.path}
          onClick={() => onNavigate(item.path)}
          className={`flex items-center gap-3 px-4 py-3 rounded-md text-left transition-colors ${
            activePath === item.path
              ? 'bg-white text-primary font-medium shadow-sm'
              : 'text-text-secondary hover:bg-white/50'
          }`}
        >
          <span className="text-lg">{item.icon}</span>
          <span className="flex-1">{item.label}</span>
          {item.badge && (
            <span className="bg-accent text-text-primary text-xs px-2 py-0.5 rounded-full">
              {item.badge}
            </span>
          )}
        </button>
      ))}
    </div>
  );
};
```

- [ ] **步骤 5：创建页面组件 - 首页**

创建 `study-ai-app/src/pages/Home.tsx`：

```tsx
import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Sidebar } from '../components/common/Sidebar';
import { Card } from '../components/common/Card';
import { Button } from '../components/common/Button';
import { useCourseStore } from '../stores/courseStore';

const navItems = [
  { icon: '📚', label: '课程', path: '/' },
  { icon: '💬', label: '咨询师', path: '/consultant' },
  { icon: '⚙️', label: '设置', path: '/settings' },
];

export const HomePage: React.FC = () => {
  const navigate = useNavigate();
  const { courses } = useCourseStore();

  return (
    <div className="flex min-h-screen">
      <Sidebar items={navItems} activePath="/" onNavigate={(path) => navigate(path)} />

      <main className="flex-1 p-8">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-2xl font-bold text-primary">我的课程</h1>
          <Button onClick={() => navigate('/consultant')}>+ 创建新课程</Button>
        </div>

        {courses.length === 0 ? (
          <Card className="text-center py-12">
            <p className="text-text-muted mb-4">还没有任何课程</p>
            <Button onClick={() => navigate('/consultant')}>前往咨询师创建课程</Button>
          </Card>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {courses.map((course) => (
              <Card key={course.id} className="hover:shadow-lg transition-shadow">
                <h3 className="font-bold text-lg text-primary mb-2">{course.name}</h3>
                <p className="text-sm text-text-secondary mb-2">目标：{course.targetLevel}</p>
                <p className="text-xs text-text-muted mb-4">时长：{course.duration}</p>
                <div className="mb-4">
                  <div className="h-2 bg-bg-secondary rounded-full overflow-hidden">
                    <div
                      className="h-full bg-primary rounded-full transition-all duration-300"
                      style={{ width: `${course.progress || 0}%` }}
                    />
                  </div>
                  <span className="text-xs text-primary mt-1">进度 {course.progress || 0}%</span>
                </div>
                <Button
                  className="w-full"
                  onClick={() => navigate(`/learning/${course.id}`)}
                >
                  {(course.progress || 0) > 0 ? '继续学习' : '开始学习'}
                </Button>
              </Card>
            ))}
          </div>
        )}
      </main>
    </div>
  );
};
```

- [ ] **步骤 6：创建页面组件 - 咨询师**

创建 `study-ai-app/src/pages/Consultant.tsx`：

```tsx
import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Sidebar } from '../components/common/Sidebar';
import { Card } from '../components/common/Card';
import { Button } from '../components/common/Button';

const navItems = [
  { icon: '📚', label: '课程', path: '/' },
  { icon: '💬', label: '咨询师', path: '/consultant' },
  { icon: '⚙️', label: '设置', path: '/settings' },
];

export const ConsultantPage: React.FC = () => {
  const navigate = useNavigate();

  return (
    <div className="flex min-h-screen">
      <Sidebar items={navItems} activePath="/consultant" onNavigate={(path) => navigate(path)} />

      <main className="flex-1 p-8">
        <h1 className="text-2xl font-bold text-primary mb-8">学习咨询师</h1>

        <Card className="max-w-2xl mx-auto">
          <div className="text-center mb-6">
            <div className="text-4xl mb-4">👨‍🏫</div>
            <h2 className="text-xl font-bold mb-2">你好！我是你的学习顾问</h2>
            <p className="text-text-secondary">
              我来帮你制定个性化的学习计划。请告诉我你想学习什么内容？
            </p>
          </div>

          <div className="space-y-4">
            <div className="border border-gray-200 rounded-md p-4 hover:border-primary cursor-pointer transition-colors">
              <h3 className="font-medium">我想学习 Python 编程</h3>
              <p className="text-sm text-text-muted">从零开始，想做数据分析</p>
            </div>

            <div className="border border-gray-200 rounded-md p-4 hover:border-primary cursor-pointer transition-colors">
              <h3 className="font-medium">我想学习 React 开发</h3>
              <p className="text-sm text-text-muted">有一些前端基础，想做项目</p>
            </div>

            <div className="border border-gray-200 rounded-md p-4 hover:border-primary cursor-pointer transition-colors">
              <h3 className="font-medium">我想学习英语</h3>
              <p className="text-sm text-text-muted">提升口语能力</p>
            </div>
          </div>

          <div className="mt-6 text-center text-text-muted text-sm">
            或者输入你想学习的内容...
          </div>
        </Card>
      </main>
    </div>
  );
};
```

- [ ] **步骤 7：创建页面组件 - 设置**

创建 `study-ai-app/src/pages/Settings.tsx`：

```tsx
import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Sidebar } from '../components/common/Sidebar';
import { Card } from '../components/common/Card';
import { Input } from '../components/common/Input';
import { Button } from '../components/common/Button';
import { useConfigStore } from '../stores/configStore';

const navItems = [
  { icon: '📚', label: '课程', path: '/' },
  { icon: '💬', label: '咨询师', path: '/consultant' },
  { icon: '⚙️', label: '设置', path: '/settings' },
];

const aiProviders = [
  { id: 'claude', name: 'Claude (Anthropic)' },
  { id: 'openai', name: 'ChatGPT (OpenAI)' },
  { id: 'qwen', name: '通义千问 (阿里云)' },
  { id: 'deepseek', name: 'DeepSeek' },
  { id: 'glm', name: '智谱 GLM' },
  { id: 'minimax', name: 'MiniMax' },
  { id: 'kimi', name: 'Kimi (Moonshot)' },
];

export const SettingsPage: React.FC = () => {
  const navigate = useNavigate();
  const { aiProvider, setConfig } = useConfigStore();

  return (
    <div className="flex min-h-screen">
      <Sidebar items={navItems} activePath="/settings" onNavigate={(path) => navigate(path)} />

      <main className="flex-1 p-8">
        <h1 className="text-2xl font-bold text-primary mb-8">设置</h1>

        <div className="max-w-2xl space-y-6">
          <Card>
            <h2 className="font-bold mb-4">AI 服务配置</h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-2">选择 AI 服务商</label>
                <select
                  className="w-full border border-gray-300 rounded-md px-3 py-2"
                  value={aiProvider}
                  onChange={(e) => setConfig({ aiProvider: e.target.value as any })}
                >
                  {aiProviders.map((p) => (
                    <option key={p.id} value={p.id}>{p.name}</option>
                  ))}
                </select>
              </div>
              <Input
                label="API 密钥"
                type="password"
                placeholder="输入你的 API 密钥"
              />
            </div>
          </Card>

          <Card>
            <h2 className="font-bold mb-4">Git 配置</h2>
            <div className="space-y-4">
              <Input label="Git 用户名" placeholder="你的 Git 用户名" />
              <Input label="Git 邮箱" placeholder="你的 Git 邮箱" />
            </div>
          </Card>

          <div className="flex justify-end gap-4">
            <Button variant="outline">取消</Button>
            <Button>保存设置</Button>
          </div>
        </div>
      </main>
    </div>
  );
};
```

- [ ] **步骤 8：Commit**

```bash
git add -A
git commit -m "feat: 创建基础 UI 组件和页面框架"
```

---

### 任务 5：配置 React Router 路由

**文件：**
- 创建：src/App.tsx (更新)
- 创建：src/main.tsx (更新)

- [ ] **步骤 1：安装 React Router**

运行：`cd study-ai-app && npm install react-router-dom`
预期：React Router 安装成功

- [ ] **步骤 2：更新入口文件**

修改 `study-ai-app/src/main.tsx`：

```tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import App from './App';
import './styles/global.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>
);
```

- [ ] **步骤 3：更新 App 组件**

修改 `study-ai-app/src/App.tsx`：

```tsx
import React from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { HomePage } from './pages/Home';
import { ConsultantPage } from './pages/Consultant';
import { SettingsPage } from './pages/Settings';
import { AuthPage } from './pages/Auth';
import { SetupPage } from './pages/Setup';
import { LearningPage } from './pages/Learning';
import { useAuthStore } from './stores/authStore';

const ProtectedRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { isAuthorized, expireAt } = useAuthStore();
  const isValid = isAuthorized && expireAt && new Date(expireAt) > new Date();

  if (!isValid) {
    return <Navigate to="/auth" replace />;
  }

  return <>{children}</>;
};

const App: React.FC = () => {
  return (
    <Routes>
      <Route path="/auth" element={<AuthPage />} />
      <Route
        path="/setup"
        element={
          <ProtectedRoute>
            <SetupPage />
          </ProtectedRoute>
        }
      />
      <Route
        path="/"
        element={
          <ProtectedRoute>
            <HomePage />
          </ProtectedRoute>
        }
      />
      <Route
        path="/consultant"
        element={
          <ProtectedRoute>
            <ConsultantPage />
          </ProtectedRoute>
        }
      />
      <Route
        path="/settings"
        element={
          <ProtectedRoute>
            <SettingsPage />
          </ProtectedRoute>
        }
      />
      <Route
        path="/learning/:courseId"
        element={
          <ProtectedRoute>
            <LearningPage />
          </ProtectedRoute>
        }
      />
    </Routes>
  );
};

export default App;
```

- [ ] **步骤 4：创建占位页面**

创建 `study-ai-app/src/pages/Auth.tsx`：

```tsx
import React from 'react';

export const AuthPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-bg-primary flex items-center justify-center">
      <div className="bg-white rounded-lg shadow-lg p-8 w-full max-w-md">
        <h1 className="text-2xl font-bold text-center mb-6">智学伴侣</h1>
        <p className="text-center text-text-muted mb-6">AI 一对一教学应用</p>
        <input
          type="text"
          placeholder="请输入授权密钥"
          className="w-full border border-gray-300 rounded-md px-4 py-3 mb-4"
        />
        <button className="w-full bg-primary text-white rounded-md px-4 py-3 font-medium hover:bg-primary/90">
          激活
        </button>
      </div>
    </div>
  );
};
```

创建 `study-ai-app/src/pages/Setup.tsx`：

```tsx
import React from 'react';

export const SetupPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-bg-primary p-8">
      <h1 className="text-2xl font-bold text-primary mb-8 text-center">配置向导</h1>
      <p className="text-center text-text-muted">配置向导页面 - 待实现</p>
    </div>
  );
};
```

创建 `study-ai-app/src/pages/Learning.tsx`：

```tsx
import React from 'react';

export const LearningPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-bg-primary p-8">
      <h1 className="text-2xl font-bold text-primary mb-8 text-center">学习页面</h1>
      <p className="text-center text-text-muted">学习页面 - 待实现</p>
    </div>
  );
};
```

- [ ] **步骤 5：验证路由工作**

运行：`cd study-ai-app && npm run tauri dev -- --no-watch`
超时：120000ms
预期：应用启动，可以访问 /auth 页面

- [ ] **步骤 6：Commit**

```bash
git add -A
git commit -m "feat: 配置 React Router 路由"
```

---

## Phase 1 完成总结

完成 Phase 1 后，你将拥有：

- ✅ 可运行的 Tauri + React + TypeScript 项目
- ✅ Tailwind CSS 和温暖风格配色
- ✅ Zustand 状态管理（4个 Store）
- ✅ 基础 UI 组件（Button, Input, Card, Sidebar）
- ✅ 页面框架（Home, Consultant, Settings, Auth, Setup, Learning）
- ✅ React Router 路由配置

---

**下一步：** Phase 2 - 授权与配置系统

准备好继续 Phase 2 了吗？
