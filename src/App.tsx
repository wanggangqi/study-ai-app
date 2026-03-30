import React from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import { HomePage } from './pages/Home';
import { ConsultantPage } from './pages/Consultant';
import { SettingsPage } from './pages/Settings';
import { AuthPage } from './pages/Auth';
import { SetupPage } from './pages/Setup';
import { LearningPage } from './pages/Learning';
import { useAuthStore } from './stores/authStore';

const ProtectedRoute: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const isAuthorized = useAuthStore((state) => state.isAuthorized);
  const expireAt = useAuthStore((state) => state.expireAt);
  const isValid = isAuthorized && expireAt && new Date(expireAt) > new Date();

  if (!isValid) {
    return <Navigate to="/auth" replace />;
  }

  return <>{children}</>;
};

const App: React.FC = () => {
  return (
    <Routes>
      <Route path="/auth" element={<AuthPage />} />
      <Route
        path="/setup"
        element={
          <ProtectedRoute>
            <SetupPage />
          </ProtectedRoute>
        }
      />
      <Route
        path="/"
        element={
          <ProtectedRoute>
            <HomePage />
          </ProtectedRoute>
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
  );
};

export default App;
