import React from 'react';
import { useNavigate } from 'react-router-dom';
import { Sidebar } from '../components/common/Sidebar';
import { Card } from '../components/common/Card';
import { Button } from '../components/common/Button';

const navItems = [
  { icon: '📚', label: '课程', path: '/' },
  { icon: '💬', label: '咨询师', path: '/consultant' },
  { icon: '⚙️', label: '设置', path: '/settings' },
];

export const ConsultantPage: React.FC = () => {
  const navigate = useNavigate();

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
            <div className="border border-gray-200 rounded-md p-4 hover:border-primary cursor-pointer transition-colors">
              <h3 className="font-medium">我想学习 Python 编程</h3>
              <p className="text-sm text-text-muted">从零开始，想做数据分析</p>
            </div>

            <div className="border border-gray-200 rounded-md p-4 hover:border-primary cursor-pointer transition-colors">
              <h3 className="font-medium">我想学习 React 开发</h3>
              <p className="text-sm text-text-muted">有一些前端基础，想做项目</p>
            </div>

            <div className="border border-gray-200 rounded-md p-4 hover:border-primary cursor-pointer transition-colors">
              <h3 className="font-medium">我想学习英语</h3>
              <p className="text-sm text-text-muted">提升口语能力</p>
            </div>
          </div>

          <div className="mt-6 text-center text-text-muted text-sm">
            或者输入你想学习的内容...
          </div>
        </Card>
      </main>
    </div>
  );
};
