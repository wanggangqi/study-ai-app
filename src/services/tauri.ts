import { invoke } from '@tauri-apps/api/core';

// Tauri 命令调用封装
export const tauriService = {
  // 授权相关
  async validateLicense(key: string): Promise<boolean> {
    return invoke('validate_license', { key });
  },

  // 配置相关
  async getMachineId(): Promise<string> {
    return invoke('get_machine_id');
  },

  // Git 相关
  async checkGitInstalled(): Promise<boolean> {
    return invoke('check_git_installed');
  },

  async setGitConfig(username: string, email: string): Promise<void> {
    return invoke('set_git_config', { username, email });
  },

  // 码云相关
  async createGiteeRepo(name: string, description: string): Promise<string> {
    return invoke('create_gitee_repo', { name, description });
  },

  async cloneRepo(url: string, path: string): Promise<void> {
    return invoke('clone_repo', { url, path });
  },

  // 数据库相关
  async getCourses(): Promise<any[]> {
    return invoke('get_courses');
  },

  async saveCourse(course: any): Promise<void> {
    return invoke('save_course', { course });
  },
};
