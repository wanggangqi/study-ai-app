import { create } from 'zustand';

interface AuthState {
  isAuthorized: boolean;
  expireAt: string | null;
  machineHash: string | null;
  setAuthorized: (expireAt: string, machineHash: string) => void;
  clearAuth: () => void;
  checkAuth: () => boolean;
}

export const useAuthStore = create<AuthState>((set, get) => ({
  isAuthorized: false,
  expireAt: null,
  machineHash: null,

  setAuthorized: (expireAt, machineHash) => {
    set({ isAuthorized: true, expireAt, machineHash });
  },

  clearAuth: () => {
    set({ isAuthorized: false, expireAt: null, machineHash: null });
  },

  checkAuth: () => {
    const state = get();
    if (!state.isAuthorized || !state.expireAt) return false;
    return new Date(state.expireAt) > new Date();
  },
}));
