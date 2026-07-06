# Changelog — SDKWork Settings App API

Contract changelog for `sdkwork-settings-app-api`. Follows the SDKWork API changelog standard from `sdkwork-specs/API_SPEC.md` (breaking, additive, deprecated, removed).

## 0.1.0

### Added
- Initial app-api contract for SDKWork Settings user preference management.
- Paths:
  - `GET /settings/v1/app-api/preferences` — list current user preferences (`preferences.list`).
  - `GET /settings/v1/app-api/preferences/{namespace}` — list preferences in a namespace (`preferences.entries.list`).
  - `GET /settings/v1/app-api/preferences/{namespace}/{key}` — retrieve a single preference (`preferences.entries.retrieve`).
  - `PUT /settings/v1/app-api/preferences/{namespace}/{key}` — update a single preference (`preferences.entries.update`).
  - `DELETE /settings/v1/app-api/preferences/{namespace}/{key}` — delete a preference, restoring default (`preferences.entries.delete`).
  - `POST /settings/v1/app-api/preferences:batchUpdate` — batch update preferences (`preferences.batchUpdate`).
- Schemas: `UserPreference`, `UpdateUserPreferenceRequest`, `BatchUpdateUserPreferenceRequest`, `BatchUpdateItem`, `PreferenceValueType`.
- `SdkWorkApiResponse` envelope (`code`/`data`/`traceId`) on success and `ProblemDetail` (`application/problem+json`) on error, per `sdkwork-specs/API_SPEC.md` §15.
- Data payload shapes: `data.item` (single resource), `data.items` + `data.pageInfo` (list), `data.accepted` + optional `data.resourceId` (command).
- Bearer token (user token) authentication via `components.securitySchemes.bearerAuth`.
- Per-operation extensions `x-sdkwork-request-context: WebRequestContext` and `x-sdkwork-api-surface: app-api`.
- Error codes: 40001 (400), 40101 (401), 40301 (403), 40401 (404), 40901 (409), 50001 (500).
