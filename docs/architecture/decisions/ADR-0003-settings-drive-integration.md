# ADR-0003: Settings Drive File Upload Integration

| Field | Value |
|-------|-------|
| ADR Number | 0003 |
| Title | Settings Drive File Upload Integration |
| Status | Accepted |
| Date | 2026-07-01 |
| Deciders | SDKWork Architecture |

## Context

Settings 配置中心涉及以下文件上传场景:
1. 用户头像上传(用户偏好中的头像设置)
2. 租户品牌 Logo 上传(租户配置中的品牌设置)
3. 配置导入文件上传(JSON/YAML 格式的配置批量导入)

根据 sdkwork-specs DRIVE_SPEC 要求,所有文件上传必须通过 sdkwork-drive 集成,做到高内聚低耦合。应用不得创建 app-local upload session、presign service 或重复的 `/upload` API。

## Decision

### 1. 集成方式

**前端(PC 应用)**:
- 依赖 `sdkwork-drive-app-sdk`
- 使用 `client.uploader.*` 接口
- 声明 `appId: "settings"`, `appResourceType`, `appResourceId`, `scene`, `source`
- 上传完成后将 Drive 返回的 `fileId` / `downloadUrl` 存入配置值

**后端(Rust 服务)**:
- 依赖 `sdkwork-drive-contract` 和 `sdkwork-drive-uploader-service`
- 使用 `DriveUploaderService` / `PrepareUploaderUploadCommand`
- 后端不直接处理文件流,而是通过 Drive Uploader Service 协调上传

### 2. 上传场景映射

| 场景 | appResourceType | scene | 说明 |
|------|-----------------|-------|------|
| 用户头像 | `user-preference` | `avatar` | 上传后 fileId 存入 `stg_user_preference` namespace=appearance, key=avatar |
| 租户 Logo | `tenant-config` | `logo` | 上传后 fileId 存入 `stg_tenant_config` namespace=branding, key=logo |
| 配置导入 | `config-import` | `import` | 上传后异步处理,通过 Drive 下载文件内容解析 |

### 3. 配置值存储格式

文件上传相关的配置值统一使用以下 JSON 结构:
```json
{
  "fileId": "drive-file-id",
  "fileName": "original-name.png",
  "downloadUrl": "https://drive.sdkwork.com/files/...",
  "size": 524288,
  "mimeType": "image/png"
}
```

### 4. Cargo.toml 依赖(已在根 Cargo.toml 声明)

```toml
sdkwork_drive_contract = { path = "../sdkwork-drive/crates/sdkwork-drive-contract" }
sdkwork_drive_uploader_service = { path = "../sdkwork-drive/crates/sdkwork-drive-uploader-service" }
```

### 5. 前端 pnpm 依赖

```json
{
  "dependencies": {
    "@sdkwork/drive-app-sdk": "workspace:*"
  }
}
```

### 6. 禁止事项

- 禁止创建 app-local upload session
- 禁止创建 presign service
- 禁止重复 `/upload` API
- 禁止后端直接处理文件流(必须通过 Drive Uploader Service)
- 禁止前端直接调用对象存储 SDK

## Consequences

### Positive

- 文件上传能力高内聚到 Drive,Settings 专注配置管理
- 低耦合:Settings 仅存储 fileId 引用,不关心存储细节
- 符合开闭原则:Drive 存储策略变更不影响 Settings
- 复用 Drive 的安全、审计、配额能力

### Negative

- 增加 Drive 依赖,需要 Drive 服务可用
- 配置导入需要异步处理(先上传到 Drive,再下载解析)

## Compliance

- `sdkwork-specs/DRIVE_SPEC.md`
- `sdkwork-specs/MEDIA_RESOURCE_SPEC.md`
- `sdkwork-specs/SECURITY_SPEC.md`(文件安全由 Drive 负责)
