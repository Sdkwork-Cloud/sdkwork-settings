# ADR-0001: Settings Application Root Architecture

| Field | Value |
|-------|-------|
| ADR Number | 0001 |
| Title | Settings Application Root Architecture |
| Status | Accepted |
| Date | 2026-07-01 |
| Deciders | SDKWork Architecture |

## Context

SDKWork Settings 是一个新的配置中心应用,需要从零开始建立。当前 SDKWork 工作空间已有多个参考应用(sdkwork-im、sdkwork-drive、sdkwork-iam、sdkwork-id),它们都遵循 sdkwork-specs 标准规范。Settings 需要对齐这些标准,并作为配置中心为其他应用提供统一的配置管理能力。

## Decision

Settings 应用采用以下根架构:

### 1. Application Identity

- Application Code: `settings`
- Domain: `system`
- App Type: `APP_HTML`(Web 应用)
- Deployment Profiles: `cloud` + `standalone`
- Default Profile: `cloud`

### 2. Application Surfaces

- **app-api**: 用户偏好管理,用户 Token 认证
- **backend-api**: 租户/系统配置管理,管理员 Token 认证

### 3. Repository Structure

采用 SDKWORK_WORKSPACE_SPEC 标准目录字典:
- `crates/`: Rust 契约、路由适配器、运行时服务库
- `services/`: 可运行的 Rust 服务进程
- `apps/sdkwork-settings-pc/`: PC 浏览器/桌面应用
- `apis/`: OpenAPI 契约权威
- `sdks/`: 生成的 SDK 家族
- `database/`: 数据库契约与生命周期
- `configs/`, `deployments/`, `scripts/`, `tools/`, `docs/`, `tests/`

### 4. Crate 划分

| Crate | 职责 |
|-------|------|
| `sdkwork-settings-contract` | 领域契约、DTO、错误类型 |
| `sdkwork-settings-database-host` | 数据库模块引导、schema 注册 |
| `sdkwork-settings-web-bootstrap` | Web 框架引导、路由装配 |
| `sdkwork-settings-gateway-assembly` | 网关装配清单 |
| `sdkwork-settings-service-host` | 进程内服务容器 |
| `sdkwork-routes-settings-app-api` | app-api 路由 crate |
| `sdkwork-routes-settings-backend-api` | backend-api 路由 crate |
| `sdkwork-settings-api-server` | HTTP 服务器进程 |
| `sdkwork-settings-standalone-gateway` | standalone 网关 |

### 5. Database Tables

- `stg_user_preference`: 用户偏好(tenant_id, user_id, namespace, preference_key, preference_value)
- `stg_tenant_config`: 租户配置(tenant_id, namespace, config_key, config_value)
- `stg_system_setting`: 系统设置(namespace, setting_key, setting_value, scope)
- `stg_config_revision`: 配置版本历史(配置变更审计)
- 表前缀 `stg_` 遵循 DATABASE_SPEC 表命名规范。

### 6. Framework Integration

- **强制接入**: sdkwork-web-framework, sdkwork-database, sdkwork-utils, sdkwork-appbase, sdkwork-iam, sdkwork-drive
- **延后接入**: sdkwork-discovery, sdkwork-rpc-framework(无 RPC 服务)

## Consequences

### Positive

- 对齐 sdkwork-specs 标准,与其他 SDKWork 应用保持一致
- 高内聚低耦合,配置能力集中管理
- 多语言/多架构应用可通过 SDK 快速集成
- 利用现有框架能力,减少重复代码

### Negative

- 需要学习和维护多个框架的集成方式
- Phase 1 不接入 RPC,后续接入时需要重构

## Compliance

- `sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `sdkwork-specs/APPLICATION_SPEC.md`
- `sdkwork-specs/RUST_CODE_SPEC.md`
- `sdkwork-specs/DATABASE_SPEC.md`
- `sdkwork-specs/NAMING_SPEC.md`
