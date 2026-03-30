import { create } from 'zustand';

export type AIProvider = 'claude' | 'openai' | 'qwen' | 'deepseek' | 'glm' | 'minimax' | 'kimi';

interface UserConfig {
  giteeUsername: string;
  giteeToken: string;
  workspacePath: string;
  aiProvider: AIProvider;
  aiApiKey: string;
  aiModel: string;
  gitUsername: string;
  gitEmail: string;
}

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
