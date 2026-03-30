import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Sidebar } from '../components/common/Sidebar';
import { Card } from '../components/common/Card';
import { Input } from '../components/common/Input';
import { Button } from '../components/common/Button';
import { useConfigStore } from '../stores/configStore';
import type { AIProvider } from '../types';

const navItems = [
  { icon: '📚', label: '课程', path: '/' },
  { icon: '💬', label: '咨询师', path: '/consultant' },
  { icon: '⚙️', label: '设置', path: '/settings' },
];

const aiProviders = [
  { id: 'claude' as AIProvider, name: 'Claude (Anthropic)', baseUrl: 'https://api.anthropic.com' },
  { id: 'openai' as AIProvider, name: 'ChatGPT (OpenAI)', baseUrl: 'https://api.openai.com' },
  { id: 'qwen' as AIProvider, name: '通义千问 (阿里云)', baseUrl: 'https://dashscope.aliyuncs.com' },
  { id: 'deepseek' as AIProvider, name: 'DeepSeek', baseUrl: 'https://api.deepseek.com' },
  { id: 'glm' as AIProvider, name: '智谱 GLM', baseUrl: 'https://open.bigmodel.cn' },
  { id: 'minimax' as AIProvider, name: 'MiniMax', baseUrl: 'https://api.minimax.chat' },
  { id: 'kimi' as AIProvider, name: 'Kimi (Moonshot)', baseUrl: 'https://api.moonshot.cn' },
];

export const SettingsPage: React.FC = () => {
  const navigate = useNavigate();
  const { aiProvider, aiApiKey, aiModel, setConfig } = useConfigStore();
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ success: boolean; message: string } | null>(null);

  const currentProvider = aiProviders.find(p => p.id === aiProvider);

  const handleTestConnection = async () => {
    if (!aiApiKey) {
      setTestResult({ success: false, message: '请先输入 API 密钥' });
      return;
    }

    setTesting(true);
    setTestResult(null);

    // TODO: Phase 4 实现实际的连接测试
    // 暂时模拟测试过程
    setTimeout(() => {
      setTesting(false);
      setTestResult({ success: true, message: '连接测试成功！（Phase 4 将实现真实测试）' });
    }, 1500);
  };

  const handleSave = () => {
    // TODO: Phase 2 实现实际的保存逻辑
    console.log('保存配置:', { aiProvider, aiApiKey, aiModel });
    alert('配置已保存！（Phase 2 将实现持久化）');
  };

  const handleCancel = () => {
    // TODO: Phase 2 实现重置逻辑
    navigate('/');
  };

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
                  className="w-full border border-gray-300 rounded-md px-3 py-2 bg-white"
                  value={aiProvider}
                  onChange={(e) => setConfig({ aiProvider: e.target.value as AIProvider })}
                >
                  {aiProviders.map((p) => (
                    <option key={p.id} value={p.id}>{p.name}</option>
                  ))}
                </select>
              </div>

              <Input
                label="API 地址"
                value={currentProvider?.baseUrl || ''}
                placeholder="API 请求地址"
                disabled
                className="bg-gray-50"
              />

              <Input
                label="API 密钥"
                type="password"
                placeholder="输入你的 API 密钥"
                value={aiApiKey}
                onChange={(e) => setConfig({ aiApiKey: e.target.value })}
              />

              <Input
                label="模型（可选）"
                placeholder="留空使用默认模型"
                value={aiModel}
                onChange={(e) => setConfig({ aiModel: e.target.value })}
              />

              <div className="flex items-center gap-2">
                <Button onClick={handleTestConnection} disabled={testing} size="sm">
                  {testing ? '测试中...' : '测试连接'}
                </Button>
                {testResult && (
                  <span className={testResult.success ? 'text-green-600' : 'text-red-600'}>
                    {testResult.message}
                  </span>
                )}
              </div>
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
            <Button variant="outline" onClick={handleCancel}>取消</Button>
            <Button onClick={handleSave}>保存设置</Button>
          </div>
        </div>
      </main>
    </div>
  );
};
