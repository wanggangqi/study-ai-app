import React, { useState, useCallback, useEffect } from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { HomePage } from './pages/Home';
import { ConsultantPage } from './pages/Consultant';
import { SettingsPage } from './pages/Settings';
import { AuthPage } from './pages/Auth';
import { SetupPage } from './pages/Setup';
import { LearningPage } from './pages/Learning';
import { useAuthStore } from './stores/authStore';
import { useConfigStore } from './stores/configStore';
import { AdminPanel } from './components/admin/AdminPanel';

const ProtectedRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const isAuthorized = useAuthStore((state) => state.isAuthorized);
  const expireAt = useAuthStore((state) => state.expireAt);
  const isValid = isAuthorized && expireAt && new Date(expireAt) > new Date();

  if (!isValid) {
    return <Navigate to="/auth" replace />;
  }

  return <>{children}</>;
};

// 快捷键监听组件
const AdminShortcutHandler: React.FC<{ onTrigger: () => void }> = ({ onTrigger }) => {
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.ctrlKey && event.shiftKey && event.key === 'G') {
        event.preventDefault();
        onTrigger();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onTrigger]);

  return null;
};

// 配置加载组件
const ConfigLoader: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const loadConfig = useConfigStore((state) => state.loadConfig);
  const isLoading = useConfigStore((state) => state.isLoading);

  useEffect(() => {
    loadConfig();
  }, [loadConfig]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center">
          <div className="text-4xl mb-4">📚</div>
          <p className="text-text-secondary">正在加载...</p>
        </div>
      </div>
    );
  }

  return <>{children}</>;
};

const App: React.FC = () => {
  const [showAdminPanel, setShowAdminPanel] = useState(false);
  const setupCompleted = useConfigStore((state) => state.setupCompleted);

  const openAdminPanel = useCallback(() => {
    setShowAdminPanel(true);
  }, []);

  return (
    <>
      <AdminShortcutHandler onTrigger={openAdminPanel} />

      <ConfigLoader>
        <Routes>
          <Route path="/auth" element={<AuthPage />} />
          <Route
            path="/setup"
            element={
              setupCompleted ? (
                <Navigate to="/" replace />
              ) : (
                <ProtectedRoute>
                  <SetupPage />
                </ProtectedRoute>
              )
            }
          />
          <Route
            path="/"
            element={
              !setupCompleted ? (
                <Navigate to="/setup" replace />
              ) : (
                <ProtectedRoute>
                  <HomePage />
                </ProtectedRoute>
              )
            }
          />
          <Route
            path="/consultant"
            element={
              <ProtectedRoute>
                <ConsultantPage />
              </ProtectedRoute>
            }
          />
          <Route
            path="/settings"
            element={
              <ProtectedRoute>
                <SettingsPage />
              </ProtectedRoute>
            }
          />
          <Route
            path="/learning/:courseId"
            element={
              <ProtectedRoute>
                <LearningPage />
              </ProtectedRoute>
            }
          />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </ConfigLoader>

      <AdminPanel isOpen={showAdminPanel} onClose={() => setShowAdminPanel(false)} />
    </>
  );
};

export default App;
