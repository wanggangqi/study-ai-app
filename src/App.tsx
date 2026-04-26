import React, { useState, useCallback, useEffect } from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import { HomePage } from './pages/Home';
import { ConsultantPage } from './pages/Consultant';
import { SettingsPage } from './pages/Settings';
import { AuthPage } from './pages/Auth';
import { SetupPage } from './pages/Setup';
import { LearningPage } from './pages/Learning';
import { useAuthStore } from './stores/authStore';
import { useConfigStore } from './stores/configStore';
import { AdminPanel } from './components/admin/AdminPanel';
import { StartupLoading } from './components/common/StartupLoading';

// 快捷键监听组件
const AdminShortcutHandler: React.FC<{ onTrigger: () => void }> = ({ onTrigger }) => {
  useEffect(() => {
    const handleKeyDown = async (event: KeyboardEvent) => {
      if (event.ctrlKey && event.shiftKey && event.key === 'G') {
        event.preventDefault();
        try {
          // 检查是否有签名密钥，没有则不允许打开管理员面板
          const hasSigningKey = await invoke<boolean>('is_signing_key_set_command');
          if (hasSigningKey) {
            onTrigger();
          }
        } catch (err) {
          console.error('Failed to check signing key:', err);
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onTrigger]);

  return null;
};

const App: React.FC = () => {
  const [showAdminPanel, setShowAdminPanel] = useState(false);
  const [isInitializing, setIsInitializing] = useState(true);

  // 所有 hooks 必须在条件返回之前调用，确保每次渲染 hooks 数量一致
  const isAuthorized = useAuthStore((state) => state.isAuthorized);
  const expireAt = useAuthStore((state) => state.expireAt);
  const setAuthorized = useAuthStore((state) => state.setAuthorized);
  const setupCompleted = useConfigStore((state) => state.setupCompleted);
  const loadConfig = useConfigStore((state) => state.loadConfig);

  // openAdminPanel 回调必须在这里定义，不能在条件返回之后
  const openAdminPanel = useCallback(() => {
    setShowAdminPanel(true);
  }, []);

  // 初始化：检查授权状态 + 加载配置
  useEffect(() => {
    const initialize = async () => {
      try {
        // 检查授权状态（包含机器码哈希，一次调用即可）
        const status = await invoke<{ is_licensed: boolean; expire_at?: string; error_message?: string; machine_hash?: string }>('get_license_status_command');

        if (status.is_licensed && status.expire_at && status.machine_hash) {
          setAuthorized(status.expire_at, status.machine_hash);
        }

        // 加载配置
        await loadConfig();
      } catch (err) {
        console.error('Initialization failed:', err);
      } finally {
        setIsInitializing(false);
      }
    };

    initialize();
  }, [setAuthorized, loadConfig]);

  // 判断授权是否有效
  const isAuthValid = isAuthorized && expireAt && new Date(expireAt) > new Date();

  // 初始化期间显示 Loading
  if (isInitializing) {
    return <StartupLoading />;
  }

  // 未授权时，所有路由都重定向到授权页（但保留授权页本身）
  if (!isAuthValid) {
    return (
      <>
        <Routes>
          <Route path="/auth" element={<AuthPage />} />
          <Route path="*" element={<Navigate to="/auth" replace />} />
        </Routes>
      </>
    );
  }

  // 已授权时的路由
  return (
    <>
      <AdminShortcutHandler onTrigger={openAdminPanel} />

      <Routes>
        <Route
          path="/setup"
          element={
            setupCompleted ? (
              <Navigate to="/" replace />
            ) : (
              <SetupPage />
            )
          }
        />
        <Route
          path="/"
          element={
            !setupCompleted ? (
              <Navigate to="/setup" replace />
            ) : (
              <HomePage />
            )
          }
        />
        <Route path="/consultant" element={<ConsultantPage />} />
        <Route path="/settings" element={<SettingsPage />} />
        <Route path="/learning/:courseId" element={<LearningPage />} />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>

      <AdminPanel isOpen={showAdminPanel} onClose={() => setShowAdminPanel(false)} />
    </>
  );
};

export default App;
