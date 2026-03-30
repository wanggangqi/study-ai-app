import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Card } from '../components/common/Card';
import { Input } from '../components/common/Input';
import { Button } from '../components/common/Button';

interface SetupStep {
  title: string;
  description: string;
  completed: boolean;
}

export const SetupPage: React.FC = () => {
  const navigate = useNavigate();
  const [currentStep, setCurrentStep] = useState(0);

  const steps: SetupStep[] = [
    { title: 'Git 安装', description: '检查 Git 是否已安装', completed: true },
    { title: '码云账户', description: '配置码云账户信息', completed: false },
    { title: '工作空间', description: '设置本地工作空间路径', completed: false },
    { title: 'AI 服务', description: '选择并配置 AI 服务商', completed: false },
  ];

  return (
    <div className="min-h-screen flex items-center justify-center bg-bg-primary p-8">
      <div className="max-w-2xl w-full">
        <div className="text-center mb-8">
          <h1 className="text-2xl font-bold text-primary mb-2">欢迎使用智学伴侣</h1>
          <p className="text-text-secondary">让我们先完成一些基本配置</p>
        </div>

        <Card>
          <div className="space-y-4">
            {steps.map((step, index) => (
              <div
                key={index}
                className={`flex items-center gap-4 p-4 rounded-md ${
                  index === currentStep ? 'bg-primary/10 border border-primary' :
                  step.completed ? 'bg-green-50 border border-green-200' :
                  'bg-gray-50 border border-gray-200'
                }`}
              >
                <div className={`w-8 h-8 rounded-full flex items-center justify-center ${
                  step.completed ? 'bg-green-500 text-white' :
                  index === currentStep ? 'bg-primary text-white' :
                  'bg-gray-300 text-gray-500'
                }`}>
                  {step.completed ? '✓' : index + 1}
                </div>
                <div className="flex-1">
                  <h3 className="font-medium">{step.title}</h3>
                  <p className="text-sm text-text-muted">{step.description}</p>
                </div>
              </div>
            ))}
          </div>

          <div className="mt-6 flex justify-between">
            <Button
              variant="outline"
              onClick={() => setCurrentStep(Math.max(0, currentStep - 1))}
              disabled={currentStep === 0}
            >
              上一步
            </Button>
            <Button onClick={() => {
              if (currentStep < steps.length - 1) {
                setCurrentStep(currentStep + 1);
              } else {
                navigate('/');
              }
            }}>
              {currentStep < steps.length - 1 ? '下一步' : '完成'}
            </Button>
          </div>
        </Card>
      </div>
    </div>
  );
};
