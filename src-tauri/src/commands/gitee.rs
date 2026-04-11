//! 码云 API 命令
//!
//! 提供与码云 Gitee API 交互的命令

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 码云 API 错误
#[derive(Error, Debug)]
pub enum GiteeError {
    #[error("HTTP 请求失败: {0}")]
    HttpError(String),
    #[error("API 返回错误: {0}")]
    ApiError(String),
    #[error("JSON 解析失败: {0}")]
    JsonError(String),
    #[error("未配置码云账户")]
    NotConfigured,
}

/// 码云 API 基础地址
const GITEE_API_BASE: &str = "https://gitee.com/api/v5";

/// 码云仓库信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiteeRepo {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub private: bool,
    pub default_branch: Option<String>,
}

/// 码云账户验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiteeAccountResult {
    pub success: bool,
    pub username: Option<String>,
    pub message: String,
}

/// 码云仓库创建结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiteeRepoResult {
    pub success: bool,
    pub repo_url: Option<String>,
    pub message: String,
}

/// 创建仓库请求（access_token 不放入 body）
#[derive(Debug, Serialize)]
struct CreateRepoRequest {
    name: String,
    description: String,
    private: bool,
    auto_init: bool,
    #[serde(rename = "default_branch")]
    default_branch: String,
}

/// 验证账户响应
#[derive(Debug, Deserialize)]
struct VerifyAccountResponse {
    login: String,
    id: i64,
    name: Option<String>,
}

/// 验证码云账户连接（内部函数）
async fn verify_gitee_account_internal(token: &str) -> Result<VerifyAccountResponse, GiteeError> {
    let url = format!("{}/user", GITEE_API_BASE);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .query(&[("access_token", token)])
        .send()
        .await
        .map_err(|e| GiteeError::HttpError(e.to_string()))?;

    if response.status().is_success() {
        response
            .json::<VerifyAccountResponse>()
            .await
            .map_err(|e| GiteeError::JsonError(e.to_string()))
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        Err(GiteeError::ApiError(format!(
            "status: {}, body: {}",
            status, text
        )))
    }
}

/// 创建码云仓库（内部函数，供模块间调用）
pub async fn create_gitee_repo_internal(
    token: &str,
    repo_name: &str,
    description: &str,
    private: bool,
) -> Result<GiteeRepo, GiteeError> {
    let url = format!("{}/user/repos", GITEE_API_BASE);
    eprintln!("[Gitee] 创建仓库 URL: {}", url);
    eprintln!("[Gitee] 仓库名称: {}", repo_name);
    eprintln!("[Gitee] Token长度: {}", token.len());

    let request = CreateRepoRequest {
        name: repo_name.to_string(),
        description: description.to_string(),
        private,
        auto_init: false,
        default_branch: "main".to_string(),
    };

    eprintln!("[Gitee] 请求体: {}", serde_json::to_string(&request).unwrap_or_default());

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .query(&[("access_token", token)])
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            eprintln!("[Gitee] HTTP请求失败: {}", e);
            GiteeError::HttpError(e.to_string())
        })?;

    eprintln!("[Gitee] 响应状态码: {}", response.status());

    if response.status().is_success() {
        let response_text = response.text().await.map_err(|e| GiteeError::HttpError(e.to_string()))?;
        eprintln!("[Gitee] 响应内容: {}", response_text);
        serde_json::from_str::<GiteeRepo>(&response_text)
            .map_err(|e| {
                eprintln!("[Gitee] JSON解析失败: {}", e);
                GiteeError::JsonError(e.to_string())
            })
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        eprintln!("[Gitee] API错误响应: status={}, body={}", status, text);
        Err(GiteeError::ApiError(format!(
            "status: {}, body: {}",
            status, text
        )))
    }
}

/// Tauri 命令：验证码云账户
#[tauri::command]
pub async fn verify_gitee_account_command(token: String) -> GiteeAccountResult {
    match verify_gitee_account_internal(&token).await {
        Ok(response) => GiteeAccountResult {
            success: true,
            username: Some(response.login),
            message: "账户验证成功".to_string(),
        },
        Err(e) => GiteeAccountResult {
            success: false,
            username: None,
            message: e.to_string(),
        },
    }
}

/// Tauri 命令：创建码云仓库
#[tauri::command]
pub async fn create_gitee_repo_command(
    token: String,
    repo_name: String,
    description: String,
    private: bool,
) -> GiteeRepoResult {
    match create_gitee_repo_internal(&token, &repo_name, &description, private).await {
        Ok(repo) => GiteeRepoResult {
            success: true,
            repo_url: Some(repo.html_url),
            message: "仓库创建成功".to_string(),
        },
        Err(e) => GiteeRepoResult {
            success: false,
            repo_url: None,
            message: e.to_string(),
        },
    }
}

/// Tauri 命令：检查仓库是否存在
#[tauri::command]
pub async fn check_gitee_repo_exists_command(token: String, owner: String, repo: String) -> bool {
    let url = format!("{}/repos/{}/{}", GITEE_API_BASE, owner, repo);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .query(&[("access_token", &token)])
        .send()
        .await;

    match response {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}
