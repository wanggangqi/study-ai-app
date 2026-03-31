import { useCourseStore } from '../stores/courseStore';

export const useCourse = () => {
  const {
    courses,
    currentCourse,
    currentChapter,
    currentLesson,
    lessonContents,
    exercises,
    setCourses,
    addCourse,
    selectCourse,
    selectChapter,
    selectLesson,
    updateLessonStatus,
    setLessonContent,
    setExercises,
    submitExerciseAnswer,
  } = useCourseStore();

  return {
    courses,
    currentCourse,
    currentChapter,
    currentLesson,
    lessonContents,
    exercises,
    setCourses,
    addCourse,
    selectCourse,
    selectChapter,
    selectLesson,
    updateLessonStatus,
    setLessonContent,
    setExercises,
    submitExerciseAnswer,
  };
};
