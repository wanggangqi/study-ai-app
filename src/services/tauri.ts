import { invoke } from '@tauri-apps/api/core';

// 课时状态枚举（与后端 LessonStatus 对应）
export enum LessonStatus {
  NotStarted = 0,
  InProgress = 1,
  Completed = 2,
}

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

  // 课时状态相关
  async updateLessonStatus(lessonId: string, status: LessonStatus): Promise<void> {
    return invoke('update_lesson_status', { id: lessonId, status });
  },

  async getLessonsByChapter(chapterId: string): Promise<object[]> {
    return invoke('get_lessons_by_chapter', { chapterId });
  },

  async getLessonById(lessonId: string): Promise<object> {
    return invoke('get_lesson_by_id', { id: lessonId });
  },

  // 练习题相关
  async updateExerciseScore(exerciseId: string, score: number): Promise<void> {
    return invoke('update_exercise_score', { id: exerciseId, score, resultFile: null });
  },
};
