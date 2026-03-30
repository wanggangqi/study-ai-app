import { create } from 'zustand';
import type { Course, Chapter, Lesson, ChapterWithLessons } from '../types';

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
}));
