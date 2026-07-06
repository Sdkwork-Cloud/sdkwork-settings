//! Settings 领域数据传输对象 (DTO)。
//!
//! 所有结构体均使用 `#[serde(rename_all = "camelCase")]` 以匹配前后端 JSON 命名约定。

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 用户偏好配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreference {
    pub id: i64,
    pub tenant_id: String,
    pub user_id: String,
    pub namespace: String,
    pub preference_key: String,
    pub preference_value: Value,
    pub value_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

/// 租户配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantConfig {
    pub id: i64,
    pub tenant_id: String,
    pub namespace: String,
    pub config_key: String,
    pub config_value: Value,
    pub value_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_by: String,
}

/// 系统级配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemSetting {
    pub id: i64,
    pub namespace: String,
    pub setting_key: String,
    pub setting_value: Value,
    pub value_type: String,
    pub scope: String,
    pub scope_value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_by: String,
}

/// 配置变更修订记录,用于审计追溯。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigRevision {
    pub id: i64,
    pub tenant_id: String,
    pub config_type: String,
    pub config_id: String,
    pub namespace: String,
    pub config_key: String,
    pub old_value: Option<Value>,
    pub new_value: Option<Value>,
    pub operation: String,
    pub operator_id: String,
    pub operator_ip: String,
    pub created_at: DateTime<Utc>,
}

/// 批量更新单条目。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateItem {
    pub namespace: String,
    pub preference_key: String,
    pub value: Value,
    pub value_type: String,
}

/// 更新用户偏好请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserPreferenceRequest {
    pub value: Value,
    pub value_type: String,
}

/// 批量更新用户偏好请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateUserPreferenceRequest {
    pub updates: Vec<BatchUpdateItem>,
}

/// 用户偏好列表查询参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferenceListQuery {
    pub namespace: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
