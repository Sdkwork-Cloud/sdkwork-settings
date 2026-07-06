//! Settings 值对象与校验工具。
//!
//! 复用 [`sdkwork_utils_rust`] 的字符串工具进行空白判断;namespace 与 key 的
//! 命名规则通过等价于正则的手写字符校验实现,避免在契约层引入 regex 依赖。

use serde::{Deserialize, Serialize};

use crate::error::SettingsError;

/// 配置作用域。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigScope {
    User,
    Tenant,
    System,
}

impl ConfigScope {
    pub fn as_str(self) -> &'static str {
        match self {
            ConfigScope::User => "user",
            ConfigScope::Tenant => "tenant",
            ConfigScope::System => "system",
        }
    }
}

/// 配置值类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigValueType {
    String,
    Number,
    Boolean,
    Object,
    Array,
    I18n,
}

impl ConfigValueType {
    pub fn as_str(self) -> &'static str {
        match self {
            ConfigValueType::String => "string",
            ConfigValueType::Number => "number",
            ConfigValueType::Boolean => "boolean",
            ConfigValueType::Object => "object",
            ConfigValueType::Array => "array",
            ConfigValueType::I18n => "i18n",
        }
    }
}

/// namespace 命名规则:等价于 `^[a-z][a-z0-9-]{0,63}$`(总长度 1..=64)。
pub fn validate_namespace(namespace: &str) -> Result<(), SettingsError> {
    if sdkwork_utils_rust::is_blank(Some(namespace)) {
        return Err(SettingsError::Validation("namespace 不能为空".to_string()));
    }
    let chars: Vec<char> = namespace.chars().collect();
    if chars.len() > 64 {
        return Err(SettingsError::Validation(
            "namespace 长度不能超过 64 个字符".to_string(),
        ));
    }
    let mut iter = chars.iter();
    match iter.next() {
        Some(first) if first.is_ascii_lowercase() => {}
        _ => {
            return Err(SettingsError::Validation(
                "namespace 必须以小写字母开头".to_string(),
            ));
        }
    }
    for ch in iter {
        let valid = ch.is_ascii_lowercase() || ch.is_ascii_digit() || *ch == '-';
        if !valid {
            return Err(SettingsError::Validation(
                "namespace 仅允许小写字母、数字与连字符".to_string(),
            ));
        }
    }
    Ok(())
}

/// key 命名规则:等价于 `^[a-z][a-z0-9-_]{0,127}$`(总长度 1..=128)。
pub fn validate_config_key(key: &str) -> Result<(), SettingsError> {
    if sdkwork_utils_rust::is_blank(Some(key)) {
        return Err(SettingsError::Validation("key 不能为空".to_string()));
    }
    let chars: Vec<char> = key.chars().collect();
    if chars.len() > 128 {
        return Err(SettingsError::Validation(
            "key 长度不能超过 128 个字符".to_string(),
        ));
    }
    let mut iter = chars.iter();
    match iter.next() {
        Some(first) if first.is_ascii_lowercase() => {}
        _ => {
            return Err(SettingsError::Validation(
                "key 必须以小写字母开头".to_string(),
            ));
        }
    }
    for ch in iter {
        let valid = ch.is_ascii_lowercase() || ch.is_ascii_digit() || *ch == '-' || *ch == '_';
        if !valid {
            return Err(SettingsError::Validation(
                "key 仅允许小写字母、数字、连字符与下划线".to_string(),
            ));
        }
    }
    Ok(())
}

/// 配置值大小校验,上限 64KB。
pub fn validate_value_size(value: &serde_json::Value) -> Result<(), SettingsError> {
    const MAX_VALUE_BYTES: usize = 64 * 1024;
    let serialized = serde_json::to_vec(value)
        .map_err(|error| SettingsError::Internal(format!("序列化配置值失败: {error}")))?;
    if serialized.len() > MAX_VALUE_BYTES {
        return Err(SettingsError::Validation(format!(
            "配置值大小 {} 字节超过 64KB 上限",
            serialized.len()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_valid_and_invalid() {
        assert!(validate_namespace("ui").is_ok());
        assert!(validate_namespace("feature-flag").is_ok());
        assert!(validate_namespace("A").is_err());
        assert!(validate_namespace("").is_err());
        assert!(validate_namespace("1abc").is_err());
        assert!(validate_namespace("a_b").is_err());
    }

    #[test]
    fn key_valid_and_invalid() {
        assert!(validate_config_key("theme").is_ok());
        assert!(validate_config_key("locale_id").is_ok());
        assert!(validate_config_key("font-size").is_ok());
        assert!(validate_config_key("").is_err());
        assert!(validate_config_key("Key").is_err());
    }

    #[test]
    fn value_size_within_limit() {
        assert!(validate_value_size(&serde_json::json!({"k": "v"})).is_ok());
    }
}
