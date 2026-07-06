-- SDKWork Settings bootstrap 种子数据(跨语言通用)
-- 注入系统默认设置,提供三层配置模型的顶层默认值。
-- tenant_id = '0' 表示系统级(非租户专属)。

-- 系统设置:外观默认值
INSERT INTO stg_system_setting (id, namespace, setting_key, setting_value, value_type, scope, scope_value, created_by, updated_by)
VALUES
    (1001, 'appearance', 'theme', '"light"', 'string', 'global', NULL, 0, 0),
    (1002, 'appearance', 'fontFamily', '"system-ui"', 'string', 'global', NULL, 0, 0),
    (1003, 'appearance', 'density', '"comfortable"', 'string', 'global', NULL, 0, 0)
ON CONFLICT (namespace, setting_key, scope, scope_value) DO NOTHING;

-- 系统设置:本地化默认值
INSERT INTO stg_system_setting (id, namespace, setting_key, setting_value, value_type, scope, scope_value, created_by, updated_by)
VALUES
    (1101, 'locale', 'language', '"zh-CN"', 'string', 'global', NULL, 0, 0),
    (1102, 'locale', 'timezone', '"Asia/Shanghai"', 'string', 'global', NULL, 0, 0),
    (1103, 'locale', 'dateFormat', '"YYYY-MM-DD"', 'string', 'global', NULL, 0, 0),
    (1104, 'locale', 'timeFormat', '"HH:mm:ss"', 'string', 'global', NULL, 0, 0)
ON CONFLICT (namespace, setting_key, scope, scope_value) DO NOTHING;

-- 系统设置:通知默认值
INSERT INTO stg_system_setting (id, namespace, setting_key, setting_value, value_type, scope, scope_value, created_by, updated_by)
VALUES
    (1201, 'notification', 'emailEnabled', 'true', 'boolean', 'global', NULL, 0, 0),
    (1202, 'notification', 'pushEnabled', 'true', 'boolean', 'global', NULL, 0, 0),
    (1203, 'notification', 'desktopEnabled', 'false', 'boolean', 'global', NULL, 0, 0)
ON CONFLICT (namespace, setting_key, scope, scope_value) DO NOTHING;

-- 系统设置:隐私默认值
INSERT INTO stg_system_setting (id, namespace, setting_key, setting_value, value_type, scope, scope_value, created_by, updated_by)
VALUES
    (1301, 'privacy', 'analyticsOptIn', 'false', 'boolean', 'global', NULL, 0, 0),
    (1302, 'privacy', 'crashReportOptIn', 'true', 'boolean', 'global', NULL, 0, 0)
ON CONFLICT (namespace, setting_key, scope, scope_value) DO NOTHING;

-- 系统设置:功能开关默认值
INSERT INTO stg_system_setting (id, namespace, setting_key, setting_value, value_type, scope, scope_value, created_by, updated_by)
VALUES
    (1401, 'feature-flag', 'betaFeatures', 'false', 'boolean', 'global', NULL, 0, 0),
    (1402, 'feature-flag', 'experimentalUi', 'false', 'boolean', 'global', NULL, 0, 0)
ON CONFLICT (namespace, setting_key, scope, scope_value) DO NOTHING;
