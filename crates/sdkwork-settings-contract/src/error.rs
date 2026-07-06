//! Settings 领域错误类型。
//!
//! 仅定义领域错误,不依赖 sqlx 等具体实现,具体转换由上层 service-host 提供。

use thiserror::Error;

/// Settings 领域错误枚举。
#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("resource not found")]
    NotFound,

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("database error: {0}")]
    Database(String),

    #[error("internal error: {0}")]
    Internal(String),
}
