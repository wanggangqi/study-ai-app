import React, { useCallback, useMemo } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Card } from '../components/common/Card';
import { Button } from '../components/common/Button';
import { Sidebar } from '../components/common/Sidebar';
import { TeacherAgent } from '../components/teacher/TeacherAgent';
import { useCourseStore } from '../stores/courseStore';
import type { Lesson, ChapterWithLessons } from '../types';

const navItems = [
  { icon: '📚', label: '课程', path: '/' },
  { icon: '💬', label: '咨询师', path: '/consultant' },
  { icon: '⚙️', label: '设置', path: '/settings' },
];

export const LearningPage: React.FC = () => {
  const { courseId } = useParams<{ courseId: string }>();
  const navigate = useNavigate();
  const {
    currentCourse,
    currentChapter,
    currentLesson,
    lessonContents,
    selectCourse,
    selectChapter,
    selectLesson,
    updateLessonStatus,
  } = useCourseStore();

  // 当前课件内容
  const currentLessonContent = currentLesson
    ? lessonContents[currentLesson.id]
    : null;

  // 计算当前课时的上下文信息（用于上一课/下一课导航）
  const { prevLesson, nextLesson } = useMemo(() => {
    if (!currentCourse?.chapters) {
      return { prevLesson: null, nextLesson: null };
    }

    const allLessons: { lesson: Lesson; chapter: ChapterWithLessons }[] = [];

    currentCourse.chapters.forEach((chapter: ChapterWithLessons) => {
      if (chapter.lessons) {
        chapter.lessons.forEach((lesson: Lesson) => {
          allLessons.push({ lesson, chapter });
        });
      }
    });

    const currentIndex = allLessons.findIndex(
      (item) => item.lesson.id === currentLesson?.id
    );

    return {
      prevLesson:
        currentIndex > 0 ? allLessons[currentIndex - 1] : null,
      nextLesson:
        currentIndex < allLessons.length - 1
          ? allLessons[currentIndex + 1]
          : null,
    };
  }, [currentCourse, currentLesson]);

  // 加载课时
  const handleSelectLesson = useCallback(
    (lesson: Lesson, chapter: ChapterWithLessons) => {
      // 如果点击的是当前课时，不做任何操作
      if (currentLesson?.id === lesson.id) {
        return;
      }

      // 更新当前章节和课时
      selectChapter(chapter);
      selectLesson(lesson);

      // 如果课时状态是 not_started，设置为 in_progress
      if (lesson.status === 'not_started') {
        updateLessonStatus(lesson.id, 'in_progress');
      }
    },
    [currentLesson, selectChapter, selectLesson, updateLessonStatus]
  );

  // 标记当前课时完成
  const handleMarkComplete = useCallback(() => {
    if (currentLesson) {
      updateLessonStatus(currentLesson.id, 'completed');
    }
  }, [currentLesson, updateLessonStatus]);

  // 上一课
  const handlePrevLesson = useCallback(() => {
    if (prevLesson) {
      handleSelectLesson(prevLesson.lesson, prevLesson.chapter);
    }
  }, [prevLesson, handleSelectLesson]);

  // 下一课
  const handleNextLesson = useCallback(() => {
    if (nextLesson) {
      handleSelectLesson(nextLesson.lesson, nextLesson.chapter);
    }
  }, [nextLesson, handleSelectLesson]);

  // 计算当前章节索引和课时在章节内的索引
  const { currentChapterIndex, currentLessonIndexInChapter } = useMemo(() => {
    if (!currentCourse?.chapters || !currentLesson) {
      return { currentChapterIndex: -1, currentLessonIndexInChapter: -1 };
    }

    for (
      let chapterIdx = 0;
      chapterIdx < currentCourse.chapters.length;
      chapterIdx++
    ) {
      const chapter = currentCourse.chapters[chapterIdx];
      if (chapter.lessons) {
        const lessonIdx = chapter.lessons.findIndex(
          (l: Lesson) => l.id === currentLesson.id
        );
        if (lessonIdx !== -1) {
          return {
            currentChapterIndex: chapterIdx,
            currentLessonIndexInChapter: lessonIdx,
          };
        }
      }
    }

    return { currentChapterIndex: -1, currentLessonIndexInChapter: -1 };
  }, [currentCourse, currentLesson]);

  React.useEffect(() => {
    if (courseId) {
      selectCourse(courseId);
    }
  }, [courseId, selectCourse]);

  if (!currentCourse) {
    return (
      <div className="flex min-h-screen">
        <Sidebar items={navItems} activePath="/" onNavigate={(path) => navigate(path)} />
        <main className="flex-1 p-8 flex items-center justify-center">
          <Card className="text-center py-12">
            <p className="text-text-muted mb-4">课程加载中...</p>
            <Button onClick={() => navigate('/')}>返回首页</Button>
          </Card>
        </main>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen flex-col">
      {/* 顶部栏 */}
      <header className="bg-white border-b border-gray-200 px-6 py-3 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Button variant="outline" size="sm" onClick={() => navigate('/')}>
            ← 返回
          </Button>
          <div className="h-6 w-px bg-gray-200" />
          <h1 className="font-bold text-lg text-gray-900">{currentCourse.name}</h1>
          {currentChapter && (
            <>
              <span className="text-gray-400">/</span>
              <span className="text-gray-600">{currentChapter.name}</span>
            </>
          )}
        </div>
        <div className="flex items-center gap-4">
          {/* 进度显示 */}
          <div className="flex items-center gap-2">
            <span className="text-sm text-gray-500">学习进度</span>
            <div className="w-32 h-2 bg-gray-200 rounded-full overflow-hidden">
              <div
                className="h-full bg-primary transition-all"
                style={{ width: `${currentCourse.progress || 0}%` }}
              />
            </div>
            <span className="text-sm font-medium text-gray-700">
              {currentCourse.progress || 0}%
            </span>
          </div>
          {/* 课时进度 */}
          {currentChapter && currentLesson && (
            <div className="text-sm text-gray-500">
              第{currentChapterIndex + 1}章 / 第{currentLessonIndexInChapter + 1}课时
            </div>
          )}
        </div>
      </header>

      {/* 三栏内容区 */}
      <div className="flex flex-1 overflow-hidden">
        {/* 左侧导航栏 - 课程大纲 */}
        <aside className="w-64 bg-bg-secondary border-r border-gray-200 overflow-y-auto">
          <div className="p-4">
            <h2 className="font-bold text-primary mb-4 px-2">
              课程大纲
            </h2>
            <div className="space-y-3">
              {currentCourse.chapters?.map((chapter: ChapterWithLessons, chapterIndex: number) => (
                <div key={chapter.id}>
                  <div className="px-2 py-2 text-sm font-medium text-text-secondary flex items-center gap-2">
                    <span className="text-primary">第{chapterIndex + 1}章</span>
                    <span className="truncate">{chapter.name}</span>
                  </div>
                  <div className="space-y-1">
                    {chapter.lessons?.map((lesson: Lesson) => {
                      const isActive = currentLesson?.id === lesson.id;
                      const isCompleted = lesson.status === 'completed';
                      const isInProgress = lesson.status === 'in_progress';

                      return (
                        <button
                          key={lesson.id}
                          onClick={() => handleSelectLesson(lesson, chapter)}
                          className={`w-full text-left px-4 py-2 text-sm rounded-md transition-all ${
                            isActive
                              ? 'bg-primary text-white shadow-sm'
                              : isCompleted
                              ? 'text-green-600 bg-green-50 hover:bg-green-100'
                              : isInProgress
                              ? 'text-primary bg-primary/10 hover:bg-primary/20'
                              : 'text-text-muted hover:bg-white/50'
                          }`}
                        >
                          <span className="mr-2 flex-shrink-0">
                            {isCompleted ? (
                              '✓'
                            ) : isInProgress ? (
                              '▶'
                            ) : (
                              '○'
                            )}
                          </span>
                          <span className="truncate">{lesson.name}</span>
                        </button>
                      );
                    })}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </aside>

        {/* 中间内容区 - 课件展示 */}
        <main className="flex-1 p-6 overflow-y-auto bg-gray-50">
          <Card className="h-full min-h-[600px] flex flex-col">
            {currentLesson ? (
              <>
                {/* 课件内容区 */}
                <div className="flex-1 overflow-y-auto p-6">
                  {currentLessonContent ? (
                    <div
                      className="prose max-w-none"
                      dangerouslySetInnerHTML={{ __html: currentLessonContent }}
                    />
                  ) : (
                    <div className="flex flex-col items-center justify-center h-full text-text-muted">
                      <div className="text-6xl mb-4">📖</div>
                      <p className="text-lg mb-2">正在加载课件...</p>
                      <p className="text-sm">课件内容将通过 AI 自动生成</p>
                    </div>
                  )}
                </div>

                {/* 底部操作区 */}
                <div className="border-t border-gray-200 p-4 bg-white">
                  <div className="flex items-center justify-between">
                    {/* 左侧：导航按钮 */}
                    <div className="flex gap-2">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={handlePrevLesson}
                        disabled={!prevLesson}
                      >
                        ← 上一课
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={handleNextLesson}
                        disabled={!nextLesson}
                      >
                        下一课 →
                      </Button>
                    </div>

                    {/* 中间：操作按钮 */}
                    <div className="flex gap-2">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={handleMarkComplete}
                        disabled={currentLesson.status === 'completed'}
                      >
                        {currentLesson.status === 'completed'
                          ? '✓ 已完成'
                          : '标记完成'}
                      </Button>
                    </div>

                    {/* 右侧：AI 按钮 */}
                    <div className="flex gap-2">
                      <Button variant="outline" size="sm">
                        生成练习题
                      </Button>
                      <Button variant="outline" size="sm">
                        向老师提问
                      </Button>
                    </div>
                  </div>
                </div>
              </>
            ) : (
              <div className="flex-1 flex items-center justify-center text-text-muted">
                <div className="text-center">
                  <div className="text-6xl mb-4">📖</div>
                  <p className="text-lg mb-2">选择一个课时开始学习</p>
                  <p className="text-sm">点击左侧课程大纲中的课时</p>
                </div>
              </div>
            )}
          </Card>
        </main>

        {/* 右侧边栏 - 教师助手 */}
        <aside className="w-80 bg-bg-secondary border-l border-gray-200 flex flex-col overflow-hidden">
          {currentLesson && currentCourse ? (
            <div className="flex-1 flex flex-col overflow-hidden">
              <div className="p-4 border-b border-gray-200 bg-white">
                <h3 className="font-bold text-primary">学习助手</h3>
                <p className="text-xs text-gray-500 mt-1 truncate">
                  {currentCourse.name} / {currentLesson.name}
                </p>
              </div>
              <div className="flex-1 overflow-hidden">
                <TeacherAgent
                  courseName={currentCourse.name}
                  chapterName={currentChapter?.name || ''}
                  lessonName={currentLesson.name}
                  teachingStyle={currentCourse.teachingStyle}
                />
              </div>
            </div>
          ) : (
            <div className="flex-1 flex flex-col items-center justify-center text-text-muted p-4">
              <div className="text-4xl mb-4">💬</div>
              <p className="text-center text-sm">
                选择一个课时后
                <br />
                可以向 AI 老师提问
              </p>
            </div>
          )}
        </aside>
      </div>
    </div>
  );
};
