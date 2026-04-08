import { useState, useCallback } from 'react';
import { useConsultant } from '../../hooks/useConsultant';
import type { CoursePlanOutline } from '../../types';
import './ConsultantAgent.css';

interface ConsultantAgentProps {
  onCoursePlanGenerated?: (plan: CoursePlanOutline) => void;
}

// 7步流程步骤定义
const STEPS = ['welcome', 'goal', 'level', 'duration', 'style', 'base', 'confirm'] as const;
type StepType = typeof STEPS[number];

// 步骤配置
const STEP_CONFIG: Record<StepType, { question?: string; description?: string; placeholder?: string; options?: string[] }> = {
  welcome: {
    description: '让我来帮您制定一个专属的学习计划',
  },
  goal: {
    question: '您想学习什么内容？',
    description: '例如：Python 编程、机器学习、英语口语、设计模式...',
    placeholder: '请描述您想学习的内容...',
  },
  level: {
    question: '您目前的基础水平是？',
    description: '这有助于我为您定制合适的学习路径',
    options: ['零基础', '入门', '初级', '中级', '高级'],
  },
  duration: {
    question: '您希望多长时间完成学习？',
    description: '根据目标设定合理的学习周期',
    options: ['1 周内', '1 个月内', '3 个月内', '半年内', '不着急'],
  },
  style: {
    question: '您喜欢什么样的教学风格？',
    description: '选择您最喜欢的教学风格',
  },
  base: {
    question: '您有哪些相关基础？',
    description: '请描述您已有的相关知识或经验（选填）',
    placeholder: '例如：学过 HTML/CSS，了解变量和循环概念...',
  },
  confirm: {
    question: '请确认您的学习计划',
    description: '以下是您提供的信息，如有需要可以点击修改',
  },
};

// 6种教学风格
const TEACHING_STYLES = [
  { id: '幽默风趣', description: '轻松愉快，寓教于乐', icon: '😊' },
  { id: '严谨专业', description: '系统完整，逻辑清晰', icon: '📚' },
  { id: '实战为主', description: '边做边学，注重实践', icon: '💻' },
  { id: '循序渐进', description: '由浅入深，稳扎稳打', icon: '📈' },
  { id: '启发式', description: '引导思考，培养能力', icon: '💡' },
  { id: '耐心细致', description: '讲解详细，不懂就问', icon: '🤝' },
];

export function ConsultantAgent({ onCoursePlanGenerated }: ConsultantAgentProps) {
  const [currentStep, setCurrentStep] = useState(0);
  const [answers, setAnswers] = useState<Record<string, string>>({});
  const [coursePlan, setCoursePlan] = useState<CoursePlanOutline | null>(null);

  const { isLoading, error, generateCoursePlan } = useConsultant();

  const currentStepType = STEPS[currentStep];
  const stepConfig = STEP_CONFIG[currentStepType];

  const handleAnswer = useCallback((stepId: string, answer: string) => {
    setAnswers((prev) => ({ ...prev, [stepId]: answer }));
  }, []);

  const handleNext = useCallback(async () => {
    if (currentStepType === 'confirm') {
      // 最后一步，生成课程计划
      const result = await generateCoursePlan(answers);
      if (result) {
        setCoursePlan(result);
        onCoursePlanGenerated?.(result);
      }
    } else if (currentStep < STEPS.length - 1) {
      setCurrentStep((prev) => prev + 1);
    }
  }, [currentStep, currentStepType, answers, generateCoursePlan, onCoursePlanGenerated]);

  const handleBack = useCallback(() => {
    if (currentStep > 0) {
      setCurrentStep((prev) => prev - 1);
    }
  }, [currentStep]);

  const handleEditField = useCallback((field: string) => {
    const fieldIndex = STEPS.indexOf(field as StepType);
    if (fieldIndex !== -1) {
      setCurrentStep(fieldIndex);
    }
  }, []);

  const handleRestart = () => {
    setCurrentStep(0);
    setAnswers({});
    setCoursePlan(null);
  };

  const canProceed = () => {
    if (currentStepType === 'welcome') return true;
    if (currentStepType === 'base') return true; // base is optional
    if (currentStepType === 'confirm') return true;
    return !!answers[currentStepType];
  };

  // 渲染欢迎步骤
  const renderWelcome = () => (
    <div className="welcome-step">
      <div className="welcome-icon">🎓</div>
      <h2 className="welcome-title">欢迎来到智学伴侣</h2>
      <p className="welcome-desc">{stepConfig.description}</p>
      <div className="welcome-features">
        <div className="feature-item">
          <span className="feature-icon">📝</span>
          <span className="feature-text">个性化学习计划</span>
        </div>
        <div className="feature-item">
          <span className="feature-icon">🎯</span>
          <span className="feature-text">AI 一对一教学</span>
        </div>
        <div className="feature-item">
          <span className="feature-icon">📚</span>
          <span className="feature-text">互动式课件</span>
        </div>
      </div>
    </div>
  );

  // 渲染选项步骤
  const renderOptions = (options: string[]) => (
    <div className="option-grid">
      {options.map((option) => (
        <button
          key={option}
          className={`option-btn ${answers[currentStepType] === option ? 'selected' : ''}`}
          onClick={() => handleAnswer(currentStepType, option)}
        >
          {option}
        </button>
      ))}
    </div>
  );

  // 渲染文本输入步骤
  const renderTextInput = () => (
    <textarea
      className="answer-textarea"
      placeholder={stepConfig.placeholder}
      value={answers[currentStepType] || ''}
      onChange={(e) => handleAnswer(currentStepType, e.target.value)}
    />
  );

  // 渲染风格选择步骤
  const renderStyleOptions = () => (
    <div className="style-cards">
      {TEACHING_STYLES.map((style) => (
        <button
          key={style.id}
          className={`style-card ${answers[currentStepType] === style.id ? 'selected' : ''}`}
          onClick={() => handleAnswer(currentStepType, style.id)}
        >
          <span className="style-card-icon">{style.icon}</span>
          <span className="style-card-name">{style.id}</span>
          <span className="style-card-desc">{style.description}</span>
        </button>
      ))}
    </div>
  );

  // 渲染确认步骤
  const renderConfirm = () => (
    <div className="confirm-step">
      <div className="confirm-summary">
        <div className="summary-item" onClick={() => handleEditField('goal')}>
          <div className="summary-label">学习内容</div>
          <div className="summary-value">{answers.goal || '-'}</div>
          <button className="edit-btn">修改</button>
        </div>
        <div className="summary-item" onClick={() => handleEditField('level')}>
          <div className="summary-label">基础水平</div>
          <div className="summary-value">{answers.level || '-'}</div>
          <button className="edit-btn">修改</button>
        </div>
        <div className="summary-item" onClick={() => handleEditField('duration')}>
          <div className="summary-label">学习时长</div>
          <div className="summary-value">{answers.duration || '-'}</div>
          <button className="edit-btn">修改</button>
        </div>
        <div className="summary-item" onClick={() => handleEditField('style')}>
          <div className="summary-label">教学风格</div>
          <div className="summary-value">{answers.style || '-'}</div>
          <button className="edit-btn">修改</button>
        </div>
        <div className="summary-item" onClick={() => handleEditField('base')}>
          <div className="summary-label">相关基础</div>
          <div className="summary-value">{answers.base || '无'}</div>
          <button className="edit-btn">修改</button>
        </div>
      </div>
    </div>
  );

  // 渲染课程计划结果
  const renderCoursePlanResult = () => {
    const plan = coursePlan as CoursePlanOutline;
    return (
      <div className="consultant-agent course-plan-result">
        <div className="result-header">
          <h2>课程计划已生成</h2>
          <p>根据您的需求，我为您定制了以下学习计划</p>
        </div>

        <div className="plan-card">
          <div className="plan-item">
            <span className="plan-label">课程名称</span>
            <span className="plan-value">{plan.courseName}</span>
          </div>
          <div className="plan-item">
            <span className="plan-label">目标水平</span>
            <span className="plan-value">{plan.targetLevel}</span>
          </div>
          <div className="plan-item">
            <span className="plan-label">学习时长</span>
            <span className="plan-value">{plan.duration}</span>
          </div>
          <div className="plan-item">
            <span className="plan-label">教学风格</span>
            <span className="plan-value">{plan.teachingStyle}</span>
          </div>
          {'chapters' in plan && plan.chapters && (
            <div className="plan-item">
              <span className="plan-label">章节数量</span>
              <span className="plan-value">{plan.chapters.length} 章</span>
            </div>
          )}
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

  // 渲染当前步骤内容
  const renderStepContent = () => {
    switch (currentStepType) {
      case 'welcome':
        return renderWelcome();
      case 'goal':
      case 'base':
        return renderTextInput();
      case 'level':
      case 'duration':
        return renderOptions(stepConfig.options || []);
      case 'style':
        return renderStyleOptions();
      case 'confirm':
        return renderConfirm();
      default:
        return null;
    }
  };

  if (coursePlan) {
    return renderCoursePlanResult();
  }

  return (
    <div className="consultant-agent">
      <div className="progress-bar">
        <div
          className="progress-fill"
          style={{ width: `${((currentStep + 1) / STEPS.length) * 100}%` }}
        />
      </div>

      <div className="step-indicator">
        {currentStepType !== 'welcome' && currentStepType !== 'confirm'
          ? `${currentStep} / ${STEPS.length - 2}`
          : currentStepType === 'welcome'
          ? '开始'
          : '确认'}
      </div>

      <div className="question-section">
        {currentStepType !== 'welcome' && currentStepType !== 'confirm' && (
          <>
            <h2 className="question-text">{stepConfig.question}</h2>
            <p className="question-desc">{stepConfig.description}</p>
          </>
        )}
        {currentStepType === 'confirm' && (
          <>
            <h2 className="question-text">{stepConfig.question}</h2>
            <p className="question-desc">{stepConfig.description}</p>
          </>
        )}

        {renderStepContent()}

        {error && <div className="error-message">{error}</div>}
      </div>

      <div className="navigation-buttons">
        {currentStep > 0 && currentStepType !== 'confirm' && (
          <button className="btn-secondary" onClick={handleBack}>
            上一步
          </button>
        )}
        <button
          className="btn-primary"
          onClick={handleNext}
          disabled={isLoading || !canProceed()}
        >
          {isLoading
            ? '生成中...'
            : currentStepType === 'welcome'
            ? '开始制定计划'
            : currentStepType === 'confirm'
            ? '生成课程计划'
            : '下一步'}
        </button>
      </div>
    </div>
  );
}
