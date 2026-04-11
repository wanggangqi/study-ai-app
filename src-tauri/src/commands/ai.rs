//! AI 命令模块
//!
//! 提供前端可调用的 AI 相关 Tauri 命令

use crate::services::{
    AIProvider,
    AIConfig,
    ChatMessage,
    StructuredExercise,
    chat,
    generate_lesson,
    generate_exercise,
    analyze_answers,
    verify_api_key,
    generate_structured_exercise,
};

use serde::{Deserialize, Serialize};

/// AI 聊天命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct AIChatParams {
    pub provider: String,
    pub api_key: String,
    pub model: Option<String>,
    pub messages: Vec<ChatMessageParams>,
}

/// 聊天消息参数
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessageParams {
    pub role: String,
    pub content: String,
}

impl From<ChatMessageParams> for ChatMessage {
    fn from(params: ChatMessageParams) -> Self {
        ChatMessage {
            role: params.role,
            content: params.content,
        }
    }
}

/// AI 课件生成命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct AIGenerateLessonParams {
    pub provider: String,
    pub api_key: String,
    pub model: Option<String>,
    pub course_name: String,
    pub chapter_name: String,
    pub lesson_name: String,
    pub teaching_style: String,
}

/// AI 练习生成命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct AIGenerateExerciseParams {
    pub provider: String,
    pub api_key: String,
    pub model: Option<String>,
    pub lesson_content: String,
}

/// AI 答案分析命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct AIAnalyzeAnswersParams {
    pub provider: String,
    pub api_key: String,
    pub model: Option<String>,
    pub exercise_content: String,
    pub user_answers: String,
}

/// AI 验证密钥命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct AIVerifyKeyParams {
    pub provider: String,
    pub api_key: String,
}

/// AI 命令结果
#[derive(Debug, Serialize, Deserialize)]
pub struct AIResult {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>,
}

/// 答案分析结果
#[derive(Debug, Serialize, Deserialize)]
pub struct AIAnalyzeResult {
    pub success: bool,
    pub score: Option<u32>,
    pub feedback: Option<String>,
    pub weak_points: Option<Vec<String>>,
    pub error: Option<String>,
}

/// 解析 AI 服务商
fn parse_provider(provider_str: &str) -> Result<AIProvider, String> {
    match provider_str.to_lowercase().as_str() {
        "qwen" => Ok(AIProvider::Qwen),
        "deepseek" => Ok(AIProvider::DeepSeek),
        "glm" => Ok(AIProvider::Glm),
        "minimax" => Ok(AIProvider::MiniMax),
        "kimi" => Ok(AIProvider::Kimi),
        _ => Err(format!("不支持的 AI 服务商: {}", provider_str)),
    }
}

/// AI 聊天命令
#[tauri::command]
pub async fn ai_chat_command(params: AIChatParams) -> AIResult {
    let provider = match parse_provider(&params.provider) {
        Ok(p) => p,
        Err(e) => return AIResult { success: false, data: None, error: Some(e) },
    };

    let config = AIConfig::new(provider, params.api_key);
    let config = if let Some(model) = params.model {
        config.with_model(model)
    } else {
        config
    };

    let messages: Vec<ChatMessage> = params.messages.into_iter()
        .map(|m| m.into())
        .collect();

    match chat(&config, messages).await {
        Ok(response) => AIResult { success: true, data: Some(response), error: None },
        Err(e) => AIResult { success: false, data: None, error: Some(e.to_string()) },
    }
}

/// AI 生成课件命令
#[tauri::command]
pub async fn ai_generate_lesson_command(params: AIGenerateLessonParams) -> AIResult {
    let provider = match parse_provider(&params.provider) {
        Ok(p) => p,
        Err(e) => return AIResult { success: false, data: None, error: Some(e) },
    };

    let config = AIConfig::new(provider, params.api_key);
    let config = if let Some(model) = params.model {
        config.with_model(model)
    } else {
        config
    };

    match generate_lesson(
        &config,
        &params.course_name,
        &params.chapter_name,
        &params.lesson_name,
        &params.teaching_style,
    ).await {
        Ok(html) => AIResult { success: true, data: Some(html), error: None },
        Err(e) => AIResult { success: false, data: None, error: Some(e.to_string()) },
    }
}

/// AI 生成练习题命令
#[tauri::command]
pub async fn ai_generate_exercise_command(params: AIGenerateExerciseParams) -> AIResult {
    let provider = match parse_provider(&params.provider) {
        Ok(p) => p,
        Err(e) => return AIResult { success: false, data: None, error: Some(e) },
    };

    let config = AIConfig::new(provider, params.api_key);
    let config = if let Some(model) = params.model {
        config.with_model(model)
    } else {
        config
    };

    match generate_exercise(&config, &params.lesson_content).await {
        Ok(html) => AIResult { success: true, data: Some(html), error: None },
        Err(e) => AIResult { success: false, data: None, error: Some(e.to_string()) },
    }
}

/// AI 分析答案命令
#[tauri::command]
pub async fn ai_analyze_answers_command(params: AIAnalyzeAnswersParams) -> AIAnalyzeResult {
    let provider = match parse_provider(&params.provider) {
        Ok(p) => p,
        Err(e) => return AIAnalyzeResult {
            success: false,
            score: None,
            feedback: None,
            weak_points: None,
            error: Some(e),
        },
    };

    let config = AIConfig::new(provider, params.api_key);
    let config = if let Some(model) = params.model {
        config.with_model(model)
    } else {
        config
    };

    match analyze_answers(&config, &params.exercise_content, &params.user_answers).await {
        Ok(analyze_result) => AIAnalyzeResult {
            success: true,
            score: Some(analyze_result.score),
            feedback: Some(analyze_result.feedback),
            weak_points: Some(analyze_result.weak_points),
            error: None,
        },
        Err(e) => AIAnalyzeResult {
            success: false,
            score: None,
            feedback: None,
            weak_points: None,
            error: Some(e.to_string()),
        },
    }
}

/// AI 验证密钥命令
#[tauri::command]
pub async fn ai_verify_key_command(params: AIVerifyKeyParams) -> AIResult {
    let provider = match parse_provider(&params.provider) {
        Ok(p) => p,
        Err(e) => return AIResult { success: false, data: None, error: Some(e) },
    };

    let config = AIConfig::new(provider, params.api_key);

    match verify_api_key(&config).await {
        Ok(valid) => AIResult {
            success: true,
            data: Some(if valid { "valid".to_string() } else { "invalid".to_string() }),
            error: None,
        },
        Err(e) => AIResult { success: false, data: None, error: Some(e.to_string()) },
    }
}

/// AI 生成结构化练习题命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct AIGenerateStructuredExerciseParams {
    pub provider: String,
    pub api_key: String,
    pub model: Option<String>,
    pub lesson_id: String,
    pub lesson_content: String,
}

/// 结构化练习题命令结果
#[derive(Debug, Serialize, Deserialize)]
pub struct AIStructuredExerciseResult {
    pub success: bool,
    pub data: Option<Vec<StructuredExercise>>,
    pub error: Option<String>,
}

/// AI 生成结构化练习题命令
#[tauri::command]
pub async fn ai_generate_structured_exercise_command(params: AIGenerateStructuredExerciseParams) -> AIStructuredExerciseResult {
    let provider = match parse_provider(&params.provider) {
        Ok(p) => p,
        Err(e) => return AIStructuredExerciseResult { success: false, data: None, error: Some(e) },
    };

    let config = AIConfig::new(provider, params.api_key);
    let config = if let Some(model) = params.model {
        config.with_model(model)
    } else {
        config
    };

    match generate_structured_exercise(&config, &params.lesson_id, &params.lesson_content).await {
        Ok(exercises) => AIStructuredExerciseResult { success: true, data: Some(exercises), error: None },
        Err(e) => AIStructuredExerciseResult { success: false, data: None, error: Some(e.to_string()) },
    }
}
