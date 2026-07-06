# ADR-0002: Settings Framework Integration Strategy

| Field | Value |
|-------|-------|
| ADR Number | 0002 |
| Title | Settings Framework Integration Strategy |
| Status | Accepted |
| Date | 2026-07-01 |
| Deciders | SDKWork Architecture |

## Context

Settings 应用需要接入多个 SDKWork 框架。需要明确每个框架的接入方式、集成边界、以及延后接入的触发条件,确保高内聚低耦合,符合开闭原则。

## Decision

### 1. sdkwork-web-framework 接入策略

**接入方式**: Cargo.toml path 依赖

```toml
sdkwork_web_core = { path = "../sdkwork-web-framework/crates/sdkwork-web-core" }
sdkwork_web_axum = { path = "../sdkwork-web-framework/crates/sdkwork-web-axum" }
sdkwork_web_bootstrap = { path = "../sdkwork-web-framework/crates/sdkwork-web-bootstrap" }
sdkwork_web_contract = { path = "../sdkwork-web-framework/crates/sdkwork-web-contract" }
sdkwork_web_store_sqlx = { path = "../sdkwork-web-framework/crates/sdkwork-web-store-sqlx" }
sdkwork_routes_web_framework_backend_api = { path = "../sdkwork-web-framework/crates/sdkwork-routes-web-framework-backend-api" }
```

**集成边界**:
- 所有 HTTP `*-api` 表面必须通过 `sdkwork-web-bootstrap` 引导
- 路由通过 `sdkwork-routes-*` crate 装配,禁止在业务仓库 fork 拦截器链
- 响应通过 `sdkwork-web-framework` 响应映射,严格遵循 `SdkWorkApiResponse` 信封
- 错误映射为 `application/problem+json`(ProblemDetail)

### 2. sdkwork-database 接入策略

**接入方式**: Cargo.toml path 依赖

```toml
sdkwork_database_config = { path = "../sdkwork-database/crates/sdkwork-database-config" }
sdkwork_database_lifecycle = { path = "../sdkwork-database/crates/sdkwork-database-lifecycle" }
sdkwork_database_spi = { path = "../sdkwork-database/crates/sdkwork-database-spi" }
sdkwork_database_sqlx = { path = "../sdkwork-database/crates/sdkwork-database-sqlx" }
```

**集成边界**:
- 数据库生命周期、迁移、种子、漂移通过 framework 管理
- `database/` 目录遵循 DATABASE_SPEC 标准
- 表 schema 通过 `sdkwork-database-spi` 注册
- 业务代码通过 `sdkwork-database-sqlx` 访问数据

### 3. sdkwork-utils 接入策略

**接入方式**: Cargo.toml path 依赖

```toml
sdkwork_utils_rust = { path = "../sdkwork-utils/packages/sdkwork-utils-rust" }
```

**使用范围**(减少重复代码):
- `string`: 字符串处理(trim, snake_case, kebab_case, mask 等)
- `datetime`: 时间处理(now, format, parse, UTC)
- `validation`: 输入校验(is_blank, ipv6, e164 等)
- `crypto`: 敏感配置加密(AES-256-GCM, HKDF-SHA256)
- `encoding`: 编码(base64url 等)
- `collection`: 集合处理(unique, chunk, group_by 等)
- `i18n`: 多语言支持
- `http_api`: API 工具(SdkWorkApiResponse 构造等)

**禁止**: 重新实现 sdkwork-utils 已有的工具函数。

### 4. sdkwork-appbase 接入策略

**接入方式**: Cargo.toml path 依赖 + pnpm 依赖

```toml
sdkwork_id = { path = "../sdkwork-appbase/crates/sdkwork-platform-id-service" }
```

**集成边界**:
- 平台 ID 服务用于生成雪花 ID(SUBJECT_ID_SPEC)
- PC React 包用于前端运行时

### 5. sdkwork-iam 接入策略

**接入方式**: Cargo.toml path 依赖

```toml
sdkwork_iam_bootstrap = { path = "../sdkwork-iam/crates/sdkwork-iam-bootstrap" }
sdkwork_iam_web_adapter = { path = "../sdkwork-iam/crates/sdkwork-iam-web-adapter" }
sdkwork_iam_database_host = { path = "../sdkwork-iam/crates/sdkwork-iam-database-host" }
sdkwork_iam_embedded_application_bootstrap = { path = "../sdkwork-iam/crates/sdkwork-iam-embedded-application-bootstrap" }
```

**集成边界**:
- 认证授权通过 `sdkwork-iam-web-adapter` 接入
- 应用引导通过 `sdkwork-iam-bootstrap` 和 `sdkwork-iam-embedded-application-bootstrap`
- 租户隔离通过 IAM 提供的 tenant_id scope

### 6. sdkwork-drive 接入策略

**接入方式**: Cargo.toml path 依赖 + pnpm 依赖

```toml
sdkwork_drive_contract = { path = "../sdkwork-drive/crates/sdkwork-drive-contract" }
sdkwork_drive_uploader_service = { path = "../sdkwork-drive/crates/sdkwork-drive-uploader-service" }
```

**集成边界**:
- 前端: 使用 `sdkwork-drive-app-sdk` 的 `client.uploader.*` 接口
- 后端: 使用 `DriveUploaderService` / `PrepareUploaderUploadCommand`
- 上传场景: 用户头像、租户品牌 Logo、配置导入文件(JSON/YAML)
- 禁止: 创建 app-local upload session / presign service / 重复 `/upload` API

### 7. sdkwork-discovery 延后接入策略

**当前决策**: 暂不接入

**触发条件**(任一满足即需接入):
- Settings 引入跨进程 gRPC 服务
- 需要动态 RPC 端点解析
- 需要版本化运行时配置控制面

**未来接入方式**:
```toml
sdkwork_rpc_server = { path = "../sdkwork-rpc-framework/crates/sdkwork-rpc-server" }
sdkwork_rpc_discovery = { path = "../sdkwork-rpc-framework/crates/sdkwork-rpc-discovery" }
```
+ 创建 ADR 记录接入决策(参考 sdkwork-im ADR-20260619 模式)

## Consequences

### Positive

- 高内聚低耦合,配置能力集中管理
- 利用框架能力减少重复代码
- 标准化接入,与其他 SDKWork 应用一致
- 开闭原则: 框架扩展开放,业务修改封闭

### Negative

- 多框架依赖增加构建复杂度
- 框架版本升级需要协调

## Compliance

- `sdkwork-specs/WEB_FRAMEWORK_SPEC.md`
- `sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md`
- `sdkwork-specs/IAM_SPEC.md`
- `sdkwork-specs/DRIVE_SPEC.md`
- `sdkwork-specs/DISCOVERY_SPEC.md`
