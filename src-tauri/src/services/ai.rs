//! AI 服务模块
//!
//! 封装多个 AI 服务商的 API 调用

use std::error::Error as StdError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// AI 服务商类型（国内供应商 + 自定义）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AIProvider {
    Qwen,
    DeepSeek,
    Glm,
    MiniMax,
    Kimi,
    Custom,
}

impl AIProvider {
    /// 获取服务商的 API 基础 URL
    pub fn base_url(&self) -> &str {
        match self {
            AIProvider::Qwen => "https://dashscope.aliyuncs.com",
            AIProvider::DeepSeek => "https://api.deepseek.com",
            AIProvider::Glm => "https://open.bigmodel.cn",
            AIProvider::MiniMax => "https://api.minimax.chat",
            AIProvider::Kimi => "https://api.moonshot.cn",
            AIProvider::Custom => "", // 自定义需要用户提供
        }
    }

    /// 获取默认模型
    pub fn default_model(&self) -> &str {
        match self {
            AIProvider::Qwen => "qwen3.5-plus",
            AIProvider::DeepSeek => "deepseek-chat",
            AIProvider::Glm => "glm-5",
            AIProvider::MiniMax => "M2.7-highspeed",
            AIProvider::Kimi => "kimi-k2.5",
            AIProvider::Custom => "", // 自定义需要用户提供
        }
    }
}

/// AI 配置
#[derive(Debug, Clone)]
pub struct AIConfig {
    pub provider: AIProvider,
    pub api_key: String,
    pub model: Option<String>,
    pub base_url: Option<String>,
}

impl AIConfig {
    pub fn new(provider: AIProvider, api_key: String) -> Self {
        Self {
            provider,
            api_key,
            model: None,
            base_url: None,
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    pub fn get_model(&self) -> &str {
        self.model.as_deref().unwrap_or_else(|| self.provider.default_model())
    }

    pub fn get_base_url(&self) -> &str {
        self.base_url.as_deref().unwrap_or_else(|| self.provider.base_url())
    }
}

/// 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// AI 错误类型
#[derive(Debug, Error)]
pub enum AIError {
    #[error("HTTP 请求失败: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("API 返回错误: {0}")]
    ApiError(String),

    #[error("响应解析失败: {0}")]
    ParseError(String),

    #[error("不支持的服务商: {0}")]
    UnsupportedProvider(String),

    #[error("网络错误: {0}")]
    NetworkError(String),
}

/// OpenAI 格式响应（通用格式）
#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    content: String,
}

/// 答案分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeResult {
    pub score: u32,
    pub feedback: String,
    pub weak_points: Vec<String>,
}

/// 练习题选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseOption {
    pub id: String,
    pub label: String,
    pub content: String,
}

/// 结构化练习题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredExercise {
    pub id: String,
    pub lesson_id: String,
    pub question: String,
    pub options: Vec<ExerciseOption>,
    pub correct_answer: String,
    pub explanation: Option<String>,
}

/// 发送聊天消息到 AI
pub async fn chat(config: &AIConfig, messages: Vec<ChatMessage>) -> Result<String, AIError> {
    eprintln!("[AI] provider={:?}, model={}, base_url={}, messages_count={}",
        config.provider, config.get_model(), config.get_base_url(), messages.len());

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .connect_timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| AIError::NetworkError(e.to_string()))?;
    let model = config.get_model();

    let result = match config.provider {
        AIProvider::Qwen => chat_qwen(&client, config, model, messages).await,
        AIProvider::DeepSeek | AIProvider::Glm | AIProvider::MiniMax | AIProvider::Kimi | AIProvider::Custom => {
            chat_openai_format(&client, config, model, messages).await
        }
    };

    if let Err(ref e) = result {
        eprintln!("[AI] request failed: {}", e);
    } else {
        eprintln!("[AI] request succeeded");
    }

    result
}

/// OpenAI 格式 API 调用（适用于 DeepSeek、GLM、MiniMax、Kimi）
async fn chat_openai_format(
    client: &Client,
    config: &AIConfig,
    model: &str,
    messages: Vec<ChatMessage>,
) -> Result<String, AIError> {
    let endpoint = match config.provider {
        AIProvider::DeepSeek => "/v1/chat/completions",
        AIProvider::Glm => "/api/paas/v4/chat/completions",
        AIProvider::MiniMax => "/v1/chat/completions",
        AIProvider::Kimi => "/v1/chat/completions",
        _ => "/v1/chat/completions",
    };

    let url = format!("{}{}", config.get_base_url(), endpoint);

    #[derive(Serialize)]
    struct OpenAIRequest {
        model: String,
        messages: Vec<ChatMessage>,
    }

    let request = OpenAIRequest {
        model: model.to_string(),
        messages,
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(AIError::ApiError(error_text));
    }

    let openai_response: OpenAIResponse = response.json().await?;
    let text = openai_response.choices.first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| AIError::ParseError("No content in response".to_string()))?;

    Ok(text)
}

/// 通义千问 API 调用（使用 OpenAI 兼容模式）
async fn chat_qwen(
    client: &Client,
    config: &AIConfig,
    model: &str,
    messages: Vec<ChatMessage>,
) -> Result<String, AIError> {
    // 使用 OpenAI 兼容模式端点
    let url = format!("{}/compatible-mode/v1/chat/completions", config.get_base_url());

    #[derive(Serialize)]
    struct OpenAIRequest {
        model: String,
        messages: Vec<ChatMessage>,
    }

    let request = OpenAIRequest {
        model: model.to_string(),
        messages,
    };

    let body_json = serde_json::to_string(&request).unwrap_or_default();
    let api_key_preview = if config.api_key.len() > 8 {
        format!("{}...{}", &config.api_key[..4], &config.api_key[config.api_key.len()-4..])
    } else {
        "***".to_string()
    };

    eprintln!("[AI-Qwen] POST {}", url);
    eprintln!("[AI-Qwen] Authorization: Bearer {}", api_key_preview);
    eprintln!("[AI-Qwen] Body length: {} bytes", body_json.len());

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            // 打印完整的错误链，方便排查 DNS/TLS/代理等问题
            eprintln!("[AI-Qwen] send failed: {}", e);
            if e.is_timeout() {
                eprintln!("[AI-Qwen] cause: timeout");
            } else if e.is_connect() {
                eprintln!("[AI-Qwen] cause: connection error");
            } else if e.is_request() {
                eprintln!("[AI-Qwen] cause: request build error");
            }
            let mut source: Option<&dyn StdError> = e.source();
            let mut depth = 0;
            while let Some(cause) = source {
                depth += 1;
                eprintln!("[AI-Qwen] cause[{}]: {}", depth, cause);
                source = cause.source();
            }
            e
        })?;

    eprintln!("[AI-Qwen] response status: {}", response.status());

    if !response.status().is_success() {
        let error_text = response.text().await?;
        eprintln!("[AI-Qwen] error body: {}", error_text);
        return Err(AIError::ApiError(error_text));
    }

    let openai_response: OpenAIResponse = response.json().await?;
    let text = openai_response.choices.first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| AIError::ParseError("No content in response".to_string()))?;
    Ok(text)
}

/// 生成 HTML 课件
pub async fn generate_lesson(
    config: &AIConfig,
    course_name: &str,
    chapter_name: &str,
    lesson_name: &str,
    teaching_style: &str,
) -> Result<String, AIError> {
    let system_prompt = format!(
        "你是一位专业的教师，擅长以「{}」风格进行教学。\
        请根据课程信息生成一个完整的 HTML 课件，内容要详实、有示例、有互动。\
        HTML 格式要求：使用简洁的样式，内嵌 CSS，适合在网页中渲染显示。\
        不要使用外部 CSS 或 JS 文件，所有样式内联。\
        课件应包含：标题、知识点讲解、代码示例（如有）、小结。",
        teaching_style
    );

    let user_prompt = format!(
        "请为以下课程生成 HTML 课件：\n\
        课程名称：{}\n\
        章节名称：{}\n\
        课时名称：{}\n\
        请生成完整的 HTML 课件内容。",
        course_name, chapter_name, lesson_name
    );

    let messages = vec![
        ChatMessage { role: "system".to_string(), content: system_prompt },
        ChatMessage { role: "user".to_string(), content: user_prompt },
    ];

    let html = chat(config, messages).await?;

    // 清理响应内容：移除可能的 markdown 代码块标记
    let cleaned_html = html
        .trim()
        .trim_start_matches("```html")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    Ok(cleaned_html.to_string())
}

/// 生成练习题 HTML
pub async fn generate_exercise(
    config: &AIConfig,
    lesson_content: &str,
) -> Result<String, AIError> {
    let system_prompt = "你是一位专业的教师，请根据课件内容生成练习题。\
        练习题格式为 HTML，包含选择题、填空题或简答题。\
        HTML 格式要求：使用表单元素（input、radio、textarea），方便用户作答。\
        每道题有唯一 ID，提交按钮触发答案收集。\
        内嵌 CSS，简洁美观。".to_string();

    let user_prompt = format!(
        "请根据以下课件内容生成 5-10 道练习题：\n\
        课件内容：\n{}\n\
        请生成包含表单的 HTML 练习题。",
        lesson_content
    );

    let messages = vec![
        ChatMessage { role: "system".to_string(), content: system_prompt },
        ChatMessage { role: "user".to_string(), content: user_prompt },
    ];

    chat(config, messages).await
}

/// 分析用户答案
pub async fn analyze_answers(
    config: &AIConfig,
    exercise_content: &str,
    user_answers: &str,
) -> Result<AnalyzeResult, AIError> {
    let system_prompt = "你是一位专业的教师，请分析学生的练习题答案。\
        返回 JSON 格式结果，包含：score（百分制分数）、feedback（总体评语）、weak_points（薄弱知识点数组）。\
        格式示例：{\"score\": 85, \"feedback\": \"整体表现良好...\", \"weak_points\": [\"变量命名\", \"循环结构\"]}".to_string();

    let user_prompt = format!(
        "请分析以下练习题答案：\n\
        练习题内容：\n{}\n\
        用户答案：\n{}\n\
        请返回 JSON 格式的分析结果。",
        exercise_content, user_answers
    );

    let messages = vec![
        ChatMessage { role: "system".to_string(), content: system_prompt },
        ChatMessage { role: "user".to_string(), content: user_prompt },
    ];

    let response = chat(config, messages).await?;

    // 解析 JSON 结果
    let result: AnalyzeResult = serde_json::from_str(&response)
        .map_err(|e| AIError::ParseError(format!("解析答案分析结果失败: {}", e)))?;

    Ok(result)
}

/// 验证 API 密钥是否有效
pub async fn verify_api_key(config: &AIConfig) -> Result<bool, AIError> {
    let test_message = vec![
        ChatMessage { role: "user".to_string(), content: "Hi".to_string() },
    ];

    // 简单测试调用
    match chat(config, test_message).await {
        Ok(_) => Ok(true),
        Err(AIError::ApiError(_)) => Ok(false),
        Err(e) => Err(e),
    }
}

/// 生成课程计划大纲
pub async fn generate_course_plan(
    config: &AIConfig,
    course_name: &str,
    target_level: &str,
    duration: &str,
    teaching_style: &str,
    base_knowledge: &str,
) -> Result<CoursePlanOutline, AIError> {
    let system_prompt = format!(
        r#"你是一位专业的课程规划教师，擅长以「{}」风格设计课程大纲。
请根据用户提供的课程信息，生成一个结构化的课程计划大纲。
返回 JSON 格式结果，结构如下：
{{
  "course_name": "课程名称",
  "target_level": "目标水平",
  "duration": "总时长",
  "teaching_style": "教学风格",
  "chapters": [
    {{
      "chapter_index": 1,
      "chapter_name": "章节名称",
      "lessons": [
        {{
          "lesson_index": 1,
          "lesson_name": "课时名称",
          "duration": "课时时长"
        }}
      ]
    }}
  ]
}}

要求：
- 章节数量建议 3-6 章，每章 2-5 课时
- 课时时长建议 20-45 分钟
- 课程内容要由浅入深，循序渐进
- 结合用户的基础知识水平合理安排内容深度
- JSON 必须严格合法，不要包含 markdown 代码块标记"#,
        teaching_style
    );

    let user_prompt = format!(
        "请为以下课程生成详细的教学大纲：\n\
        课程名称：{}\n\
        目标水平：{}\n\
        总时长：{}\n\
        教学风格：{}\n\
        用户基础知识水平：{}\n\n\
        请返回 JSON 格式的课程大纲。",
        course_name, target_level, duration, teaching_style, base_knowledge
    );

    let messages = vec![
        ChatMessage { role: "system".to_string(), content: system_prompt },
        ChatMessage { role: "user".to_string(), content: user_prompt },
    ];

    let response = chat(config, messages).await?;

    // 解析 JSON 结果
    let plan: CoursePlanOutline = serde_json::from_str(&response)
        .map_err(|e| AIError::ParseError(format!("解析课程大纲失败: {}", e)))?;

    Ok(plan)
}

/// 课程计划大纲（AI 生成输出）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoursePlanOutline {
    #[serde(rename = "courseName", alias = "course_name")]
    pub course_name: String,
    #[serde(rename = "targetLevel", alias = "target_level")]
    pub target_level: String,
    pub duration: String,
    #[serde(rename = "teachingStyle", alias = "teaching_style")]
    pub teaching_style: String,
    pub chapters: Vec<ChapterPlanOutline>,
}

/// 章节大纲
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterPlanOutline {
    #[serde(rename = "chapterIndex", alias = "chapter_index")]
    pub chapter_index: i32,
    #[serde(rename = "chapterName", alias = "chapter_name")]
    pub chapter_name: String,
    pub lessons: Vec<LessonPlanOutline>,
}

/// 课时大纲
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonPlanOutline {
    #[serde(rename = "lessonIndex", alias = "lesson_index")]
    pub lesson_index: i32,
    #[serde(rename = "lessonName", alias = "lesson_name")]
    pub lesson_name: String,
    pub duration: String,
}

/// 生成结构化练习题
pub async fn generate_structured_exercise(
    config: &AIConfig,
    lesson_id: &str,
    lesson_content: &str,
) -> Result<Vec<StructuredExercise>, AIError> {
    let system_prompt = r#"你是一位专业的教师，请根据课件内容生成结构化的练习题。
返回 JSON 格式的数组，每道题包含：
- id: 题目唯一ID（格式：exercise_1, exercise_2, ...）
- lesson_id: 课时ID
- question: 题目文本
- options: 选项数组（选择题有此字段，填空题为空数组），每个选项包含 id（a/b/c/d）、label（A/B/C/D）、content（选项内容）
- correct_answer: 正确答案（选择题为选项ID，填空题为正确答案文本）
- explanation: 题目的详细解析

返回格式示例：
[
  {
    "id": "exercise_1",
    "lesson_id": "lesson_001",
    "question": "以下哪个是 Python 的变量命名规则？",
    "options": [
      {"id": "a", "label": "A", "content": "可以以数字开头"},
      {"id": "b", "label": "B", "content": "可以包含空格"},
      {"id": "c", "label": "C", "content": "区分大小写"},
      {"id": "d", "label": "D", "content": "以关键字命名"}
    ],
    "correct_answer": "c",
    "explanation": "Python 变量命名规则：必须以字母或下划线开头，不能以数字开头，不能包含空格，区分大小写。"
  },
  {
    "id": "exercise_2",
    "lesson_id": "lesson_001",
    "question": "在 Python 中，print 函数用于_______。",
    "options": [],
    "correct_answer": "输出内容到控制台",
    "explanation": "print 函数是 Python 的基本输出函数，用于将内容打印显示到控制台。"
  }
]

请生成 5-8 道练习题，包含选择题和填空题，难度适中。"#.to_string();

    let user_prompt = format!(
        "请根据以下课件内容生成结构化的练习题：\n\
        课件内容：\n{}\n\
        课时ID：{}\n\
        请返回 JSON 格式的练习题数组。",
        lesson_content, lesson_id
    );

    let messages = vec![
        ChatMessage { role: "system".to_string(), content: system_prompt },
        ChatMessage { role: "user".to_string(), content: user_prompt },
    ];

    let response = chat(config, messages).await?;

    // 打印原始响应以便调试
    eprintln!("[AI] 原始练习题响应（前500字符）: {}", if response.len() > 500 { &response[..500] } else { &response });

    // 清理响应内容：移除可能的 markdown 代码块标记
    let cleaned_response = response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    // 解析 JSON 结果
    let exercises: Vec<StructuredExercise> = serde_json::from_str(cleaned_response)
        .map_err(|e| AIError::ParseError(format!("解析练习题数据失败: {}，原始响应开头: {}", e,
            if cleaned_response.len() > 100 { &cleaned_response[..100] } else { cleaned_response })))?;

    Ok(exercises)
}