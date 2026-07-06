# Settings Database

SDKWork Settings 数据库契约与生命周期资产目录。

Authority: `../sdkwork-specs/DATABASE_SPEC.md`, `../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md`。

## Initialization state

This module is in **initialization state** for greenfield deployments:

1. **Baseline** — `database/ddl/baseline/{engine}/0001_settings_baseline.sql` contains the full DDL snapshot.
2. **Migrations** — `database/migrations/{engine}/` is reserved for post-GA incremental schema changes only.
3. **Drift** — run `pnpm db:drift:check` before release.

## Commands

```bash
pnpm run db:validate
pnpm run db:materialize:contract
pnpm run db:plan
pnpm run db:init
pnpm run db:migrate
pnpm run db:seed
pnpm run db:status
pnpm run db:drift:check
```

## 结构

- `database.manifest.json`: 数据库模块清单,声明 moduleId、表前缀、生命周期配置。
- `contract/`: 数据库契约(schema.yaml、table-registry.json、prefix-registry.json)。
- `ddl/baseline/`: 初始化 baseline DDL(postgres + sqlite)。
- `ddl/generated/`: 生成的 DDL(预留)。
- `drift/`: 漂移观测策略。
- `migrations/`: GA 后的增量迁移(预留)。
- `seeds/`: 种子数据(common 通用 + locales 多语言)。
- `fixtures/`: 测试 fixtures(预留)。

## 表模型

| 表 | 用途 | 前缀 |
|----|------|------|
| `stg_user_preference` | 用户偏好(三层配置最底层) | stg_ |
| `stg_tenant_config` | 租户配置(三层配置中间层) | stg_ |
| `stg_system_setting` | 系统设置(三层配置最顶层) | stg_ |
| `stg_config_revision` | 配置变更历史(审计与回滚) | stg_ |

## 三层配置模型

```
System Setting (global)
       │
       ▼ (inherit)
Tenant Config (per tenant)
       │
       ▼ (inherit + override)
User Preference (per user)
```

## 引擎支持

- PostgreSQL(默认引擎,生产推荐)
- SQLite(开发和小团队部署)

## 生命周期

- `autoMigrate: true`: 启动时自动执行迁移
- `seedOnBoot: false`: 不在启动时自动播种(通过 `pnpm db:init` 手动执行)
- `driftCheckIntervalSec: 60`: 漂移观测间隔

## 数据库初始化

```bash
# 开发环境(PostgreSQL)
pnpm db:postgres:init

# 开发环境(SQLite)
pnpm db:init
```
