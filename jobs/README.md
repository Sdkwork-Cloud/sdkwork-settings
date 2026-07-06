# Jobs

Background jobs and scheduled tasks for the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/RUST_CODE_SPEC.md` (worker naming: `sdkwork-<domain>-<capability>-worker`).

## Rules

- Background workers use `sdkwork-<domain>-<capability>-worker` naming.
- Jobs that require persistence follow `DATABASE_SPEC.md` and `DATABASE_FRAMEWORK_SPEC.md`.
- Async jobs and event-driven work follow `EVENT_SPEC.md`.
