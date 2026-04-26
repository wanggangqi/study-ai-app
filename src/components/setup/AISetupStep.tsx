import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useConfigStore } from '../../stores/configStore';
import { Input } from '../common/Input';
import { Button } from '../common/Button';
import { SetupStepProps } from './SetupWizard';

// 国内 AI 服务商列表（移除国外供应商）
const AI_PROVIDERS = [
  { id: 'qwen', name: '通义千问 (阿里云)', icon: '🌐', baseUrl: 'https://dashscope.aliyuncs.com/compatible-mode/v1', defaultModel: 'qwen3.5-plus' },
  { id: 'deepseek', name: 'DeepSeek', icon: '🔮', baseUrl: 'https://api.deepseek.com', defaultModel: 'deepseek-chat' },
  { id: 'glm', name: '智谱 GLM', icon: '✨', baseUrl: 'https://open.bigmodel.cn/api/paas/v4', defaultModel: 'glm-5' },
  { id: 'minimax', name: 'MiniMax', icon: '🎯', baseUrl: 'https://api.minimax.chat/v1', defaultModel: 'M2.7-highspeed' },
  { id: 'kimi', name: 'Kimi (Moonshot)', icon: '🌙', baseUrl: 'https://api.moonshot.cn/v1', defaultModel: 'kimi-k2.5' },
  { id: 'custom', name: '自定义', icon: '⚙️', baseUrl: '', defaultModel: '' },
];

export const AISetupStep: React.FC<SetupStepProps> = ({ onNext, onBack }) => {
  const [selectedProvider, setSelectedProvider] = useState('');
  const [apiKey, setApiKey] = useState('');
  const [selectedModel, setSelectedModel] = useState('');
  const [customBaseUrl, setCustomBaseUrl] = useState('');
  const [customModel, setCustomModel] = useState('');
  const [isValidating, setIsValidating] = useState(false);
  const [isTesting, setIsTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ success: boolean; message: string } | null>(null);
  const [error, setError] = useState('');
  const { setConfig, saveConfig } = useConfigStore();

  const handleProviderSelect = (providerId: string) => {
    const provider = AI_PROVIDERS.find(p => p.id === providerId);
    setSelectedProvider(providerId);
    setSelectedModel(provider?.defaultModel || ''); // 自动设置默认模型
    setCustomBaseUrl('');
    setCustomModel('');
    setError('');
    setTestResult(null);
  };

  // 测试连接
  const handleTestConnection = async () => {
    if (!apiKey.trim()) {
      setTestResult({ success: false, message: '请先输入 API 密钥' });
      return;
    }

    setIsTesting(true);
    setTestResult(null);

    try {
      const result = await invoke<{
        success: boolean;
        data?: string;
        error?: string;
      }>('ai_verify_key_command', {
        params: {
          provider: selectedProvider,
          api_key: apiKey,
          model: selectedProvider === 'custom' ? customModel : selectedModel,
          base_url: selectedProvider === 'custom' ? customBaseUrl : null,
        },
      });

      if (result.success && result.data !== 'invalid') {
        setTestResult({ success: true, message: 'API 密钥验证通过' });
      } else {
        setTestResult({ success: false, message: result.error || 'API 密钥验证失败' });
      }
    } catch (err) {
      setTestResult({ success: false, message: '连接测试失败' });
    } finally {
      setIsTesting(false);
    }
  };

  const handleSubmit = async () => {
    // 自定义服务商需要填写 baseUrl 和 model
    if (selectedProvider === 'custom') {
      if (!apiKey.trim() || !customBaseUrl.trim() || !customModel.trim()) {
        return;
      }
      setIsValidating(true);
      setError('');
      setConfig({
        aiProvider: 'custom',
        aiApiKey: apiKey,
        aiModel: customModel,
        customBaseUrl: customBaseUrl,
      });
      await saveConfig();
      setIsValidating(false);
      onNext();
      return;
    }

    if (!selectedProvider || !apiKey.trim() || !selectedModel) {
      return;
    }

    setIsValidating(true);
    setError('');

    // 如果已经验证通过，直接保存配置
    if (testResult?.success) {
      setConfig({
        aiProvider: selectedProvider as any,
        aiApiKey: apiKey,
        aiModel: selectedModel,
      });
      await saveConfig();
      setIsValidating(false);
      onNext();
      return;
    }

    try {
      // 验证 API 密钥
      const result = await invoke<{
        success: boolean;
        data?: string;
        error?: string;
      }>('ai_verify_key_command', {
        params: {
          provider: selectedProvider,
          api_key: apiKey,
          model: selectedProvider === 'custom' ? customModel : selectedModel,
        },
      });

      if (!result.success || result.data === 'invalid') {
        setError('API 密钥验证失败，请检查配置是否正确');
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
      setError('API 密钥验证失败，请检查配置');
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

        {/* 模型 */}
        {selectedProviderData && selectedProvider !== 'custom' && (
          <Input
            label="模型"
            type="text"
            placeholder="输入模型名称"
            value={selectedModel}
            onChange={(e) => setSelectedModel(e.target.value)}
          />
        )}

        {/* 自定义服务商配置 */}
        {selectedProvider === 'custom' && (
          <>
            <Input
              label="API 地址"
              type="text"
              placeholder="例如: https://api.example.com/v1"
              value={customBaseUrl}
              onChange={(e) => setCustomBaseUrl(e.target.value)}
            />
            <Input
              label="模型名称"
              type="text"
              placeholder="例如: gpt-4o"
              value={customModel}
              onChange={(e) => setCustomModel(e.target.value)}
            />
          </>
        )}

        {/* API 地址显示 */}
        {selectedProviderData && selectedProvider !== 'custom' && (
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              API 地址
            </label>
            <div className="w-full border border-gray-200 rounded-md px-3 py-2 bg-gray-50 text-gray-600 text-sm">
              {selectedProviderData.baseUrl}
            </div>
          </div>
        )}

        {/* API 密钥 */}
        <Input
          label="API 密钥"
          type="password"
          placeholder="输入你的 API 密钥"
          value={apiKey}
          onChange={(e) => {
            setApiKey(e.target.value);
            setTestResult(null); // 密钥变更时清除测试结果
          }}
        />

        {/* 测试连接 */}
        {selectedProvider && selectedProvider !== 'custom' && apiKey.trim() && (
          <div className="flex items-center gap-3">
            <Button
              variant="outline"
              size="sm"
              onClick={handleTestConnection}
              disabled={isTesting}
            >
              {isTesting ? '测试中...' : '测试连接'}
            </Button>
            {testResult && (
              <span className={`text-sm ${testResult.success ? 'text-green-600' : 'text-red-600'}`}>
                {testResult.message}
              </span>
            )}
          </div>
        )}

        {error && <p className="text-sm text-red-500">{error}</p>}
      </div>

      <div className="flex justify-between mt-6">
        <Button variant="outline" onClick={onBack}>
          上一步
        </Button>
        <Button
          onClick={handleSubmit}
          disabled={
            selectedProvider === 'custom'
              ? !apiKey.trim() || !customBaseUrl.trim() || !customModel.trim() || isValidating
              : !selectedProvider || !apiKey.trim() || !selectedModel || isValidating
          }
        >
          {isValidating ? '验证中...' : '下一步'}
        </Button>
      </div>
    </div>
  );
};