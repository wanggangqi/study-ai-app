// AI 服务封装 - 完整实现在 Phase 4
export type AIProvider = 'claude' | 'openai' | 'qwen' | 'deepseek' | 'glm' | 'minimax' | 'kimi';

export interface AIMessage {
  role: 'user' | 'assistant';
  content: string;
}

export interface AnalyzeExerciseResult {
  score: number;
  feedback: string;
}

export const aiService = {
  async chat(_provider: AIProvider, _apiKey: string, model: string, messages: AIMessage[]): Promise<string> {
    // TODO: Phase 4 实现多服务商适配
    console.log('AI chat called', { model, messages });
    return 'AI response placeholder';
  },

  async generateLesson(_provider: AIProvider, _apiKey: string, _context: object): Promise<string> {
    // TODO: Phase 4 实现课件生成
    return '<html><body><h1>Placeholder Lesson</h1></body></html>';
  },

  async generateExercise(_provider: AIProvider, _apiKey: string, _context: object): Promise<string> {
    // TODO: Phase 4 实现练习题生成
    return '<html><body><h1>Placeholder Exercise</h1></body></html>';
  },

  async analyzeExercise(_provider: AIProvider, _apiKey: string, _exercise: object, _answers: object): Promise<AnalyzeExerciseResult> {
    // TODO: Phase 4 实现练习分析
    return { score: 100, feedback: 'Placeholder feedback' };
  },
};
