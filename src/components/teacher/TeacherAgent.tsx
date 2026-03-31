import { useState, useCallback, useEffect } from 'react';
import { generateLessonHTML, generateExerciseHTML, analyzeUserAnswers, chatWithAI } from '../../hooks/useAI';
import './TeacherAgent.css';

interface TeacherAgentProps {
  courseName: string;
  chapterName: string;
  lessonName: string;
  teachingStyle: string;
  initialLessonHtml?: string;
  onAnswersAnalyzed?: (score: number, feedback: string, weakPoints: string[]) => void;
}

// 内部聊天消息类型（不含数据库字段）
interface InternalChatMessage {
  role: 'system' | 'user' | 'assistant';
  content: string;
}

export function TeacherAgent({
  courseName,
  chapterName,
  lessonName,
  teachingStyle,
  initialLessonHtml,
  onAnswersAnalyzed,
}: TeacherAgentProps) {
  const [lessonHtml, setLessonHtml] = useState<string>(initialLessonHtml || '');
  const [exerciseHtml, setExerciseHtml] = useState<string>('');
  const [messages, setMessages] = useState<InternalChatMessage[]>([]);
  const [userInput, setUserInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [activeTab, setActiveTab] = useState<'lesson' | 'exercise' | 'chat'>('lesson');
  const [error, setError] = useState<string | null>(null);

  // 加载课件
  const loadLesson = useCallback(async () => {
    if (lessonHtml) return;

    setIsLoading(true);
    setError(null);

    try {
      const result = await generateLessonHTML(
        courseName,
        chapterName,
        lessonName,
        teachingStyle
      );

      if (result.success && result.data) {
        setLessonHtml(result.data);
      } else {
        setError(result.error || '加载课件失败');
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  }, [courseName, chapterName, lessonName, teachingStyle, lessonHtml]);

  useEffect(() => {
    loadLesson();
  }, [loadLesson]);

  // 生成练习题
  const handleGenerateExercise = async () => {
    if (!lessonHtml) return;

    setIsLoading(true);
    setError(null);

    try {
      const result = await generateExerciseHTML(lessonHtml);

      if (result.success && result.data) {
        setExerciseHtml(result.data);
        setActiveTab('exercise');
      } else {
        setError(result.error || '生成练习题失败');
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  };

  // 提交练习题答案
  const handleSubmitExercise = async () => {
    if (!exerciseHtml) return;

    // 收集用户答案（从 DOM 中获取表单数据）
    const inputs = document.querySelectorAll('input, textarea');

    const userAnswers = Array.from(inputs)
      .filter((input) => input.id)
      .map((input) => `${input.id}: ${(input as HTMLInputElement).value}`)
      .join('\n');

    setIsLoading(true);
    setError(null);

    try {
      const result = await analyzeUserAnswers(exerciseHtml, userAnswers);

      if (result.success) {
        onAnswersAnalyzed?.(
          result.score || 0,
          result.feedback || '',
          result.weak_points || []
        );
        setActiveTab('chat');
      } else {
        setError(result.error || '分析答案失败');
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  };

  // 发送聊天消息
  const handleSendMessage = async () => {
    if (!userInput.trim()) return;

    const userMessage: InternalChatMessage = {
      role: 'user',
      content: userInput,
    };

    setMessages((prev) => [...prev, userMessage]);
    setUserInput('');

    const systemPrompt = `你是一位专业的教师，擅长以「${teachingStyle}」风格进行教学。
请根据课件内容和用户的问题，提供清晰、准确的回答。
如果用户在做练习题，请适时提供帮助和提示。
保持友好、耐心的教学态度。`;

    const contextMessages: InternalChatMessage[] = [
      { role: 'system', content: systemPrompt },
      ...messages,
      userMessage,
    ];

    setIsLoading(true);
    setError(null);

    try {
      const result = await chatWithAI(contextMessages);

      if (result.success && result.data) {
        const assistantMessage: InternalChatMessage = {
          role: 'assistant',
          content: result.data,
        };
        setMessages((prev) => [...prev, assistantMessage]);
      } else {
        setError(result.error || 'AI 响应失败');
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  return (
    <div className="teacher-agent">
      <div className="teacher-tabs">
        <button
          className={`tab-btn ${activeTab === 'lesson' ? 'active' : ''}`}
          onClick={() => setActiveTab('lesson')}
        >
          课件
        </button>
        <button
          className={`tab-btn ${activeTab === 'exercise' ? 'active' : ''}`}
          onClick={() => setActiveTab('exercise')}
        >
          练习
        </button>
        <button
          className={`tab-btn ${activeTab === 'chat' ? 'active' : ''}`}
          onClick={() => setActiveTab('chat')}
        >
          答疑
        </button>
      </div>

      <div className="teacher-content">
        {error && <div className="error-banner">{error}</div>}

        {activeTab === 'lesson' && (
          <div className="lesson-panel">
            {isLoading && !lessonHtml ? (
              <div className="loading-state">
                <div className="spinner" />
                <span>正在生成课件...</span>
              </div>
            ) : lessonHtml ? (
              <>
                <div
                  className="lesson-html"
                  dangerouslySetInnerHTML={{ __html: lessonHtml }}
                />
                <div className="lesson-actions">
                  <button
                    className="btn-primary"
                    onClick={handleGenerateExercise}
                    disabled={isLoading}
                  >
                    生成练习题
                  </button>
                </div>
              </>
            ) : (
              <div className="empty-state">暂无课件内容</div>
            )}
          </div>
        )}

        {activeTab === 'exercise' && (
          <div className="exercise-panel">
            {exerciseHtml ? (
              <>
                <div
                  className="exercise-html"
                  dangerouslySetInnerHTML={{ __html: exerciseHtml }}
                />
                <div className="exercise-actions">
                  <button
                    className="btn-primary"
                    onClick={handleSubmitExercise}
                    disabled={isLoading}
                  >
                    提交答案
                  </button>
                  <button
                    className="btn-secondary"
                    onClick={handleGenerateExercise}
                    disabled={isLoading}
                  >
                    重新生成
                  </button>
                </div>
              </>
            ) : (
              <div className="empty-state">
                <p>点击下方按钮生成练习题</p>
                <button
                  className="btn-primary"
                  onClick={handleGenerateExercise}
                  disabled={isLoading || !lessonHtml}
                >
                  生成练习题
                </button>
              </div>
            )}
          </div>
        )}

        {activeTab === 'chat' && (
          <div className="chat-panel">
            <div className="chat-messages">
              {messages.length === 0 ? (
                <div className="chat-empty">
                  <p>有问题随时问我！</p>
                </div>
              ) : (
                messages.map((msg, idx) => (
                  <div
                    key={idx}
                    className={`chat-message ${msg.role === 'user' ? 'user' : 'assistant'}`}
                  >
                    <div className="message-content">{msg.content}</div>
                  </div>
                ))
              )}
              {isLoading && (
                <div className="chat-message assistant">
                  <div className="message-content loading">
                    <div className="spinner small" />
                    <span>思考中...</span>
                  </div>
                </div>
              )}
            </div>

            <div className="chat-input-area">
              <textarea
                className="chat-input"
                placeholder="输入您的问题..."
                value={userInput}
                onChange={(e) => setUserInput(e.target.value)}
                onKeyDown={handleKeyDown}
                rows={2}
              />
              <button
                className="send-btn"
                onClick={handleSendMessage}
                disabled={isLoading || !userInput.trim()}
              >
                发送
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
