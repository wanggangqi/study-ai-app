import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Sidebar } from '../components/common/Sidebar';
import { Card } from '../components/common/Card';
import { Button } from '../components/common/Button';
import { Input } from '../components/common/Input';
import { message } from '@tauri-apps/plugin-dialog';

const navItems = [
  { icon: '📚', label: '课程', path: '/' },
  { icon: '💬', label: '咨询师', path: '/consultant' },
  { icon: '⚙️', label: '设置', path: '/settings' },
];

const topicOptions = [
  { title: 'Python 编程', desc: '从零开始，想做数据分析' },
  { title: 'React 开发', desc: '有一些前端基础，想做项目' },
  { title: '英语', desc: '提升口语能力' },
];

export const ConsultantPage: React.FC = () => {
  const navigate = useNavigate();
  const [customTopic, setCustomTopic] = useState('');

  const handleTopicSelect = async (topic: string) => {
    // TODO: Phase 4 实现实际的 AI 咨询逻辑
    console.log('选择主题:', topic);
    await message(`你选择了: ${topic}\n\nAI 咨询功能将在 Phase 4 实现。`, { title: '提示', kind: 'info' });
  };

  const handleStartConsult = () => {
    if (customTopic.trim()) {
      handleTopicSelect(customTopic.trim());
    }
  };

  return (
    <div className="flex min-h-screen">
      <Sidebar items={navItems} activePath="/consultant" onNavigate={(path) => navigate(path)} />

      <main className="flex-1 p-8">
        <h1 className="text-2xl font-bold text-primary mb-8">学习咨询师</h1>

        <Card className="max-w-2xl mx-auto">
          <div className="text-center mb-6">
            <div className="text-4xl mb-4">👨‍🏫</div>
            <h2 className="text-xl font-bold mb-2">你好！我是你的学习顾问</h2>
            <p className="text-text-secondary">
              我来帮你制定个性化的学习计划。请告诉我你想学习什么内容？
            </p>
          </div>

          <div className="space-y-4">
            {topicOptions.map((option, index) => (
              <div
                key={index}
                onClick={() => handleTopicSelect(option.title)}
                className="border border-gray-200 rounded-md p-4 hover:border-primary cursor-pointer transition-colors bg-white"
              >
                <h3 className="font-medium">{option.title}</h3>
                <p className="text-sm text-gray-500">{option.desc}</p>
              </div>
            ))}
          </div>

          <div className="mt-6">
            <p className="text-center text-gray-500 text-sm mb-3">或者输入你想学习的内容...</p>
            <div className="flex gap-2">
              <Input
                placeholder="例如：机器学习、摄影、历史..."
                value={customTopic}
                onChange={(e) => setCustomTopic(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleStartConsult()}
                className="flex-1"
              />
              <Button onClick={handleStartConsult}>开始咨询</Button>
            </div>
          </div>
        </Card>
      </main>
    </div>
  );
};
