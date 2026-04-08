import { invoke } from '@tauri-apps/api/core';
import type { AIProvider, Chapter, Course, CoursePlanOutline } from '../types';

// 课时状态枚举（与后端 LessonStatus 对应）
export enum LessonStatus {
  NotStarted = 0,
  InProgress = 1,
  Completed = 2,
}

// 后端 Course 数据结构（snake_case）
interface BackendCourse {
  id: string;
  name: string;
  gitee_repo_url: string | null;
  local_path: string | null;
  target_level: string | null;
  duration: string | null;
  teaching_style: string | null;
  created_at: string;
  status: number;
}

// 课程状态映射：后端数值 -> 前端字符串
const statusFromBackend = (status: number): Course['status'] => {
  switch (status) {
    case 0:
      return 'active';
    case 1:
      return 'completed';
    case 2:
      return 'paused';
    default:
      return 'active';
  }
};

// 转换后端数据为前端格式
const transformCourse = (data: BackendCourse): Course => ({
  id: data.id,
  name: data.name,
  giteeRepoUrl: data.gitee_repo_url || '',
  localPath: data.local_path || '',
  targetLevel: data.target_level || '',
  duration: data.duration || '',
  teachingStyle: data.teaching_style || '',
  createdAt: data.created_at,
  status: statusFromBackend(data.status),
});

// Tauri 命令调用封装
export const tauriService = {
  async validateLicense(key: string): Promise<boolean> {
    return invoke('validate_license_command', { key });
  },

  async getMachineId(): Promise<string> {
    return invoke('get_machine_id_command');
  },

  async checkGitInstalled(): Promise<boolean> {
    return invoke('check_git_installed_command');
  },

  async createGiteeRepo(name: string, description: string): Promise<string> {
    return invoke('create_gitee_repo_command', { name, description });
  },

  async cloneRepo(url: string, path: string): Promise<void> {
    return invoke('git_clone_command', { url, path });
  },

  async getCourses(): Promise<Course[]> {
    const data = await invoke<BackendCourse[]>('get_all_courses_command');
    return data.map(transformCourse);
  },

  async saveCourse(course: object): Promise<void> {
    return invoke('create_course_command', { course });
  },

  // 课时状态相关
  async updateLessonStatus(lessonId: string, status: LessonStatus): Promise<void> {
    return invoke('update_lesson_status_command', { id: lessonId, status });
  },

  async getLessonsByChapter(chapterId: string): Promise<object[]> {
    return invoke('get_lessons_by_chapter_command', { chapterId });
  },

  async getLessonById(lessonId: string): Promise<object> {
    return invoke('get_lesson_by_id_command', { id: lessonId });
  },

  // 练习题相关
  async updateExerciseScore(exerciseId: string, score: number): Promise<void> {
    return invoke('update_exercise_score_command', { id: exerciseId, score, resultFile: null });
  },

  // 码云同步相关
  async syncCourseToGitee(courseId: string): Promise<{ success: boolean; message: string; repoUrl?: string }> {
    return invoke('sync_course_to_git_command', { courseId });
  },

  async createCourseRepository(courseId: string): Promise<{ success: boolean; message: string; repoUrl?: string }> {
    return invoke('create_course_repository_command', { courseId });
  },

  // 课程大纲生成
  async generateCoursePlan(params: {
    provider: AIProvider;
    apiKey: string;
    model?: string;
    courseName: string;
    targetLevel: string;
    duration: string;
    teachingStyle: string;
    baseKnowledge: string;
  }): Promise<CoursePlanOutline> {
    return invoke('ai_generate_course_plan_command', { params });
  },

  // 批量创建章节和课时
  async createChaptersWithLessons(params: {
    courseId: string;
    chapters: Array<{
      chapterIndex: number;
      chapterName: string;
      lessons: Array<{
        lessonIndex: number;
        lessonName: string;
        duration: string;
      }>;
    }>;
  }): Promise<Chapter[]> {
    return invoke('create_chapters_with_lessons_command', { params });
  },
};
