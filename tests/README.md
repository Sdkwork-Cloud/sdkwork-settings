# Tests

Test assets, fixtures, and integration harnesses for the `sdkwork-settings` repository.

Authority: `../sdkwork-specs/TEST_SPEC.md`.

## Structure

- `contract/`: contract tests (API envelope, database framework, SDK generation).
- `integration/`: integration tests across crates and services.
- `e2e/`: end-to-end tests for the full application stack.
- `fixtures/`: shared test fixtures.

## Rules

- Contract tests, SDK generation tests, module tests, security tests, and parity tests follow `TEST_SPEC.md`.
- Rust crate tests live in each crate's `tests/` directory.
- Frontend tests live in each package's `tests/` directory or alongside source files.
