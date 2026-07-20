# Services

Reserved directory for future non-HTTP service processes owned by the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/RUST_CODE_SPEC.md`, `../sdkwork-specs/APPLICATION_GATEWAY_SPEC.md`.

## Active Runtime Placement

No active Settings application HTTP ingress lives under `services/`.

The current application public ingress is `crates/sdkwork-api-settings-standalone-gateway/`.
It composes application routes through `crates/sdkwork-api-settings-assembly/` and serves the `application.public-ingress` surface for both `standalone.*` and the current Settings `cloud.*` profiles.

The shared platform plane remains `sdkwork-api-cloud-gateway` on `platform.api-gateway`.

## Rules

- Do not add any `*-api-server` listener as the default application ingress.
- Do not add extra application-plane HTTP listeners for app-api or backend-api sidecars.
- Future workers or schedulers may live here only when they do not terminate `application.public-ingress`.
