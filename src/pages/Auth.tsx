import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useNavigate } from 'react-router-dom';
import { Card } from '../components/common/Card';
import { Input } from '../components/common/Input';
import { Button } from '../components/common/Button';
import { useAuthStore } from '../stores/authStore';

export const AuthPage: React.FC = () => {
  const [licenseKey, setLicenseKey] = useState('');
  const [isValidating, setIsValidating] = useState(false);
  const [error, setError] = useState('');
  const [machineId, setMachineId] = useState('');
  const navigate = useNavigate();
  const { setAuthorized } = useAuthStore();

  useEffect(() => {
    // 获取机器码显示
    loadMachineId();
    // 检查当前授权状态
    checkLicenseStatus();
  }, []);

  const loadMachineId = async () => {
    try {
      const id = await invoke<string>('get_machine_id');
      setMachineId(id);
    } catch (err) {
      console.error('Failed to get machine ID:', err);
    }
  };

  const checkLicenseStatus = async () => {
    try {
      const status = await invoke<{ is_licensed: boolean; expire_at?: string; error_message?: string }>('get_license_status');
      if (status.is_licensed && status.expire_at) {
        setAuthorized(status.expire_at, machineId);
        navigate('/setup');
      }
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
      }>('validate_license', { key: licenseKey });

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

  return (
    <div className="min-h-screen flex items-center justify-center bg-bg-primary">
      <Card className="max-w-md w-full">
        <div className="text-center mb-6">
          <div className="text-4xl mb-4">🔐</div>
          <h1 className="text-2xl font-bold text-primary mb-2">智学伴侣</h1>
          <p className="text-text-secondary">输入激活密钥开始使用</p>
        </div>

        {machineId && (
          <div className="mb-4 p-2 bg-gray-50 rounded text-xs text-text-muted text-center">
            机器码：{machineId.substring(0, 32)}...
          </div>
        )}

        <div className="space-y-4">
          <Input
            label="激活密钥"
            placeholder="请输入你的激活密钥"
            value={licenseKey}
            onChange={(e) => setLicenseKey(e.target.value)}
            error={error}
          />
          <Button
            className="w-full"
            onClick={handleActivate}
            disabled={!licenseKey.trim() || isValidating}
          >
            {isValidating ? '激活中...' : '激活'}
          </Button>
        </div>

        <div className="mt-4 text-center text-sm text-text-muted">
          <p>没有密钥？<a href="#" className="text-primary hover:underline">获取密钥</a></p>
        </div>
      </Card>
    </div>
  );
};