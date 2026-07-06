//! SDKWork Settings 网关装配清单。
//!
//! Phase 1 采用静态服务发现拓扑,路由清单由本 crate 静态声明。
//! 后续接入 `sdkwork-discovery` / `sdkwork-rpc-framework` 时,可扩展为
//! 动态服务发现(参见 ADR-0002)。

use serde::{Deserialize, Serialize};

/// 网关装配清单。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayAssemblyManifest {
    /// 清单类型标识。
    pub kind: String,
    /// 清单 schema 版本。
    pub schema_version: u32,
    /// 应用编码。
    pub application_code: String,
    /// crate 包名。
    pub package_name: String,
    /// 路由 crate 列表。
    pub route_crates: Vec<RouteCrateEntry>,
    /// 服务发现配置(Phase 1 为静态)。
    pub service_discovery: ServiceDiscoveryConfig,
}

/// 路由 crate 条目。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteCrateEntry {
    /// crate 包名(kebab-case)。
    pub package_name: String,
    /// lib 名(snake_case)。
    pub lib_name: String,
    /// 路由面(open-api / app-api / backend-api)。
    pub surface: String,
    /// 路径前缀,`None` 表示由路由 crate 自行决定。
    pub path_prefix: Option<String>,
    /// 挂载顺序。
    pub mount_order: u32,
}

/// 服务发现配置。
///
/// Phase 1 为静态拓扑,通过环境变量配置上游地址。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceDiscoveryConfig {
    /// 发现模式:`static` 或 `dynamic`。
    pub mode: String,
    /// 静态上游服务列表。
    pub upstreams: Vec<UpstreamEntry>,
}

/// 上游服务条目。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpstreamEntry {
    /// 服务名称。
    pub name: String,
    /// 服务基础 URL(可由环境变量覆盖)。
    pub base_url: String,
    /// 对应的环境变量名,运行时用于覆盖 `base_url`。
    pub url_env: String,
}

/// 返回 Settings 网关的默认装配清单。
///
/// Phase 1 包含 app-api 与 backend-api 两个路由面,服务发现采用静态拓扑。
pub fn default_manifest() -> GatewayAssemblyManifest {
    GatewayAssemblyManifest {
        kind: "sdkwork.gateway.assembly".to_string(),
        schema_version: 1,
        application_code: "settings".to_string(),
        package_name: "sdkwork-settings-gateway-assembly".to_string(),
        route_crates: vec![
            RouteCrateEntry {
                package_name: "sdkwork-routes-settings-app-api".to_string(),
                lib_name: "sdkwork_routes_settings_app_api".to_string(),
                surface: "app-api".to_string(),
                path_prefix: None,
                mount_order: 0,
            },
            RouteCrateEntry {
                package_name: "sdkwork-routes-settings-backend-api".to_string(),
                lib_name: "sdkwork_routes_settings_backend_api".to_string(),
                surface: "backend-api".to_string(),
                path_prefix: None,
                mount_order: 1,
            },
        ],
        service_discovery: ServiceDiscoveryConfig {
            mode: "static".to_string(),
            upstreams: vec![UpstreamEntry {
                name: "settings-api-server".to_string(),
                base_url: "http://127.0.0.1:8080".to_string(),
                url_env: "SDKWORK_SETTINGS_API_SERVER_URL".to_string(),
            }],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_manifest_serializes() {
        let manifest = default_manifest();
        let json = serde_json::to_string(&manifest).expect("序列化清单");
        assert!(json.contains("settings"));
        assert_eq!(manifest.route_crates.len(), 2);
        assert_eq!(manifest.service_discovery.mode, "static");
    }
}
