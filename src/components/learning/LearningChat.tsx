import React, { useState, useCallback, useEffect, useRef } from 'react';
import { Button } from '../common/Button';
import { chatWithAI } from '../../hooks/useAI';
import { cleanMessageContent } from '../../hooks/useChat';
import { useChatStore } from '../../stores/chatStore';
import './LearningChat.css';

interface LearningChatProps {
  courseId: string;
  lessonId: string;
  courseName: string;
  chapterName: string;
  lessonName: string;
  teachingStyle: string;
  initialLessonHtml?: string;
}

/**
 * 学习聊天容器组件
 * 包装 TeacherAgent，提供快捷问题按钮，专用于三栏布局的答疑区域
 */

// 快捷问题配置
const QUICK_QUESTIONS = [
  { id: 'exercise', label: '生成练习题', icon: '📝' },
  { id: 'explain', label: '解释这个概念', icon: '💡' },
  { id: 'example', label: '给我一个例子', icon: '📖' },
];

export const LearningChat: React.FC<LearningChatProps> = ({
  courseId,
  lessonId,
  courseName,
  chapterName,
  lessonName,
  teachingStyle,
  initialLessonHtml,
}) => {
  const [userInput, setUserInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // 使用 chatStore 持久化消息，按 courseId/lessonId 隔离
  const allMessages = useChatStore((state) => state.teacherMessages);
  const addTeacherMessage = useChatStore((state) => state.addTeacherMessage);

  // 按当前 courseId/lessonId 过滤消息
  const messages = allMessages.filter(
    (msg) => msg.courseId === courseId && msg.lessonId === lessonId
  );

  // 跟踪欢迎消息是否已添加（防止切换课时时重复添加）
  const welcomeAddedRef = useRef(false);

  // 自动滚动到最新消息
  const scrollToBottom = useCallback(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  useEffect(() => {
    scrollToBottom();
  }, [messages, scrollToBottom]);

  // 初始化欢迎消息
  useEffect(() => {
    // 重置欢迎消息标志（切换课时时）
    welcomeAddedRef.current = false;

    // 延迟检查，确保消息已加载
    const timer = setTimeout(() => {
      // 避免重复添加欢迎消息
      if (messages.length === 0 && !welcomeAddedRef.current) {
        welcomeAddedRef.current = true;
        addTeacherMessage({
          courseId,
          lessonId,
          agentType: 'teacher',
          role: 'assistant',
          content: `你好！我是你的学习助手。\n\n关于「${lessonName}」这个课时，有什么问题可以随时问我。我可以帮你：\n- 解答疑惑\n- 解释概念\n- 生成练习题\n- 提供更多例子`,
        });
      }
    }, 100);

    return () => clearTimeout(timer);
  }, [lessonName, courseId, lessonId, addTeacherMessage]);

  // 发送消息
  const handleSendMessage = useCallback(async () => {
    if (!userInput.trim()) return;

    const userMessageContent = userInput;
    setUserInput('');
    setIsLoading(true);
    setError(null);

    // 添加用户消息到 store
    addTeacherMessage({
      courseId,
      lessonId,
      agentType: 'teacher',
      role: 'user',
      content: userMessageContent,
    });

    const systemPrompt = `你是一位专业的教师，擅长以「${teachingStyle}」风格进行教学。
当前课程：${courseName}
当前章节：${chapterName}
当前课时：${lessonName}
${initialLessonHtml ? `课件内容：\n${initialLessonHtml}` : ''}

请根据课件内容和用户的问题，提供清晰、准确的回答。保持友好、耐心的教学态度。`;

    // 构建 AI 上下文消息（使用本地格式）
    const aiContextMessages = [
      { role: 'system' as const, content: systemPrompt },
      ...messages.map((m) => ({ role: m.role, content: m.content })),
      { role: 'user' as const, content: userMessageContent },
    ];

    try {
      const result = await chatWithAI(aiContextMessages);

      if (result.success && result.data) {
        // 添加助手消息到 store
        addTeacherMessage({
          courseId,
          lessonId,
          agentType: 'teacher',
          role: 'assistant',
          content: result.data,
        });
      } else {
        setError(result.error || 'AI 响应失败');
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  }, [userInput, messages, teachingStyle, courseName, chapterName, lessonName, initialLessonHtml, courseId, lessonId, addTeacherMessage]);

  // 快捷问题
  const handleQuickQuestion = useCallback(
    async (questionType: string) => {
      let question = '';

      switch (questionType) {
        case 'exercise':
          question = '请根据课件内容生成几道练习题，帮助我巩固所学知识。';
          break;
        case 'explain':
          question = '请详细解释一下这个课时中的核心概念。';
          break;
        case 'example':
          question = '请给我举一个实际生活中的例子，帮助我理解这个知识点。';
          break;
      }

      // 直接发送快捷问题
      setUserInput(question);

      // 延迟发送以显示用户输入
      setTimeout(async () => {
        setUserInput('');
        setIsLoading(true);
        setError(null);

        // 添加用户消息到 store
        addTeacherMessage({
          courseId,
          lessonId,
          agentType: 'teacher',
          role: 'user',
          content: question,
        });

        const systemPrompt = `你是一位专业的教师，擅长以「${teachingStyle}」风格进行教学。
当前课程：${courseName}
当前章节：${chapterName}
当前课时：${lessonName}
${initialLessonHtml ? `课件内容：\n${initialLessonHtml}` : ''}

请根据课件内容和用户的问题，提供清晰、准确的回答。保持友好、耐心的教学态度。`;

        // 构建 AI 上下文消息（使用本地格式）
        const aiContextMessages = [
          { role: 'system' as const, content: systemPrompt },
          ...messages.map((m) => ({ role: m.role, content: m.content })),
          { role: 'user' as const, content: question },
        ];

        try {
          const result = await chatWithAI(aiContextMessages);

          if (result.success && result.data) {
            // 添加助手消息到 store
            addTeacherMessage({
              courseId,
              lessonId,
              agentType: 'teacher',
              role: 'assistant',
              content: result.data,
            });
          } else {
            setError(result.error || 'AI 响应失败');
          }
        } catch (err) {
          setError(String(err));
        } finally {
          setIsLoading(false);
        }
      }, 100);
    },
    [messages, teachingStyle, courseName, chapterName, lessonName, initialLessonHtml, courseId, lessonId, addTeacherMessage]
  );

  // 键盘事件
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        handleSendMessage();
      }
    },
    [handleSendMessage]
  );

  return (
    <div className="learning-chat">
      {/* 快捷问题按钮 */}
      <div className="quick-questions">
        {QUICK_QUESTIONS.map((q) => (
          <button
            key={q.id}
            className="quick-question-btn"
            onClick={() => handleQuickQuestion(q.id)}
            disabled={isLoading}
          >
            <span className="quick-question-icon">{q.icon}</span>
            <span className="quick-question-label">{q.label}</span>
          </button>
        ))}
      </div>

      {/* 聊天消息区域 */}
      <div className="chat-messages">
        {messages.map((msg, idx) => (
          <div
            key={idx}
            className={`chat-message ${msg.role === 'user' ? 'user' : 'assistant'}`}
          >
            <div className="message-content">{cleanMessageContent(msg.content)}</div>
          </div>
        ))}

        {isLoading && (
          <div className="chat-message assistant">
            <div className="message-content loading">
              <div className="spinner small" />
              <span>思考中...</span>
            </div>
          </div>
        )}

        {error && <div className="chat-error">{error}</div>}

        <div ref={messagesEndRef} />
      </div>

      {/* 输入区域 */}
      <div className="chat-input-area">
        <textarea
          className="chat-input"
          placeholder="输入您的问题..."
          value={userInput}
          onChange={(e) => setUserInput(e.target.value)}
          onKeyDown={handleKeyDown}
          rows={2}
        />
        <Button
          variant="primary"
          size="sm"
          onClick={handleSendMessage}
          disabled={isLoading || !userInput.trim()}
        >
          发送
        </Button>
      </div>

    </div>
  );
};
