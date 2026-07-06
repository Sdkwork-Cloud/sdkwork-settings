# Changelog — SDKWork Settings Backend API

Contract changelog for `sdkwork-settings-backend-api`. Follows the SDKWork API changelog standard from `sdkwork-specs/API_SPEC.md` (breaking, additive, deprecated, removed).

## 0.1.0

### Added
- Initial backend-api contract for SDKWork Settings tenant configuration, system settings, and configuration revision management.
- Paths:
  - `GET /settings/v1/backend-api/tenant-configs` — list tenant configurations (`tenantConfigs.list`).
  - `GET /settings/v1/backend-api/tenant-configs/{namespace}/{key}` — retrieve a tenant configuration (`tenantConfigs.retrieve`).
  - `PUT /settings/v1/backend-api/tenant-configs/{namespace}/{key}` — update a tenant configuration (`tenantConfigs.update`).
  - `GET /settings/v1/backend-api/system-settings` — list system settings (`systemSettings.list`).
  - `GET /settings/v1/backend-api/system-settings/{namespace}/{key}` — retrieve a system setting (`systemSettings.retrieve`).
  - `PUT /settings/v1/backend-api/system-settings/{namespace}/{key}` — update a system setting (`systemSettings.update`).
  - `GET /settings/v1/backend-api/revisions` — list configuration revision history (`revisions.list`).
- Schemas: `TenantConfig`, `UpdateTenantConfigRequest`, `SystemSetting`, `UpdateSystemSettingRequest`, `ConfigRevision`, `ConfigValueType`, `ConfigScope`, `ConfigType`, `ConfigOperation`.
- `SdkWorkApiResponse` envelope (`code`/`data`/`traceId`) on success and `ProblemDetail` (`application/problem+json`) on error, per `sdkwork-specs/API_SPEC.md` §15.
- Data payload shapes: `data.item` (single resource), `data.items` + `data.pageInfo` (list).
- Bearer token (administrator token) authentication via `components.securitySchemes.bearerAuth`.
- Per-operation extensions `x-sdkwork-request-context: WebRequestContext` and `x-sdkwork-api-surface: backend-api`.
- Error codes: 40001 (400), 40101 (401), 40301 (403), 40401 (404), 40901 (409), 50001 (500).
