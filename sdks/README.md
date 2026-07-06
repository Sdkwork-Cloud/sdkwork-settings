# SDKs

SDK families, OpenAPI authorities, route manifests, and generated SDK artifacts for the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/SDK_SPEC.md`, `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`.

## Active SDK Families

- `sdkwork-settings-app-sdk/`: generated app-api SDK (consumer-facing, authenticated).
- `sdkwork-settings-backend-sdk/`: generated backend-api SDK (admin/operator surface).

## Rules

- Generated SDK family workspaces and generated transport output live under `sdks/`.
- Authored API contracts, examples, changelogs, and validation fixtures live under `apis/`.
- SDK generation uses the canonical `@sdkwork/sdk-generator` / `sdkgen` with `--standard-profile sdkwork-v3`.
- Generated HTTP SDKs unwrap `data` by default; use `.raw` when the full envelope is required.
- Do not hand-edit generated SDK output. Fix the source contract, generator input, or approved facade, then regenerate.
