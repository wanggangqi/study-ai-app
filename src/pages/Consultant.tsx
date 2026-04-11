import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Sidebar } from '../components/common/Sidebar';
import { Card } from '../components/common/Card';
import { Button } from '../components/common/Button';
import { message } from '@tauri-apps/plugin-dialog';
import { ConsultantAgent } from '../components/consultant';
import { CoursePlanOutline } from '../types';
import { tauriService } from '../services/tauri';

const navItems = [
  { icon: '📚', label: '课程', path: '/' },
  { icon: '💬', label: '咨询师', path: '/consultant' },
  { icon: '⚙️', label: '设置', path: '/settings' },
];

export const ConsultantPage: React.FC = () => {
  const navigate = useNavigate();
  const [coursePlan, setCoursePlan] = useState<CoursePlanOutline | null>(null);

  const handleCoursePlanGenerated = (plan: CoursePlanOutline) => {
    setCoursePlan(plan);
  };

  const handleStartLearning = async () => {
    if (!coursePlan) return;

    try {
      // 保存课程到数据库
      const courseData = {
        name: coursePlan.courseName,
        targetLevel: coursePlan.targetLevel,
        duration: coursePlan.duration,
        teachingStyle: coursePlan.teachingStyle,
      };

      // 使用 invoke 直接调用后端命令创建课程
      const { invoke } = await import('@tauri-apps/api/core');
      const result = await invoke<{ id: string }>('create_course_command', {
        name: courseData.name,
        targetLevel: courseData.targetLevel,
        duration: courseData.duration,
        teachingStyle: courseData.teachingStyle,
      });

      if (result?.id) {
        // 创建章节和课时
        try {
          await tauriService.createChaptersWithLessons({
            courseId: result.id,
            chapters: coursePlan.chapters.map((ch) => ({
              chapterIndex: ch.chapterIndex,
              chapterName: ch.chapterName,
              lessons: ch.lessons.map((ls) => ({
                lessonIndex: ls.lessonIndex,
                lessonName: ls.lessonName,
                duration: ls.duration,
              })),
            })),
          });
        } catch (chaptersError) {
          console.error('创建章节和课时失败:', chaptersError);
          await message(`课程"${coursePlan.courseName}"创建成功，但创建章节和课时失败。`, {
            title: '警告',
            kind: 'warning',
          });
        }

        // 创建码云仓库
        try {
          const syncResult = await tauriService.createCourseRepository(result.id);
          if (syncResult.success) {
            await message(`课程"${coursePlan.courseName}"创建成功！\n仓库地址：${syncResult.repoUrl}`, {
              title: '课程创建成功',
              kind: 'info',
            });
          } else {
            console.warn('码云同步失败:', syncResult.message);
            await message(`课程"${coursePlan.courseName}"创建成功，但码云同步失败。\n您可以稍后在设置中手动同步。`, {
              title: '课程创建成功',
              kind: 'warning',
            });
          }
        } catch (syncError) {
          console.error('码云同步失败:', syncError);
          await message(`课程"${coursePlan.courseName}"创建成功，但码云同步失败。\n您可以稍后在设置中手动同步。`, {
            title: '课程创建成功',
            kind: 'warning',
          });
        }

        // 导航到首页
        navigate('/');
      }
    } catch (error) {
      console.error('创建课程失败:', error);
      await message(`创建课程失败：${error}`, {
        title: '错误',
        kind: 'error',
      });
    }
  };

  // 如果有课程计划，显示课程计划确认界面
  if (coursePlan) {
    return (
      <div className="h-screen overflow-hidden">
        <Sidebar items={navItems} activePath="/consultant" onNavigate={(path) => navigate(path)} />

        <main className="ml-48 h-full overflow-y-auto p-8">
          <h1 className="text-2xl font-bold text-[#588157] mb-8">课程计划已生成</h1>

          <Card className="max-w-2xl mx-auto">
            <div className="text-center mb-6">
              <div className="text-4xl mb-4">✅</div>
              <h2 className="text-xl font-bold mb-2">恭喜！您的学习计划已准备好</h2>
            </div>

            <div className="space-y-4 mb-6">
              <div className="flex justify-between items-center py-2 border-b">
                <span className="text-[#666666]">课程名称</span>
                <span className="font-medium">{coursePlan.courseName}</span>
              </div>
              <div className="flex justify-between items-center py-2 border-b">
                <span className="text-[#666666]">目标水平</span>
                <span className="font-medium">{coursePlan.targetLevel}</span>
              </div>
              <div className="flex justify-between items-center py-2 border-b">
                <span className="text-[#666666]">学习时长</span>
                <span className="font-medium">{coursePlan.duration}</span>
              </div>
              <div className="flex justify-between items-center py-2 border-b">
                <span className="text-[#666666]">教学风格</span>
                <span className="font-medium">{coursePlan.teachingStyle}</span>
              </div>
            </div>

            <div className="flex gap-4">
              <Button variant="secondary" className="flex-1" onClick={() => setCoursePlan(null)}>
                重新制定
              </Button>
              <Button className="flex-1" onClick={handleStartLearning}>
                开始学习
              </Button>
            </div>
          </Card>
        </main>
      </div>
    );
  }

  return (
    <div className="h-screen overflow-hidden">
      <Sidebar items={navItems} activePath="/consultant" onNavigate={(path) => navigate(path)} />

      <main className="ml-48 h-full overflow-y-auto p-8">
        <h1 className="text-2xl font-bold text-[#588157] mb-8">学习咨询师</h1>

        <Card className="max-w-2xl mx-auto">
          <ConsultantAgent onCoursePlanGenerated={handleCoursePlanGenerated} />
        </Card>
      </main>
    </div>
  );
};
