//! SDKWork Settings 领域契约模块。
//!
//! 该 crate 定义配置中心领域的 DTO、错误类型与值对象,不依赖具体实现
//! (无 sqlx、无 axum),可被 service-host、web 层与 SDK 共享。
//!
//! 命名遵循 sdkwork-specs:crate 名使用 kebab-case,lib 名使用 snake_case。

pub mod error;
pub mod models;
pub mod value_types;

pub use error::SettingsError;
pub use models::{
    BatchUpdateItem, BatchUpdateUserPreferenceRequest, ConfigRevision, PreferenceListQuery,
    SystemSetting, TenantConfig, UpdateUserPreferenceRequest, UserPreference,
};
pub use value_types::{
    ConfigScope, ConfigValueType, validate_config_key, validate_namespace, validate_value_size,
};

/// 校验 namespace 与 key 是否符合命名规范。
///
/// 复用 [`sdkwork_utils_rust`] 的字符串工具进行空白判断,命名规则由
/// [`value_types`] 模块中的正则等价实现保证。
pub fn validate_namespace_and_key(namespace: &str, key: &str) -> Result<(), SettingsError> {
    validate_namespace(namespace)?;
    validate_config_key(key)?;
    Ok(())
}
