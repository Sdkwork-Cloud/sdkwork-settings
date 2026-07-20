# Crates

Rust contracts, route adapters, and runtime service libraries for the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/RUST_CODE_SPEC.md`, `../sdkwork-specs/NAMING_SPEC.md`.

## Crate Naming

Business logic: `sdkwork-<domain>-<capability>-service`
SQLx database access: `sdkwork-<domain>-<capability>-repository-sqlx`
HTTP route adapters: `sdkwork-routes-<capability>-<surface>`
Database module host: `sdkwork-<application-code>-database-host`
Web bootstrap: `sdkwork-<application-code>-web-bootstrap`
Gateway assembly: `sdkwork-<application-code>-gateway-assembly`
In-process service container: `sdkwork-<application-code>-service-host`

## Active Crates

- `sdkwork-settings-contract/`: domain contracts, DTOs, error types.
- `sdkwork-settings-database-host/`: database module bootstrap and schema registration.
- `sdkwork-settings-web-bootstrap/`: web framework bootstrap and route assembly.
- `sdkwork-api-settings-assembly/`: gateway assembly manifest.
- `sdkwork-settings-service-host/`: in-process service container.
- `sdkwork-routes-settings-app-api/`: app-api route crate.
- `sdkwork-routes-settings-backend-api/`: backend-api route crate.

## Rules

- Rust `src/lib.rs` is a module assembly and re-export boundary. It must not become a catch-all file.
- Crates must be named by responsibility. Generic suffixes (`product`, `runtime`, `backend`, `core`, `common`, `manager`) are not compliant.
