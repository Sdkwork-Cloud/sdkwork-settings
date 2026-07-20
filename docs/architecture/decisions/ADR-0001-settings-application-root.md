# ADR-0001: Settings Application Root Architecture

| Field | Value |
| --- | --- |
| ADR Number | 0001 |
| Title | Settings Application Root Architecture |
| Status | Accepted |
| Date | 2026-07-01 |
| Updated | 2026-07-08 |
| Deciders | SDKWork Architecture |

## Context

SDKWork Settings is a configuration center application. It must provide one reusable settings capability for SDKWork PC, H5, Flutter, mini program, native mobile, and backend consumers instead of letting every application reimplement tenant, system, user, and locale configuration.

The repository is an unreleased SDKWork application root, so it should align directly to the current `sdkwork-specs` standards without preserving retired listener names, split-mode profile segments, or compatibility aliases.

## Decision

### 1. Application Identity

- Application code: `settings`
- Domain: `system`
- Application type: web application
- Deployment profiles: `standalone` and `cloud`
- Topology profile id format: `<deploymentProfile>.<environment>`
- Default development profile: `standalone.development`

### 2. Application Surfaces

| Surface | Purpose | Auth context |
| --- | --- | --- |
| `app-api` | Current-user preference management | user token and tenant/user request context |
| `backend-api` | Tenant and system configuration management | operator token and tenant/system permissions |

Both surfaces are exposed through `application.public-ingress`.

### 3. Runtime Ingress

Settings uses `sdkwork-settings-standalone-gateway` as the active application public ingress. The gateway calls `sdkwork-settings-gateway-assembly` to compose Settings route crates.

Retired `*-api-server` listeners must not be used as default dev, release, topology, or deployment ingress.

The current Settings cloud profile packages platform API gateway configuration through `sdkwork-api-cloud-gateway` for `platform.api-gateway`. It does not introduce split mode or multiple application-plane HTTP listeners.

### 4. Repository Structure

The application root follows the SDKWork workspace dictionary:

| Directory | Responsibility |
| --- | --- |
| `apis/` | Authored OpenAPI authorities |
| `apps/sdkwork-settings-pc/` | PC browser and desktop app root |
| `crates/` | Rust contracts, route adapters, service host, gateway assembly, and application ingress |
| `configs/` | Safe source-controlled topology and config templates |
| `deployments/` | Deployment descriptors |
| `docs/` | Product, architecture, decision, and operational documentation |
| `sdks/` | SDK family workspaces and generated SDK artifacts |
| `specs/` | Repository/application machine contracts |
| `tests/` | Contract and integration verification |

`services/` is reserved for future non-HTTP workers or schedulers. It is not the home of the active Settings HTTP ingress.

### 5. Rust Crate Split

| Crate | Responsibility |
| --- | --- |
| `sdkwork-settings-contract` | Domain DTOs, commands, value objects, and typed errors |
| `sdkwork-settings-database-host` | Database lifecycle and schema/repository wiring |
| `sdkwork-settings-service-host` | In-process service container |
| `sdkwork-settings-web-bootstrap` | SDKWork web-framework and IAM request-context bootstrap |
| `sdkwork-settings-gateway-assembly` | Application route composition and assembly exports |
| `sdkwork-settings-standalone-gateway` | Settings application public ingress |
| `sdkwork-routes-settings-app-api` | app-api route adapter |
| `sdkwork-routes-settings-backend-api` | backend-api route adapter |

### 6. Database Tables

| Table | Responsibility |
| --- | --- |
| `stg_user_preference` | User preference values |
| `stg_tenant_config` | Tenant-level configuration values |
| `stg_system_setting` | System-level defaults |
| `stg_config_revision` | Configuration revision audit history |

The table prefix `stg_` is the Settings database ownership prefix.

### 7. Framework Integration

Mandatory integrations:

- `sdkwork-web-framework`
- `sdkwork-database`
- `sdkwork-utils`
- `sdkwork-appbase`
- `sdkwork-iam`
- `sdkwork-drive`

Deferred integrations:

- `sdkwork-discovery`, until Settings introduces cross-process RPC services.
- `sdkwork-rpc-framework`, until Settings ships RPC service processes.

## Consequences

Positive:

- Settings follows the same topology, gateway, API, SDK, database, and IAM standards as other SDKWork applications.
- The application exposes one clear public ingress per plane.
- Route crates stay high-cohesion HTTP adapters and can be composed by gateway assembly.
- The repository avoids compatibility debt before first production release.

Tradeoffs:

- Cloud scale-out internals must stay behind the declared application ingress until a dedicated `sdkwork-settings-cloud-gateway` is introduced by a new ADR.
- Developers must use topology profiles and gateway assembly validation instead of local ad hoc HTTP listener scripts.

## Compliance

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/APPLICATION_GATEWAY_SPEC.md`
- `../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md`
- `../sdkwork-specs/APPLICATION_LAYERED_ARCHITECTURE_SPEC.md`
- `../sdkwork-specs/API_SPEC.md`
- `../sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `../sdkwork-specs/RUST_CODE_SPEC.md`
- `../sdkwork-specs/DATABASE_SPEC.md`
- `../sdkwork-specs/IAM_SPEC.md`
- `../sdkwork-specs/NAMING_SPEC.md`

## Verification

```bash
pnpm check:pnpm-script-standard
pnpm test:single-http-ingress
pnpm api:assembly:validate
pnpm gateway:route-composition:audit
pnpm topology:validate
cargo test --workspace
```
