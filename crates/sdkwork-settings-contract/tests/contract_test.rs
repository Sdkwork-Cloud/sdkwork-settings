//! SDKWork Settings contract 单元测试。

use sdkwork_settings_contract::{
    ConfigScope, ConfigValueType, validate_config_key, validate_namespace, validate_value_size,
};

#[test]
fn test_validate_namespace_valid() {
    assert!(validate_namespace("appearance").is_ok());
    assert!(validate_namespace("feature-flag").is_ok());
    assert!(validate_namespace("a").is_ok());
    assert!(validate_namespace("locale").is_ok());
}

#[test]
fn test_validate_namespace_invalid() {
    assert!(validate_namespace("").is_err());
    assert!(validate_namespace("Appearance").is_err()); // 大写
    assert!(validate_namespace("appearance!").is_err()); // 特殊字符
    assert!(validate_namespace("1appearance").is_err()); // 数字开头
    assert!(validate_namespace(&"a".repeat(65)).is_err()); // 过长
}

#[test]
fn test_validate_config_key_valid() {
    assert!(validate_config_key("theme").is_ok());
    assert!(validate_config_key("font_family").is_ok());
    assert!(validate_config_key("email-enabled").is_ok());
    assert!(validate_config_key("a").is_ok());
}

#[test]
fn test_validate_config_key_invalid() {
    assert!(validate_config_key("").is_err());
    assert!(validate_config_key("Theme").is_err()); // 大写
    assert!(validate_config_key("theme!").is_err()); // 特殊字符
    assert!(validate_config_key(&"a".repeat(129)).is_err()); // 过长
}

#[test]
fn test_validate_value_size_within_limit() {
    let small_value = serde_json::json!({"theme": "dark"});
    assert!(validate_value_size(&small_value).is_ok());
}

#[test]
fn test_validate_value_size_exceeds_limit() {
    // 64KB = 65536 bytes,构造超过限制的值
    let large_string = "x".repeat(70000);
    let large_value = serde_json::json!({"data": large_string});
    assert!(validate_value_size(&large_value).is_err());
}

#[test]
fn test_config_scope_variants() {
    assert_eq!(ConfigScope::User.as_str(), "user");
    assert_eq!(ConfigScope::Tenant.as_str(), "tenant");
    assert_eq!(ConfigScope::System.as_str(), "system");
}

#[test]
fn test_config_value_type_variants() {
    assert_eq!(ConfigValueType::String.as_str(), "string");
    assert_eq!(ConfigValueType::Number.as_str(), "number");
    assert_eq!(ConfigValueType::Boolean.as_str(), "boolean");
    assert_eq!(ConfigValueType::Object.as_str(), "object");
    assert_eq!(ConfigValueType::Array.as_str(), "array");
    assert_eq!(ConfigValueType::I18n.as_str(), "i18n");
}
