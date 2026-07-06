# SDKWork Settings — Technical Architecture

| Field | Value |
|-------|-------|
| Application | SDKWork Settings |
| Application Code | `settings` |
| Domain | `system` |
| Version | 0.1.0 |

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     Client Applications                          │
│  PC React │ H5 │ Flutter │ Mini Program │ Android │ iOS │ Harmony│
└───────────────────────────┬─────────────────────────────────────┘
                            │ SDK
┌───────────────────────────▼─────────────────────────────────────┐
│                  sdkwork-settings-app-sdk                        │
│                sdkwork-settings-backend-sdk                      │
└───────────────────────────┬─────────────────────────────────────┘
                            │ HTTP (SdkWorkApiResponse envelope)
┌───────────────────────────▼─────────────────────────────────────┐
│              sdkwork-settings-api-server                         │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │           sdkwork-settings-web-bootstrap                 │   │
│  │  (sdkwork-web-framework: core/axum/bootstrap/contract)   │   │
│  └──────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────┐  ┌─────────────────────────────────┐  │
│  │  app-api routes     │  │  backend-api routes             │  │
│  │  (user preference)  │  │  (tenant/system config)         │  │
│  └─────────────────────┘  └─────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │            sdkwork-settings-service-host                 │   │
│  │  (in-process service container, business logic)          │   │
│  └──────────────────────────────────────────────────────────┘   │
└──────┬──────────────┬──────────────┬──────────────┬─────────────┘
       │              │              │              │
       ▼              ▼              ▼              ▼
┌──────────┐  ┌────────────┐  ┌───────────┐  ┌──────────────┐
│ database │  │   IAM      │  │  Drive    │  │   utils      │
│ framework│  │  adapter   │  │ uploader  │  │  (rust)      │
│ (sqlx)   │  │            │  │ service   │  │              │
└────┬─────┘  └─────┬──────┘  └─────┬─────┘  └──────────────┘
     │              │               │
     ▼              ▼               ▼
┌──────────┐  ┌────────────┐  ┌───────────┐
│PostgreSQL│  │  IAM DB    │  │ Drive Store│
│ / SQLite │  │  (shared)  │  │ (object)  │
└──────────┘  └────────────┘  └───────────┘
```

## 2. Layered Architecture

### 2.1 Presentation Layer (HTTP API)

- **Route Crates**: `sdkwork-routes-settings-app-api`, `sdkwork-routes-settings-backend-api`
- **Framework**: sdkwork-web-framework(axum-based)
- **Response Envelope**: `SdkWorkApiResponse`(`{ code: 0, data, traceId }`)
- **Error Format**: `application/problem+json`(ProblemDetail)
- **Authentication**: sdkwork-iam-web-adapter(Token 验证、租户隔离)

### 2.2 Application Layer (Service Host)

- **Container**: `sdkwork-settings-service-host`
- **职责**: 编排领域服务、事务管理、缓存协调
- **依赖**: domain contracts + database SPI

### 2.3 Domain Layer (Contract)

- **Contract**: `sdkwork-settings-contract`
- **内容**: 领域模型、DTO、错误类型、值对象
- **原则**: 不依赖具体实现(无 sqlx、无 axum)

### 2.4 Infrastructure Layer (Database)

- **Database Host**: `sdkwork-settings-database-host`
- **Framework**: sdkwork-database(config/lifecycle/spi/sqlx)
- **Schema**: `stg_user_preference`, `stg_tenant_config`, `stg_system_setting`, `stg_config_revision`

## 3. Configuration Model

### 3.1 Three-Tier Configuration

```
System Setting (global)
       │
       ▼ (inherit)
Tenant Config (per tenant)
       │
       ▼ (inherit + override)
User Preference (per user)
```

- **System Setting**: 全局参数,仅系统管理员可改
- **Tenant Config**: 租户级配置,继承系统默认,租户管理员可覆盖
- **User Preference**: 用户偏好,继承租户默认,用户可覆盖

### 3.2 Namespace 分组

配置按 namespace 分组管理:
- `appearance`: 主题、字体、密度
- `locale`: 语言、时区、日期格式
- `notification`: 邮件、推送、桌面通知
- `privacy`: 数据可见性、追踪
- `security`: 密码策略、二次验证(租户级)
- `branding`: Logo、颜色、自定义 CSS(租户级)
- `feature-flag`: 功能开关(租户级/系统级)

### 3.3 Value Types

配置 value 支持以下类型(JSON 编码):
- `string`: 字符串
- `number`: 数字
- `boolean`: 布尔
- `object`: 对象(嵌套 JSON)
- `array`: 数组
- `i18n`: 多语言对象(`{ "zh-CN": "...", "en-US": "..." }`)

## 4. Data Model

### 4.1 stg_user_preference

| 列 | 类型 | 说明 |
|----|------|------|
| id | BIGINT | 雪花 ID(PK) |
| tenant_id | BIGINT | 租户 ID |
| user_id | BIGINT | 用户 ID |
| namespace | VARCHAR(64) | 命名空间 |
| preference_key | VARCHAR(128) | 配置键 |
| preference_value | JSON | 配置值 |
| value_type | VARCHAR(16) | 值类型 |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |
| created_by | BIGINT | 创建人 |
| updated_by | BIGINT | 更新人 |

**索引**: `(tenant_id, user_id, namespace, preference_key)` UNIQUE

### 4.2 stg_tenant_config

| 列 | 类型 | 说明 |
|----|------|------|
| id | BIGINT | 雪花 ID(PK) |
| tenant_id | BIGINT | 租户 ID |
| namespace | VARCHAR(64) | 命名空间 |
| config_key | VARCHAR(128) | 配置键 |
| config_value | JSON | 配置值 |
| value_type | VARCHAR(16) | 值类型 |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |
| created_by | BIGINT | 创建人 |
| updated_by | BIGINT | 更新人 |

**索引**: `(tenant_id, namespace, config_key)` UNIQUE

### 4.3 stg_system_setting

| 列 | 类型 | 说明 |
|----|------|------|
| id | BIGINT | 雪花 ID(PK) |
| namespace | VARCHAR(64) | 命名空间 |
| setting_key | VARCHAR(128) | 配置键 |
| setting_value | JSON | 配置值 |
| value_type | VARCHAR(16) | 值类型 |
| scope | VARCHAR(32) | 作用域(global/region) |
| scope_value | VARCHAR(64) | 作用域值(如区域代码) |
| created_at | TIMESTAMPTZ | 创建时间 |
| updated_at | TIMESTAMPTZ | 更新时间 |
| created_by | BIGINT | 创建人 |
| updated_by | BIGINT | 更新人 |

**索引**: `(namespace, setting_key, scope, scope_value)` UNIQUE

### 4.4 stg_config_revision

| 列 | 类型 | 说明 |
|----|------|------|
| id | BIGINT | 雪花 ID(PK) |
| tenant_id | BIGINT | 租户 ID(系统设置为 0) |
| config_type | VARCHAR(16) | user/tenant/system |
| config_id | BIGINT | 配置记录 ID |
| namespace | VARCHAR(64) | 命名空间 |
| config_key | VARCHAR(128) | 配置键 |
| old_value | JSON | 旧值 |
| new_value | JSON | 新值 |
| operation | VARCHAR(16) | create/update/delete |
| operator_id | BIGINT | 操作人 |
| operator_ip | VARCHAR(45) | 操作 IP |
| created_at | TIMESTAMPTZ | 操作时间 |

**索引**: `(tenant_id, config_type, config_id, created_at)`, `(created_at)`

## 5. API Design

### 5.1 app-api (User Preference)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/settings/v1/app-api/preferences` | 查询当前用户偏好列表 |
| GET | `/settings/v1/app-api/preferences/{namespace}` | 查询指定 namespace 偏好 |
| GET | `/settings/v1/app-api/preferences/{namespace}/{key}` | 查询单个偏好 |
| PUT | `/settings/v1/app-api/preferences/{namespace}/{key}` | 更新单个偏好 |
| DELETE | `/settings/v1/app-api/preferences/{namespace}/{key}` | 删除偏好(恢复默认) |
| POST | `/settings/v1/app-api/preferences:batchUpdate` | 批量更新偏好 |

### 5.2 backend-api (Tenant/System Config)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/settings/v1/backend-api/tenant-configs` | 查询租户配置列表 |
| GET | `/settings/v1/backend-api/tenant-configs/{namespace}/{key}` | 查询单个租户配置 |
| PUT | `/settings/v1/backend-api/tenant-configs/{namespace}/{key}` | 更新租户配置 |
| GET | `/settings/v1/backend-api/system-settings` | 查询系统设置列表 |
| GET | `/settings/v1/backend-api/system-settings/{namespace}/{key}` | 查询单个系统设置 |
| PUT | `/settings/v1/backend-api/system-settings/{namespace}/{key}` | 更新系统设置 |
| GET | `/settings/v1/backend-api/revisions` | 查询配置变更历史 |

## 6. Security Architecture

### 6.1 Authentication & Authorization

- 所有 API 通过 sdkwork-iam-web-adapter 验证 Token
- 租户隔离: 所有查询自动附加 `tenant_id` scope
- 权限矩阵:
  - 用户偏好: 仅本人可访问(`iam:self`)
  - 租户配置: 租户管理员可访问(`iam:tenant:admin`)
  - 系统设置: 系统管理员可访问(`iam:system:admin`)

### 6.2 Data Security

- 敏感配置(密钥、Token)使用 sdkwork-utils crypto AES-256-GCM 加密存储
- 密钥派生使用 HKDF-SHA256
- 配置变更全量审计(stg_config_revision)
- 审计日志包含 before/after diff、操作人、IP、时间

### 6.3 Input Validation

- 所有输入通过 sdkwork-utils validation 校验
- namespace: `^[a-z][a-z0-9-]{0,63}$`
- key: `^[a-z][a-z0-9-_]{0,127}$`
- value 大小限制: 64KB

## 7. Performance Architecture

### 7.1 Caching

- L1: 进程内 LRU 缓存(TTL 60s,容量 10000)
- L2: Redis 缓存(TTL 5min)— Phase 2
- 缓存键: `settings:{tenant_id}:{scope}:{namespace}:{key}`
- 写操作: write-through(先写 DB,再更新缓存)
- 失效: 配置变更时主动失效相关缓存键

### 7.2 Database Optimization

- 索引覆盖核心查询路径
- 读多写少场景: 读写分离(Phase 2)
- 批量查询: 使用 IN 而非循环查询

## 8. Observability

### 8.1 Metrics

- `settings_query_total{scope,namespace,hit}`
- `settings_query_duration_seconds{scope,namespace}`
- `settings_update_total{scope,namespace,result}`
- `settings_cache_hit_ratio`

### 8.2 Logging

- 配置变更日志(INFO 级别,含 diff)
- 查询慢日志(> 200ms,WARNING 级别)
- 错误日志(ERROR 级别,含 traceId)

### 8.3 Tracing

- 全链路 traceId(由 sdkwork-web-framework 注入)
- Span: HTTP handler → service → repository → DB

## 9. Deployment

### 9.1 Standalone Profile

- 单进程: api-server + standalone-gateway
- 数据库: SQLite 或 PostgreSQL
- 适用: 个人/小团队部署

### 9.2 Cloud Profile

- 多进程: api-server + cloud-gateway
- 数据库: PostgreSQL(主从)
- 缓存: Redis
- 适用: 企业级部署

## 10. Technology Stack

| 层 | 技术 |
|----|------|
| 后端语言 | Rust |
| Web 框架 | sdkwork-web-framework (axum) |
| 数据库 | PostgreSQL / SQLite (sqlx) |
| 前端框架 | React + Vite + TypeScript |
| 前端样式 | Tailwind CSS v4 |
| 桌面打包 | Tauri(可选) |
| 包管理 | pnpm (前端) + Cargo (后端) |
| 认证 | sdkwork-iam |
| 文件上传 | sdkwork-drive |
| 工具库 | sdkwork-utils |
