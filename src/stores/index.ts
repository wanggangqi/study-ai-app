export { useAuthStore } from './authStore';
export { useConfigStore } from './configStore';
export { useCourseStore } from './courseStore';
export { useChatStore } from './chatStore';
export { checkAuth } from './authStore';

// Re-export types from centralized types file
export type {
  AIProvider,
  AuthState,
  UserConfig,
  Course,
  Chapter,
  Lesson,
  ChatMessage,
  TeachingStyle,
} from '../types';

