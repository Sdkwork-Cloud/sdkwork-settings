# Local Specs

Module-local spec system for the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/COMPONENT_SPEC.md` section 1.

## Files

- `component.spec.json`: machine-readable integration contract for this repository. Identity, surface, `canonicalSpecs` links, SDK/route/runtime contracts, and verification commands.
- `README.md` (this file): human index for the module spec system.

## Rules

- Module `specs/` `MAY` add narrowing extension files such as `FRONTEND_SPEC.md` or `RELEASE_SPEC.md` only when the module needs rules beyond global standards.
- Module `specs/` `MUST NOT` copy, fork, or paraphrase global `*_SPEC.md` bodies. Link through `canonicalSpecs` instead.
- Durable module rules belong here, not in `AGENTS.md` preserved-guidance blocks or repository README prose.
- When `README` and a spec disagree, the global spec or machine contract wins unless a governance exception exists.
