//! SDKWork Settings backend-api 路由 crate,处理租户配置与系统设置管理。
//!
//! 路由前缀: [`BACKEND_API_PREFIX`] = `/settings/v1/backend-api`
//!
//! # 路由清单
//!
//! | Method | Path                                  | Handler              | 说明             |
//! |--------|---------------------------------------|----------------------|------------------|
//! | GET    | `/tenant-configs`                     | list_tenant_configs  | 查询租户配置列表 |
//! | GET    | `/tenant-configs/{namespace}/{key}`   | get_tenant_config    | 查询单个租户配置 |
//! | PUT    | `/tenant-configs/{namespace}/{key}`   | update_tenant_config | 更新租户配置     |
//! | GET    | `/system-settings`                    | list_system_settings | 查询系统设置列表 |
//! | GET    | `/system-settings/{namespace}/{key}`  | get_system_setting   | 查询单个系统设置 |
//! | PUT    | `/system-settings/{namespace}/{key}`  | update_system_setting| 更新系统设置     |
//! | GET    | `/revisions`                          | list_revisions       | 查询配置变更修订 |
//!
//! # 权限模型
//!
//! - 租户配置: `iam:tenant:admin`
//! - 系统设置: `iam:system:admin`
//! - 配置修订: `iam:tenant:admin`
//!
//! # 响应格式
//!
//! 成功: `SdkWorkApiResponse` 信封 `{ "code": 0, "data": <payload>, "traceId": "<server-uuid>" }`
//! 错误: `application/problem+json` (`ProblemDetail`) 含数字 `code` 和 `traceId`。

use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderName, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use sdkwork_settings_contract::{
    ConfigRevision, SettingsError, validate_config_key, validate_namespace,
};
use sdkwork_settings_service_host::{ServiceError, SettingsServiceHost};
use sdkwork_utils_rust::{
    PageInfo, PageMode, SdkWorkApiResponse, SdkWorkPageData, SdkWorkProblemDetail,
    SdkWorkProblemRouting, SdkWorkResourceData, SdkWorkResultCode,
};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// backend-api 路由前缀常量。
pub const BACKEND_API_PREFIX: &str = "/settings/v1/backend-api";

/// IAM 权限常量。
const IAM_TENANT_ADMIN: &str = "iam:tenant:admin";
const IAM_SYSTEM_ADMIN: &str = "iam:system:admin";

/// 装配 backend-api 路由,返回 `Router<Arc<SettingsServiceHost>>`。
///
/// 调用方负责通过 `.with_state(host)` 附带状态,以及通过
/// `sdkwork_settings_web_bootstrap::wrap_settings_router_with_framework` 接入拦截链。
pub fn routes() -> Router<Arc<SettingsServiceHost>> {
    Router::new()
        .route("/tenant-configs", get(list_tenant_configs))
        .route(
            "/tenant-configs/{namespace}/{key}",
            get(get_tenant_config).put(update_tenant_config),
        )
        .route("/system-settings", get(list_system_settings))
        .route(
            "/system-settings/{namespace}/{key}",
            get(get_system_setting).put(update_system_setting),
        )
        .route("/revisions", get(list_revisions))
}

// ===== 查询参数 =====

/// 租户配置列表查询参数。
#[derive(Debug, Deserialize)]
pub struct TenantConfigListQuery {
    pub namespace: Option<String>,
    pub page: Option<i32>,
        pub page_size: Option<i32>,
}

/// 系统设置列表查询参数。
#[derive(Debug, Deserialize)]
pub struct SystemSettingListQuery {
    pub namespace: Option<String>,
    pub page: Option<i32>,
        pub page_size: Option<i32>,
}

/// 配置修订列表查询参数。
#[derive(Debug, Deserialize)]
pub struct RevisionListQuery {
    pub namespace: Option<String>,
    #[serde(rename = "configType", alias = "config_type")]
    pub config_type: Option<String>,
    pub page: Option<i32>,
        pub page_size: Option<i32>,
}

// ===== 请求体 =====

/// 更新租户配置请求体。
#[derive(Debug, Deserialize)]
pub struct UpdateTenantConfigRequest {
    pub value: serde_json::Value,
    #[serde(rename = "valueType", alias = "value_type")]
    pub value_type: String,
}

/// 更新系统设置请求体。
#[derive(Debug, Deserialize)]
pub struct UpdateSystemSettingRequest {
    pub value: serde_json::Value,
    #[serde(rename = "valueType", alias = "value_type")]
    pub value_type: String,
    pub scope: String,
    #[serde(rename = "scopeValue", alias = "scope_value")]
    pub scope_value: Option<String>,
}

// ===== Handler =====

/// 查询租户配置列表。
///
/// 需要查询参数 `namespace`(必填),用于限定命名空间范围。
pub async fn list_tenant_configs(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Query(query): Query<TenantConfigListQuery>,
) -> Response {
    let (tenant_id, _operator_id) = match require_tenant_admin(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };
    let namespace = match query.namespace.as_deref() {
        Some(ns) if !ns.is_empty() => ns,
        _ => return validation(&ctx, "namespace query parameter is required"),
    };
    if let Err(error) = validate_namespace(namespace) {
        return validation(&ctx, validation_message(error));
    }

    info!(target: "sdkwork_settings::backend_api", tenant_id, namespace, "list tenant configs");

    match host.list_tenant_configs(tenant_id, namespace).await {
        Ok(items) => success_items(&ctx, items, query.page, query.page_size),
        Err(error) => map_service_error(&ctx, error),
    }
}

/// 查询单个租户配置。
pub async fn get_tenant_config(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Path((namespace, key)): Path<(String, String)>,
) -> Response {
    let (tenant_id, _operator_id) = match require_tenant_admin(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };
    if let Err(error) = validate_namespace(&namespace) {
        return validation(&ctx, validation_message(error));
    }
    if let Err(error) = validate_config_key(&key) {
        return validation(&ctx, validation_message(error));
    }

    info!(target: "sdkwork_settings::backend_api", tenant_id, namespace = %namespace, key = %key, "get tenant config");

    match host.get_tenant_config(tenant_id, &namespace, &key).await {
        Ok(Some(item)) => success_item(&ctx, item),
        Ok(None) => not_found(&ctx, "tenant config was not found"),
        Err(error) => map_service_error(&ctx, error),
    }
}

/// 更新租户配置(新建或覆盖)。
pub async fn update_tenant_config(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Path((namespace, key)): Path<(String, String)>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    let (tenant_id, operator_id) = match require_tenant_admin(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };
    if let Err(error) = validate_namespace(&namespace) {
        return validation(&ctx, validation_message(error));
    }
    if let Err(error) = validate_config_key(&key) {
        return validation(&ctx, validation_message(error));
    }
    let request: UpdateTenantConfigRequest = match serde_json::from_value(body) {
        Ok(value) => value,
        Err(error) => return validation(&ctx, format!("invalid request body: {error}")),
    };

    info!(target: "sdkwork_settings::backend_api", tenant_id, namespace = %namespace, key = %key, operator_id, "update tenant config");

    match host
        .update_tenant_config(
            tenant_id,
            &namespace,
            &key,
            request.value,
            request.value_type,
            operator_id,
        )
        .await
    {
        Ok(item) => success_item(&ctx, item),
        Err(error) => map_service_error(&ctx, error),
    }
}

/// 查询系统设置列表。
///
/// 需要查询参数 `namespace`(必填),用于限定命名空间范围。
pub async fn list_system_settings(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Query(query): Query<SystemSettingListQuery>,
) -> Response {
    let _operator_id = match require_system_admin(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };
    let namespace = match query.namespace.as_deref() {
        Some(ns) if !ns.is_empty() => ns,
        _ => return validation(&ctx, "namespace query parameter is required"),
    };
    if let Err(error) = validate_namespace(namespace) {
        return validation(&ctx, validation_message(error));
    }

    info!(target: "sdkwork_settings::backend_api", namespace, "list system settings");

    match host.list_system_settings(namespace).await {
        Ok(items) => success_items(&ctx, items, query.page, query.page_size),
        Err(error) => map_service_error(&ctx, error),
    }
}

/// 查询单个系统设置。
pub async fn get_system_setting(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Path((namespace, key)): Path<(String, String)>,
) -> Response {
    let _operator_id = match require_system_admin(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };
    if let Err(error) = validate_namespace(&namespace) {
        return validation(&ctx, validation_message(error));
    }
    if let Err(error) = validate_config_key(&key) {
        return validation(&ctx, validation_message(error));
    }

    info!(target: "sdkwork_settings::backend_api", namespace = %namespace, key = %key, "get system setting");

    match host.get_system_setting(&namespace, &key).await {
        Ok(Some(item)) => success_item(&ctx, item),
        Ok(None) => not_found(&ctx, "system setting was not found"),
        Err(error) => map_service_error(&ctx, error),
    }
}

/// 更新系统设置(新建或覆盖)。
pub async fn update_system_setting(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Path((namespace, key)): Path<(String, String)>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    let operator_id = match require_system_admin(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };
    if let Err(error) = validate_namespace(&namespace) {
        return validation(&ctx, validation_message(error));
    }
    if let Err(error) = validate_config_key(&key) {
        return validation(&ctx, validation_message(error));
    }
    let request: UpdateSystemSettingRequest = match serde_json::from_value(body) {
        Ok(value) => value,
        Err(error) => return validation(&ctx, format!("invalid request body: {error}")),
    };

    info!(target: "sdkwork_settings::backend_api", namespace = %namespace, key = %key, operator_id, scope = %request.scope, "update system setting");

    match host
        .update_system_setting(
            &namespace,
            &key,
            request.value,
            request.value_type,
            request.scope,
            request.scope_value,
            operator_id,
        )
        .await
    {
        Ok(item) => success_item(&ctx, item),
        Err(error) => map_service_error(&ctx, error),
    }
}

/// 查询配置变更修订记录。
///
/// 当前服务层暂未实现 revisions 查询,返回空列表骨架,后续扩展服务层后即可生效。
pub async fn list_revisions(
    ctx: WebRequestContext,
    State(_host): State<Arc<SettingsServiceHost>>,
    Query(query): Query<RevisionListQuery>,
) -> Response {
    let (tenant_id, _operator_id) = match require_tenant_admin(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };

    info!(target: "sdkwork_settings::backend_api", tenant_id, "list revisions");

    // 服务层暂未实现 revisions 查询,返回空列表骨架
    let items: Vec<ConfigRevision> = Vec::new();
    success_items(&ctx, items, query.page, query.page_size)
}

// ===== 鉴权辅助 =====

/// 要求 `iam:tenant:admin` 权限,返回 `(tenant_id, operator_id)`。
///
/// `tenant_id` 与 `operator_id`(用户 ID)均从 `WebRequestContext` 的 principal 提取,
/// 返回的 `&str` 生命周期与 `ctx` 的借用一致,与服务层 `&str` 签名直接对齐。
fn require_tenant_admin(ctx: &WebRequestContext) -> Result<(&str, &str), Box<Response>> {
    let principal = match ctx.principal() {
        Some(principal) => principal,
        None => {
            warn!(target: "sdkwork_settings::backend_api", "missing principal in request context");
            return Err(Box::new(unauthorized(
                ctx,
                "authenticated principal is required",
            )));
        }
    };
    if !ctx.has_permission(IAM_TENANT_ADMIN) {
        warn!(target: "sdkwork_settings::backend_api", permission = IAM_TENANT_ADMIN, "missing tenant admin permission");
        return Err(Box::new(forbidden(
            ctx,
            format!("missing required permission: {IAM_TENANT_ADMIN}"),
        )));
    }
    let tenant_id = principal.tenant_id();
    let operator_id = principal.user_id();
    if tenant_id.is_empty() {
        return Err(Box::new(validation(ctx, "tenant_id must not be empty")));
    }
    if operator_id.is_empty() {
        return Err(Box::new(validation(ctx, "operator_id must not be empty")));
    }
    Ok((tenant_id, operator_id))
}

/// 要求 `iam:system:admin` 权限,返回 `operator_id`。
///
/// 系统设置为平台级数据,不限定租户范围,仅需操作者 ID 用于审计。
fn require_system_admin(ctx: &WebRequestContext) -> Result<&str, Box<Response>> {
    let principal = match ctx.principal() {
        Some(principal) => principal,
        None => {
            warn!(target: "sdkwork_settings::backend_api", "missing principal in request context");
            return Err(Box::new(unauthorized(
                ctx,
                "authenticated principal is required",
            )));
        }
    };
    if !ctx.has_permission(IAM_SYSTEM_ADMIN) {
        warn!(target: "sdkwork_settings::backend_api", permission = IAM_SYSTEM_ADMIN, "missing system admin permission");
        return Err(Box::new(forbidden(
            ctx,
            format!("missing required permission: {IAM_SYSTEM_ADMIN}"),
        )));
    }
    let operator_id = principal.user_id();
    if operator_id.is_empty() {
        return Err(Box::new(validation(ctx, "operator_id must not be empty")));
    }
    Ok(operator_id)
}

/// 将契约层 `SettingsError` 转换为可读消息(校验错误提取内部消息,其他使用 Display)。
fn validation_message(error: SettingsError) -> String {
    match error {
        SettingsError::Validation(message) => message,
        other => other.to_string(),
    }
}

// ===== 响应辅助 =====

fn resolve_trace_id(ctx: &WebRequestContext) -> String {
    ctx.resolved_trace_id()
}

fn problem_routing(ctx: &WebRequestContext) -> SdkWorkProblemRouting {
    ctx.problem_routing()
}

/// 构造单项资源成功响应(`data.item` 信封)。
fn success_item<T: Serialize>(ctx: &WebRequestContext, item: T) -> Response {
    let trace_id = resolve_trace_id(ctx);
    let envelope = SdkWorkApiResponse::success(SdkWorkResourceData { item }, trace_id.clone());
    attach_trace_header(Json(envelope).into_response(), &trace_id)
}

/// 构造列表成功响应(`data.items` + `data.pageInfo` 信封)。
fn success_items<T: Serialize>(
    ctx: &WebRequestContext,
    items: Vec<T>,
    page: Option<i32>,
    page_size: Option<i32>,
) -> Response {
    let trace_id = resolve_trace_id(ctx);
    let envelope = SdkWorkApiResponse::success(
        SdkWorkPageData {
            items,
            page_info: PageInfo {
                mode: PageMode::Offset,
                page,
                page_size,
                total_items: None,
                total_pages: None,
                next_cursor: None,
                has_more: None,
            },
        },
        trace_id.clone(),
    );
    attach_trace_header(Json(envelope).into_response(), &trace_id)
}

/// 构造校验失败 ProblemDetail 响应(400)。
fn validation(ctx: &WebRequestContext, detail: impl Into<String>) -> Response {
    problem_for_context(
        ctx,
        StatusCode::BAD_REQUEST,
        SdkWorkResultCode::ValidationError,
        detail,
    )
}

/// 构造资源不存在 ProblemDetail 响应(404)。
fn not_found(ctx: &WebRequestContext, detail: impl Into<String>) -> Response {
    problem_for_context(
        ctx,
        StatusCode::NOT_FOUND,
        SdkWorkResultCode::NotFound,
        detail,
    )
}

/// 构造未认证 ProblemDetail 响应(401)。
fn unauthorized(ctx: &WebRequestContext, detail: impl Into<String>) -> Response {
    problem_for_context(
        ctx,
        StatusCode::UNAUTHORIZED,
        SdkWorkResultCode::AuthenticationRequired,
        detail,
    )
}

/// 构造权限不足 ProblemDetail 响应(403)。
fn forbidden(ctx: &WebRequestContext, detail: impl Into<String>) -> Response {
    problem_for_context(
        ctx,
        StatusCode::FORBIDDEN,
        SdkWorkResultCode::PermissionRequired,
        detail,
    )
}

/// 将服务层 `ServiceError` 映射为 ProblemDetail 响应。
fn map_service_error(ctx: &WebRequestContext, error: ServiceError) -> Response {
    let (status, result_code, detail) = match error {
        ServiceError::Validation(message) => (
            StatusCode::BAD_REQUEST,
            SdkWorkResultCode::ValidationError,
            message,
        ),
        ServiceError::NotFound => (
            StatusCode::NOT_FOUND,
            SdkWorkResultCode::NotFound,
            "resource not found".to_owned(),
        ),
        ServiceError::Unauthorized => (
            StatusCode::UNAUTHORIZED,
            SdkWorkResultCode::AuthenticationRequired,
            "unauthorized".to_owned(),
        ),
        ServiceError::Forbidden => (
            StatusCode::FORBIDDEN,
            SdkWorkResultCode::PermissionRequired,
            "forbidden".to_owned(),
        ),
        ServiceError::Database(message) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            SdkWorkResultCode::InternalError,
            message,
        ),
        ServiceError::Internal(message) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            SdkWorkResultCode::InternalError,
            message,
        ),
    };
    problem_for_context(ctx, status, result_code, detail)
}

/// 构造 ProblemDetail 响应,附带 `x-sdkwork-trace-id` 响应头并覆盖 HTTP 状态码。
fn problem_for_context(
    ctx: &WebRequestContext,
    status: StatusCode,
    result_code: SdkWorkResultCode,
    detail: impl Into<String>,
) -> Response {
    let trace_id = resolve_trace_id(ctx);
    let problem = SdkWorkProblemDetail::platform_enriched(
        result_code,
        detail,
        trace_id.clone(),
        problem_routing(ctx),
    );
    attach_trace_header(Json(problem).into_response(), &trace_id).with_status(status)
}

/// 为响应附加 `x-sdkwork-trace-id` 响应头。
fn attach_trace_header(response: Response, trace_id: &str) -> Response {
    let mut response = response;
    if let Ok(value) = HeaderValue::from_str(trace_id) {
        response
            .headers_mut()
            .insert(HeaderName::from_static("x-sdkwork-trace-id"), value);
    }
    response
}

/// 辅助 trait:覆盖 `Response` 的 HTTP 状态码。
///
/// `axum::Json(problem).into_response()` 默认产生 200 状态码,ProblemDetail 需要使用
/// 正确的 4xx/5xx 状态码,因此通过本 trait 在序列化后覆写状态码。
trait WithStatusExt {
    fn with_status(self, status: StatusCode) -> Response;
}

impl WithStatusExt for Response {
    fn with_status(mut self, status: StatusCode) -> Response {
        *self.status_mut() = status;
        self
    }
}

pub fn gateway_mount(host: Arc<SettingsServiceHost>) -> axum::Router {
    routes().with_state(host)
}
