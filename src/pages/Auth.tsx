import React from 'react';
import { Card } from '../components/common/Card';
import { Input } from '../components/common/Input';
import { Button } from '../components/common/Button';

export const AuthPage: React.FC = () => {
  return (
    <div className="min-h-screen flex items-center justify-center bg-bg-primary">
      <Card className="max-w-md w-full">
        <div className="text-center mb-6">
          <div className="text-4xl mb-4">🔐</div>
          <h1 className="text-2xl font-bold text-primary mb-2">智学伴侣</h1>
          <p className="text-text-secondary">输入激活密钥开始使用</p>
        </div>

        <div className="space-y-4">
          <Input
            label="激活密钥"
            placeholder="请输入你的激活密钥"
          />
          <Button className="w-full">激活</Button>
        </div>

        <div className="mt-4 text-center text-sm text-text-muted">
          <p>没有密钥？<a href="#" className="text-primary hover:underline">获取密钥</a></p>
        </div>
      </Card>
    </div>
  );
};
