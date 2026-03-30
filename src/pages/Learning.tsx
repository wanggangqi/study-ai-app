import React from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Card } from '../components/common/Card';
import { Button } from '../components/common/Button';
import { Sidebar } from '../components/common/Sidebar';
import { useCourseStore } from '../stores/courseStore';

const navItems = [
  { icon: '📚', label: '课程', path: '/' },
  { icon: '💬', label: '咨询师', path: '/consultant' },
  { icon: '⚙️', label: '设置', path: '/settings' },
];

export const LearningPage: React.FC = () => {
  const { courseId } = useParams<{ courseId: string }>();
  const navigate = useNavigate();
  const { currentCourse, selectCourse } = useCourseStore();

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
    <div className="flex min-h-screen">
      {/* 左侧导航栏 - 课程大纲 */}
      <div className="w-64 bg-bg-secondary min-h-screen p-4">
        <h2 className="font-bold text-primary mb-4 px-2">{currentCourse.name}</h2>
        <div className="space-y-2">
          {currentCourse.chapters?.map((chapter, chapterIndex) => (
            <div key={chapter.id}>
              <div className="px-2 py-2 text-sm font-medium text-text-secondary">
                第{chapterIndex + 1}章 {chapter.name}
              </div>
              <div className="space-y-1">
                {chapter.lessons?.map((lesson) => (
                  <button
                    key={lesson.id}
                    className={`w-full text-left px-4 py-2 text-sm rounded-md transition-colors ${
                      lesson.status === 'completed'
                        ? 'text-green-600 bg-green-50'
                        : lesson.status === 'in_progress'
                        ? 'text-primary bg-primary/10'
                        : 'text-text-muted hover:bg-white/50'
                    }`}
                  >
                    <span className="mr-2">
                      {lesson.status === 'completed' ? '✓' :
                       lesson.status === 'in_progress' ? '▶' : '○'}
                    </span>
                    {lesson.name}
                  </button>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* 中间内容区 - 课件展示 */}
      <div className="flex-1 p-8">
        <Card className="h-full min-h-[600px] flex flex-col">
          <div className="flex-1 flex items-center justify-center text-text-muted">
            <div className="text-center">
              <div className="text-6xl mb-4">📖</div>
              <p>选择一个课时开始学习</p>
            </div>
          </div>
        </Card>
      </div>

      {/* 右侧边栏 - 聊天答疑 */}
      <div className="w-80 bg-bg-secondary min-h-screen p-4 flex flex-col">
        <h3 className="font-bold text-primary mb-4">学习助手</h3>
        <div className="flex-1 bg-white rounded-lg p-4 overflow-y-auto">
          <p className="text-text-muted text-sm text-center">
            这里是 AI 答疑区域。你可以随时向助手提问。
          </p>
        </div>
        <div className="mt-4 flex gap-2">
          <input
            type="text"
            placeholder="输入你的问题..."
            className="flex-1 border border-gray-300 rounded-md px-3 py-2 text-sm"
          />
          <Button size="sm">发送</Button>
        </div>
      </div>
    </div>
  );
};
