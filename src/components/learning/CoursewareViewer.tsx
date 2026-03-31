import React from 'react';
import './CoursewareViewer.css';

interface CoursewareViewerProps {
  lessonId: string;
  content: string | null;
}

/**
 * 课件查看器组件
 * 用于渲染 AI 生成的 HTML 课件内容
 */
export const CoursewareViewer: React.FC<CoursewareViewerProps> = ({
  lessonId,
  content,
}) => {
  if (!content) {
    return (
      <div className="courseware-viewer-empty">
        <div className="courseware-viewer-empty-icon">📖</div>
        <p className="courseware-viewer-empty-title">正在加载课件...</p>
        <p className="courseware-viewer-empty-hint">课件内容将通过 AI 自动生成</p>
      </div>
    );
  }

  return (
    <div
      className="courseware-content"
      key={lessonId}
      dangerouslySetInnerHTML={{ __html: content }}
    />
  );
};
