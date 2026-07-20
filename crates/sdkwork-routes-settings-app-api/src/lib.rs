//! SDKWork Settings app-api 路由 crate,处理用户偏好管理。
//!
//! 路由前缀: [`APP_API_PREFIX`] = `/settings/v1/app-api`
//!
//! # 路由清单
//!
//! | Method | Path                            | Handler                  | 说明             |
//! |--------|---------------------------------|--------------------------|------------------|
//! | GET    | `/preferences`                  | list_user_preferences    | 查询用户偏好列表 |
//! | GET    | `/preferences/{namespace}`      | get_namespace_preferences| 查询指定 namespace 偏好 |
//! | GET    | `/preferences/{namespace}/{key}`| get_user_preference      | 查询单个偏好     |
//! | PUT    | `/preferences/{namespace}/{key}`| update_user_preference   | 更新单个偏好     |
//! | DELETE | `/preferences/{namespace}/{key}`| delete_user_preference   | 删除偏好(恢复默认) |
//! | POST   | `/preferences:batchUpdate`      | batch_update_preferences | 批量更新偏好     |
//!
//! # 权限模型
//!
//! 用户偏好为自服务资源,权限: `iam:self`。
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
use axum::routing::{get, post};
use sdkwork_settings_contract::{
    BatchUpdateUserPreferenceRequest, SettingsError, UpdateUserPreferenceRequest,
    validate_config_key, validate_namespace,
};
use sdkwork_settings_service_host::{ServiceError, SettingsServiceHost};
use sdkwork_utils_rust::{
    PageInfo, PageMode, SdkWorkApiResponse, SdkWorkPageData, SdkWorkProblemDetail,
    SdkWorkProblemRouting, SdkWorkResourceData, SdkWorkResultCode,
};
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// app-api 路由前缀常量。
pub const APP_API_PREFIX: &str = "/settings/v1/app-api";

/// IAM 权限常量:用户自服务。
const IAM_SELF: &str = "iam:self";

/// 装配 app-api 路由,返回 `Router<Arc<SettingsServiceHost>>`。
///
/// 调用方负责通过 `.with_state(host)` 附带状态,以及通过
/// `sdkwork_settings_web_bootstrap::wrap_settings_router_with_framework` 接入拦截链。
pub fn routes() -> Router<Arc<SettingsServiceHost>> {
    Router::new()
        .route("/preferences", get(list_user_preferences))
        .route("/preferences/{namespace}", get(get_namespace_preferences))
        .route(
            "/preferences/{namespace}/{key}",
            get(get_user_preference)
                .put(update_user_preference)
                .delete(delete_user_preference),
        )
        .route("/preferences:batchUpdate", post(batch_update_preferences))
}

// ===== 查询参数 =====

/// 用户偏好列表查询参数。
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub namespace: Option<String>,
    pub page: Option<i32>,
        pub page_size: Option<i32>,
}

// ===== Handler =====

/// 查询当前用户偏好列表。
///
/// 需要查询参数 `namespace`(必填),用于限定命名空间范围。
pub async fn list_user_preferences(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Query(query): Query<ListQuery>,
) -> Response {
    let (tenant_id, user_id) = match require_self(&ctx) {
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

    info!(target: "sdkwork_settings::app_api", tenant_id, user_id, namespace, "list user preferences");

    match host
        .list_user_preferences(tenant_id, user_id, namespace)
        .await
    {
        Ok(items) => success_items(&ctx, items, query.page, query.page_size),
        Err(error) => map_service_error(&ctx, error),
    }
}

/// 查询指定 namespace 下的偏好。
pub async fn get_namespace_preferences(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Path(namespace): Path<String>,
) -> Response {
    let (tenant_id, user_id) = match require_self(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };
    if let Err(error) = validate_namespace(&namespace) {
        return validation(&ctx, validation_message(error));
    }

    info!(target: "sdkwork_settings::app_api", tenant_id, user_id, namespace = %namespace, "get namespace preferences");

    match host
        .list_user_preferences(tenant_id, user_id, &namespace)
        .await
    {
        Ok(items) => success_items(&ctx, items, None, None),
        Err(error) => map_service_error(&ctx, error),
    }
}

/// 查询单个偏好。
pub async fn get_user_preference(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Path((namespace, key)): Path<(String, String)>,
) -> Response {
    let (tenant_id, user_id) = match require_self(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };
    if let Err(error) = validate_namespace(&namespace) {
        return validation(&ctx, validation_message(error));
    }
    if let Err(error) = validate_config_key(&key) {
        return validation(&ctx, validation_message(error));
    }

    info!(target: "sdkwork_settings::app_api", tenant_id, user_id, namespace = %namespace, key = %key, "get user preference");

    match host
        .get_user_preference(tenant_id, user_id, &namespace, &key)
        .await
    {
        Ok(Some(item)) => success_item(&ctx, item),
        Ok(None) => not_found(&ctx, "user preference was not found"),
        Err(error) => map_service_error(&ctx, error),
    }
}

/// 更新单个偏好。
pub async fn update_user_preference(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Path((namespace, key)): Path<(String, String)>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    let (tenant_id, user_id) = match require_self(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };
    if let Err(error) = validate_namespace(&namespace) {
        return validation(&ctx, validation_message(error));
    }
    if let Err(error) = validate_config_key(&key) {
        return validation(&ctx, validation_message(error));
    }
    let request: UpdateUserPreferenceRequest = match serde_json::from_value(body) {
        Ok(value) => value,
        Err(error) => return validation(&ctx, format!("invalid request body: {error}")),
    };

    info!(target: "sdkwork_settings::app_api", tenant_id, user_id, namespace = %namespace, key = %key, "update user preference");

    match host
        .update_user_preference(
            tenant_id,
            user_id,
            &namespace,
            &key,
            request.value,
            request.value_type,
            user_id,
        )
        .await
    {
        Ok(item) => success_item(&ctx, item),
        Err(error) => map_service_error(&ctx, error),
    }
}

/// 删除偏好(恢复默认)。
pub async fn delete_user_preference(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Path((namespace, key)): Path<(String, String)>,
) -> Response {
    let (tenant_id, user_id) = match require_self(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };
    if let Err(error) = validate_namespace(&namespace) {
        return validation(&ctx, validation_message(error));
    }
    if let Err(error) = validate_config_key(&key) {
        return validation(&ctx, validation_message(error));
    }

    info!(target: "sdkwork_settings::app_api", tenant_id, user_id, namespace = %namespace, key = %key, "delete user preference");

    match host
        .delete_user_preference(tenant_id, user_id, &namespace, &key, user_id)
        .await
    {
        Ok(()) => no_content(&ctx),
        Err(error) => map_service_error(&ctx, error),
    }
}

/// 批量更新偏好。
///
/// 逐条调用服务层 `update_user_preference`,任一失败则返回错误。
pub async fn batch_update_preferences(
    ctx: WebRequestContext,
    State(host): State<Arc<SettingsServiceHost>>,
    Json(body): Json<serde_json::Value>,
) -> Response {
    let (tenant_id, user_id) = match require_self(&ctx) {
        Ok(value) => value,
        Err(response) => return *response,
    };
    let request: BatchUpdateUserPreferenceRequest = match serde_json::from_value(body) {
        Ok(value) => value,
        Err(error) => return validation(&ctx, format!("invalid request body: {error}")),
    };
    // 先校验全部条目,避免部分写入后因校验失败回滚困难
    for item in &request.updates {
        if let Err(error) = validate_namespace(&item.namespace) {
            return validation(&ctx, validation_message(error));
        }
        if let Err(error) = validate_config_key(&item.preference_key) {
            return validation(&ctx, validation_message(error));
        }
    }

    info!(target: "sdkwork_settings::app_api", tenant_id, user_id, count = request.updates.len(), "batch update preferences");

    let mut results = Vec::with_capacity(request.updates.len());
    for item in request.updates {
        match host
            .update_user_preference(
                tenant_id,
                user_id,
                &item.namespace,
                &item.preference_key,
                item.value,
                item.value_type,
                user_id,
            )
            .await
        {
            Ok(preference) => results.push(preference),
            Err(error) => return map_service_error(&ctx, error),
        }
    }
    success_items(&ctx, results, None, None)
}

// ===== 鉴权辅助 =====

/// 要求 `iam:self` 权限,返回 `(tenant_id, user_id)`。
///
/// `tenant_id` 与 `user_id` 均从 `WebRequestContext` 的 principal 提取,
/// 返回的 `&str` 生命周期与 `ctx` 的借用一致,与服务层 `&str` 签名直接对齐。
fn require_self(ctx: &WebRequestContext) -> Result<(&str, &str), Box<Response>> {
    let principal = match ctx.principal() {
        Some(principal) => principal,
        None => {
            warn!(target: "sdkwork_settings::app_api", "missing principal in request context");
            return Err(Box::new(unauthorized(
                ctx,
                "authenticated principal is required",
            )));
        }
    };
    if !ctx.has_permission(IAM_SELF) {
        warn!(target: "sdkwork_settings::app_api", permission = IAM_SELF, "missing self permission");
        return Err(Box::new(forbidden(
            ctx,
            format!("missing required permission: {IAM_SELF}"),
        )));
    }
    let tenant_id = principal.tenant_id();
    let user_id = principal.user_id();
    if tenant_id.is_empty() {
        return Err(Box::new(validation(ctx, "tenant_id must not be empty")));
    }
    if user_id.is_empty() {
        return Err(Box::new(validation(ctx, "user_id must not be empty")));
    }
    Ok((tenant_id, user_id))
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

/// 构造无内容成功响应(204)。
fn no_content(ctx: &WebRequestContext) -> Response {
    let trace_id = resolve_trace_id(ctx);
    let mut response = StatusCode::NO_CONTENT.into_response();
    if let Ok(value) = HeaderValue::from_str(&trace_id) {
        response
            .headers_mut()
            .insert(HeaderName::from_static("x-sdkwork-trace-id"), value);
    }
    response
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
