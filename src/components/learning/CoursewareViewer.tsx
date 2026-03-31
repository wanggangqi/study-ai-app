import React, { useMemo } from 'react';
import DOMPurify from 'dompurify';
import './CoursewareViewer.css';

interface CoursewareViewerProps {
  lessonId: string;
  content: string | null;
}

/**
 * 课件查看器组件
 * 用于渲染 AI 生成的 HTML 课件内容
 * 使用 DOMPurify 消毒 HTML 内容，防止 XSS 攻击
 */
export const CoursewareViewer: React.FC<CoursewareViewerProps> = ({
  lessonId,
  content,
}) => {
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

  if (!sanitizedContent) {
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
      dangerouslySetInnerHTML={{ __html: sanitizedContent }}
    />
  );
};
