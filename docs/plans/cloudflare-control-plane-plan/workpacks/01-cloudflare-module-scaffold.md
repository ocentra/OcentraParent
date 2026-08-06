# Workpack 01: Cloudflare Module Scaffold

## Goal

Keep the repo-local `infra/cloudflare/` module shape honest without overclaiming runtime completion.

## Current status

`source-present / focused-validation-recorded / no-production-claim`

## First-touch surface

- `infra/cloudflare/package.json`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PARENT_CLOUDFLARE_MODULE_SPEC.md](../PARENT_CLOUDFLARE_MODULE_SPEC.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)

## Output files

- `infra/cloudflare/`
- `infra/cloudflare/package.json`
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- `output/cloudflare-control-plane-plan-proof/01-cloudflare-module-scaffold/03-package-dependency-graph.md`
- [tracked receipt](../../../proof/cloudflare-control-plane-plan/01-cloudflare-module-scaffold.md)

## Execution truth

- The repo-local `infra/cloudflare/` module tree is real:
  - package scripts exist in `infra/cloudflare/package.json`
  - dev/prod Wrangler configs and `.dev.vars.example` exist
  - concentrated runtime files exist under `src/`
  - real runner/seed scripts exist under `scripts/`
  - real tests exist under `tests/unit`, `tests/integration`, `tests/e2e`, `tests/contract`, `tests/security`, `tests/property`, and `tests/fuzz`
  - module-local docs exist under `docs/`
- The concentrated runtime versus placeholder subdirectories is explicit:
  - real runtime files include `src/index.ts`, `src/env.ts`, `src/routes.ts`, `src/billing-binding-read-model.ts`, `src/fixtures.ts`, `src/testing.ts`, `src/auth/model.ts`, `src/auth/verifier.ts`, and `src/security/redaction.ts`
  - scaffold-only directories still carry `README.md` placeholders under `src/durable-objects/`, `src/flows/`, `src/handlers/`, `src/observability/`, `src/providers/`, `src/queues/`, and `src/storage/`
- The former missing `billing-domain` import blocker is stale: `infra/cloudflare` now consumes module-local generated billing contracts.
- The selected dependency graph now resolves normally: `wrangler@4.115.0` with `@cloudflare/workers-types@5.20260804.1`, from the declared `^5.20260722.1` range. The former Wrangler/Workers Types peer mismatch is closed for this module graph.
- Focused current gates pass: module lint; unit `49/49`; contract `14/14`; integration `70/70`, including real local Wrangler Worker boot; and Cloudflare architecture policy. The detailed command log and negative/teardown records are retained under `output/cloudflare-control-plane-plan-proof/01-cloudflare-module-scaffold/` with a tracked receipt alongside this plan.

## Acceptance

- The module tree exists.
- Package scripts are explicit.
- The concentrated runtime surface versus placeholder subdirectories is explicit.
- The selected `wrangler` and `@cloudflare/workers-types` declarations resolve without a peer mismatch, with the exact resolver output retained in `03-package-dependency-graph.md`.
- WP07 remains a separate proof-only workpack and is not closed or started by this WP01 refresh.

## Proof IDs

- `cloudflare-control.module-exists`
- `cloudflare-control.package-scripts`

## Validation

- Scoped validation: `npm --prefix infra/cloudflare run lint`
- Dependency reconciliation: `npm --prefix infra/cloudflare install --ignore-scripts --no-audit --no-fund --no-package-lock`
- Resolved graph: `npm --prefix infra/cloudflare ls wrangler @cloudflare/workers-types`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare`

## Negative cases

- Reject fake `implemented` claims from empty files or README-only directories.

## Failure conditions

- Do not claim module completion from directory existence alone.

## Current focused validations

- `npm --prefix infra/cloudflare run lint` -> passed
- `npm --prefix infra/cloudflare run test:unit` -> passed (49 tests across 7 suites)
- `npm --prefix infra/cloudflare run test:contract` -> passed (14 tests)
- `npm --prefix infra/cloudflare run test:integration` -> passed (70 tests, including real local Wrangler Worker boot)
- `npm run lint:architecture -- --files infra/cloudflare` -> passed (architecture-policy and generated-artifacts)

## No-claim boundary

- This workpack does not claim payment readiness, account authority, trusted-device authority, storage operations readiness, or deployment readiness.
- This workpack does not claim runtime completeness from placeholder directories, docs, or script presence alone.
- This workpack claims only its scaffold/package-script acceptance and its scoped validation. It does not claim broader Cloudflare runtime readiness.
- This workpack records a clean local dependency graph and focused validation only; it does not grant a bypass for WP07's separate local-dev proof requirements.
