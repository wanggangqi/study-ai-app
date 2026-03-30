import { useAuthStore } from '../stores/authStore';

export const useAuth = () => {
  const { isAuthorized, expireAt, setAuthorized, clearAuth, checkAuth } = useAuthStore();

  return {
    isAuthorized,
    expireAt,
    setAuthorized,
    clearAuth,
    checkAuth,
  };
};
