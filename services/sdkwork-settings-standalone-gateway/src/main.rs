//! SDKWork Settings standalone 网关进程入口。
//!
//! standalone 部署 profile 下,网关与 API 服务合并为单进程,直接提供:
//! 1. 基础设施探测(`/healthz`、`/readyz`、`/livez`、`/metrics`)
//! 2. app-api 业务路由(`/settings/v1/app-api/*`)
//! 3. backend-api 业务路由(`/settings/v1/backend-api/*`)
//!
//! 与 `sdkwork-settings-api-server` 的区别:standalone-gateway 面向个人/小团队部署,
//! 单进程承载全部职责;api-server 面向 cloud profile,可与独立网关分离部署。
//!
//! # 环境变量
//!
//! - `SETTINGS_DATABASE_URL`: 数据库连接字符串
//! - `SDKWORK_SETTINGS_GATEWAY_BIND`: 监听地址(默认 `0.0.0.0:8080`)

use std::sync::Arc;

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::create_pool_from_config;
use sdkwork_settings_database_host::bootstrap_settings_database;
use sdkwork_settings_service_host::SettingsServiceHost;
use sdkwork_settings_web_bootstrap::{
    SettingsAppState, create_settings_router, mount_settings_infra_routes,
    settings_service_router_config, wrap_settings_router_with_framework,
};
use sdkwork_web_bootstrap::serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!(target: "sdkwork_settings::standalone_gateway", "starting sdkwork-settings-standalone-gateway");

    // 引导数据库
    let db_config = DatabaseConfig::from_env("SETTINGS")
        .map_err(|e| format!("读取 Settings 数据库配置失败: {e}"))?;
    let pool = create_pool_from_config(db_config)
        .await
        .map_err(|e| format!("创建 Settings 数据库连接池失败: {e}"))?;
    let _db_host = bootstrap_settings_database(pool.clone())
        .await
        .map_err(|e| format!("Settings 数据库引导失败: {e}"))?;
    tracing::info!(target: "sdkwork_settings::standalone_gateway", "database bootstrap completed");

    // 构造服务宿主与状态
    let host = Arc::new(SettingsServiceHost::new(pool));
    let state = SettingsAppState::new(host);

    // 装配路由(单进程合并基础设施 + 业务路由)
    let business_router = create_settings_router(state);
    let router = wrap_settings_router_with_framework(business_router).await;
    let router = mount_settings_infra_routes(router, settings_service_router_config());
    tracing::info!(target: "sdkwork_settings::standalone_gateway", "router assembled (standalone single-process)");

    // 启动 HTTP 服务
    let bind = std::env::var("SDKWORK_SETTINGS_GATEWAY_BIND")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_owned());
    tracing::info!(target: "sdkwork_settings::standalone_gateway", bind = %bind, "listening");

    let addr: std::net::SocketAddr = bind.parse()?;
    serve(router, addr).await?;

    tracing::info!(target: "sdkwork_settings::standalone_gateway", "gateway shutdown complete");
    Ok(())
}
