# Configs

Configuration templates and topology profiles for the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/CONFIG_SPEC.md`, `../sdkwork-specs/ENVIRONMENT_SPEC.md`, `../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md`.

## Structure

- `topology/`: deployment topology env snippets (standalone/cloud profiles).
- `*.example.toml`: configuration file examples (database, gateway, etc.).

## Rules

- Dev/test/staging/prod config templates must be safe checked-in examples.
- Host-local files such as `.env.local`, `.env.<profile>.local`, `.env.postgres`, and `config/*.local.toml` must be ignored.
- Separate browser public runtime config, desktop user runtime config, desktop-started server config, container runtime config, and Tauri platform config per `CONFIG_SPEC.md`.
