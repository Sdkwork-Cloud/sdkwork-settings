# Scripts

Thin command entrypoints and dev runners for the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/PNPM_SCRIPT_SPEC.md`, `../sdkwork-specs/CODE_STYLE_SPEC.md` section 7.

## Rules

- Build scripts, dev runners, and cross-repository dependency preparation tooling must follow `CODE_STYLE_SPEC.md` section 7 (Build Source Integrity And Self-Healing).
- Build-critical source files must be verified before invoking builds and self-healed from git when missing.
- `pnpm clean` must not delete git-tracked build-critical source files.
- Scripts must be thin command entrypoints that delegate to `pnpm` root scripts or canonical tools.
