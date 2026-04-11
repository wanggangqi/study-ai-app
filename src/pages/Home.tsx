import React, { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Sidebar } from '../components/common/Sidebar';
import { Card } from '../components/common/Card';
import { Button } from '../components/common/Button';
import { useCourseStore } from '../stores/courseStore';

const navItems = [
  { icon: '📚', label: '课程', path: '/' },
  { icon: '💬', label: '咨询师', path: '/consultant' },
  { icon: '⚙️', label: '设置', path: '/settings' },
];

export const HomePage: React.FC = () => {
  const navigate = useNavigate();
  const { courses, loadCourses } = useCourseStore();

  useEffect(() => {
    loadCourses();
  }, [loadCourses]);

  return (
    <div className="h-screen overflow-hidden">
      <Sidebar items={navItems} activePath="/" onNavigate={(path) => navigate(path)} />

      <main className="ml-48 h-full overflow-y-auto p-8">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-2xl font-bold text-[#588157]">我的课程</h1>
          <Button onClick={() => navigate('/consultant')}>+ 创建新课程</Button>
        </div>

        {courses.length === 0 ? (
          <Card className="text-center py-12">
            <p className="text-[#999999] mb-4">还没有任何课程</p>
            <Button onClick={() => navigate('/consultant')}>前往咨询师创建课程</Button>
          </Card>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {courses.map((course) => (
              <Card key={course.id} className="hover:shadow-lg transition-shadow">
                <h3 className="font-bold text-lg text-[#588157] mb-2">{course.name}</h3>
                <p className="text-sm text-[#666666] mb-2">目标：{course.targetLevel}</p>
                <p className="text-xs text-[#999999] mb-4">时长：{course.duration}</p>
                <div className="mb-4">
                  <div className="h-2 bg-[#f5ebe0] rounded-full overflow-hidden">
                    <div
                      className="h-full bg-[#588157] rounded-full transition-all duration-300"
                      style={{ width: `${course.progress || 0}%` }}
                    />
                  </div>
                  <span className="text-xs text-[#588157] mt-1">进度 {course.progress || 0}%</span>
                </div>
                <Button
                  className="w-full"
                  onClick={() => navigate(`/learning/${course.id}`)}
                >
                  {(course.progress || 0) > 0 ? '继续学习' : '开始学习'}
                </Button>
              </Card>
            ))}
          </div>
        )}
      </main>
    </div>
  );
};
