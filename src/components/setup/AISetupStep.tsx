import React, { useState } from 'react';
import { useConfigStore } from '../../stores/configStore';
import { Input } from '../common/Input';
import { Button } from '../common/Button';
import { SetupStepProps } from './SetupWizard';

const AI_PROVIDERS = [
  { id: 'claude', name: 'Claude (Anthropic)', icon: '🤖', models: ['claude-3-opus', 'claude-3-sonnet', 'claude-3-haiku'] },
  { id: 'openai', name: 'ChatGPT (OpenAI)', icon: '💬', models: ['gpt-4', 'gpt-4-turbo', 'gpt-3.5-turbo'] },
  { id: 'qwen', name: '通义千问 (阿里云)', icon: '🌐', models: ['qwen-turbo', 'qwen-plus', 'qwen-max'] },
  { id: 'deepseek', name: 'DeepSeek', icon: '🔮', models: ['deepseek-chat', 'deepseek-coder'] },
  { id: 'glm', name: '智谱 GLM', icon: '✨', models: ['glm-4', 'glm-4-flash', 'glm-3-turbo'] },
  { id: 'minimax', name: 'MiniMax', icon: '🎯', models: ['abab6-chat', 'abab5.5-chat'] },
  { id: 'kimi', name: 'Kimi (Moonshot)', icon: '🌙', models: ['moonshot-v1-8k', 'moonshot-v1-32k'] },
];

export const AISetupStep: React.FC<SetupStepProps> = ({ onNext, onBack }) => {
  const [selectedProvider, setSelectedProvider] = useState('');
  const [apiKey, setApiKey] = useState('');
  const [selectedModel, setSelectedModel] = useState('');
  const [isValidating, setIsValidating] = useState(false);
  const [error, setError] = useState('');
  const { setConfig } = useConfigStore();

  const handleProviderSelect = (providerId: string) => {
    setSelectedProvider(providerId);
    setSelectedModel(''); // 重置模型选择
    setError('');
  };

  const handleSubmit = async () => {
    if (!selectedProvider || !apiKey.trim() || !selectedModel) {
      return;
    }

    setIsValidating(true);
    setError('');

    try {
      setConfig({
        aiProvider: selectedProvider as any,
        aiApiKey: apiKey,
        aiModel: selectedModel,
      });
      onNext();
    } catch (err) {
      setError('配置保存失败');
    } finally {
      setIsValidating(false);
    }
  };

  const selectedProviderData = AI_PROVIDERS.find(p => p.id === selectedProvider);

  return (
    <div>
      <div className="text-center mb-6">
        <div className="text-4xl mb-4">⚡</div>
        <h2 className="text-xl font-bold mb-2">AI 服务配置</h2>
        <p className="text-text-secondary">选择你喜欢的 AI 服务商并配置 API 密钥</p>
      </div>

      <div className="space-y-6">
        {/* 服务商选择 */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-3">
            选择 AI 服务商
          </label>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
            {AI_PROVIDERS.map((provider) => (
              <button
                key={provider.id}
                onClick={() => handleProviderSelect(provider.id)}
                className={`p-3 rounded-lg border-2 transition-all text-left ${
                  selectedProvider === provider.id
                    ? 'border-primary bg-primary/5'
                    : 'border-gray-200 hover:border-gray-300'
                }`}
              >
                <span className="text-2xl">{provider.icon}</span>
                <span className="block mt-1 text-sm font-medium">{provider.name}</span>
              </button>
            ))}
          </div>
        </div>

        {/* 模型选择 */}
        {selectedProviderData && (
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-3">
              选择模型
            </label>
            <select
              className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-primary"
              value={selectedModel}
              onChange={(e) => setSelectedModel(e.target.value)}
            >
              <option value="">请选择模型</option>
              {selectedProviderData.models.map((model) => (
                <option key={model} value={model}>
                  {model}
                </option>
              ))}
            </select>
          </div>
        )}

        {/* API 密钥 */}
        <Input
          label="API 密钥"
          type="password"
          placeholder="输入你的 API 密钥"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
        />
        {error && <p className="text-sm text-red-500">{error}</p>}
      </div>

      <div className="flex justify-between mt-6">
        <Button variant="outline" onClick={onBack}>
          上一步
        </Button>
        <Button
          onClick={handleSubmit}
          disabled={!selectedProvider || !apiKey.trim() || !selectedModel || isValidating}
        >
          {isValidating ? '验证中...' : '下一步'}
        </Button>
      </div>
    </div>
  );
};