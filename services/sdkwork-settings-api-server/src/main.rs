//! SDKWork Settings API 服务器进程入口。
//!
//! 本二进制是 Settings 配置中心的主 HTTP 服务进程,负责:
//! 1. 通过 `sdkwork-settings-database-host` 引导数据库生命周期(迁移、种子)
//! 2. 通过 `sdkwork-settings-service-host` 构造进程内服务容器
//! 3. 通过 `sdkwork-settings-web-bootstrap` 装配 Web 框架与路由
//! 4. 绑定监听端口并提供 HTTP 服务
//!
//! # 环境变量
//!
//! - `SETTINGS_DATABASE_URL`: 数据库连接字符串(PostgreSQL 或 SQLite)
//! - `SETTINGS_APP_ROOT`: 应用根目录(用于定位 database/ 资产)
//! - `SDKWORK_SETTINGS_API_BIND`: 监听地址(默认 `0.0.0.0:8080`)

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
    // 加载 .env 文件(开发环境)
    let _ = dotenvy::dotenv();

    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!(target: "sdkwork_settings::api_server", "starting sdkwork-settings-api-server");

    // 1. 引导数据库
    let db_config = DatabaseConfig::from_env("SETTINGS")
        .map_err(|e| format!("读取 Settings 数据库配置失败: {e}"))?;
    let pool = create_pool_from_config(db_config)
        .await
        .map_err(|e| format!("创建 Settings 数据库连接池失败: {e}"))?;
    let _db_host = bootstrap_settings_database(pool.clone())
        .await
        .map_err(|e| format!("Settings 数据库引导失败: {e}"))?;
    tracing::info!(target: "sdkwork_settings::api_server", "database bootstrap completed");

    // 2. 构造服务宿主
    let host = Arc::new(SettingsServiceHost::new(pool));
    let state = SettingsAppState::new(host);
    tracing::info!(target: "sdkwork_settings::api_server", "service host initialized");

    // 3. 装配路由
    let business_router = create_settings_router(state);
    let router = wrap_settings_router_with_framework(business_router).await;
    let router = mount_settings_infra_routes(router, settings_service_router_config());
    tracing::info!(target: "sdkwork_settings::api_server", "router assembled");

    // 4. 启动 HTTP 服务
    let bind =
        std::env::var("SDKWORK_SETTINGS_API_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_owned());
    tracing::info!(target: "sdkwork_settings::api_server", bind = %bind, "listening");

    let addr: std::net::SocketAddr = bind.parse()?;
    serve(router, addr).await?;

    tracing::info!(target: "sdkwork_settings::api_server", "server shutdown complete");
    Ok(())
}
