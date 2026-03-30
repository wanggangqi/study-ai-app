import { invoke } from '@tauri-apps/api/core';

// Tauri 命令调用封装
export const tauriService = {
  async validateLicense(key: string): Promise<boolean> {
    return invoke('validate_license', { key });
  },

  async getMachineId(): Promise<string> {
    return invoke('get_machine_id');
  },

  async checkGitInstalled(): Promise<boolean> {
    return invoke('check_git_installed');
  },

  async setGitConfig(username: string, email: string): Promise<void> {
    return invoke('set_git_config', { username, email });
  },

  async createGiteeRepo(name: string, description: string): Promise<string> {
    return invoke('create_gitee_repo', { name, description });
  },

  async cloneRepo(url: string, path: string): Promise<void> {
    return invoke('clone_repo', { url, path });
  },

  async getCourses(): Promise<object[]> {
    return invoke('get_courses');
  },

  async saveCourse(course: object): Promise<void> {
    return invoke('save_course', { course });
  },
};
