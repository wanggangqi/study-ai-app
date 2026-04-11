// AI 服务商类型（国内供应商）
export type AIProvider = 'qwen' | 'deepseek' | 'glm' | 'minimax' | 'kimi';

// 授权状态
export interface AuthState {
  isAuthorized: boolean;
  expireAt: string | null;
  machineHash: string | null;
}

// 用户配置（与后端 AppConfig 对应）
export interface UserConfig {
  setupCompleted: boolean;
  giteeUsername: string;
  giteeToken: string;
  workspacePath: string;
  aiProvider: AIProvider;
  aiApiKey: string;
  aiModel: string;
  gitUsername: string;
  gitEmail: string;
  teachingStyle: string;
}

// 课程计划（咨询师输出）
export interface CoursePlan {
  name: string;
  targetLevel: string;
  duration: string;
  teachingStyle: string;
  baseKnowledge: string;
}

// 课程
export interface Course {
  id: string;
  name: string;
  giteeRepoUrl: string;
  localPath: string;
  targetLevel: string;
  duration: string;
  teachingStyle: string;
  createdAt: string;
  status: 'active' | 'completed' | 'paused';
  progress?: number;
  totalLessons?: number;
  completedLessons?: number;
  chapters?: ChapterWithLessons[];
}

// 带课时的章节
export interface ChapterWithLessons extends Chapter {
  lessons?: Lesson[];
}

// 章节
export interface Chapter {
  id: string;
  courseId: string;
  chapterIndex: number;
  name: string;
}

// 课时
export interface Lesson {
  id: string;
  chapterId: string;
  lessonIndex: number;
  name: string;
  duration: string;
  status: 'not_started' | 'in_progress' | 'completed';
  completedAt?: string;
  lessonFile?: string;
}

// 课件 HTML 内容
export interface CourseContent {
  lessonId: string;
  html: string;
  updatedAt: string;
}

// 练习题选项
export interface ExerciseOption {
  id: string;
  label: string;
  content: string;
}

// 练习题
export interface Exercise {
  id: string;
  lessonId: string;
  question: string;
  options: ExerciseOption[];
  correctAnswer: string; // 选项 id
  explanation?: string;
  userAnswer?: string; // 用户选择的选项 id
  isCorrect?: boolean; // 回答是否正确
}

// 消息
export interface ChatMessage {
  id: string;
  courseId: string;
  lessonId?: string;
  agentType: 'consultant' | 'teacher';
  role: 'user' | 'assistant';
  content: string;
  createdAt: string;
}

// 教学风格
export interface TeachingStyle {
  id: string;
  name: string;
  description: string;
  icon: string;
}

// 课时计划大纲
export interface LessonPlanOutline {
  lessonIndex: number;
  lessonName: string;
  duration: string;
}

// 章节计划大纲
export interface ChapterPlanOutline {
  chapterIndex: number;
  chapterName: string;
  lessons: LessonPlanOutline[];
}

// 课程计划大纲（AI 生成）
export interface CoursePlanOutline {
  courseName: string;
  targetLevel: string;
  duration: string;
  teachingStyle: string;
  chapters: ChapterPlanOutline[];
}

// 默认模型映射（国内供应商）
export const DEFAULT_MODELS: Record<AIProvider, string> = {
  qwen: 'qwen-plus',
  deepseek: 'deepseek-chat',
  glm: 'glm-4-flash',
  minimax: 'abab6.5s-chat',
  kimi: 'moonshot-v1-8k',
};