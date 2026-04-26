import { useChatStore } from '../stores/chatStore';

/**
 * 清理消息内容中的思考过程标记
 * 去除类似 "这种(...)的" 这样的括号内容
 */
export function cleanMessageContent(content: string): string {
  // 去除类似 "这种(...)的" 或 "(...)" 格式的思考过程标记
  return content
    .replace(/[（(][^）)]*[）)]/g, '')
    .trim();
}

export const useChat = () => {
  const {
    consultantMessages,
    teacherMessages,
    addConsultantMessage,
    addTeacherMessage,
    clearConsultantMessages,
    clearTeacherMessages,
    setTeacherMessages,
  } = useChatStore();

  return {
    consultantMessages,
    teacherMessages,
    addConsultantMessage,
    addTeacherMessage,
    clearConsultantMessages,
    clearTeacherMessages,
    setTeacherMessages,
  };
};
