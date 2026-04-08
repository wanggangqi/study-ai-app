import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

interface AdminPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

export const AdminPanel: React.FC<AdminPanelProps> = ({ isOpen, onClose }) => {
  const [machineHash, setMachineHash] = useState('');
  const [expireDate, setExpireDate] = useState('');
  const [generatedKey, setGeneratedKey] = useState('');
  const [error, setError] = useState('');
  const [copied, setCopied] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [signingKeyStatus, setSigningKeyStatus] = useState<string>('');

  useEffect(() => {
    if (isOpen) {
      loadMachineHash();
      checkSigningKeyStatus();
    }
  }, [isOpen]);

  const checkSigningKeyStatus = async () => {
    try {
      const isSet = await invoke<boolean>('is_signing_key_set_command');
      setSigningKeyStatus(isSet ? '已设置' : '使用内置默认密钥');
    } catch (err) {
      setSigningKeyStatus('使用内置默认密钥');
    }
  };

  const loadMachineHash = async () => {
    try {
      const hash = await invoke<string>('get_machine_hash_command');
      setMachineHash(hash);
    } catch (err) {
      console.error('Failed to get machine hash:', err);
    }
  };

  const handleGenerateKey = async () => {
    if (!machineHash || !expireDate) {
      setError('请填写所有字段');
      return;
    }
    setError('');
    setIsLoading(true);
    try {
      const key = await invoke<string>('generate_license_key_command', {
        expireDate,
        machineHash: machineHash.trim(),
      });
      setGeneratedKey(key);
    } catch (err) {
      setError('生成密钥失败');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCopyKey = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  const handleClose = () => {
    setMachineHash('');
    setExpireDate('');
    setGeneratedKey('');
    setError('');
    setCopied(false);
    onClose();
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <Card className="w-full max-w-md max-h-[90vh] overflow-y-auto">
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle>管理员模式</CardTitle>
            <button
              onClick={handleClose}
              className="text-muted-foreground hover:text-foreground"
            >
              ✕
            </button>
          </div>
          <CardDescription>
            签名密钥状态：{signingKeyStatus}
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div>
              <label className="text-sm font-medium">机器码</label>
              <Input
                value={machineHash}
                onChange={(e) => setMachineHash(e.target.value)}
                placeholder="输入用户机器码"
                className="mt-1 font-mono text-xs"
              />
            </div>
            <div>
              <label className="text-sm font-medium">过期日期</label>
              <Input
                type="date"
                value={expireDate}
                onChange={(e) => setExpireDate(e.target.value)}
                className="mt-1"
              />
            </div>
            {error && <p className="text-sm text-red-500">{error}</p>}

            {!generatedKey ? (
              <Button
                onClick={handleGenerateKey}
                disabled={isLoading}
                className="w-full"
              >
                {isLoading ? '生成中...' : '生成密钥'}
              </Button>
            ) : (
              <div className="space-y-2">
                <div className="p-3 bg-muted rounded-lg">
                  <div className="flex items-center justify-between mb-1">
                    <span className="text-xs text-muted-foreground">生成的密钥</span>
                    <button
                      onClick={() => handleCopyKey(generatedKey)}
                      className="text-xs text-primary hover:text-primary/80"
                    >
                      {copied ? '已复制!' : '点击复制'}
                    </button>
                  </div>
                  <div className="font-mono text-xs break-all">{generatedKey}</div>
                </div>
                <Button onClick={() => setGeneratedKey('')} variant="outline" className="w-full">
                  继续生成
                </Button>
              </div>
            )}

            <Button onClick={handleClose} variant="ghost" className="w-full mt-4">
              关闭
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};
