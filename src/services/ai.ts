// AI 服务封装 - 完整实现在 Phase 4
export type AIProvider = 'claude' | 'openai' | 'qwen' | 'deepseek' | 'glm' | 'minimax' | 'kimi';

interface AIMessage {
  role: 'user' | 'assistant';
  content: string;
}

export const aiService = {
  async chat(provider: AIProvider, apiKey: string, model: string, messages: AIMessage[]): Promise<string> {
    // TODO: Phase 4 实现多服务商适配
    console.log('AI chat called', { provider, model, messages });
    return 'AI response placeholder';
  },

  async generateLesson(provider: AIProvider, apiKey: string, context: any): Promise<string> {
    // TODO: Phase 4 实现课件生成
    return '<html><body><h1>Placeholder Lesson</h1></body></html>';
  },

  async generateExercise(provider: AIProvider, apiKey: string, context: any): Promise<string> {
    // TODO: Phase 4 实现练习题生成
    return '<html><body><h1>Placeholder Exercise</h1></body></html>';
  },

  async analyzeExercise(provider: AIProvider, apiKey: string, exercise: any, answers: any): Promise<any> {
    // TODO: Phase 4 实现练习分析
    return { score: 100, feedback: 'Placeholder feedback' };
  },
};
