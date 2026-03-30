import { create } from 'zustand';

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
}

export interface Chapter {
  id: string;
  courseId: string;
  chapterIndex: number;
  name: string;
}

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

interface CourseStore {
  courses: Course[];
  currentCourse: Course | null;
  currentChapter: Chapter | null;
  currentLesson: Lesson | null;

  setCourses: (courses: Course[]) => void;
  addCourse: (course: Course) => void;
  selectCourse: (courseId: string) => void;
  selectChapter: (chapter: Chapter | null) => void;
  selectLesson: (lesson: Lesson | null) => void;
  updateLessonStatus: (lessonId: string, status: Lesson['status']) => void;
}

export const useCourseStore = create<CourseStore>((set, get) => ({
  courses: [],
  currentCourse: null,
  currentChapter: null,
  currentLesson: null,

  setCourses: (courses) => set({ courses }),

  addCourse: (course) => set((state) => ({ courses: [...state.courses, course] })),

  selectCourse: (courseId) => {
    const course = get().courses.find((c) => c.id === courseId);
    set({ currentCourse: course || null, currentChapter: null, currentLesson: null });
  },

  selectChapter: (chapter) => set({ currentChapter: chapter }),

  selectLesson: (lesson) => set({ currentLesson: lesson }),

  updateLessonStatus: (lessonId, status) => {
    set((state) => ({
      currentLesson: state.currentLesson?.id === lessonId
        ? { ...state.currentLesson, status, completedAt: status === 'completed' ? new Date().toISOString() : undefined }
        : state.currentLesson,
    }));
  },
}));
