// 本地存储服务
const STORAGE_KEYS = {
  AUTH_STATE: 'studymate_auth',
  CONFIG: 'studymate_config',
  COURSES: 'studymate_courses',
} as const;

export const storageService = {
  get<T>(key: string, defaultValue: T): T {
    const item = localStorage.getItem(key);
    return item ? JSON.parse(item) : defaultValue;
  },

  set<T>(key: string, value: T): void {
    localStorage.setItem(key, JSON.stringify(value));
  },

  remove(key: string): void {
    localStorage.removeItem(key);
  },

  clear(): void {
    Object.values(STORAGE_KEYS).forEach((key) => localStorage.removeItem(key));
  },
};
