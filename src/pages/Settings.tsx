import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Sidebar } from '../components/common/Sidebar';
import { Card } from '../components/common/Card';
import { Input } from '../components/common/Input';
import { Button } from '../components/common/Button';
import { useConfigStore } from '../stores/configStore';

const navItems = [
  { icon: '📚', label: '课程', path: '/' },
  { icon: '💬', label: '咨询师', path: '/consultant' },
  { icon: '⚙️', label: '设置', path: '/settings' },
];

const aiProviders = [
  { id: 'claude', name: 'Claude (Anthropic)' },
  { id: 'openai', name: 'ChatGPT (OpenAI)' },
  { id: 'qwen', name: '通义千问 (阿里云)' },
  { id: 'deepseek', name: 'DeepSeek' },
  { id: 'glm', name: '智谱 GLM' },
  { id: 'minimax', name: 'MiniMax' },
  { id: 'kimi', name: 'Kimi (Moonshot)' },
];

export const SettingsPage: React.FC = () => {
  const navigate = useNavigate();
  const { aiProvider, setConfig } = useConfigStore();

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
                  className="w-full border border-gray-300 rounded-md px-3 py-2"
                  value={aiProvider}
                  onChange={(e) => setConfig({ aiProvider: e.target.value as any })}
                >
                  {aiProviders.map((p) => (
                    <option key={p.id} value={p.id}>{p.name}</option>
                  ))}
                </select>
              </div>
              <Input
                label="API 密钥"
                type="password"
                placeholder="输入你的 API 密钥"
              />
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
            <Button variant="outline">取消</Button>
            <Button>保存设置</Button>
          </div>
        </div>
      </main>
    </div>
  );
};
