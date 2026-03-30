import { create } from 'zustand';
import type { Course, Chapter, Lesson } from '../types';

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
    set((state) => {
      const updatedLesson = state.currentLesson?.id === lessonId
        ? { ...state.currentLesson, status, completedAt: status === 'completed' ? new Date().toISOString() : undefined }
        : state.currentLesson;

      // Update currentCourse progress if the lesson belongs to currentCourse
      let updatedCurrentCourse = state.currentCourse;
      if (updatedLesson && state.currentCourse) {
        const completedCount = state.currentCourse.completedLessons || 0;
        const wasCompleted = state.currentLesson?.status === 'completed';
        const isNowCompleted = status === 'completed';

        let newCompletedLessons = completedCount;
        if (!wasCompleted && isNowCompleted) {
          newCompletedLessons = completedCount + 1;
        } else if (wasCompleted && !isNowCompleted) {
          newCompletedLessons = Math.max(0, completedCount - 1);
        }

        updatedCurrentCourse = {
          ...state.currentCourse,
          completedLessons: newCompletedLessons,
          progress: state.currentCourse.totalLessons
            ? Math.round((newCompletedLessons / state.currentCourse.totalLessons) * 100)
            : undefined,
        };
      }

      return {
        currentLesson: updatedLesson,
        currentCourse: updatedCurrentCourse,
      };
    });
  },
}));
