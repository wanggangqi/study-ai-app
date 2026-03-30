import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Input } from '../common/Input';
import { Button } from '../common/Button';
import { SetupStepProps } from './SetupWizard';
import { useConfigStore } from '../../stores/configStore';

export const GitSetupStep: React.FC<SetupStepProps> = ({ onNext, onBack }) => {
  const [isGitInstalled, setIsGitInstalled] = useState<boolean | null>(null);
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [isChecking, setIsChecking] = useState(true);
  const { setConfig } = useConfigStore();

  useEffect(() => {
    checkGitInstallation();
  }, []);

  const checkGitInstallation = async () => {
    try {
      const result = await invoke<boolean>('check_git_installed');
      setIsGitInstalled(result);
    } catch (error) {
      console.error('Failed to check git:', error);
      setIsGitInstalled(false);
    } finally {
      setIsChecking(false);
    }
  };

  const handleSubmit = async () => {
    if (!username.trim() || !email.trim()) {
      return;
    }

    try {
      await invoke('set_git_config', { username, email });
      setConfig({ gitUsername: username, gitEmail: email });
      onNext();
    } catch (error) {
      console.error('Failed to set git config:', error);
    }
  };

  if (isChecking) {
    return (
      <div className="text-center py-8">
        <div className="text-4xl mb-4">🔍</div>
        <p className="text-text-secondary">正在检查 Git 安装...</p>
      </div>
    );
  }

  if (!isGitInstalled) {
    return (
      <div className="text-center py-8">
        <div className="text-4xl mb-4">❌</div>
        <h2 className="text-xl font-bold mb-4">未检测到 Git</h2>
        <p className="text-text-secondary mb-4">
          请先安装 Git 才能继续。访问
          <a
            href="https://git-scm.com/download/win"
            target="_blank"
            rel="noopener noreferrer"
            className="text-primary underline ml-1"
          >
            git-scm.com
          </a>
          下载安装。
        </p>
        <Button onClick={checkGitInstallation} className="mt-4">
          重新检测
        </Button>
      </div>
    );
  }

  return (
    <div>
      <div className="text-center mb-6">
        <div className="text-4xl mb-4">✅</div>
        <h2 className="text-xl font-bold mb-2">Git 已安装</h2>
        <p className="text-text-secondary">请配置你的 Git 用户信息</p>
      </div>

      <div className="space-y-4">
        <Input
          label="Git 用户名"
          placeholder="请输入你的 Git 用户名"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
        <Input
          label="Git 邮箱"
          type="email"
          placeholder="请输入你的 Git 邮箱"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
      </div>

      <div className="flex justify-between mt-6">
        <Button variant="outline" onClick={onBack}>
          上一步
        </Button>
        <Button onClick={handleSubmit} disabled={!username.trim() || !email.trim()}>
          下一步
        </Button>
      </div>
    </div>
  );
};