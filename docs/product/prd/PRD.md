# SDKWork Settings — Product Requirements Document

| Field | Value |
|-------|-------|
| Application | SDKWork Settings |
| Application Code | `settings` |
| Domain | `system` |
| Version | 0.1.0 |
| Status | Draft |
| Owner | SDKWork Product |

## 1. Overview

### 1.1 Vision

SDKWork Settings 是一个统一的配置中心,让不同架构的 SDKWork 应用(PC、H5、Flutter、小程序、原生 Android/iOS/Harmony、后端服务)通过统一的 API 和 SDK 接入设置、偏好和租户配置,而不是每个应用重复实现自己的设置层。

### 1.2 Problem Statement

当前 SDKWork 工作空间中,每个业务应用(IM、Drive、IAM、Membership 等)都需要处理用户偏好、租户配置、系统设置。这导致:

1. **重复实现**:每个应用各自实现配置 CRUD、缓存、校验、国际化、审计逻辑。
2. **配置分散**:用户偏好在多个应用间不一致,无法统一管理。
3. **集成成本高**:新应用接入配置能力需要从零开发。
4. **多语言/多架构适配困难**:不同客户端(PC/H5/移动端)需要不同的 SDK,且配置数据结构不统一。

### 1.3 Target Users

| 用户角色 | 使用场景 |
|----------|----------|
| 最终用户 | 管理个人偏好(主题、语言、通知、隐私) |
| 租户管理员 | 配置租户级设置(品牌、安全策略、功能开关) |
| 系统管理员 | 管理系统级设置(全局参数、维护窗口) |
| 应用开发者 | 通过 SDK 集成配置中心到自己的应用 |
| 运维人员 | 监控配置变更、审计、回滚 |

## 2. Goals and Non-Goals

### 2.1 Goals

- 提供统一的配置中心 API(app-api + backend-api),严格遵循 `SdkWorkApiResponse` 信封规范。
- 支持三层配置粒度:系统设置(system)、租户配置(tenant)、用户偏好(user)。
- 支持多语言、多架构客户端通过 SDK 快速集成。
- 提供配置版本历史、审计、回滚能力。
- 支持配置热更新(WebSocket 推送 + 轮询兜底)。
- 文件上传(头像、Logo、配置导入)通过 sdkwork-drive 集成,高内聚低耦合。
- 认证授权通过 sdkwork-iam 集成,支持租户隔离。
- 数据持久化通过 sdkwork-database 框架管理,支持 PostgreSQL 和 SQLite。
- HTTP 服务通过 sdkwork-web-framework 接入,统一拦截器链和请求上下文。
- 使用 sdkwork-utils 减少重复代码,提高标准化程度。

### 2.2 Non-Goals

- 不做业务应用的领域逻辑(如 IM 消息、Drive 文件存储)。
- 不做用户身份管理(由 sdkwork-iam 负责)。
- 不做文件存储(由 sdkwork-drive 负责)。
- 不做跨进程 RPC 服务(Phase 1 无 RPC 需求,延后接入 sdkwork-discovery)。
- 不做密钥管理服务(敏感配置加密使用 sdkwork-utils 的 crypto 能力)。

## 3. Functional Requirements

### 3.1 用户偏好(User Preference)

| 需求 ID | 描述 | 优先级 |
|---------|------|--------|
| FR-PREF-001 | 用户可查询自己的偏好配置 | P0 |
| FR-PREF-002 | 用户可更新偏好配置(主题、语言、通知、隐私) | P0 |
| FR-PREF-003 | 用户可重置偏好为默认值 | P1 |
| FR-PREF-004 | 偏好配置按 namespace 分组管理 | P0 |

### 3.2 租户配置(Tenant Config)

| 需求 ID | 描述 | 优先级 |
|---------|------|--------|
| FR-TENANT-001 | 租户管理员可查询租户配置 | P0 |
| FR-TENANT-002 | 租户管理员可更新租户配置(品牌、安全策略、功能开关) | P0 |
| FR-TENANT-003 | 租户管理员可上传/更新品牌 Logo(通过 Drive) | P1 |
| FR-TENANT-004 | 租户配置支持继承和覆盖系统默认值 | P1 |

### 3.3 系统设置(System Setting)

| 需求 ID | 描述 | 优先级 |
|---------|------|--------|
| FR-SYS-001 | 系统管理员可查询系统设置 | P0 |
| FR-SYS-002 | 系统管理员可更新系统设置 | P0 |
| FR-SYS-003 | 系统设置支持 scope(全局、区域) | P1 |

### 3.4 配置版本与审计

| 需求 ID | 描述 | 优先级 |
|---------|------|--------|
| FR-AUDIT-001 | 所有配置变更记录版本历史 | P0 |
| FR-AUDIT-002 | 配置变更可审计(谁、何时、改了什么) | P0 |
| FR-AUDIT-003 | 支持配置回滚到历史版本 | P1 |

### 3.5 配置导入导出

| 需求 ID | 描述 | 优先级 |
|---------|------|--------|
| FR-IMP-001 | 支持导出配置为 JSON/YAML 文件(通过 Drive) | P1 |
| FR-IMP-002 | 支持从 JSON/YAML 文件导入配置(通过 Drive 上传) | P1 |

### 3.6 配置热更新

| 需求 ID | 描述 | 优先级 |
|---------|------|--------|
| FR-HOT-001 | 配置变更后通过 WebSocket 推送到已连接客户端 | P1 |
| FR-HOT-002 | 客户端可订阅特定 namespace 的配置变更 | P1 |
| FR-HOT-003 | 提供轮询兜底接口供不支持 WebSocket 的客户端使用 | P2 |

## 4. Non-Functional Requirements

### 4.1 Performance

- 单次配置查询 P99 < 50ms(命中缓存)。
- 单次配置查询 P99 < 200ms(回源数据库)。
- 配置更新 P99 < 300ms。
- 支持 1000 QPS 配置查询。

### 4.2 Security

- 所有 API 通过 sdkwork-iam 认证,租户隔离。
- 用户偏好仅本人可访问。
- 租户配置仅租户管理员可访问。
- 系统设置仅系统管理员可访问。
- 敏感配置(如密钥)加密存储(使用 sdkwork-utils crypto)。
- 配置变更全量审计。

### 4.3 Reliability

- 数据库主从切换不影响配置读取(读从库)。
- 配置缓存本地化,数据库不可用时降级到缓存。
- 配置变更通过事务保证一致性。

### 4.4 Observability

- 关键指标:配置查询 QPS、延迟、缓存命中率、更新成功率。
- 日志:配置变更日志(包含 before/after diff)。
- Tracing:全链路 traceId。

### 4.5 Internationalization

- 配置 value 支持 i18n(多语言值)。
- API 错误消息支持多语言。
- 使用 sdkwork-utils i18n 能力。

## 5. Integration Architecture

### 5.1 Framework Integration

| 框架 | 用途 | 必需 |
|------|------|------|
| sdkwork-web-framework | HTTP API 框架、拦截器链、请求上下文、响应映射 | 是 |
| sdkwork-database | 数据库生命周期、迁移、种子、SPI | 是 |
| sdkwork-utils | 通用工具(string/datetime/validation/crypto/i18n) | 是 |
| sdkwork-appbase | 平台 ID 服务、PC React 运行时 | 是 |
| sdkwork-iam | 认证、授权、应用引导 | 是 |
| sdkwork-drive | 文件上传(头像、Logo、配置导入) | 是 |
| sdkwork-discovery | RPC 服务发现 | 否(延后) |
| sdkwork-rpc-framework | RPC 服务 | 否(延后) |

### 5.2 Application Surfaces

| Surface | 用途 | 认证 |
|---------|------|------|
| app-api | 用户偏好管理 | 用户 Token |
| backend-api | 租户/系统配置管理 | 管理员 Token |
| open-api | 公开配置查询(无敏感数据) | 可选 |

### 5.3 SDK Family

| SDK | 用途 | 生成来源 |
|-----|------|----------|
| sdkwork-settings-app-sdk | app-api 客户端 SDK | app-api OpenAPI |
| sdkwork-settings-backend-sdk | backend-api 客户端 SDK | backend-api OpenAPI |

## 6. Release Plan

### Phase 1 (MVP) — 0.1.0

- 用户偏好 CRUD(app-api)
- 租户配置 CRUD(backend-api)
- 系统设置 CRUD(backend-api)
- 数据库契约与迁移
- sdkwork-iam 认证集成
- sdkwork-drive 文件上传集成(头像/Logo)
- PC 前端应用骨架

### Phase 2 — 0.2.0

- 配置版本历史与审计
- 配置回滚
- 配置导入导出(JSON/YAML)
- 配置热更新(WebSocket)
- 缓存层优化

### Phase 3 — 0.3.0

- RPC 服务(如需跨进程调用)
- sdkwork-discovery 接入
- 多区域部署支持
- 高级监控与告警

## 7. Success Metrics

- 配置查询 P99 延迟 < 50ms(缓存命中)
- 配置查询 P99 延迟 < 200ms(回源)
- 缓存命中率 > 95%
- 配置更新成功率 > 99.9%
- SDK 集成时间 < 30 分钟(新应用接入)
- 零配置不一致事件(通过租户隔离和审计保证)
