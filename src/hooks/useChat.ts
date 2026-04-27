import { useChatStore } from '../stores/chatStore';

/**
 * 将 Markdown 格式的代码块转换为 HTML 格式
 * 处理多行代码块（```code```）和行内代码（`code`）
 */
function convertMarkdownCodeToHtml(content: string): string {
  // 处理带语言标记的多行代码块：```language\ncode\n```
  content = content.replace(/```(\w+)?\n([\s\S]*?)```/g, (_match, _lang, code) => {
    // 对代码内容进行 HTML 实体编码，防止 XSS
    const encodedCode = code
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
    return `<pre style="background:#f5f5f5;padding:15px;border-radius:8px;overflow-x:auto;font-family:monospace;margin:10px 0;"><code>${encodedCode}</code></pre>`;
  });

  // 处理不带语言标记的多行代码块：```code```
  content = content.replace(/```([\s\S]*?)```/g, (_match, code) => {
    const encodedCode = code
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
    return `<pre style="background:#f5f5f5;padding:15px;border-radius:8px;overflow-x:auto;font-family:monospace;margin:10px 0;"><code>${encodedCode}</code></pre>`;
  });

  // 处理行内代码：`code`（注意避免处理已经转换的代码块内的内容）
  // 使用正则确保不匹配已经转换的 HTML 中的内容
  content = content.replace(/`([^`<>\n]+)`/g, (_match, code) => {
    const encodedCode = code
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
    return `<code style="background:#f0f0f0;padding:2px 6px;border-radius:4px;font-family:monospace;font-size:0.9em;">${encodedCode}</code>`;
  });

  return content;
}

/**
 * 清理消息内容中的思考过程标记
 * 去除类似 "这种(...)的" 这样的括号内容
 * 并将 Markdown 代码转换为 HTML
 */
export function cleanMessageContent(content: string): string {
  // 先去除类似 "这种(...)的" 或 "(...)" 格式的思考过程标记
  let cleaned = content
    .replace(/[（(][^）)]*[）)]/g, '')
    .trim();

  // 将 Markdown 代码块转换为 HTML
  cleaned = convertMarkdownCodeToHtml(cleaned);

  return cleaned;
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
