# Phase 5: 学习界面实现计划

## 概述

实现学习界面核心功能，包括三栏布局完善、课件渲染和练习题交互系统。

## 任务列表

### 任务 1: 类型扩展和存储完善

**目标**: 添加 Phase 5 所需的类型定义和存储扩展

**具体需求**:
1. 添加 `Exercise` 类型定义（题目、选项、答案、分析结果）
2. 添加 `CourseContent` 类型（课件 HTML 内容）
3. 扩展 `courseStore`:
   - 添加 `currentLessonContent` 存储当前课件内容
   - 添加 `exercises` 存储练习题列表
   - 添加 `setLessonContent` action
   - 添加 `setExercises` action
   - 添加 `submitExerciseAnswer` action
4. 扩展 `useCourse` hook 导出新功能

**文件**:
- `src/types/index.ts` - 添加 Exercise, CourseContent 类型
- `src/stores/courseStore.ts` - 扩展 store
- `src/hooks/useCourse.ts` - 扩展 hook

---

### 任务 2: 三栏布局完善

**目标**: 完善学习界面的三栏布局，添加交互功能

**具体需求**:
1. 左侧课程大纲:
   - 点击课时加载对应课件
   - 当前课时高亮显示
   - 显示当前章节/课时进度
   - 点击事件调用 `selectLesson`

2. 中间课件区:
   - 显示当前课件 HTML 内容（使用 `dangerouslySetInnerHTML`）
   - 底部添加「生成练习题」和「向老师提问」按钮
   - 添加上一课/下一课导航
   - 课时完成时标记状态

3. 右侧聊天区:
   - 集成 TeacherAgent 组件
   - 显示当前课程/课时上下文
   - 保持聊天历史

**文件**:
- `src/pages/Learning.tsx` - 完善布局和交互

---

### 任务 3: 课件渲染系统

**目标**: 实现课件内容的存储、加载和展示

**具体需求**:
1. 扩展 `courseStore` 添加课件内容管理:
   - `lessonContents: Record<string, string>` 存储课件 HTML
   - `setLessonContent(lessonId, content)` action
   - `getLessonContent(lessonId)` selector

2. 课件加载逻辑:
   - 点击课时时检查是否有缓存
   - 如无缓存，调用 AI 生成或从本地加载
   - 课件存储到 store 和本地文件

3. 课件展示组件:
   - 创建 `CoursewareViewer` 组件
   - 支持 HTML 内容渲染
   - 添加样式隔离（防止 HTML 样式污染）

**新文件**:
- `src/components/learning/CoursewareViewer.tsx`

**修改**:
- `src/stores/courseStore.ts` - 添加课件内容管理
- `src/pages/Learning.tsx` - 集成 CoursewareViewer

---

### 任务 4: 练习题交互系统

**目标**: 实现练习题生成、答题和结果分析

**具体需求**:
1. 练习题类型 `Exercise`:
   ```typescript
   interface Exercise {
     id: string;
     lessonId: string;
     type: 'choice' | 'fill' | 'code';
     question: string;
     options?: string[];
     correctAnswer: string | string[];
     userAnswer?: string;
     score?: number;
     analysis?: string;
   }
   ```

2. 练习题生成:
   - 「生成练习题」按钮触发 AI 生成
   - 生成 3-5 道选择题/填空题
   - 保存到 `exercises` store

3. 练习题展示组件:
   - 创建 `ExercisePanel` 组件
   - 支持选择题（单选/多选）、填空题、代码编辑框
   - 答题后提交给 AI 分析

4. 答案分析:
   - 调用 AI 分析答案
   - 返回评分、解析、薄弱点建议
   - 保存分析结果

**新文件**:
- `src/components/learning/ExercisePanel.tsx`

**修改**:
- `src/stores/courseStore.ts` - 添加练习题状态管理
- `src/pages/Learning.tsx` - 集成 ExercisePanel

---

### 任务 5: 聊天答疑集成

**目标**: 将 TeacherAgent 集成到学习界面

**具体需求**:
1. 创建学习专用的聊天容器组件 `LearningChat`:
   - 包装 TeacherAgent
   - 自动传入当前课程/课时上下文
   - 自定义样式适配三栏布局

2. 聊天功能:
   - 用户输入问题
   - AI 结合当前课件内容回答
   - 保持对话历史
   - 保存聊天记录到 store

3. 快捷问题按钮:
   - 「生成练习题」
   - 「解释这个概念」
   - 「给我一个例子」

**新文件**:
- `src/components/learning/LearningChat.tsx`

**修改**:
- `src/pages/Learning.tsx` - 集成 LearningChat

---

### 任务 6: 导航和状态管理

**目标**: 实现课时间的导航和状态同步

**具体需求**:
1. 上一课/下一课导航:
   - 在 CoursewareViewer 底部添加导航按钮
   - 点击加载相邻课时
   - 到达边界时禁用按钮

2. 进度追踪:
   - 课件浏览完成标记课时为 `in_progress`
   - 练习题得分 > 60 标记为 `completed`
   - 更新 `currentCourse.progress`

3. 状态持久化:
   - 课时状态变化时同步到后端
   - 刷新页面恢复状态

**修改**:
- `src/components/learning/CoursewareViewer.tsx` - 添加导航
- `src/stores/courseStore.ts` - 添加状态持久化逻辑
- `src/pages/Learning.tsx` - 状态同步

---

## 技术要点

1. **HTML 渲染安全**: 使用 `dompurify` 清理用户内容
2. **样式隔离**: 课件内容使用独立的 CSS 作用域
3. **响应式**: 三栏布局在小屏幕上自适应（可折叠）

## 验收标准

- [ ] 点击课程大纲中的课时，中间区域显示对应课件
- [ ] 课件内容支持 HTML 渲染，样式不污染全局
- [ ] 「生成练习题」按钮生成 3-5 道题目
- [ ] 练习题可以作答并提交
- [ ] 提交后显示 AI 分析结果（评分、解析）
- [ ] 右侧聊天区可以向 AI 提问
- [ ] AI 回答结合当前课件上下文
- [ ] 上一课/下一课导航正常工作
- [ ] 课时状态正确更新（未开始→进行中→已完成）
- [ ] 整体布局符合设计规格中的三栏布局图
