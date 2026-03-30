import React, { useState } from 'react';
import { Button } from '../common/Button';
import { SetupStepProps } from './SetupWizard';

const TEACHING_STYLES = [
  {
    id: 'academic',
    name: '严谨学术型',
    description: '结构严谨、概念清晰，适合系统学习',
    icon: '📚',
  },
  {
    id: 'practical',
    name: '实战应用型',
    description: '案例驱动、边学边做，强调动手实践',
    icon: '💻',
  },
  {
    id: 'story',
    name: '轻松故事型',
    description: '故事/比喻引导，语言轻松有趣',
    icon: '📖',
  },
  {
    id: 'progressive',
    name: '循序渐进型',
    description: '小步前进、充分练习，打牢基础',
    icon: '🎓',
  },
  {
    id: 'explorative',
    name: '启发探索型',
    description: '提问引导、独立思考，培养能力',
    icon: '🔍',
  },
  {
    id: 'efficient',
    name: '快速高效型',
    description: '精炼要点、直奔目标，高效学习',
    icon: '⚡',
  },
];

export const StyleSelectStep: React.FC<SetupStepProps> = ({ onNext, onBack }) => {
  const [selectedStyle, setSelectedStyle] = useState('');

  const handleSubmit = () => {
    // 风格选择目前只保存 ID，实际存储可以扩展
    onNext();
  };

  return (
    <div>
      <div className="text-center mb-6">
        <div className="text-4xl mb-4">🎨</div>
        <h2 className="text-xl font-bold mb-2">选择教学风格</h2>
        <p className="text-text-secondary">选择你喜欢的教学风格，获得个性化学习体验</p>
      </div>

      <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
        {TEACHING_STYLES.map((style) => (
          <button
            key={style.id}
            onClick={() => setSelectedStyle(style.id)}
            className={`p-4 rounded-lg border-2 transition-all text-center ${
              selectedStyle === style.id
                ? 'border-primary bg-primary/5'
                : 'border-gray-200 hover:border-gray-300'
            }`}
          >
            <span className="text-4xl">{style.icon}</span>
            <h3 className="mt-2 font-medium">{style.name}</h3>
            <p className="mt-1 text-xs text-text-muted">{style.description}</p>
          </button>
        ))}
      </div>

      <div className="flex justify-between mt-6">
        <Button variant="outline" onClick={onBack}>
          上一步
        </Button>
        <Button onClick={handleSubmit}>
          完成配置
        </Button>
      </div>
    </div>
  );
};