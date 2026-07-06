# APIs

Authored OpenAPI and RPC contract authorities for the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/API_SPEC.md`, `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`.

## Structure

- `app-api/settings/`: app-api OpenAPI contract (authenticated user-facing surface).
- `backend-api/settings/`: backend-api OpenAPI contract (admin/operator surface).
- `open-api/settings/`: open-api OpenAPI contract (public/anonymous surface, when applicable).
- `rpc/`: RPC contract authority (deferred until RPC services ship).

## Rules

- API contracts, examples, changelogs, and validation fixtures live under `apis/`.
- Generated SDK family workspaces and generated transport output remain under `sdks/`.
- Every HTTP `*-api` operation MUST declare `x-sdkwork-request-context: WebRequestContext` and `x-sdkwork-api-surface`.
- All HTTP responses MUST follow the `SdkWorkApiResponse` envelope per `API_SPEC.md` section 4.5.
