import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
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

const builtInTeachingStyles = [
  { id: 'academic', name: '严谨学术型', icon: '📚', description: '结构严谨、概念清晰，适合系统学习' },
  { id: 'practical', name: '实战应用型', icon: '💻', description: '案例驱动、边学边做，强调动手实践' },
  { id: 'story', name: '轻松故事型', icon: '📖', description: '故事/比喻引导，语言轻松有趣' },
  { id: 'progressive', name: '循序渐进型', icon: '🎓', description: '小步前进、充分练习，打牢基础' },
  { id: 'explorative', name: '启发探索型', icon: '🔍', description: '提问引导、独立思考，培养能力' },
  { id: 'efficient', name: '快速高效型', icon: '⚡', description: '精炼要点、直奔目标，高效学习' },
];

export const SettingsPage: React.FC = () => {
  const navigate = useNavigate();
  const {
    aiProvider, aiApiKey, aiModel,
    gitUsername, gitEmail,
    giteeUsername, giteeToken,
    teachingStyle,
    setConfig, saveConfig,
  } = useConfigStore();

  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ success: boolean; message: string } | null>(null);

  const currentProvider = aiProviders.find(p => p.id === aiProvider);

  // 从 teachingStyle 中解析自定义描述
  const getCustomStyleDesc = (): string => {
    if (teachingStyle.startsWith('custom:')) {
      return teachingStyle.slice(7); // 去掉 "custom:" 前缀
    }
    return '';
  };

  // 判断是否为内置风格
  const isBuiltInStyle = (styleId: string) => {
    return builtInTeachingStyles.some(s => s.id === styleId);
  };

  const handleTestConnection = async () => {
    if (!aiApiKey) {
      setTestResult({ success: false, message: '请先输入 API 密钥' });
      return;
    }

    setTesting(true);
    setTestResult(null);

    try {
      const result = await invoke<{
        success: boolean;
        data?: string;
        error?: string;
      }>('ai_verify_key_command', {
        params: { provider: aiProvider, api_key: aiApiKey },
      });

      if (result.success && result.data !== 'invalid') {
        setTestResult({ success: true, message: 'API 密钥验证通过' });
      } else {
        setTestResult({ success: false, message: result.error || 'API 密钥验证失败' });
      }
    } catch (err) {
      setTestResult({ success: false, message: '连接测试失败' });
    } finally {
      setTesting(false);
    }
  };

  const handleStyleSelect = (styleId: string) => {
    if (styleId === 'custom') {
      // 选择自定义风格，保持之前的描述
      const existingDesc = getCustomStyleDesc();
      setConfig({ teachingStyle: existingDesc ? `custom:${existingDesc}` : 'custom:' });
    } else {
      setConfig({ teachingStyle: styleId });
    }
  };

  const handleCustomStyleChange = (desc: string) => {
    // 输入描述时直接更新 teachingStyle
    setConfig({ teachingStyle: desc ? `custom:${desc}` : 'custom:' });
  };

  const handleSave = async () => {
    try {
      await saveConfig();
      alert('配置已保存');
    } catch (err) {
      console.error('Failed to save config:', err);
      alert('保存失败');
    }
  };

  const handleCancel = () => {
    navigate('/');
  };

  return (
    <div className="flex min-h-screen">
      <Sidebar items={navItems} activePath="/settings" onNavigate={(path) => navigate(path)} />

      <main className="flex-1 p-8">
        <h1 className="text-2xl font-bold text-primary mb-8">设置</h1>

        <div className="max-w-2xl space-y-6">
          {/* AI 服务配置 */}
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

          {/* Git 配置 */}
          <Card>
            <h2 className="font-bold mb-4">Git 配置</h2>
            <div className="space-y-4">
              <Input
                label="Git 用户名"
                placeholder="你的 Git 用户名"
                value={gitUsername}
                onChange={(e) => setConfig({ gitUsername: e.target.value })}
              />
              <Input
                label="Git 邮箱"
                placeholder="你的 Git 邮箱"
                value={gitEmail}
                onChange={(e) => setConfig({ gitEmail: e.target.value })}
              />
            </div>
          </Card>

          {/* 码云配置 */}
          <Card>
            <h2 className="font-bold mb-4">码云配置</h2>
            <div className="space-y-4">
              <Input
                label="码云用户名"
                placeholder="你的码云用户名"
                value={giteeUsername}
                onChange={(e) => setConfig({ giteeUsername: e.target.value })}
              />
              <Input
                label="码云访问令牌"
                type="password"
                placeholder="你的码云个人访问令牌"
                value={giteeToken}
                onChange={(e) => setConfig({ giteeToken: e.target.value })}
              />
            </div>
          </Card>

          {/* 教学风格 */}
          <Card>
            <h2 className="font-bold mb-4">教学风格</h2>
            <div className="grid grid-cols-3 gap-3 mb-4">
              {builtInTeachingStyles.map((style) => (
                <button
                  key={style.id}
                  onClick={() => handleStyleSelect(style.id)}
                  className={`p-3 rounded-lg border-2 text-center transition-all ${
                    teachingStyle === style.id
                      ? 'border-primary bg-primary/5'
                      : 'border-gray-200 hover:border-gray-300'
                  }`}
                >
                  <span className="text-2xl">{style.icon}</span>
                  <span className="block mt-1 text-sm font-medium">{style.name}</span>
                </button>
              ))}
            </div>

            {/* 自定义风格 */}
            <div className="border-2 rounded-lg p-3 mt-4">
              <div className="flex items-center gap-3 mb-2">
                <button
                  onClick={() => handleStyleSelect('custom')}
                  className={`px-3 py-2 rounded-lg border-2 transition-all ${
                    !isBuiltInStyle(teachingStyle)
                      ? 'border-primary bg-primary/5'
                      : 'border-gray-200 hover:border-gray-300'
                  }`}
                >
                  ✏️ 自定义风格
                </button>
                <span className="text-sm text-gray-500">描述你喜欢的教学风格</span>
              </div>
              {!isBuiltInStyle(teachingStyle) && (
                <textarea
                  className="w-full border border-gray-300 rounded-md px-3 py-2 mt-2"
                  rows={2}
                  placeholder="例如：喜欢用图表和实例讲解，注重动手实践"
                  value={getCustomStyleDesc()}
                  onChange={(e) => handleCustomStyleChange(e.target.value)}
                />
              )}
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
