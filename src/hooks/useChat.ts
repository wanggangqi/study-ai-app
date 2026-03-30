import { useChatStore } from '../stores/chatStore';

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
