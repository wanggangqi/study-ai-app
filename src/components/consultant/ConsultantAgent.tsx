import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useConfigStore } from '../../stores/configStore';
import { CoursePlan } from '../../types';
import './ConsultantAgent.css';

interface ConsultantAgentProps {
  onCoursePlanGenerated?: (plan: CoursePlan) => void;
}

// 咨询问题配置
const CONSULT_QUESTIONS: Array<{
  id: string;
  question: string;
  description: string;
  placeholder?: string;
  options?: string[];
  styleOptions?: Array<{ id: string; description: string }>;
}> = [
  {
    id: 'goal',
    question: '您想学习什么内容？',
    description: '例如：Python 编程、机器学习、英语口语、设计模式...',
    placeholder: '请描述您想学习的内容...',
  },
  {
    id: 'level',
    question: '您目前的基础水平是？',
    description: '这有助于我为您定制合适的学习路径',
    options: ['零基础', '入门', '初级', '中级', '高级'],
  },
  {
    id: 'duration',
    question: '您希望多长时间完成学习？',
    description: '根据目标设定合理的学习周期',
    options: ['1 周内', '1 个月内', '3 个月内', '半年内', '不着急'],
  },
  {
    id: 'style',
    question: '您喜欢什么样的教学风格？',
    description: '选择您最喜欢的教学风格',
    styleOptions: [
      { id: '幽默风趣', description: '轻松愉快，寓教于乐' },
      { id: '严谨专业', description: '系统完整，逻辑清晰' },
      { id: '实战为主', description: '边做边学，注重实践' },
      { id: '循序渐进', description: '由浅入深，稳扎稳打' },
    ],
  },
  {
    id: 'base',
    question: '您有哪些相关基础？',
    description: '请描述您已有的相关知识或经验',
    placeholder: '例如：学过 HTML/CSS，了解变量和循环概念...',
  },
];

export function ConsultantAgent({ onCoursePlanGenerated }: ConsultantAgentProps) {
  const [currentStep, setCurrentStep] = useState(0);
  const [answers, setAnswers] = useState<Record<string, string>>({});
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [coursePlan, setCoursePlan] = useState<CoursePlan | null>(null);

  const config = useConfigStore();
  const currentQuestion = CONSULT_QUESTIONS[currentStep];

  const handleAnswer = useCallback((answer: string) => {
    setAnswers((prev) => ({ ...prev, [currentQuestion.id]: answer }));
    setError(null);
  }, [currentQuestion.id]);

  const handleNext = useCallback(async () => {
    if (!answers[currentQuestion.id]) {
      setError('请选择一个答案');
      return;
    }

    if (currentStep < CONSULT_QUESTIONS.length - 1) {
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
      const systemPrompt = `你是一位专业的学习咨询师，擅长根据用户的需求制定个性化的课程计划。
请根据用户的信息，生成一个结构化的课程计划 JSON。
课程计划应包含：
- name: 课程名称
- targetLevel: 目标水平
- duration: 学习时长
- teachingStyle: 教学风格
- baseKnowledge: 基础知识要求

请只返回 JSON 格式，不要包含其他文字。`;

      const userPrompt = `用户信息：
- 学习目标：${answers.goal}
- 现有水平：${answers.level}
- 期望时长：${answers.duration}
- 教学风格：${answers.style}
- 已有基础：${answers.base || '无'}

请生成课程计划 JSON。`;

      const result = await invoke<{ success: boolean; data?: string; error?: string }>(
        'ai_chat_command',
        {
          params: {
            provider: config.aiProvider,
            api_key: config.aiApiKey,
            model: config.aiModel || null,
            messages: [
              { role: 'system', content: systemPrompt },
              { role: 'user', content: userPrompt },
            ],
          },
        }
      );

      if (result.success && result.data) {
        const plan = JSON.parse(result.data) as CoursePlan;
        setCoursePlan(plan);
        onCoursePlanGenerated?.(plan);
      } else {
        setError(result.error || '生成课程计划失败');
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

  const renderQuestion = () => {
    if (currentQuestion.id === 'style' && currentQuestion.styleOptions) {
      return (
        <div className="style-options">
          {currentQuestion.styleOptions.map((style) => (
            <button
              key={style.id}
              className={`style-option ${answers[currentQuestion.id] === style.id ? 'selected' : ''}`}
              onClick={() => handleAnswer(style.id)}
            >
              <span className="style-name">{style.id}</span>
              <span className="style-desc">{style.description}</span>
            </button>
          ))}
        </div>
      );
    }

    if (currentQuestion.options) {
      return (
        <div className="option-grid">
          {currentQuestion.options.map((option) => (
            <button
              key={option}
              className={`option-btn ${answers[currentQuestion.id] === option ? 'selected' : ''}`}
              onClick={() => handleAnswer(option)}
            >
              {option}
            </button>
          ))}
        </div>
      );
    }

    return (
      <textarea
        className="answer-textarea"
        placeholder={currentQuestion.placeholder}
        value={answers[currentQuestion.id] || ''}
        onChange={(e) => handleAnswer(e.target.value)}
      />
    );
  };

  if (coursePlan) {
    return (
      <div className="consultant-agent course-plan-result">
        <div className="result-header">
          <h2>课程计划已生成</h2>
          <p>根据您的需求，我为您定制了以下学习计划</p>
        </div>

        <div className="plan-card">
          <div className="plan-item">
            <span className="plan-label">课程名称</span>
            <span className="plan-value">{coursePlan.name}</span>
          </div>
          <div className="plan-item">
            <span className="plan-label">目标水平</span>
            <span className="plan-value">{coursePlan.targetLevel}</span>
          </div>
          <div className="plan-item">
            <span className="plan-label">学习时长</span>
            <span className="plan-value">{coursePlan.duration}</span>
          </div>
          <div className="plan-item">
            <span className="plan-label">教学风格</span>
            <span className="plan-value">{coursePlan.teachingStyle}</span>
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
  }

  return (
    <div className="consultant-agent">
      <div className="progress-bar">
        <div
          className="progress-fill"
          style={{ width: `${((currentStep + 1) / CONSULT_QUESTIONS.length) * 100}%` }}
        />
      </div>

      <div className="step-indicator">
        {currentStep + 1} / {CONSULT_QUESTIONS.length}
      </div>

      <div className="question-section">
        <h2 className="question-text">{currentQuestion.question}</h2>
        <p className="question-desc">{currentQuestion.description}</p>

        {renderQuestion()}

        {error && <div className="error-message">{error}</div>}
      </div>

      <div className="navigation-buttons">
        {currentStep > 0 && (
          <button className="btn-secondary" onClick={handleBack}>
            上一步
          </button>
        )}
        <button
          className="btn-primary"
          onClick={handleNext}
          disabled={isLoading || !answers[currentQuestion.id]}
        >
          {isLoading ? '生成中...' : currentStep === CONSULT_QUESTIONS.length - 1 ? '生成课程计划' : '下一步'}
        </button>
      </div>
    </div>
  );
}
