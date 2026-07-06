# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v2 -->

## SDKWORK Soul

Read `../../../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../../../sdkwork-specs/README.md`
- `../../../sdkwork-specs/SOUL.md`
- `../../../sdkwork-specs/AGENTS_SPEC.md`
- `../../../sdkwork-specs/PNPM_SCRIPT_SPEC.md`
- `../../../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`
- `../../../sdkwork-specs/FRONTEND_CODE_SPEC.md`
- `../../../sdkwork-specs/TAILWIND_CSS_INTEGRATION_SPEC.md`
- `../../../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../../../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this application root. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

This is the PC browser application root for SDKWork Settings (`apps/sdkwork-settings-pc/`). The parent repository `sdkwork-settings` owns the application identity, backend services, API contracts, and SDK generation. Read `../../sdkwork.app.config.json` for application identity, runtime config, SDK wiring, release metadata, or app-owned capabilities.

Application code: `settings`. Domain: `system`.

## Local Dictionary Structure

- `AGENTS.md`: PC application agent entrypoint and relative SDKWork spec index.
- `sdkwork.app.config.json`: PC application-level composition config (references parent `../../sdkwork.app.config.json` as root config).
- `specs/`: local PC application contracts and narrowing rules.
- `src/`: React + TypeScript source code.
- `config/browser/`: browser runtime environment config templates.
- `package.json`: pnpm package manifest with standard scripts (`dev`, `build`, `lint`, `preview`, `clean`).

## Spec Resolution Order

1. Read this `AGENTS.md`.
2. Read `../../AGENTS.md` for repository-level rules.
3. Read `../../sdkwork.app.config.json` only when app identity, runtime config, SDK wiring, release, packaging, or owned capabilities are touched.
4. Read local `specs/README.md` only when local contracts are relevant.
5. Read `../../../sdkwork-specs/README.md`, then only the task-specific root specs.
6. Inspect implementation files after the dictionary and relevant specs are clear.

Do not load all specs, generated SDKs, or source trees before the task surface is known.

## Required Specs By Task Type

- Agent/workflow changes: `../../../sdkwork-specs/SOUL.md`, `../../../sdkwork-specs/AGENTS_SPEC.md`, `../../../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`, and `../../../sdkwork-specs/TEST_SPEC.md`.
- Package script changes: `../../../sdkwork-specs/PNPM_SCRIPT_SPEC.md`, `../../../sdkwork-specs/CONFIG_SPEC.md`, and `../../../sdkwork-specs/TEST_SPEC.md`.
- Any code change: `../../../sdkwork-specs/CODE_STYLE_SPEC.md`, `../../../sdkwork-specs/NAMING_SPEC.md`, plus only the touched language/framework spec.
- TypeScript/Node code: `../../../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`.
- Frontend/UI code: `../../../sdkwork-specs/FRONTEND_CODE_SPEC.md`, `../../../sdkwork-specs/TAILWIND_CSS_INTEGRATION_SPEC.md` when Tailwind CSS is touched, `../../../sdkwork-specs/UI_ARCHITECTURE_SPEC.md`, and exactly one detailed UI architecture spec.
- API/SDK changes: `../../../sdkwork-specs/API_SPEC.md`, `../../../sdkwork-specs/SDK_SPEC.md`, `../../../sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md`, and `../../../sdkwork-specs/TEST_SPEC.md` as applicable.
- Runtime/deployment/release changes: `../../../sdkwork-specs/CONFIG_SPEC.md`, `../../../sdkwork-specs/DEPLOYMENT_SPEC.md`, `../../../sdkwork-specs/RELEASE_SPEC.md`, and `../../../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md`.

Language-specific specs are on-demand; do not load Rust, Java, and frontend specs for unrelated tasks.

## Code Style Rules

Read `../../../sdkwork-specs/CODE_STYLE_SPEC.md` and `../../../sdkwork-specs/NAMING_SPEC.md` before code changes. Use function components and Hooks. TypeScript strict mode. Tailwind CSS v4 single bootstrap through `@tailwindcss/vite`.

## Build, Test, and Verification

- `pnpm dev`: start Vite dev server.
- `pnpm build`: type-check and build for production.
- `pnpm lint`: TypeScript type-check.
- `pnpm clean`: remove `dist/`.

Run the narrowest relevant check first, then broader verification when API contracts, SDK generation, or cross-package boundaries change.

## Agent Execution Rules

Use dynamic progressive loading and the convention dictionary instead of broad context loading. Do not hand-edit generated SDK output unless the source contract is verified. Do not replace generated SDK integration with raw HTTP. Do not preserve retired commands, copied workflow bodies, or legacy local guidance blocks. Record exact verification commands and important outputs before reporting completion.

## List And Search Pagination

All L2+ list/search APIs and their backing services, repositories, SDK consumers, and interactive frontend lists `MUST` follow `PAGINATION_SPEC.md`:

- **Input:** standard `SdkWorkListQuery` or query params (`page`/`page_size` or `cursor`/`page_size` per `API_SPEC.md` §14.1); default `page_size` `20`; max `200` unless a documented exception exists.
- **Output:** `SdkWorkApiResponse.data.items` + `data.pageInfo` with `PageInfo.mode` (`offset` or `cursor`) per `API_SPEC.md` §16.
- **Store-level pagination:** push filtering, sorting, and page selection to SQL `LIMIT`/keyset or incrementally maintained indexes — never unbounded collect then `skip`/`take`/`slice` in process memory (`PAGINATION_SPEC.md` §2).
- **SDK and frontend:** interactive lists request one page at a time from the server; no default `listAll*` on P0/P1 paths; no client-side `slice` pagination over full downloads.

Before completing list/search API, repository, SDK list helper, projection read model, or paginated UI work, run:

```bash
node <sdkwork-specs>/tools/check-pagination.mjs --workspace <workspace-root>
```

Authority: `PAGINATION_SPEC.md`, `API_SPEC.md` §14.1/§16, `DATABASE_SPEC.md` §20.5, `WEB_BACKEND_SPEC.md` §12, `SDK_SPEC.md` §4.2/§6, `FRONTEND_SPEC.md`, `APP_SDK_INTEGRATION_SPEC.md` §9.

## Human Review Rules

Request human review before breaking SDKWork standards, changing public naming, altering security/auth behavior, changing database migrations or production deployment config, deleting data/files, changing generated SDK ownership, or modifying release/deployment governance. Surface unresolved spec paths, app identity conflicts, component ownership conflicts, and API authority ambiguity instead of guessing.

## App SDK Consumer Imports



Application, feature, shell, and service packages `MUST` consume HTTP SDKs through scoped composed consumer packages, not generator transport package names.



- App API clients: `@sdkwork/<application-code>-app-sdk`

- Backend API clients (`backend-admin` only): `@sdkwork/<application-code>-backend-sdk`

- Federated Claw Router domain surfaces: `@sdkwork/clawrouter-app-sdk/domains` and `@sdkwork/clawrouter-backend-sdk/domains`

- Open/domain API clients: `@sdkwork/<domain>-sdk`



Canonical examples (IAM):



```typescript

import { createClient, type SdkworkAppClient } from '@sdkwork/iam-app-sdk';

import type { SdkworkBackendClient } from '@sdkwork/iam-backend-sdk'; // backend-admin only

import { createClient as createClawRouterDomainsClient } from '@sdkwork/clawrouter-app-sdk/domains';

```



Forbidden in application `apps/`, `packages/`, bootstrap, services, UI, contract tests, and composed SDK `src/**` outside generator ownership:



- `sdkwork-*-app-sdk-generated-typescript`, `sdkwork-*-backend-sdk-generated-typescript`, and other generator transport names as consumer imports

- `@sdkwork/commerce-app-sdk`, `@sdkwork/commerce-backend-sdk`, `@sdkwork/clawrouter-*-domain-transport-sdk`

- filesystem paths containing `domain-transport-typescript`, `domain-transport-sdk`, or sibling `*-typescript/generated` hops from composed `src/**`

- deep imports into `generated/server-openapi/src/*` from consumers when a composed facade exists



Allowed:



- Composed facade entry imports such as `@sdkwork/iam-app-sdk`, `@sdkwork/knowledgebase-app-sdk`, and `@sdkwork/clawrouter-app-sdk/domains`

- Composed re-exports that import only from `../generated/**` within the same `*-sdk-typescript` family root

- Generated transport ownership inside `sdks/**/generated/**` only



Each SDK family `MUST` expose the composed TypeScript facade at `sdks/<sdk-family>/<sdk-family>-typescript/src/index.ts` (and optional subpath exports such as `./domains`) with `package.json#name` equal to the scoped consumer package.



Before completing SDK integration or frontend service work, run:



```bash

node <sdkwork-specs>/tools/check-app-sdk-consumer-imports.mjs --workspace <workspace-root>

```



Authority: `APP_SDK_INTEGRATION_SPEC.md` section 9, `SDK_SPEC.md` package naming table, `SDK_WORKSPACE_GENERATION_SPEC.md` composed facade rules.



