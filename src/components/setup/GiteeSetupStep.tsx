import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useConfigStore } from '../../stores/configStore';
import { Input } from '../common/Input';
import { Button } from '../common/Button';
import { SetupStepProps } from './SetupWizard';

export const GiteeSetupStep: React.FC<SetupStepProps> = ({ onNext, onBack }) => {
  const [token, setToken] = useState('');
  const [isValidating, setIsValidating] = useState(false);
  const [error, setError] = useState('');
  const { setConfig, saveConfig } = useConfigStore();

  const handleSubmit = async () => {
    if (!token.trim()) {
      return;
    }

    setIsValidating(true);
    setError('');

    try {
      // 验证码云账户
      const result = await invoke<{
        success: boolean;
        username?: string;
        message: string;
      }>('verify_gitee_account_command', { token });

      if (!result.success) {
        setError(result.message || '验证码云账户失败，请检查令牌');
        setIsValidating(false);
        return;
      }

      // 保存用户名和 token
      setConfig({
        giteeUsername: result.username || '',
        giteeToken: token,
      });
      await saveConfig();
      onNext();
    } catch (err) {
      setError('验证码云账户失败，请检查令牌是否正确');
    } finally {
      setIsValidating(false);
    }
  };

  return (
    <div>
      <div className="text-center mb-6">
        <div className="text-4xl mb-4">🔐</div>
        <h2 className="text-xl font-bold mb-2">码云账户配置</h2>
        <p className="text-text-secondary">配置你的码云（Gitee）访问令牌</p>
      </div>

      <div className="space-y-4">
        <Input
          label="个人访问令牌"
          type="password"
          placeholder="请输入你的码云个人访问令牌"
          value={token}
          onChange={(e) => setToken(e.target.value)}
        />
        {error && <p className="text-sm text-red-500">{error}</p>}
        <p className="text-xs text-text-muted">
          如何获取令牌？
          <a
            href="https://gitee.com/profile/personal_access_tokens"
            target="_blank"
            rel="noopener noreferrer"
            className="text-primary underline ml-1"
          >
            码云 → 个人设置 → 私人令牌
          </a>
        </p>
      </div>

      <div className="flex justify-between mt-6">
        <Button variant="outline" onClick={onBack}>
          上一步
        </Button>
        <Button onClick={handleSubmit} disabled={!token.trim() || isValidating}>
          {isValidating ? '验证中...' : '下一步'}
        </Button>
      </div>
    </div>
  );
};
