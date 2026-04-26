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

// 练习题选项
interface ExerciseOption {
  id: string;
  label: string;
  content: string;
}

// 结构化练习题
interface StructuredExercise {
  id: string;
  lesson_id: string;
  question: string;
  options: ExerciseOption[];
  correct_answer: string;
  explanation?: string;
}

// 结构化练习题结果
interface AIStructuredExerciseResult {
  success: boolean;
  data?: StructuredExercise[];
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
  model?: string,
  baseUrl?: string
): Promise<AIResult> {
  const config = useConfigStore.getState();

  // 优先使用传入的 model，其次使用配置中的 aiModel（只有在非空时才使用）
  const effectiveModel = model || (config.aiModel ? config.aiModel : null);
  const effectiveBaseUrl = baseUrl || (config.customBaseUrl ? config.customBaseUrl : null);

  return invoke<AIResult>('ai_chat_command', {
    params: {
      provider: provider || config.aiProvider,
      api_key: apiKey || config.aiApiKey,
      model: effectiveModel,
      base_url: effectiveBaseUrl,
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
  model?: string,
  baseUrl?: string
): Promise<AIResult> {
  const config = useConfigStore.getState();

  const effectiveModel = model || (config.aiModel ? config.aiModel : null);
  const effectiveBaseUrl = baseUrl || (config.customBaseUrl ? config.customBaseUrl : null);

  return invoke<AIResult>('ai_generate_lesson_command', {
    params: {
      provider: provider || config.aiProvider,
      api_key: apiKey || config.aiApiKey,
      model: effectiveModel,
      base_url: effectiveBaseUrl,
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
  model?: string,
  baseUrl?: string
): Promise<AIResult> {
  const config = useConfigStore.getState();

  const effectiveModel = model || (config.aiModel ? config.aiModel : null);
  const effectiveBaseUrl = baseUrl || (config.customBaseUrl ? config.customBaseUrl : null);

  return invoke<AIResult>('ai_generate_exercise_command', {
    params: {
      provider: provider || config.aiProvider,
      api_key: apiKey || config.aiApiKey,
      model: effectiveModel,
      base_url: effectiveBaseUrl,
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
  model?: string,
  baseUrl?: string
): Promise<AIAnalyzeResult> {
  const config = useConfigStore.getState();

  const effectiveModel = model || (config.aiModel ? config.aiModel : null);
  const effectiveBaseUrl = baseUrl || (config.customBaseUrl ? config.customBaseUrl : null);

  return invoke<AIAnalyzeResult>('ai_analyze_answers_command', {
    params: {
      provider: provider || config.aiProvider,
      api_key: apiKey || config.aiApiKey,
      model: effectiveModel,
      base_url: effectiveBaseUrl,
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
  apiKey: string,
  baseUrl?: string
): Promise<AIResult> {
  const config = useConfigStore.getState();

  const effectiveBaseUrl = baseUrl || (provider === 'custom' && config.customBaseUrl ? config.customBaseUrl : null);

  return invoke<AIResult>('ai_verify_key_command', {
    params: {
      provider,
      api_key: apiKey,
      base_url: effectiveBaseUrl,
    },
  });
}

/**
 * 生成结构化练习题
 */
export async function generateStructuredExercise(
  lessonId: string,
  lessonContent: string,
  provider?: AIProvider,
  apiKey?: string,
  model?: string,
  baseUrl?: string
): Promise<AIStructuredExerciseResult> {
  const config = useConfigStore.getState();

  const effectiveModel = model || (config.aiModel ? config.aiModel : null);
  const effectiveBaseUrl = baseUrl || (config.customBaseUrl ? config.customBaseUrl : null);

  return invoke<AIStructuredExerciseResult>('ai_generate_structured_exercise_command', {
    params: {
      provider: provider || config.aiProvider,
      api_key: apiKey || config.aiApiKey,
      model: effectiveModel,
      base_url: effectiveBaseUrl,
      lesson_id: lessonId,
      lesson_content: lessonContent,
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
