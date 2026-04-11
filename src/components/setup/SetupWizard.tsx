import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Card } from '../common/Card';
import { GitSetupStep } from './GitSetupStep';
import { GiteeSetupStep } from './GiteeSetupStep';
import { WorkspaceStep } from './WorkspaceStep';
import { AISetupStep } from './AISetupStep';
import { StyleSelectStep } from './StyleSelectStep';
import { useConfigStore } from '../../stores/configStore';

export interface SetupStepProps {
  onNext: () => void;
  onBack: () => void;
}

const STEPS = [
  { id: 'git', title: 'Git 安装', description: '检查 Git 是否已安装并配置用户信息' },
  { id: 'gitee', title: '码云账户', description: '配置码云账户信息' },
  { id: 'workspace', title: '工作空间', description: '设置本地工作空间路径' },
  { id: 'ai', title: 'AI 服务', description: '选择并配置 AI 服务商' },
  { id: 'style', title: '教学风格', description: '选择喜欢的教学风格' },
];

export const SetupWizard: React.FC = () => {
  const navigate = useNavigate();
  const [currentStep, setCurrentStep] = useState(0);
  const [completedSteps, setCompletedSteps] = useState<Set<number>>(new Set());
  const { setConfig, saveConfig } = useConfigStore();

  const handleStepComplete = async (stepIndex: number) => {
    setCompletedSteps(prev => new Set([...prev, stepIndex]));
    if (currentStep < STEPS.length - 1) {
      setCurrentStep(currentStep + 1);
    } else {
      // 所有步骤完成，保存配置并跳转到首页
      setConfig({ setupCompleted: true });
      await saveConfig();
      navigate('/');
    }
  };

  const handleBack = () => {
    if (currentStep > 0) {
      setCurrentStep(currentStep - 1);
    }
  };

  const renderStep = () => {
    const stepProps: SetupStepProps = {
      onNext: () => handleStepComplete(currentStep),
      onBack: handleBack,
    };

    switch (currentStep) {
      case 0:
        return <GitSetupStep {...stepProps} />;
      case 1:
        return <GiteeSetupStep {...stepProps} />;
      case 2:
        return <WorkspaceStep {...stepProps} />;
      case 3:
        return <AISetupStep {...stepProps} />;
      case 4:
        return <StyleSelectStep {...stepProps} />;
      default:
        return null;
    }
  };

  return (
    <div className="h-screen flex items-center justify-center bg-[#fefae0] p-8 overflow-hidden">
      <div className="max-w-2xl w-full">
        <div className="text-center mb-8">
          <h1 className="text-2xl font-bold text-[#588157] mb-2">欢迎使用智学伴侣</h1>
          <p className="text-[#666666]">让我们先完成一些基本配置</p>
        </div>

        {/* 步骤指示器 */}
        <div className="flex justify-center mb-8">
          {STEPS.map((step, index) => (
            <div key={step.id} className="flex items-center">
              <div className="flex flex-col items-center">
                <div
                  className={`w-10 h-10 rounded-full flex items-center justify-center text-sm font-medium transition-colors ${
                    completedSteps.has(index)
                      ? 'bg-green-500 text-white'
                      : index === currentStep
                      ? 'bg-[#588157] text-white'
                      : 'bg-gray-200 text-gray-500'
                  }`}
                >
                  {completedSteps.has(index) ? '✓' : index + 1}
                </div>
                <span
                  className={`text-xs mt-2 ${
                    index === currentStep ? 'text-[#588157] font-medium' : 'text-[#999999]'
                  }`}
                >
                  {step.title}
                </span>
              </div>
              {index < STEPS.length - 1 && (
                <div
                  className={`w-12 h-0.5 mx-2 ${
                    completedSteps.has(index + 1) ? 'bg-green-500' : 'bg-gray-200'
                  }`}
                />
              )}
            </div>
          ))}
        </div>

        {/* 步骤内容 */}
        <Card className="p-6">
          {renderStep()}
        </Card>
      </div>
    </div>
  );
};