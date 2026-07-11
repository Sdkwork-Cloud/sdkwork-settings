//! Gateway bootstrap for sdkwork-settings.

use axum::Router;
use sdkwork_settings_web_bootstrap::{
    SettingsAppState, create_settings_router, wrap_settings_router_with_framework,
};

pub struct ApplicationAssembly {
    pub router: Router,
}

pub async fn assemble_application_router(state: SettingsAppState) -> ApplicationAssembly {
    let router = create_settings_router(state);
    let router = wrap_settings_router_with_framework(router).await;
    ApplicationAssembly { router }
}
