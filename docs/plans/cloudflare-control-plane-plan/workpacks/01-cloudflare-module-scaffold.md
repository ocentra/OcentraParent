# Workpack 01: Cloudflare Module Scaffold

## Goal

Keep the repo-local `infra/cloudflare/` module shape honest without overclaiming runtime completion.

## Current status

`proved / scoped-package-graph`

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
- `docs/proof/cloudflare-control-plane-plan/01-cloudflare-module-scaffold/03-package-dependency-graph.md`
- `docs/proof/cloudflare-control-plane-plan/01-cloudflare-module-scaffold/04-module-scaffold-and-scripts.md`
- `docs/proof/cloudflare-control-plane-plan/01-cloudflare-module-scaffold/16-validation-commands.log`

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
- WP01 reconciles the declared peer relationship with `@cloudflare/workers-types ^5.20260730.1`. A clean module install resolves `wrangler@4.118.0` with deduped `@cloudflare/workers-types@5.20260804.1`; the durable `docs/proof/cloudflare-control-plane-plan/01-cloudflare-module-scaffold/03-package-dependency-graph.md` retains the command result. The local file dependency on `@ocentra-parent/logging-domain` must be built before the module lint resolves its exported test-log declarations; that preparation is a local validation prerequisite, not a Cloudflare product change.

## Acceptance

- The module tree exists.
- Package scripts are explicit.
- The concentrated runtime surface versus placeholder subdirectories is explicit.
- Static module-tree and package-script evidence is retained in `04-module-scaffold-and-scripts.md` for `cloudflare-control.module-exists` and `cloudflare-control.package-scripts`.
- The selected `wrangler` and `@cloudflare/workers-types` declarations resolve without a peer mismatch, with the exact resolver output retained in `03-package-dependency-graph.md`.
- WP07 remains blocked until WP01 retains that clean graph plus the focused command results; a recorded resolver blocker is not permission to run or close WP07.

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

## Current scoped validation

- `npm --prefix infra/cloudflare install --ignore-scripts --no-audit --no-fund --no-package-lock` -> passed
- `npm --prefix infra/cloudflare ls wrangler @cloudflare/workers-types` -> passed with `wrangler@4.118.0` and deduped `@cloudflare/workers-types@5.20260804.1`
- `npm --prefix infra/cloudflare run lint` -> passed after building the declared local logging-domain dependency
- `npm --prefix infra/cloudflare run test:unit` -> passed (49 tests across 7 suites)
- `npm run lint:architecture -- --files infra/cloudflare` -> passed (architecture-policy and generated-artifacts)

The retained WP01 files record this package-scope result. They make WP07's
separate proof-only work selectable, but do not run or close WP07.

## No-claim boundary

- This workpack does not claim payment readiness, account authority, trusted-device authority, storage operations readiness, or deployment readiness.
- This workpack does not claim runtime completeness from placeholder directories, docs, or script presence alone.
- This workpack claims only its scaffold/package-script acceptance and its scoped validation. It does not claim broader Cloudflare runtime readiness.
- This workpack gates WP07's proof-only successor on the retained clean Wrangler/Workers-types graph; it does not grant a bypass from a precise dependency blocker.
