import { useCourseStore } from '../stores/courseStore';

export const useCourse = () => {
  const {
    courses,
    currentCourse,
    currentChapter,
    currentLesson,
    setCourses,
    addCourse,
    selectCourse,
    selectChapter,
    selectLesson,
    updateLessonStatus,
  } = useCourseStore();

  return {
    courses,
    currentCourse,
    currentChapter,
    currentLesson,
    setCourses,
    addCourse,
    selectCourse,
    selectChapter,
    selectLesson,
    updateLessonStatus,
  };
};
