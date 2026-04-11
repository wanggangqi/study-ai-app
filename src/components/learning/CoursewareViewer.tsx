import React, { useEffect, useMemo, useState } from 'react';
import DOMPurify from 'dompurify';
import { invoke } from '@tauri-apps/api/core';
import { useCourseStore } from '../../stores/courseStore';
import { generateLessonHTML } from '../../hooks/useAI';
import './CoursewareViewer.css';

interface CoursewareViewerProps {
  lessonId: string;
  content: string | null;
}

/**
 * 课件查看器组件
 * 用于渲染 AI 生成的 HTML 课件内容
 * 如果没有课件内容，自动调用 AI 生成
 * 使用 DOMPurify 消毒 HTML 内容，防止 XSS 攻击
 */
export const CoursewareViewer: React.FC<CoursewareViewerProps> = ({
  lessonId,
  content,
}) => {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const { currentCourse, currentChapter, currentLesson, setLessonContent } = useCourseStore();

  useEffect(() => {
    if (!content && currentCourse && currentChapter && currentLesson) {
      loadLessonContent();
    }
  }, [lessonId]);

  const loadLessonContent = async () => {
    if (!currentCourse || !currentChapter || !currentLesson) {
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const result = await generateLessonHTML(
        currentCourse.name,
        currentChapter.name,
        currentLesson.name,
        currentCourse.teachingStyle || '实战应用型'
      );

      if (result.success && result.data) {
        setLessonContent(lessonId, result.data);

        // 将课件内容保存到文件系统
        try {
          await invoke<string>('save_lesson_file_command', {
            courseId: currentCourse.id,
            lessonId: lessonId,
            content: result.data,
          });
        } catch (saveError) {
          console.error('保存课件文件失败:', saveError);
          // 文件保存失败不影响课件显示，仅记录错误
        }
      } else {
        setError(result.error || '加载课件失败');
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  };

  // 使用 DOMPurify 消毒 HTML 内容
  const sanitizedContent = useMemo(() => {
    if (!content) return null;
    return DOMPurify.sanitize(content, {
      ALLOWED_TAGS: [
        'h1', 'h2', 'h3', 'h4', 'h5', 'h6',
        'p', 'br', 'hr',
        'ul', 'ol', 'li',
        'blockquote', 'pre', 'code',
        'strong', 'em', 'b', 'i', 'u', 's',
        'a', 'img',
        'table', 'thead', 'tbody', 'tr', 'th', 'td',
        'div', 'span',
        'details', 'summary',
      ],
      ALLOWED_ATTR: ['href', 'src', 'alt', 'title', 'class', 'id', 'target', 'rel'],
    });
  }, [content]);

  if (isLoading) {
    return (
      <div className="courseware-viewer-loading">
        <div className="courseware-loading-spinner" />
        <p className="courseware-viewer-empty-title">正在生成课件...</p>
        <p className="courseware-viewer-empty-hint">AI 正在为您创作个性化的学习内容，请稍候</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="courseware-viewer-error">
        <div className="courseware-viewer-empty-icon">⚠️</div>
        <p className="courseware-viewer-empty-title">加载失败</p>
        <p className="courseware-viewer-empty-hint">{error}</p>
        <button onClick={loadLessonContent} className="courseware-retry-btn">重试</button>
      </div>
    );
  }

  if (!sanitizedContent) {
    return (
      <div className="courseware-viewer-empty">
        <div className="courseware-viewer-empty-icon">📖</div>
        <p className="courseware-viewer-empty-title">暂无课件内容</p>
        <p className="courseware-viewer-empty-hint">请先选择课程和课时</p>
        <button onClick={() => loadLessonContent()} className="courseware-retry-btn">手动加载课件</button>
      </div>
    );
  }

  return (
    <div
      className="courseware-content"
      key={lessonId}
      dangerouslySetInnerHTML={{ __html: sanitizedContent }}
    />
  );
};
