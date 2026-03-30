import { create } from 'zustand';

export interface ChatMessage {
  id: string;
  courseId: string;
  lessonId?: string;
  agentType: 'consultant' | 'teacher';
  role: 'user' | 'assistant';
  content: string;
  createdAt: string;
}

interface ChatStore {
  consultantMessages: ChatMessage[];
  teacherMessages: ChatMessage[];

  addConsultantMessage: (message: Omit<ChatMessage, 'id' | 'createdAt'>) => void;
  addTeacherMessage: (message: Omit<ChatMessage, 'id' | 'createdAt'>) => void;
  clearConsultantMessages: () => void;
  clearTeacherMessages: () => void;
  setTeacherMessages: (messages: ChatMessage[]) => void;
}

export const useChatStore = create<ChatStore>((set) => ({
  consultantMessages: [],
  teacherMessages: [],

  addConsultantMessage: (message) =>
    set((state) => ({
      consultantMessages: [
        ...state.consultantMessages,
        {
          ...message,
          id: crypto.randomUUID(),
          createdAt: new Date().toISOString(),
        },
      ],
    })),

  addTeacherMessage: (message) =>
    set((state) => ({
      teacherMessages: [
        ...state.teacherMessages,
        {
          ...message,
          id: crypto.randomUUID(),
          createdAt: new Date().toISOString(),
        },
      ],
    })),

  clearConsultantMessages: () => set({ consultantMessages: [] }),
  clearTeacherMessages: () => set({ teacherMessages: [] }),
  setTeacherMessages: (messages) => set({ teacherMessages: messages }),
}));
