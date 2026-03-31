import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useConfigStore } from '../stores/configStore';
import { AIProvider } from '../types';

interface AIResult {
  success: boolean;
  data?: string;
  error?: string;
}

interface AIAnalyzeResult {
  success: boolean;
  score?: number;
  feedback?: string;
  weak_points?: string[];
  error?: string;
}

interface ChatMessage {
  role: 'system' | 'user' | 'assistant';
  content: string;
}

/**
 * 发送聊天消息到 AI
 */
export async function chatWithAI(
  messages: ChatMessage[],
  provider?: AIProvider,
  apiKey?: string,
  model?: string
): Promise<AIResult> {
  const config = useConfigStore.getState();

  return invoke<AIResult>('ai_chat_command', {
    params: {
      provider: provider || config.aiProvider,
      api_key: apiKey || config.aiApiKey,
      model: model || config.aiModel || null,
      messages,
    },
  });
}

/**
 * 生成 HTML 课件
 */
export async function generateLessonHTML(
  courseName: string,
  chapterName: string,
  lessonName: string,
  teachingStyle: string,
  provider?: AIProvider,
  apiKey?: string,
  model?: string
): Promise<AIResult> {
  const config = useConfigStore.getState();

  return invoke<AIResult>('ai_generate_lesson_command', {
    params: {
      provider: provider || config.aiProvider,
      api_key: apiKey || config.aiApiKey,
      model: model || config.aiModel || null,
      course_name: courseName,
      chapter_name: chapterName,
      lesson_name: lessonName,
      teaching_style: teachingStyle,
    },
  });
}

/**
 * 生成练习题 HTML
 */
export async function generateExerciseHTML(
  lessonContent: string,
  provider?: AIProvider,
  apiKey?: string,
  model?: string
): Promise<AIResult> {
  const config = useConfigStore.getState();

  return invoke<AIResult>('ai_generate_exercise_command', {
    params: {
      provider: provider || config.aiProvider,
      api_key: apiKey || config.aiApiKey,
      model: model || config.aiModel || null,
      lesson_content: lessonContent,
    },
  });
}

/**
 * 分析用户答案
 */
export async function analyzeUserAnswers(
  exerciseContent: string,
  userAnswers: string,
  provider?: AIProvider,
  apiKey?: string,
  model?: string
): Promise<AIAnalyzeResult> {
  const config = useConfigStore.getState();

  return invoke<AIAnalyzeResult>('ai_analyze_answers_command', {
    params: {
      provider: provider || config.aiProvider,
      api_key: apiKey || config.aiApiKey,
      model: model || config.aiModel || null,
      exercise_content: exerciseContent,
      user_answers: userAnswers,
    },
  });
}

/**
 * 验证 API 密钥
 */
export async function verifyAPIKey(
  provider: AIProvider,
  apiKey: string
): Promise<AIResult> {
  return invoke<AIResult>('ai_verify_key_command', {
    params: {
      provider,
      api_key: apiKey,
    },
  });
}

/**
 * 通用 AI 聊天 Hook
 */
export function useAIChat() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const sendMessage = async (messages: ChatMessage[]) => {
    setIsLoading(true);
    setError(null);

    try {
      const result = await chatWithAI(messages);
      if (result.success && result.data) {
        return result.data;
      } else {
        setError(result.error || 'AI 响应失败');
        return null;
      }
    } catch (err) {
      setError(String(err));
      return null;
    } finally {
      setIsLoading(false);
    }
  };

  return { sendMessage, isLoading, error };
}
