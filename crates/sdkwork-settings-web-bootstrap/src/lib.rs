//! SDKWork Settings HTTP 服务引导模块。
//!
//! 本 crate 负责 Settings 服务的 Web 框架引导与路由装配,接入 `sdkwork-web-framework`。
//! 通过 [`service_router`] 挂载 `/healthz`、`/readyz`、`/livez`、`/metrics` 等基础设施探测,
//! 通过 [`create_settings_router`] 装配 app-api 与 backend-api 路由。
//!
//! 上游服务进程(如 `sdkwork-settings-standalone-gateway`)
//! 通过本 crate 完成引导,而非自行构造拦截链。

use std::sync::Arc;

use axum::Router;
use sdkwork_iam_web_adapter::iam_web_request_context_resolver_from_env;
use sdkwork_routes_settings_app_api::routes as app_api_routes;
use sdkwork_routes_settings_backend_api::routes as backend_api_routes;
use sdkwork_settings_service_host::SettingsServiceHost;
use sdkwork_web_axum::{WebFrameworkLayer, with_web_request_context};
use sdkwork_web_bootstrap::{ServiceRouterConfig, service_router};
use sdkwork_web_core::WebRequestContextProfile;
use tracing::info;

/// Settings HTTP 服务的应用状态。
///
/// 持有 [`SettingsServiceHost`] 的共享句柄,供路由 handler 通过 `State` 提取器访问。
/// 之所以包裹 `Arc`,是因为 axum 要求 Router 状态满足 `Clone + Send + Sync + 'static`。
#[derive(Clone)]
pub struct SettingsAppState {
    /// 配置中心服务宿主,聚合领域服务与数据库访问。
    pub host: Arc<SettingsServiceHost>,
}

impl SettingsAppState {
    /// 构造应用状态。
    pub fn new(host: Arc<SettingsServiceHost>) -> Self {
        Self { host }
    }

    /// 获取内部服务宿主句柄。
    pub fn host(&self) -> &Arc<SettingsServiceHost> {
        &self.host
    }
}

impl From<Arc<SettingsServiceHost>> for SettingsAppState {
    fn from(host: Arc<SettingsServiceHost>) -> Self {
        Self::new(host)
    }
}

/// Settings HTTP 服务的基础设施路由配置。
///
/// 使用默认配置,生产环境可通过 `.with_readiness_check(...)` 等链式 API 追加依赖探测。
pub fn settings_service_router_config() -> ServiceRouterConfig {
    ServiceRouterConfig::default()
}

/// 挂载 Settings HTTP 服务的基础设施探测路由(`/healthz`、`/readyz`、`/livez`、`/metrics`)。
///
/// 由 `sdkwork-web-bootstrap::service_router` 提供标准实现,业务层不应自行重复挂载。
pub fn mount_settings_infra_routes(router: Router, config: ServiceRouterConfig) -> Router {
    info!(target: "sdkwork_settings::web_bootstrap", "mounting settings infra routes");
    service_router(router, config)
}

/// 装配 Settings 服务的业务路由(app-api + backend-api)。
///
/// 该函数:
/// 1. 调用 [`app_api_routes`] 与 [`backend_api_routes`] 装配业务路由
/// 2. 通过 [`Arc<SettingsServiceHost>`] 作为 Router 状态
/// 3. 返回的 `Router<()>` 已附带状态,可直接与基础设施路由合并
///
/// # 调用示例
///
/// ```ignore
/// let host = Arc::new(SettingsServiceHost::from_env().await?);
/// let state = SettingsAppState::new(host.clone());
/// let business_router = create_settings_router(state);
/// let router = mount_settings_infra_routes(business_router, settings_service_router_config());
/// ```
pub fn create_settings_router(state: SettingsAppState) -> Router {
    info!(target: "sdkwork_settings::web_bootstrap", "assembling settings business routes");

    let host = state.host.clone();
    Router::new()
        .merge(app_api_routes())
        .merge(backend_api_routes())
        .with_state(host)
}

/// 使用 IAM Web 适配器将业务路由接入标准 SDKWork 拦截链。
///
/// 该函数从环境变量解析 IAM 解析器,构造 [`WebFrameworkLayer`] 并附加 Settings 服务的
/// 路由前缀(`/settings/v1/app-api`、`/settings/v1/backend-api`)。返回的 Router 已具备
/// `WebRequestContext` 注入、租户隔离等能力。
pub async fn wrap_settings_router_with_framework(router: Router) -> Router {
    let resolver = iam_web_request_context_resolver_from_env().await;
    let layer = WebFrameworkLayer::new(resolver).with_profile(WebRequestContextProfile {
        app_api_prefix: sdkwork_routes_settings_app_api::APP_API_PREFIX.to_owned(),
        backend_api_prefix: sdkwork_routes_settings_backend_api::BACKEND_API_PREFIX.to_owned(),
        open_api_prefixes: Vec::new(),
        public_path_prefixes: sdkwork_web_bootstrap::infra_public_path_prefixes(),
        gateway_api_prefixes: Vec::new(),
        ..WebRequestContextProfile::default()
    });

    info!(target: "sdkwork_settings::web_bootstrap", "wrapped settings router with web framework layer");
    with_web_request_context(router, layer)
}
