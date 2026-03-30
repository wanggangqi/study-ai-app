import { create } from 'zustand';
import type { AuthState } from '../types';

export interface AuthStore extends AuthState {
  setAuthorized: (expireAt: string, machineHash: string) => void;
  clearAuth: () => void;
  checkAuth: () => boolean;
}

export const useAuthStore = create<AuthStore>((set, get) => ({
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

// Helper function to check auth from non-component code
export const checkAuth = () => useAuthStore.getState().checkAuth();
