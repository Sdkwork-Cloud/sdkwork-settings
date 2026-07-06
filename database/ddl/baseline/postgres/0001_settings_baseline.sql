-- SDKWork Settings 初始化 baseline (postgres)
-- 应用处于初始化阶段:完整 DDL 位于此文件;migrations/ 保留用于 GA 后的变更。
-- 表前缀: stg_
-- 锚点表: stg_user_preference

-- 用户偏好表(三层配置模型的最底层)
CREATE TABLE IF NOT EXISTS stg_user_preference (
    id BIGINT NOT NULL,
    tenant_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    namespace VARCHAR(64) NOT NULL,
    preference_key VARCHAR(128) NOT NULL,
    preference_value JSONB NOT NULL,
    value_type VARCHAR(16) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    created_by BIGINT,
    updated_by BIGINT,
    CONSTRAINT pk_stg_user_preference PRIMARY KEY (id),
    CONSTRAINT uk_stg_user_preference_tenant_user_ns_key UNIQUE (tenant_id, user_id, namespace, preference_key),
    CONSTRAINT ck_stg_user_preference_value_type CHECK (value_type IN ('string', 'number', 'boolean', 'object', 'array', 'i18n'))
);

CREATE INDEX IF NOT EXISTS idx_stg_user_preference_tenant_user_ns
    ON stg_user_preference (tenant_id, user_id, namespace);

CREATE INDEX IF NOT EXISTS idx_stg_user_preference_updated_at
    ON stg_user_preference (updated_at);

-- 租户配置表(三层配置模型的中间层)
CREATE TABLE IF NOT EXISTS stg_tenant_config (
    id BIGINT NOT NULL,
    tenant_id TEXT NOT NULL,
    namespace VARCHAR(64) NOT NULL,
    config_key VARCHAR(128) NOT NULL,
    config_value JSONB NOT NULL,
    value_type VARCHAR(16) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    created_by BIGINT,
    updated_by BIGINT,
    CONSTRAINT pk_stg_tenant_config PRIMARY KEY (id),
    CONSTRAINT uk_stg_tenant_config_tenant_ns_key UNIQUE (tenant_id, namespace, config_key),
    CONSTRAINT ck_stg_tenant_config_value_type CHECK (value_type IN ('string', 'number', 'boolean', 'object', 'array', 'i18n'))
);

CREATE INDEX IF NOT EXISTS idx_stg_tenant_config_tenant_ns
    ON stg_tenant_config (tenant_id, namespace);

CREATE INDEX IF NOT EXISTS idx_stg_tenant_config_updated_at
    ON stg_tenant_config (updated_at);

-- 系统设置表(三层配置模型的最顶层)
CREATE TABLE IF NOT EXISTS stg_system_setting (
    id BIGINT NOT NULL,
    namespace VARCHAR(64) NOT NULL,
    setting_key VARCHAR(128) NOT NULL,
    setting_value JSONB NOT NULL,
    value_type VARCHAR(16) NOT NULL,
    scope VARCHAR(32) NOT NULL DEFAULT 'global',
    scope_value VARCHAR(64),
    created_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    created_by BIGINT,
    updated_by BIGINT,
    CONSTRAINT pk_stg_system_setting PRIMARY KEY (id),
    CONSTRAINT uk_stg_system_setting_ns_key_scope UNIQUE (namespace, setting_key, scope, scope_value),
    CONSTRAINT ck_stg_system_setting_value_type CHECK (value_type IN ('string', 'number', 'boolean', 'object', 'array', 'i18n')),
    CONSTRAINT ck_stg_system_setting_scope CHECK (scope IN ('global', 'region'))
);

CREATE INDEX IF NOT EXISTS idx_stg_system_setting_ns
    ON stg_system_setting (namespace);

CREATE INDEX IF NOT EXISTS idx_stg_system_setting_scope
    ON stg_system_setting (scope, scope_value);

-- 配置变更历史表(审计与回滚)
CREATE TABLE IF NOT EXISTS stg_config_revision (
    id BIGINT NOT NULL,
    tenant_id TEXT NOT NULL,
    config_type VARCHAR(16) NOT NULL,
    config_id BIGINT NOT NULL,
    namespace VARCHAR(64) NOT NULL,
    config_key VARCHAR(128) NOT NULL,
    old_value JSONB,
    new_value JSONB,
    operation VARCHAR(16) NOT NULL,
    operator_id BIGINT NOT NULL,
    operator_ip VARCHAR(45),
    created_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    CONSTRAINT pk_stg_config_revision PRIMARY KEY (id),
    CONSTRAINT ck_stg_config_revision_type CHECK (config_type IN ('user', 'tenant', 'system')),
    CONSTRAINT ck_stg_config_revision_operation CHECK (operation IN ('create', 'update', 'delete'))
);

CREATE INDEX IF NOT EXISTS idx_stg_config_revision_tenant_type_id_time
    ON stg_config_revision (tenant_id, config_type, config_id, created_at);

CREATE INDEX IF NOT EXISTS idx_stg_config_revision_created_at
    ON stg_config_revision (created_at);
