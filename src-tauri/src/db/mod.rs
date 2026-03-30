//! 数据库模块
//!
//! 提供 SQLite 数据库的创建、初始化和 CRUD 操作

pub mod schema;
pub mod operations;

use std::path::PathBuf;
use std::sync::Mutex;
use rusqlite::Connection;
use thiserror::Error;

pub use schema::*;
pub use operations::*;

/// 数据库错误类型
#[derive(Error, Debug)]
pub enum DbError {
    #[error("SQLite error: {0}")]
    SqliteError(#[from] rusqlite::Error),

    #[error("数据库未初始化")]
    NotInitialized,

    #[error("记录不存在: {0}")]
    NotFound(String),

    #[error("记录已存在: {0}")]
    AlreadyExists(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// 数据库连接包装器
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// 创建或打开数据库
    ///
    /// # Arguments
    /// * `path` - 数据库文件路径
    ///
    /// # Returns
    /// 数据库实例
    pub fn new(path: PathBuf) -> Result<Self, DbError> {
        // 确保父目录存在
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&path)?;

        // 启用外键约束
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        let db = Database {
            conn: Mutex::new(conn),
        };

        // 初始化表结构
        db.initialize_schema()?;

        Ok(db)
    }

    /// 获取数据库路径
    ///
    /// 返回 `%LOCALAPPDATA%\StudyMate\data.db`
    pub fn get_default_path() -> PathBuf {
        let local_app_data = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        local_app_data.join("StudyMate").join("data.db")
    }

    /// 初始化数据库表结构
    fn initialize_schema(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();

        // 创建 courses 表
        conn.execute(
            &schema::CREATE_COURSES_TABLE,
            [],
        )?;

        // 创建 chapters 表
        conn.execute(
            &schema::CREATE_CHAPTERS_TABLE,
            [],
        )?;

        // 创建 lessons 表
        conn.execute(
            &schema::CREATE_LESSONS_TABLE,
            [],
        )?;

        // 创建 lesson_exercises 表
        conn.execute(
            &schema::CREATE_LESSON_EXERCISES_TABLE,
            [],
        )?;

        // 创建 chat_messages 表
        conn.execute(
            &schema::CREATE_CHAT_MESSAGES_TABLE,
            [],
        )?;

        // 创建 user_config 表
        conn.execute(
            &schema::CREATE_USER_CONFIG_TABLE,
            [],
        )?;

        Ok(())
    }

    /// 获取数据库连接（用于事务操作）
    pub fn get_connection(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_path() {
        let path = Database::get_default_path();
        println!("Default database path: {:?}", path);
        assert!(path.to_string_lossy().contains("StudyMate"));
    }

    #[test]
    fn test_database_creation() {
        let temp_path = std::env::temp_dir().join("test_studymate.db");
        let _ = std::fs::remove_file(&temp_path); // 删除已存在的测试数据库

        let db = Database::new(temp_path.clone());
        assert!(db.is_ok());

        // 验证数据库文件已创建
        assert!(temp_path.exists());

        // 清理
        let _ = std::fs::remove_file(&temp_path);
    }
}
