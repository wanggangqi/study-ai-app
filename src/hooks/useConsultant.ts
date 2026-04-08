import { useState, useCallback } from 'react';
import { useConfigStore } from '../stores/configStore';
import { tauriService } from '../services/tauri';
import type { CoursePlanOutline } from '../types';

interface UseConsultantReturn {
  isLoading: boolean;
  error: string | null;
  generateCoursePlan: (answers: Record<string, string>) => Promise<CoursePlanOutline | null>;
}

export function useConsultant(): UseConsultantReturn {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const config = useConfigStore();

  const generateCoursePlan = useCallback(async (answers: Record<string, string>): Promise<CoursePlanOutline | null> => {
    setIsLoading(true);
    setError(null);

    try {
      const result = await tauriService.generateCoursePlan({
        provider: config.aiProvider,
        apiKey: config.aiApiKey,
        model: config.aiModel || undefined,
        courseName: answers.goal || '',
        targetLevel: answers.level || '',
        duration: answers.duration || '',
        teachingStyle: answers.style || '',
        baseKnowledge: answers.base || '无',
      });

      return result;
    } catch (err) {
      setError(String(err));
      return null;
    } finally {
      setIsLoading(false);
    }
  }, [config.aiProvider, config.aiApiKey, config.aiModel]);

  return { isLoading, error, generateCoursePlan };
}
