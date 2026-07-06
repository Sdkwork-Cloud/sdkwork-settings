# Apps

Runnable application surfaces for the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/APPLICATION_SPEC.md`, `../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`.

## Active App Roots

- `sdkwork-settings-pc/`: PC browser/desktop application root (React + Vite + Tauri-capable).

## Rules

- `apps/` is a collection of selected language/architecture application roots.
- Every independent application repository must provide `apps/README.md` as the directory index per `DOCUMENTATION_SPEC.md` section 3.3.
- Architecture-specific `src/`, `packages/`, and `config/` directories belong inside a child app root, not at the repository root.
