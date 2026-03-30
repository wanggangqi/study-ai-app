import { create } from 'zustand';
import type { UserConfig } from '../types';

interface ConfigStore extends UserConfig {
  isSetupComplete: boolean;
  setConfig: (config: Partial<UserConfig>) => void;
  setSetupComplete: (complete: boolean) => void;
  resetConfig: () => void;
}

const defaultConfig: UserConfig = {
  giteeUsername: '',
  giteeToken: '',
  workspacePath: '',
  aiProvider: 'claude',
  aiApiKey: '',
  aiModel: '',
  gitUsername: '',
  gitEmail: '',
};

export const useConfigStore = create<ConfigStore>((set) => ({
  ...defaultConfig,
  isSetupComplete: false,

  setConfig: (config) => set((state) => ({ ...state, ...config })),

  setSetupComplete: (complete) => set({ isSetupComplete: complete }),

  resetConfig: () => set({ ...defaultConfig, isSetupComplete: false }),
}));
