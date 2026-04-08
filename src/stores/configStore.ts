import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { UserConfig } from '../types';

interface ConfigStore extends UserConfig {
  isLoading: boolean;
  setConfig: (config: Partial<UserConfig>) => void;
  loadConfig: () => Promise<void>;
  saveConfig: () => Promise<void>;
  resetConfig: () => void;
}

const defaultConfig: UserConfig = {
  setupCompleted: false,
  giteeToken: '',
  workspacePath: '',
  aiProvider: 'claude',
  aiApiKey: '',
  aiModel: '',
  gitUsername: '',
  gitEmail: '',
};

export const useConfigStore = create<ConfigStore>((set, get) => ({
  ...defaultConfig,
  isLoading: true,

  setConfig: (config) => set((state) => ({ ...state, ...config })),

  loadConfig: async () => {
    try {
      set({ isLoading: true });
      const config = await invoke<{
        setup_completed: boolean;
        gitee_token: string | null;
        workspace_path: string | null;
        ai_provider: string | null;
        ai_api_key: string | null;
        ai_model: string | null;
      }>('get_config_command');

      set({
        setupCompleted: config.setup_completed,
        giteeToken: config.gitee_token || '',
        workspacePath: config.workspace_path || '',
        aiProvider: (config.ai_provider as UserConfig['aiProvider']) || 'claude',
        aiApiKey: config.ai_api_key || '',
        aiModel: config.ai_model || '',
        isLoading: false,
      });
    } catch (error) {
      console.error('Failed to load config:', error);
      set({ isLoading: false });
    }
  },

  saveConfig: async () => {
    const state = get();
    try {
      await invoke('set_config_command', {
        config: {
          setup_completed: state.setupCompleted,
          gitee_token: state.giteeToken || null,
          workspace_path: state.workspacePath || null,
          ai_provider: state.aiProvider || null,
          ai_api_key: state.aiApiKey || null,
          ai_model: state.aiModel || null,
        },
      });
    } catch (error) {
      console.error('Failed to save config:', error);
    }
  },

  resetConfig: () => set({ ...defaultConfig, isLoading: false }),
}));
