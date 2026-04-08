import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

type AdminMode = 'login' | 'set-password' | 'init-signing-key' | 'generate';

interface AdminPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

interface SigningKeyInfo {
  signing_key: string;
  verify_key: string;
}

export const AdminPanel: React.FC<AdminPanelProps> = ({ isOpen, onClose }) => {
  const [mode, setMode] = useState<AdminMode>('login');
  const [password, setPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [machineHash, setMachineHash] = useState('');
  const [expireDate, setExpireDate] = useState('');
  const [generatedKey, setGeneratedKey] = useState('');
  const [error, setError] = useState('');
  const [copied, setCopied] = useState(false);

  // 签名密钥相关状态
  const [signingKeyInfo, setSigningKeyInfo] = useState<SigningKeyInfo | null>(null);
  const [verifyKeyInput, setVerifyKeyInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  // 加载管理员密码状态和机器码
  useEffect(() => {
    if (isOpen) {
      checkAdminPasswordStatus();
      loadMachineHash();
    }
  }, [isOpen]);

  // 检查签名密钥状态
  useEffect(() => {
    if (mode === 'generate') {
      checkSigningKeyStatus();
    }
  }, [mode]);

  const checkAdminPasswordStatus = async () => {
    try {
      const isSet = await invoke<boolean>('is_admin_password_set_command');
      setMode(isSet ? 'login' : 'set-password');
    } catch (err) {
      console.error('Failed to check admin password:', err);
      setMode('set-password');
    }
  };

  const checkSigningKeyStatus = async () => {
    try {
      const isSet = await invoke<boolean>('is_signing_key_set_command');
      if (!isSet) {
        setMode('init-signing-key');
      }
    } catch (err) {
      console.error('Failed to check signing key:', err);
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

  const handleVerifyPassword = async () => {
    if (!password) return;
    setError('');
    try {
      const isValid = await invoke<boolean>('verify_admin_password_command', { password });
      if (isValid) {
        setMode('generate');
        setPassword('');
      } else {
        setError('密码错误');
      }
    } catch (err) {
      setError('验证失败');
    }
  };

  const handleSetPassword = async () => {
    if (!newPassword || !confirmPassword) {
      setError('请填写所有字段');
      return;
    }
    if (newPassword !== confirmPassword) {
      setError('两次输入的密码不一致');
      return;
    }
    if (newPassword.length < 6) {
      setError('密码长度至少6位');
      return;
    }
    setError('');
    try {
      await invoke('set_admin_password_command', { password: newPassword });
      setMode('generate');
      setNewPassword('');
      setConfirmPassword('');
    } catch (err) {
      setError('设置密码失败');
    }
  };

  const handleGenerateKeyPair = async () => {
    setError('');
    setIsLoading(true);
    try {
      const info = await invoke<SigningKeyInfo>('generate_signing_key_pair_command');
      setSigningKeyInfo(info);
    } catch (err) {
      setError('生成密钥对失败');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSetVerifyKey = async () => {
    if (!verifyKeyInput.trim()) {
      setError('请输入公钥');
      return;
    }
    if (!signingKeyInfo) {
      setError('请先生成密钥对');
      return;
    }

    // 简单验证公钥格式（Base64）
    try {
      atob(verifyKeyInput.trim());
    } catch {
      setError('公钥格式无效');
      return;
    }

    setError('');
    setIsLoading(true);
    try {
      await invoke('set_signing_key_command', { signingKey: verifyKeyInput.trim() });
      setMode('generate');
      setSigningKeyInfo(null);
      setVerifyKeyInput('');
    } catch (err) {
      setError('设置公钥失败');
      console.error(err);
    } finally {
      setIsLoading(false);
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
    setPassword('');
    setNewPassword('');
    setConfirmPassword('');
    setGeneratedKey('');
    setError('');
    setCopied(false);
    setSigningKeyInfo(null);
    setVerifyKeyInput('');
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
            {mode === 'login' && '请输入管理员密码'}
            {mode === 'set-password' && '首次使用，请设置管理员密码'}
            {mode === 'init-signing-key' && '初始化签名密钥'}
            {mode === 'generate' && '生成用户激活密钥'}
          </CardDescription>
        </CardHeader>
        <CardContent>
          {mode === 'login' && (
            <div className="space-y-4">
              <div>
                <label className="text-sm font-medium">管理员密码</label>
                <Input
                  type="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  placeholder="输入密码"
                  className="mt-1"
                />
              </div>
              {error && <p className="text-sm text-red-500">{error}</p>}
              <Button onClick={handleVerifyPassword} className="w-full">
                确认
              </Button>
            </div>
          )}

          {mode === 'set-password' && (
            <div className="space-y-4">
              <div>
                <label className="text-sm font-medium">新密码</label>
                <Input
                  type="password"
                  value={newPassword}
                  onChange={(e) => setNewPassword(e.target.value)}
                  placeholder="输入新密码"
                  className="mt-1"
                />
              </div>
              <div>
                <label className="text-sm font-medium">确认密码</label>
                <Input
                  type="password"
                  value={confirmPassword}
                  onChange={(e) => setConfirmPassword(e.target.value)}
                  placeholder="再次输入新密码"
                  className="mt-1"
                />
              </div>
              {error && <p className="text-sm text-red-500">{error}</p>}
              <Button onClick={handleSetPassword} className="w-full">
                设置密码
              </Button>
            </div>
          )}

          {mode === 'init-signing-key' && (
            <div className="space-y-4">
              <div className="p-3 bg-yellow-50 border border-yellow-200 rounded-lg text-sm">
                <p className="font-medium text-yellow-800 mb-2">重要：签名密钥初始化</p>
                <ol className="list-decimal list-inside text-yellow-700 space-y-1">
                  <li>点击"生成密钥对"按钮</li>
                  <li><strong>妥善保存私钥</strong>（用于生成许可证）</li>
                  <li>将公钥填入下方输入框并确认</li>
                </ol>
              </div>

              {!signingKeyInfo ? (
                <Button
                  onClick={handleGenerateKeyPair}
                  disabled={isLoading}
                  className="w-full"
                >
                  {isLoading ? '生成中...' : '生成密钥对'}
                </Button>
              ) : (
                <>
                  <div>
                    <label className="text-sm font-medium">私钥（请妥善保管）</label>
                    <div className="relative">
                      <Input
                        value={signingKeyInfo.signing_key}
                        readOnly
                        className="mt-1 font-mono text-xs pr-16"
                      />
                      <Button
                        size="sm"
                        variant="ghost"
                        className="absolute right-1 top-1/2 -translate-y-1/2 h-7 px-2 text-xs"
                        onClick={() => handleCopyKey(signingKeyInfo.signing_key)}
                      >
                        {copied ? '已复制' : '复制'}
                      </Button>
                    </div>
                    <p className="text-xs text-red-500 mt-1">⚠️ 私钥丢失将无法生成许可证！</p>
                  </div>

                  <div>
                    <label className="text-sm font-medium">公钥</label>
                    <Input
                      value={signingKeyInfo.verify_key}
                      readOnly
                      className="mt-1 font-mono text-xs"
                    />
                  </div>

                  <div className="border-t pt-4 mt-4">
                    <label className="text-sm font-medium">设置公钥（验证用）</label>
                    <p className="text-xs text-muted-foreground mb-2">
                      请将上方的公钥复制到输入框中确认设置
                    </p>
                    <Input
                      value={verifyKeyInput}
                      onChange={(e) => setVerifyKeyInput(e.target.value)}
                      placeholder="粘贴公钥确认"
                      className="mt-1 font-mono text-xs"
                    />
                    <p className="text-xs text-muted-foreground mt-1">
                      为确保公钥正确，请重新粘贴上方显示的公钥
                    </p>
                  </div>

                  {error && <p className="text-sm text-red-500">{error}</p>}

                  <Button
                    onClick={handleSetVerifyKey}
                    disabled={isLoading}
                    className="w-full"
                  >
                    {isLoading ? '设置中...' : '确认设置公钥'}
                  </Button>
                </>
              )}
            </div>
          )}

          {mode === 'generate' && (
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
          )}
        </CardContent>
      </Card>
    </div>
  );
};
