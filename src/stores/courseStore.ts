import { create } from 'zustand';
import type { Course, Chapter, Lesson, ChapterWithLessons, Exercise } from '../types';
import { tauriService, LessonStatus as BackendLessonStatus } from '../services/tauri';

// 课时状态映射：前端状态 -> 后端状态
const statusToBackend = (status: Lesson['status']): BackendLessonStatus => {
  switch (status) {
    case 'not_started':
      return BackendLessonStatus.NotStarted;
    case 'in_progress':
      return BackendLessonStatus.InProgress;
    case 'completed':
      return BackendLessonStatus.Completed;
    default:
      return BackendLessonStatus.NotStarted;
  }
};

interface CourseStore {
  courses: Course[];
  currentCourse: Course | null;
  currentChapter: Chapter | null;
  currentLesson: Lesson | null;
  // 课件 HTML 内容，key 为 lessonId
  lessonContents: Record<string, string>;
  // 练习题列表
  exercises: Exercise[];
  // 加载状态
  isLoading: boolean;

  setCourses: (courses: Course[]) => void;
  addCourse: (course: Course) => void;
  loadCourses: () => Promise<void>;
  selectCourse: (courseId: string) => void;
  selectChapter: (chapter: Chapter | null) => void;
  selectLesson: (lesson: Lesson | null) => void;
  updateLessonStatus: (lessonId: string, status: Lesson['status']) => void;
  setLessonContent: (lessonId: string, content: string) => void;
  setExercises: (exercises: Exercise[]) => void;
  submitExerciseAnswer: (exerciseId: string, answer: string) => void;
}

export const useCourseStore = create<CourseStore>((set, get) => ({
  courses: [],
  currentCourse: null,
  currentChapter: null,
  currentLesson: null,
  lessonContents: {},
  exercises: [],
  isLoading: false,

  setCourses: (courses) => set({ courses }),

  addCourse: (course) => set((state) => ({ courses: [...state.courses, course] })),

  loadCourses: async () => {
    set({ isLoading: true });
    try {
      const courses = await tauriService.getCourses();
      set({ courses, isLoading: false });
    } catch (error) {
      console.error('Failed to load courses:', error);
      set({ isLoading: false });
    }
  },

  selectCourse: (courseId) => {
    const course = get().courses.find((c) => c.id === courseId);
    set({ currentCourse: course || null, currentChapter: null, currentLesson: null });
  },

  selectChapter: (chapter) => set({ currentChapter: chapter }),

  selectLesson: (lesson) => set({ currentLesson: lesson }),

  updateLessonStatus: (lessonId, status) => {
    // 持久化到后端
    tauriService.updateLessonStatus(lessonId, statusToBackend(status)).catch((err) => {
      console.error('Failed to persist lesson status:', err);
    });

    set((state) => {
      const newCompletedAt = status === 'completed' ? new Date().toISOString() : undefined;

      // Update courses array - find and update the lesson in the nested structure
      const updatedCourses = state.courses.map((course) => {
        const chapters = course.chapters as ChapterWithLessons[] | undefined;
        let lessonFound = false;
        let totalLessons = 0;

        // First pass: count total lessons and check if lesson belongs to this course
        chapters?.forEach((chapter: ChapterWithLessons) => {
          chapter.lessons?.forEach((lesson: Lesson) => {
            totalLessons++;
            if (lesson.id === lessonId) {
              lessonFound = true;
            }
          });
        });

        if (!lessonFound) {
          return course;
        }

        // Second pass: update the lesson and recalculate progress
        const updatedChapters: ChapterWithLessons[] = chapters?.map((chapter: ChapterWithLessons) => ({
          ...chapter,
          lessons: chapter.lessons?.map((lesson: Lesson) =>
            lesson.id === lessonId
              ? { ...lesson, status, completedAt: newCompletedAt }
              : lesson
          ),
        })) || [];

        // Recalculate completed lessons count
        let completedLessons = 0;
        updatedChapters.forEach((chapter: ChapterWithLessons) => {
          chapter.lessons?.forEach((lesson: Lesson) => {
            if (lesson.status === 'completed') {
              completedLessons++;
            }
          });
        });

        return {
          ...course,
          chapters: updatedChapters,
          completedLessons,
          progress: totalLessons > 0 ? Math.round((completedLessons / totalLessons) * 100) : 0,
        };
      });

      // Update currentLesson
      const updatedLesson = state.currentLesson?.id === lessonId
        ? { ...state.currentLesson, status, completedAt: newCompletedAt }
        : state.currentLesson;

      // Update currentCourse if it contains this lesson
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
        courses: updatedCourses,
        currentLesson: updatedLesson,
        currentCourse: updatedCurrentCourse,
      };
    });
  },

  setLessonContent: (lessonId, content) => {
    set((state) => ({
      lessonContents: {
        ...state.lessonContents,
        [lessonId]: content,
      },
    }));
  },

  setExercises: (exercises) => set({ exercises }),

  submitExerciseAnswer: (exerciseId, answer) => {
    set((state) => ({
      exercises: state.exercises.map((ex) =>
        ex.id === exerciseId
          ? {
              ...ex,
              userAnswer: answer,
              isCorrect: answer === ex.correctAnswer,
            }
          : ex
      ),
    }));
  },
}));
