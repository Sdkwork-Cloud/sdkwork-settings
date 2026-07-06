//! Settings 服务层错误类型。
//!
//! 实现 `From<sqlx::Error>` 与 `From<SettingsError>`,使服务方法可通过 `?`
//! 透明地传播数据库与领域错误。

use sdkwork_settings_contract::SettingsError;
use thiserror::Error;

/// 服务层错误枚举。
#[derive(Debug, Error)]
pub enum ServiceError {
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

impl From<sqlx::Error> for ServiceError {
    fn from(error: sqlx::Error) -> Self {
        if matches!(error, sqlx::Error::RowNotFound) {
            return ServiceError::NotFound;
        }
        ServiceError::Database(error.to_string())
    }
}

impl From<SettingsError> for ServiceError {
    fn from(error: SettingsError) -> Self {
        match error {
            SettingsError::NotFound => ServiceError::NotFound,
            SettingsError::Unauthorized => ServiceError::Unauthorized,
            SettingsError::Forbidden => ServiceError::Forbidden,
            SettingsError::Validation(message) => ServiceError::Validation(message),
            SettingsError::Database(message) => ServiceError::Database(message),
            SettingsError::Internal(message) => ServiceError::Internal(message),
        }
    }
}
