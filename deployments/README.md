# Deployments

Deployment descriptors and runbooks for the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/DEPLOYMENT_SPEC.md`, `../sdkwork-specs/SDKWORK_DEPLOY_SPEC.md`.

## Files

- `deploy.yaml`: per-application deploy manifest, install layouts, adaptive Web, API/WebSocket inference, Nginx site generation, client package orchestration per `SDKWORK_DEPLOY_SPEC.md`.
- `kubernetes/`: Kubernetes deployment descriptors (when applicable).
- `observability/`: observability deployment config (when applicable).

## Rules

- Standalone/cloud deployment profile parity must be maintained per `DEPLOYMENT_SPEC.md`.
- Application deploy manifests follow `SDKWORK_DEPLOY_SPEC.md` and are validated by `deployctl`.
