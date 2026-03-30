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
    pub default_branch: String,
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

/// 创建仓库请求
#[derive(Debug, Serialize)]
struct CreateRepoRequest {
    access_token: String,
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

    let request = CreateRepoRequest {
        access_token: token.to_string(),
        name: repo_name.to_string(),
        description: description.to_string(),
        private,
        auto_init: false,
        default_branch: "main".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| GiteeError::HttpError(e.to_string()))?;

    if response.status().is_success() {
        response
            .json::<GiteeRepo>()
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

/// Tauri 命令：验证码云账户
#[tauri::command]
pub async fn verify_gitee_account(token: String) -> GiteeAccountResult {
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
pub async fn create_gitee_repo(
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
pub async fn check_gitee_repo_exists(token: String, owner: String, repo: String) -> bool {
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
