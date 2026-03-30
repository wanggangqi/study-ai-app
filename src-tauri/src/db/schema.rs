//! 数据库表结构定义
//!
//! 定义所有数据表的 SQL 创建语句

/// courses 表创建语句
pub const CREATE_COURSES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS courses (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    gitee_repo_url TEXT,
    local_path TEXT,
    target_level TEXT,
    duration TEXT,
    teaching_style TEXT,
    created_at TEXT NOT NULL,
    status INTEGER NOT NULL DEFAULT 0
)
"#;

/// chapters 表创建语句
pub const CREATE_CHAPTERS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS chapters (
    id TEXT PRIMARY KEY,
    course_id TEXT NOT NULL,
    chapter_index INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE
)
"#;

/// lessons 表创建语句
pub const CREATE_LESSONS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS lessons (
    id TEXT PRIMARY KEY,
    chapter_id TEXT NOT NULL,
    lesson_index INTEGER NOT NULL,
    name TEXT NOT NULL,
    duration TEXT,
    status INTEGER NOT NULL DEFAULT 0,
    completed_at TEXT,
    lesson_file TEXT,
    FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE
)
"#;

/// lesson_exercises 表创建语句
pub const CREATE_LESSON_EXERCISES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS lesson_exercises (
    id TEXT PRIMARY KEY,
    lesson_id TEXT NOT NULL,
    exercise_file TEXT,
    score INTEGER,
    submitted_at TEXT,
    result_file TEXT,
    FOREIGN KEY (lesson_id) REFERENCES lessons(id) ON DELETE CASCADE
)
"#;

/// chat_messages 表创建语句
pub const CREATE_CHAT_MESSAGES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS chat_messages (
    id TEXT PRIMARY KEY,
    course_id TEXT NOT NULL,
    lesson_id TEXT,
    agent_type TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE,
    FOREIGN KEY (lesson_id) REFERENCES lessons(id) ON DELETE SET NULL
)
"#;

/// user_config 表创建语句
pub const CREATE_USER_CONFIG_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS user_config (
    id INTEGER PRIMARY KEY,
    gitee_username TEXT,
    gitee_token TEXT,
    workspace_path TEXT,
    ai_provider TEXT,
    ai_api_key TEXT,
    ai_model TEXT,
    git_username TEXT,
    git_email TEXT
)
"#;

/// 课程状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CourseStatus {
    InProgress = 0,
    Completed = 1,
    Paused = 2,
}

impl CourseStatus {
    pub fn from_i32(value: i32) -> Self {
        match value {
            1 => CourseStatus::Completed,
            2 => CourseStatus::Paused,
            _ => CourseStatus::InProgress,
        }
    }

    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

/// 课时状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LessonStatus {
    NotStarted = 0,
    InProgress = 1,
    Completed = 2,
}

impl LessonStatus {
    pub fn from_i32(value: i32) -> Self {
        match value {
            1 => LessonStatus::InProgress,
            2 => LessonStatus::Completed,
            _ => LessonStatus::NotStarted,
        }
    }

    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

/// Agent 类型枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentType {
    Consultant,
    Teacher,
}

impl AgentType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "teacher" => AgentType::Teacher,
            _ => AgentType::Consultant,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            AgentType::Consultant => "consultant",
            AgentType::Teacher => "teacher",
        }
    }
}

/// 消息角色枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageRole {
    User,
    Assistant,
}

impl MessageRole {
    pub fn from_str(s: &str) -> Self {
        match s {
            "assistant" => MessageRole::Assistant,
            _ => MessageRole::User,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            MessageRole::User => "user",
            MessageRole::Assistant => "assistant",
        }
    }
}
