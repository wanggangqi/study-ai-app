import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useNavigate } from 'react-router-dom';
import { Card } from '../components/ui/card';
import { Input } from '../components/ui/input';
import { Button } from '../components/ui/button';
import { useAuthStore } from '../stores/authStore';

export const AuthPage: React.FC = () => {
  const [licenseKey, setLicenseKey] = useState('');
  const [isValidating, setIsValidating] = useState(false);
  const [error, setError] = useState('');
  const [machineId, setMachineId] = useState('');
  const [copied, setCopied] = useState(false);
  const navigate = useNavigate();
  const { setAuthorized } = useAuthStore();

  useEffect(() => {
    loadMachineId();
    checkLicenseStatus();
  }, []);

  const loadMachineId = async () => {
    try {
      // 使用 get_machine_hash_command 获取十六进制哈希（64位）
      const hash = await invoke<string>('get_machine_hash_command');
      setMachineId(hash);
    } catch (err) {
      console.error('Failed to get machine ID:', err);
    }
  };

  const checkLicenseStatus = async () => {
    try {
      const status = await invoke<{ is_licensed: boolean; expire_at?: string; error_message?: string }>('get_license_status_command');
      if (status.is_licensed && status.expire_at) {
        setAuthorized(status.expire_at, machineId);
        navigate('/setup');
      }
      // 没license就停留在授权页面
    } catch (err) {
      console.error('Failed to check license status:', err);
    }
  };

  const handleActivate = async () => {
    if (!licenseKey.trim()) {
      return;
    }

    setIsValidating(true);
    setError('');

    try {
      const result = await invoke<{
        is_licensed: boolean;
        expire_at?: string;
        error_message?: string;
      }>('validate_license_command', { licenseKey });

      if (result.is_licensed && result.expire_at) {
        setAuthorized(result.expire_at, machineId);
        navigate('/setup');
      } else {
        setError(result.error_message || '激活失败');
      }
    } catch (err) {
      console.error('Activation failed:', err);
      setError('激活失败，请检查密钥是否正确');
    } finally {
      setIsValidating(false);
    }
  };

  const handleCopyMachineId = async () => {
    try {
      await navigator.clipboard.writeText(machineId);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  return (
    <div className="h-screen flex items-center justify-center bg-[#fefae0] p-4 overflow-hidden">
      <Card className="max-w-md w-full">
        <div className="text-center mb-6">
          <div className="text-4xl mb-4">&#128273;</div>
          <h1 className="text-2xl font-bold text-[#588157] mb-2">智学伴侣</h1>
          <p className="text-[#666666]">输入激活密钥开始使用</p>
        </div>

        {machineId && (
          <div className="mb-6 p-4 bg-muted rounded-lg">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium text-foreground">您的机器码</span>
              <button
                onClick={handleCopyMachineId}
                className="text-xs text-primary hover:text-primary/80 transition-colors"
              >
                {copied ? '已复制!' : '点击复制'}
              </button>
            </div>
            <div className="p-2 bg-background rounded border border-input font-mono text-xs text-muted-foreground break-all">
              {machineId}
            </div>
            <p className="text-xs text-muted-foreground mt-2">
              请将此机器码提供给管理员以获取激活密钥
            </p>
          </div>
        )}

        <div className="space-y-4">
          <div className="flex flex-col gap-1">
            <label className="text-sm font-medium text-foreground">激活密钥</label>
            <Input
              placeholder="请输入你的激活密钥"
              value={licenseKey}
              onChange={(e) => setLicenseKey(e.target.value)}
            />
            {error && <span className="text-sm text-red-500">{error}</span>}
          </div>
          <Button
            className="w-full"
            onClick={handleActivate}
            disabled={!licenseKey.trim() || isValidating}
          >
            {isValidating ? '激活中...' : '激活'}
          </Button>
        </div>

        <div className="mt-6 p-4 bg-accent/10 rounded-lg">
          <p className="text-sm text-muted-foreground text-center">
            <span className="font-medium">没有密钥?</span>
            <br />
            请联系管理员，提供您的机器码获取激活密钥
          </p>
        </div>
      </Card>
    </div>
  );
};
