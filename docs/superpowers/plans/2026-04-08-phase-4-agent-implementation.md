# Phase 4 实现计划：Agent 核心功能

> **面向 AI 代理的工作者：** 必需子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 逐任务实现此计划。步骤使用复选框（`- [ ]`）语法来跟踪进度。

**目标：** 实现咨询师 Agent 7步引导流程（生成结构化章节/课时计划）并完善教师 Agent 三大功能（课件生成、练习题生成、答疑）

**架构：** 咨询师 Agent 调用 AI 生成完整课程大纲（章节+课时），存储到数据库；教师 Agent 在学习界面中提供课件/练习/答疑三大功能

**技术栈：** React + TypeScript + Tauri + Rust + SQLite

---

## 背景

Phase 3 完成后，码云同步已打通。当前状态：
- `ConsultantAgent` 只有5个问题，缺少"欢迎"和"确认"步骤，且不生成章节/课时结构
- `TeacherAgent` 已实现但与 `LearningPage` 三栏布局的集成需要完善
- 课程创建后没有生成章节和课时数据

**设计规格要求的 plan.json 结构：**
```json
{
  "courseId": "c-uuid-001",
  "courseName": "Python 编程入门",
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

---

## 文件清单

**新建：**
- `src/hooks/useConsultant.ts` - 咨询师 Agent 专用 Hook（生成课程大纲）

**修改：**
- `src/components/consultant/ConsultantAgent.tsx` - 扩展为7步流程，调用 AI 生成章节/课时
- `src/components/consultant/ConsultantAgent.css` - 新增样式
- `src/pages/Consultant.tsx` - 简化，委托给 ConsultantAgent
- `src/services/tauri.ts` - 添加 `generateCoursePlanCommand` 接口
- `src-tauri/src/commands/sync.rs` - 添加 `generate_course_plan_command` Tauri 命令
- `src-tauri/src/lib.rs` - 注册新命令
- `src/components/learning/CoursewareViewer.tsx` - 完善课件加载和展示
- `src/stores/courseStore.ts` - 添加 `loadCourseChapters` 方法
- `src/types/index.ts` - 扩展 `CoursePlan` 类型

---

## 任务 1：后端 AI 课程大纲生成命令

**文件：**
- 修改：`src-tauri/src/commands/sync.rs`
- 修改：`src-tauri/src/lib.rs`

- [ ] **步骤 1：在 `sync.rs` 末尾添加 AI 生成课程大纲命令**

```rust
use serde::{Deserialize, Serialize};

/// 课程计划大纲（AI 生成输出）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoursePlanOutline {
    pub course_name: String,
    pub target_level: String,
    pub duration: String,
    pub teaching_style: String,
    pub chapters: Vec<ChapterPlanOutline>,
}

/// 章节大纲
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterPlanOutline {
    pub chapter_index: i32,
    pub chapter_name: String,
    pub lessons: Vec<LessonPlanOutline>,
}

/// 课时大纲
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonPlanOutline {
    pub lesson_index: i32,
    pub lesson_name: String,
    pub duration: String,
}

/// AI 生成课程大纲命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct AIGenerateCoursePlanParams {
    pub provider: String,
    pub api_key: String,
    pub model: Option<String>,
    pub course_name: String,
    pub target_level: String,
    pub duration: String,
    pub teaching_style: String,
    pub base_knowledge: String,
}

/// AI 生成课程大纲命令
#[tauri::command]
pub fn ai_generate_course_plan_command(
    params: AIGenerateCoursePlanParams,
) -> Result<CoursePlanOutline, String> {
    use crate::services::AIProvider;
    use crate::services::AIConfig;
    use crate::services::generate_course_plan;

    let provider = match params.provider.to_lowercase().as_str() {
        "claude" => AIProvider::Claude,
        "openai" => AIProvider::OpenAI,
        "qwen" => AIProvider::Qwen,
        "deepseek" => AIProvider::DeepSeek,
        "glm" => AIProvider::Glm,
        "minimax" => AIProvider::MiniMax,
        "kimi" => AIProvider::Kimi,
        _ => return Err(format!("不支持的 AI 服务商: {}", params.provider)),
    };

    let config = AIConfig::new(provider, params.api_key);
    let config = if let Some(model) = params.model {
        config.with_model(model)
    } else {
        config
    };

    let result = crate::services::ai::generate_course_plan(
        &config,
        &params.course_name,
        &params.target_level,
        &params.duration,
        &params.teaching_style,
        &params.base_knowledge,
    );

    match result {
        Ok(plan) => Ok(plan),
        Err(e) => Err(e.to_string()),
    }
}
```

- [ ] **步骤 2：在 `src-tauri/src/services/ai.rs` 中实现 `generate_course_plan` 函数**

首先查看现有 `ai.rs` 文件了解 AI 服务调用模式：

```rust
/// AI 生成课程大纲
pub async fn generate_course_plan(
    config: &AIConfig,
    course_name: &str,
    target_level: &str,
    duration: &str,
    teaching_style: &str,
    base_knowledge: &str,
) -> Result<CoursePlanOutline, Box<dyn std::error::Error + Send + Sync>> {
    let system_prompt = r#"你是一位专业的课程规划师。根据用户的学习需求，生成一个结构化的课程大纲。
课程大纲应包含：
- course_name: 课程名称
- target_level: 目标水平
- duration: 学习时长
- teaching_style: 教学风格
- chapters: 章节数组，每个章节包含：
  - chapter_index: 章节序号
  - chapter_name: 章节名称
  - lessons: 课时数组，每个课时包含：
    - lesson_index: 课时序号
    - lesson_name: 课时名称
    - duration: 预计时长

请根据学习时长合理安排课时数量，确保知识体系完整。

请只返回 JSON 格式，不要包含其他文字。""#;

    let user_prompt = format!(
        "学习目标：{}\n目标水平：{}\n学习时长：{}\n教学风格：{}\n已有基础：{}",
        course_name, target_level, duration, teaching_style, base_knowledge
    );

    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: user_prompt,
        },
    ];

    let response = chat(config, messages).await?;
    let plan: CoursePlanOutline = serde_json::from_str(&response)
        .map_err(|e| format!("JSON 解析失败: {}，原始响应: {}", e, response))?;
    Ok(plan)
}
```

同时在 `ai.rs` 顶部添加新的数据结构导入（如果尚未定义在顶部）。

- [ ] **步骤 3：在 `src-tauri/src/lib.rs` 中注册新命令**

在 `invoke_handler` 的 `generate_handler![]` 宏中添加：
```rust
ai_generate_course_plan_command,
```

- [ ] **步骤 4：验证 Rust 编译**

```bash
cd D:\wgq_ai\study-ai-app\src-tauri && cargo check
```

预期：无编译错误

- [ ] **步骤 5：Commit**

```bash
git add src-tauri/src/commands/sync.rs src-tauri/src/services/ai.rs src-tauri/src/lib.rs
git commit -m "feat: add AI course plan generation command"
```

---

## 任务 2：前端课程大纲生成接口

**文件：**
- 修改：`src/services/tauri.ts`
- 修改：`src/types/index.ts`

- [ ] **步骤 1：在 `types/index.ts` 中添加课程大纲类型**

```typescript
// 课时计划大纲
export interface LessonPlanOutline {
  lessonIndex: number;
  lessonName: string;
  duration: string;
}

// 章节计划大纲
export interface ChapterPlanOutline {
  chapterIndex: number;
  chapterName: string;
  lessons: LessonPlanOutline[];
}

// 课程计划大纲（AI 生成）
export interface CoursePlanOutline {
  courseName: string;
  targetLevel: string;
  duration: string;
  teachingStyle: string;
  chapters: ChapterPlanOutline[];
}
```

- [ ] **步骤 2：在 `tauriService` 中添加生成课程大纲方法**

在 `tauri.ts` 的 `tauriService` 对象中添加：

```typescript
async generateCoursePlan(params: {
  provider: AIProvider;
  apiKey: string;
  model?: string;
  courseName: string;
  targetLevel: string;
  duration: string;
  teachingStyle: string;
  baseKnowledge: string;
}): Promise<CoursePlanOutline> {
  return invoke('ai_generate_course_plan_command', { params });
},
```

- [ ] **步骤 3：验证 TypeScript 编译**

```bash
cd D:\wgq_ai\study-ai-app && npx tsc --noEmit
```

预期：无 TypeScript 错误

- [ ] **步骤 4：Commit**

```bash
git add src/services/tauri.ts src/types/index.ts
git commit -m "feat: add course plan generation interface to frontend"
```

---

## 任务 3：咨询师 Agent 7步流程

**文件：**
- 创建：`src/hooks/useConsultant.ts`
- 修改：`src/components/consultant/ConsultantAgent.tsx`
- 修改：`src/components/consultant/ConsultantAgent.css`
- 修改：`src/pages/Consultant.tsx`

- [ ] **步骤 1：创建 `useConsultant.ts` Hook**

```typescript
import { useState, useCallback } from 'react';
import { useConfigStore } from '../stores/configStore';
import { tauriService } from '../services/tauri';
import type { CoursePlanOutline } from '../types';

interface UseConsultantReturn {
  isLoading: boolean;
  error: string | null;
  generateCoursePlan: (answers: Record<string, string>) => Promise<CoursePlanOutline | null>;
}

export function useConsultant(): UseConsultantReturn {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const config = useConfigStore();

  const generateCoursePlan = useCallback(async (answers: Record<string, string>): Promise<CoursePlanOutline | null> => {
    setIsLoading(true);
    setError(null);

    try {
      const result = await tauriService.generateCoursePlan({
        provider: config.aiProvider,
        apiKey: config.aiApiKey,
        model: config.aiModel || undefined,
        courseName: answers.goal || '',
        targetLevel: answers.level || '',
        duration: answers.duration || '',
        teachingStyle: answers.style || '',
        baseKnowledge: answers.base || '无',
      });

      return result;
    } catch (err) {
      setError(String(err));
      return null;
    } finally {
      setIsLoading(false);
    }
  }, [config.aiProvider, config.aiApiKey, config.aiModel]);

  return { isLoading, error, generateCoursePlan };
}
```

- [ ] **步骤 2：重写 `ConsultantAgent.tsx` 为7步流程**

将现有的5步扩展为7步（欢迎、追问[原base合并到追问]、目标、时长、风格、确认、生成）：

```typescript
import React, { useState, useCallback } from 'react';
import { useConfigStore } from '../../stores/configStore';
import { CoursePlan, CoursePlanOutline } from '../../types';
import { tauriService } from '../../services/tauri';
import './ConsultantAgent.css';

// 7步咨询问题配置
const CONSULT_STEPS = [
  {
    id: 'welcome',
    question: '你好！我来帮你制定学习计划。',
    description: '我是你的学习咨询师，可以根据你的需求为你定制专属课程',
    type: 'welcome' as const,
  },
  {
    id: 'goal',
    question: '你想学习什么内容？',
    description: '请描述你想学习的具体内容或领域',
    placeholder: '例如：Python 编程、机器学习、英语口语、设计模式...',
    type: 'input' as const,
  },
  {
    id: 'level',
    question: '你目前的基础水平是？',
    description: '这有助于我为你定制合适的学习路径',
    options: ['零基础', '入门', '初级', '中级', '高级'],
    type: 'select' as const,
  },
  {
    id: 'duration',
    question: '你计划多长时间完成学习？',
    description: '根据目标设定合理的学习周期',
    options: ['1周内', '1个月内', '3个月内', '半年内', '不着急'],
    type: 'select' as const,
  },
  {
    id: 'style',
    question: '你喜欢什么样的教学风格？',
    description: '选择你最喜欢的教学风格',
    styleOptions: [
      { id: '严谨学术型', description: '结构严谨、概念清晰', icon: '📚' },
      { id: '实战应用型', description: '案例驱动、边学边做', icon: '💻' },
      { id: '轻松故事型', description: '故事/比喻、语言轻松', icon: '🎭' },
      { id: '循序渐进型', description: '小步前进、充分练习', icon: '📈' },
      { id: '启发探索型', description: '提问引导、独立思考', icon: '💡' },
      { id: '快速高效型', description: '精炼要点、直奔目标', icon: '⚡' },
    ],
    type: 'style' as const,
  },
  {
    id: 'base',
    question: '你有哪些相关基础？',
    description: '请描述你已有的相关知识或经验（可选）',
    placeholder: '例如：学过 HTML/CSS，了解变量和循环概念...',
    type: 'input' as const,
  },
  {
    id: 'confirm',
    question: '请确认你的学习需求',
    description: '以下是系统为你生成的学习计划',
    type: 'confirm' as const,
  },
];

export function ConsultantAgent({ onCoursePlanGenerated }: {
  onCoursePlanGenerated?: (plan: CoursePlanOutline) => void;
}) {
  const [currentStep, setCurrentStep] = useState(0);
  const [answers, setAnswers] = useState<Record<string, string>>({});
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [coursePlan, setCoursePlan] = useState<CoursePlanOutline | null>(null);
  const config = useConfigStore();

  const currentQuestion = CONSULT_STEPS[currentStep];

  const handleAnswer = useCallback((answer: string) => {
    setAnswers((prev) => ({ ...prev, [currentQuestion.id]: answer }));
    setError(null);
  }, [currentQuestion.id]);

  const handleNext = useCallback(async () => {
    // 欢迎步骤不需要选择
    if (currentQuestion.id !== 'welcome' && !answers[currentQuestion.id]) {
      setError('请选择或输入答案');
      return;
    }

    if (currentStep < CONSULT_STEPS.length - 1) {
      setCurrentStep((prev) => prev + 1);
    } else {
      // 最后一步，生成课程计划
      await generateCoursePlan();
    }
  }, [currentStep, currentQuestion.id, answers]);

  const handleBack = useCallback(() => {
    if (currentStep > 0) {
      setCurrentStep((prev) => prev - 1);
      setError(null);
    }
  }, [currentStep]);

  const generateCoursePlan = async () => {
    setIsLoading(true);
    setError(null);

    try {
      const result = await tauriService.generateCoursePlan({
        provider: config.aiProvider,
        apiKey: config.aiApiKey,
        model: config.aiModel || undefined,
        courseName: answers.goal || '',
        targetLevel: answers.level || '',
        duration: answers.duration || '',
        teachingStyle: answers.style || '',
        baseKnowledge: answers.base || '无',
      });

      if (result) {
        setCoursePlan(result);
        onCoursePlanGenerated?.(result);
      } else {
        setError('生成课程计划失败');
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  };

  const handleRestart = () => {
    setCurrentStep(0);
    setAnswers({});
    setCoursePlan(null);
    setError(null);
  };

  const handleEditAnswer = (stepId: string) => {
    const stepIndex = CONSULT_STEPS.findIndex(s => s.id === stepId);
    if (stepIndex !== -1) {
      setCurrentStep(stepIndex);
    }
  };

  // 渲染确认页面
  const renderConfirmStep = () => {
    return (
      <div className="confirm-step">
        <div className="confirm-summary">
          <div className="summary-item">
            <span className="summary-label">学习内容</span>
            <span className="summary-value">{answers.goal}</span>
            <button className="edit-btn" onClick={() => handleEditAnswer('goal')}>修改</button>
          </div>
          <div className="summary-item">
            <span className="summary-label">基础水平</span>
            <span className="summary-value">{answers.level}</span>
            <button className="edit-btn" onClick={() => handleEditAnswer('level')}>修改</button>
          </div>
          <div className="summary-item">
            <span className="summary-label">学习时长</span>
            <span className="summary-value">{answers.duration}</span>
            <button className="edit-btn" onClick={() => handleEditAnswer('duration')}>修改</button>
          </div>
          <div className="summary-item">
            <span className="summary-label">教学风格</span>
            <span className="summary-value">{answers.style}</span>
            <button className="edit-btn" onClick={() => handleEditAnswer('style')}>修改</button>
          </div>
          {answers.base && (
            <div className="summary-item">
              <span className="summary-label">相关基础</span>
              <span className="summary-value">{answers.base}</span>
              <button className="edit-btn" onClick={() => handleEditAnswer('base')}>修改</button>
            </div>
          )}
        </div>
        <p className="confirm-hint">点击"生成课程计划"开始为你规划学习路径</p>
      </div>
    );
  };

  // 渲染课程计划结果
  const renderCoursePlanResult = () => {
    if (!coursePlan) return null;

    return (
      <div className="course-plan-result">
        <div className="result-header">
          <h2>课程计划已生成</h2>
          <p>根据你的需求，系统为你定制了以下学习计划</p>
        </div>

        <div className="plan-card">
          <div className="plan-header">
            <h3>{coursePlan.courseName}</h3>
            <span className="plan-duration">{coursePlan.duration}</span>
          </div>

          <div className="plan-meta">
            <span>目标：{coursePlan.targetLevel}</span>
            <span>风格：{coursePlan.teachingStyle}</span>
          </div>

          <div className="plan-outline">
            <h4>课程大纲</h4>
            {coursePlan.chapters.map((chapter) => (
              <div key={chapter.chapterIndex} className="chapter-item">
                <div className="chapter-name">
                  第{chapter.chapterIndex}章：{chapter.chapterName}
                </div>
                <div className="lesson-list">
                  {chapter.lessons.map((lesson) => (
                    <div key={lesson.lessonIndex} className="lesson-item">
                      <span className="lesson-index">{lesson.lessonIndex}</span>
                      <span className="lesson-name">{lesson.lessonName}</span>
                      <span className="lesson-duration">{lesson.duration}</span>
                    </div>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="result-actions">
          <button className="btn-secondary" onClick={handleRestart}>
            重新制定
          </button>
          <button className="btn-primary">开始学习</button>
        </div>
      </div>
    );
  };

  // 渲染欢迎步骤
  const renderWelcomeStep = () => (
    <div className="welcome-step">
      <div className="welcome-icon">👋</div>
      <h2>{currentQuestion.question}</h2>
      <p>{currentQuestion.description}</p>
    </div>
  );

  // 渲染输入型问题
  const renderInputQuestion = () => (
    <div className="input-question">
      <h2 className="question-text">{currentQuestion.question}</h2>
      <p className="question-desc">{currentQuestion.description}</p>
      <textarea
        className="answer-textarea"
        placeholder={currentQuestion.placeholder}
        value={answers[currentQuestion.id] || ''}
        onChange={(e) => handleAnswer(e.target.value)}
      />
    </div>
  );

  // 渲染选择型问题
  const renderSelectQuestion = () => (
    <div className="select-question">
      <h2 className="question-text">{currentQuestion.question}</h2>
      <p className="question-desc">{currentQuestion.description}</p>
      <div className="option-grid">
        {currentQuestion.options?.map((option) => (
          <button
            key={option}
            className={`option-btn ${answers[currentQuestion.id] === option ? 'selected' : ''}`}
            onClick={() => handleAnswer(option)}
          >
            {option}
          </button>
        ))}
      </div>
    </div>
  );

  // 渲染风格选择
  const renderStyleQuestion = () => (
    <div className="style-question">
      <h2 className="question-text">{currentQuestion.question}</h2>
      <p className="question-desc">{currentQuestion.description}</p>
      <div className="style-grid">
        {currentQuestion.styleOptions?.map((style) => (
          <button
            key={style.id}
            className={`style-card ${answers[currentQuestion.id] === style.id ? 'selected' : ''}`}
            onClick={() => handleAnswer(style.id)}
          >
            <span className="style-icon">{style.icon}</span>
            <span className="style-name">{style.id}</span>
            <span className="style-desc">{style.description}</span>
          </button>
        ))}
      </div>
    </div>
  );

  const renderCurrentStep = () => {
    switch (currentQuestion.type) {
      case 'welcome':
        return renderWelcomeStep();
      case 'input':
        return renderInputQuestion();
      case 'select':
        return renderSelectQuestion();
      case 'style':
        return renderStyleQuestion();
      case 'confirm':
        return renderConfirmStep();
      default:
        return null;
    }
  };

  // 如果已有课程计划结果，显示结果页面
  if (coursePlan) {
    return renderCoursePlanResult();
  }

  return (
    <div className="consultant-agent">
      <div className="progress-bar">
        <div
          className="progress-fill"
          style={{ width: `${((currentStep + 1) / CONSULT_STEPS.length) * 100}%` }}
        />
      </div>

      <div className="step-indicator">
        {currentQuestion.id === 'welcome' ? '开始' : `${currentStep} / ${CONSULT_STEPS.length - 1}`}
      </div>

      <div className="question-section">
        {renderCurrentStep()}

        {error && <div className="error-message">{error}</div>}
      </div>

      <div className="navigation-buttons">
        {currentStep > 0 && currentQuestion.id !== 'welcome' && (
          <button className="btn-secondary" onClick={handleBack}>
            上一步
          </button>
        )}
        <button
          className="btn-primary"
          onClick={handleNext}
          disabled={isLoading || (currentQuestion.id !== 'welcome' && !answers[currentQuestion.id])}
        >
          {isLoading ? '生成中...' :
            currentQuestion.id === 'confirm' ? '生成课程计划' :
            currentStep === 0 ? '开始' :
            '下一步'}
        </button>
      </div>
    </div>
  );
}
```

- [ ] **步骤 3：更新 `ConsultantAgent.css` 样式**

```css
.consultant-agent {
  max-width: 700px;
  margin: 0 auto;
  padding: 24px;
}

.welcome-step {
  text-align: center;
  padding: 40px 20px;
}

.welcome-icon {
  font-size: 64px;
  margin-bottom: 20px;
}

.welcome-step h2 {
  font-size: 24px;
  color: #333;
  margin-bottom: 12px;
}

.welcome-step p {
  color: #666;
  font-size: 16px;
}

.confirm-step {
  padding: 10px 0;
}

.confirm-summary {
  background: #f8f8f8;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 16px;
}

.summary-item {
  display: flex;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid #eee;
}

.summary-item:last-child {
  border-bottom: none;
}

.summary-label {
  width: 100px;
  color: #888;
  font-size: 14px;
}

.summary-value {
  flex: 1;
  color: #333;
  font-weight: 500;
}

.edit-btn {
  background: none;
  border: none;
  color: #588157;
  font-size: 14px;
  cursor: pointer;
  padding: 4px 8px;
}

.edit-btn:hover {
  text-decoration: underline;
}

.confirm-hint {
  text-align: center;
  color: #666;
  font-size: 14px;
}

.course-plan-result {
  padding: 10px 0;
}

.result-header {
  text-align: center;
  margin-bottom: 24px;
}

.result-header h2 {
  font-size: 22px;
  color: #333;
  margin-bottom: 8px;
}

.result-header p {
  color: #666;
}

.plan-card {
  background: #f8f8f8;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
}

.plan-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.plan-header h3 {
  font-size: 20px;
  color: #333;
}

.plan-duration {
  background: #588157;
  color: white;
  padding: 4px 12px;
  border-radius: 20px;
  font-size: 14px;
}

.plan-meta {
  display: flex;
  gap: 20px;
  color: #666;
  font-size: 14px;
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid #ddd;
}

.plan-outline h4 {
  font-size: 16px;
  color: #333;
  margin-bottom: 16px;
}

.chapter-item {
  margin-bottom: 16px;
}

.chapter-name {
  font-weight: 600;
  color: #588157;
  margin-bottom: 8px;
  padding-left: 8px;
  border-left: 3px solid #588157;
}

.lesson-list {
  padding-left: 20px;
}

.lesson-item {
  display: flex;
  align-items: center;
  padding: 8px 0;
  color: #666;
  font-size: 14px;
}

.lesson-index {
  width: 24px;
  height: 24px;
  background: #e8f0e8;
  color: #588157;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  margin-right: 12px;
}

.lesson-name {
  flex: 1;
}

.lesson-duration {
  color: #999;
  font-size: 12px;
}

.style-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  margin-top: 16px;
}

.style-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px 16px;
  background: white;
  border: 2px solid #e8e8e8;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.style-card:hover {
  border-color: #588157;
  background: #f8f8f8;
}

.style-card.selected {
  border-color: #588157;
  background: #e8f0e8;
}

.style-icon {
  font-size: 32px;
  margin-bottom: 8px;
}

.style-name {
  font-weight: 600;
  color: #333;
  margin-bottom: 4px;
}

.style-desc {
  font-size: 12px;
  color: #888;
  text-align: center;
}
```

- [ ] **步骤 4：Commit**

```bash
git add src/hooks/useConsultant.ts src/components/consultant/ConsultantAgent.tsx src/components/consultant/ConsultantAgent.css
git commit -m "feat: expand consultant agent to 7-step flow"
```

---

## 任务 4：课程计划创建时生成章节和课时

**文件：**
- 修改：`src/pages/Consultant.tsx`
- 修改：`src/services/tauri.ts`
- 修改：`src-tauri/src/commands/database.rs`

- [ ] **步骤 1：在 Tauri 后端添加创建章节和课时的批量命令**

在 `src-tauri/src/commands/database.rs` 末尾添加：

```rust
/// 批量创建章节和课时
#[derive(Debug, serde::Serialize, Deserialize)]
pub struct CreateChaptersWithLessonsParams {
    pub course_id: String,
    pub chapters: Vec<ChapterWithLessonsParams>,
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct ChapterWithLessonsParams {
    pub chapter_index: i32,
    pub chapter_name: String,
    pub lessons: Vec<LessonParams>,
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct LessonParams {
    pub lesson_index: i32,
    pub lesson_name: String,
    pub duration: String,
}

/// 批量创建章节和课时命令
#[tauri::command]
pub fn create_chapters_with_lessons_command(
    state: State<DbState>,
    params: CreateChaptersWithLessonsParams,
) -> DbResult<Vec<Chapter>> {
    let db = state.0.lock().unwrap();
    let conn = db.get_connection();

    let mut chapters = Vec::new();

    for chapter_params in params.chapters {
        let chapter = operations::create_chapter(
            &conn,
            params.course_id.clone(),
            chapter_params.chapter_index,
            chapter_params.chapter_name,
        ).map_err(|e| DbCommandError::from(e))?;

        for lesson_params in chapter_params.lessons {
            operations::create_lesson(
                &conn,
                chapter.id.clone(),
                lesson_params.lesson_index,
                lesson_params.lesson_name,
                Some(lesson_params.duration),
            ).map_err(|e| DbCommandError::from(e))?;
        }

        chapters.push(chapter);
    }

    Ok(chapters)
}
```

- [ ] **步骤 2：在 `lib.rs` 中注册新命令**

在 `invoke_handler` 的 `generate_handler![]` 宏中添加：
```rust
create_chapters_with_lessons_command,
```

- [ ] **步骤 3：在 `src/services/tauri.ts` 中添加创建章节课时接口**

```typescript
async createChaptersWithLessons(params: {
  courseId: string;
  chapters: Array<{
    chapterIndex: number;
    chapterName: string;
    lessons: Array<{
      lessonIndex: number;
      lessonName: string;
      duration: string;
    }>;
  }>;
}): Promise<Chapter[]> {
  return invoke('create_chapters_with_lessons_command', { params });
},
```

- [ ] **步骤 4：更新 `src/pages/Consultant.tsx` 在创建课程时同时创建章节和课时**

修改 `handleStartLearning` 函数：

```typescript
const handleStartLearning = async () => {
  if (!coursePlan) return;

  try {
    // 1. 创建课程
    const { invoke } = await import('@tauri-apps/api/core');
    const course = await invoke<Course>('create_course_command', {
      name: coursePlan.courseName,
      targetLevel: coursePlan.targetLevel,
      duration: coursePlan.duration,
      teachingStyle: coursePlan.teachingStyle,
    });

    if (course?.id) {
      // 2. 创建章节和课时
      await tauriService.createChaptersWithLessons({
        courseId: course.id,
        chapters: coursePlan.chapters.map((chapter) => ({
          chapterIndex: chapter.chapterIndex,
          chapterName: chapter.chapterName,
          lessons: chapter.lessons.map((lesson) => ({
            lessonIndex: lesson.lessonIndex,
            lessonName: lesson.lessonName,
            duration: lesson.duration,
          })),
        })),
      });

      // 3. 创建码云仓库
      try {
        const syncResult = await tauriService.createCourseRepository(course.id);
        if (syncResult.success) {
          await message(`课程"${coursePlan.courseName}"创建成功！\n仓库地址：${syncResult.repoUrl}`, {
            title: '课程创建成功',
            kind: 'info',
          });
        }
      } catch (syncError) {
        console.warn('码云同步失败:', syncError);
      }

      // 4. 导航到首页
      navigate('/');
    }
  } catch (error) {
    console.error('创建课程失败:', error);
    await message(`创建课程失败：${error}`, {
      title: '错误',
      kind: 'error',
    });
  }
};
```

同时需要更新类型导入 `CoursePlanOutline` 替换 `CoursePlan`。

- [ ] **步骤 5：验证编译**

```bash
cd D:\wgq_ai\study-ai-app && npx tsc --noEmit
cd D:\wgq_ai\study-ai-app\src-tauri && cargo check
```

预期：无编译错误

- [ ] **步骤 6：Commit**

```bash
git add src-tauri/src/commands/database.rs src-tauri/src/lib.rs src/services/tauri.ts src/pages/Consultant.tsx
git commit -m "feat: create chapters and lessons when course is created"
```

---

## 任务 5：courseStore 加载章节和课时

**文件：**
- 修改：`src/stores/courseStore.ts`

- [ ] **步骤 1：在 `courseStore` 中添加加载章节和课时的方法**

```typescript
// 添加到 CourseStore 接口
loadCourseChapters: (courseId: string) => Promise<void>;
```

在 store 实现中添加：

```typescript
loadCourseChapters: async (courseId) => {
  try {
    const chapters = await tauriService.getChaptersByCourse(courseId);
    // 加载每个章节的课时
    const chaptersWithLessons = await Promise.all(
      chapters.map(async (chapter) => {
        const lessons = await tauriService.getLessonsByChapter(chapter.id);
        return { ...chapter, lessons };
      })
    );

    set((state) => {
      const currentCourse = state.courses.find((c) => c.id === courseId);
      if (currentCourse) {
        return {
          courses: state.courses.map((c) =>
            c.id === courseId
              ? { ...c, chapters: chaptersWithLessons }
              : c
          ),
          currentCourse: { ...currentCourse, chapters: chaptersWithLessons },
        };
      }
      return state;
    });
  } catch (error) {
    console.error('Failed to load course chapters:', error);
  }
},
```

- [ ] **步骤 2：在 `tauriService` 中添加获取章节和课时接口**

```typescript
async getChaptersByCourse(courseId: string): Promise<Chapter[]> {
  return invoke('get_chapters_by_course_command', { courseId });
},

async getLessonsByChapter(chapterId: string): Promise<Lesson[]> {
  return invoke('get_lessons_by_chapter_command', { chapterId });
},
```

- [ ] **步骤 3：在 `selectCourse` 时自动加载章节和课时**

修改 `selectCourse` 方法：

```typescript
selectCourse: async (courseId) => {
  const course = get().courses.find((c) => c.id === courseId);
  if (course) {
    set({ currentCourse: course, currentChapter: null, currentLesson: null });
    // 加载章节和课时
    await get().loadCourseChapters(courseId);
  } else {
    set({ currentCourse: null, currentChapter: null, currentLesson: null });
  }
},
```

- [ ] **步骤 4：Commit**

```bash
git add src/stores/courseStore.ts src/services/tauri.ts
git commit -m "feat: load course chapters and lessons in courseStore"
```

---

## 任务 6：完善 CoursewareViewer 课件加载

**文件：**
- 修改：`src/components/learning/CoursewareViewer.tsx`

- [ ] **步骤 1：更新 `CoursewareViewer` 组件以支持 AI 生成课件**

当前的 `CoursewareViewer` 组件需要检查是否已有课件内容，如果没有则调用 AI 生成。

首先查看现有实现：

```typescript
// 当前应该已经有类似实现，需要确保：
// 1. 如果没有课件内容，调用 generateLessonHTML
// 2. 生成后保存到 lessonContents
// 3. 展示课件 HTML
```

如果组件已实现此逻辑，跳过此步骤。否则更新组件：

```typescript
import React, { useEffect, useState } from 'react';
import { useCourseStore } from '../../stores/courseStore';
import { generateLessonHTML } from '../../hooks/useAI';

interface CoursewareViewerProps {
  lessonId: string;
  content?: string | null;
}

export const CoursewareViewer: React.FC<CoursewareViewerProps> = ({
  lessonId,
  content,
}) => {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const { currentCourse, currentChapter, currentLesson, setLessonContent } = useCourseStore();

  useEffect(() => {
    if (!content && currentCourse && currentChapter && currentLesson) {
      loadLessonContent();
    }
  }, [lessonId, content]);

  const loadLessonContent = async () => {
    if (!currentCourse || !currentChapter || !currentLesson) return;

    setIsLoading(true);
    setError(null);

    try {
      const result = await generateLessonHTML(
        currentCourse.name,
        currentChapter.name,
        currentLesson.name,
        currentCourse.teachingStyle || '实战应用型'
      );

      if (result.success && result.data) {
        setLessonContent(lessonId, result.data);
      } else {
        setError(result.error || '加载课件失败');
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  };

  if (isLoading) {
    return (
      <div className="courseware-viewer loading">
        <div className="spinner" />
        <span>正在生成课件...</span>
      </div>
    );
  }

  if (error) {
    return (
      <div className="courseware-viewer error">
        <p>{error}</p>
        <button onClick={loadLessonContent}>重试</button>
      </div>
    );
  }

  if (!content) {
    return (
      <div className="courseware-viewer empty">
        <p>暂无课件内容</p>
      </div>
    );
  }

  return (
    <div className="courseware-viewer">
      <div
        className="courseware-html"
        dangerouslySetInnerHTML={{ __html: content }}
      />
    </div>
  );
};
```

- [ ] **步骤 2：Commit**

```bash
git add src/components/learning/CoursewareViewer.tsx
git commit -m "feat: enhance courseware viewer with AI generation"
```

---

## 任务 7：验收测试

- [ ] **步骤 1：启动应用进行功能测试**

```bash
npm run tauri dev
```

测试流程：
1. 进入咨询师页面，确认7步流程正确显示
2. 完成7步引导，确认生成包含章节和课时的课程计划
3. 点击"开始学习"，确认课程创建成功并包含章节和课时
4. 进入学习页面，确认左侧课程大纲显示章节和课时
5. 点击某个课时，确认中间区域显示 AI 生成的课件
6. 在右侧答疑区域发送问题，确认 AI 正常回答
7. 在右侧练习区域点击"生成练习题"，确认正常生成

- [ ] **步骤 2：验证数据持久化**

1. 关闭应用并重新打开
2. 进入之前创建的课程，确认章节、课时和进度都已保存

- [ ] **步骤 3：Commit 最终验证**

```bash
git status
git log --oneline -5
```

---

## 验收标准

| 功能 | 验收条件 |
|------|---------|
| 咨询师 Agent | 7步引导流程完整显示，欢迎页、确认页正常工作 |
| 课程计划生成 | AI 生成包含章节和课时的完整课程大纲 |
| 章节/课时创建 | 课程创建时自动创建章节和课时，存储到数据库 |
| 课件生成 | 学习界面中 AI 自动生成 HTML 课件并展示 |
| 练习题生成 | 可生成结构化练习题并提交答案 |
| 答疑功能 | 可与 AI 教师进行对话答疑 |
| 数据持久化 | 章节、课时、聊天记录在重新打开应用后正常恢复 |

---

## 备注

1. **AI API 调用失败处理**：所有 AI 调用都应捕获异常并显示友好错误提示，不阻塞用户操作

2. **课程大纲生成 Token 消耗**：生成完整课程大纲可能消耗较多 token，确保用户配置了有效的 API 密钥

3. **章节/课时数量限制**：根据学习时长合理安排，建议：
   - 1周内：3-5章节，每章2-3课时
   - 1个月内：6-10章节，每章3-5课时
   - 3个月内：10-15章节，每章3-5课时
   - 半年内：15-20章节，每章4-6课时

4. **码云同步**：课程创建后自动同步到码云仓库，但章节/课时变更需要手动同步
