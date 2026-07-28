# Workpack 01: Cloudflare Module Scaffold

## Goal

Keep the repo-local `infra/cloudflare/` module shape honest without overclaiming runtime completion.

## Current status

`blocked / proof-required`

## First-touch surface

- `infra/cloudflare/package.json`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PARENT_CLOUDFLARE_MODULE_SPEC.md](../PARENT_CLOUDFLARE_MODULE_SPEC.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)

## Output files

- `infra/cloudflare/`
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- `output/cloudflare-control-plane-plan-proof/01-cloudflare-module-scaffold/`

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
- The proof root `output/cloudflare-control-plane-plan-proof/01-cloudflare-module-scaffold/` is an expected output path, but no tracked proof root exists in this checkout.
- The packet remains proof-required because the current checkout still lacks the physical proof bundle, and the scaffold-only directories still remain placeholders.

## Acceptance

- The module tree exists.
- Package scripts are explicit.
- The concentrated runtime surface versus placeholder subdirectories is explicit.

## Proof IDs

- `cloudflare-control.module-exists`
- `cloudflare-control.package-scripts`

## Validation

- Scoped validation: `npm --prefix infra/cloudflare run lint`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare`

## Negative cases

- Reject fake `implemented` claims from empty files or README-only directories.

## Failure conditions

- Do not claim module completion from directory existence alone.

## Exact validations

- `cmd /c npm --prefix infra/cloudflare run lint` -> blocked
- `cmd /c npm run lint:architecture -- --files infra/cloudflare/src/index.ts infra/cloudflare/src/env.ts infra/cloudflare/src/routes.ts infra/cloudflare/src/auth/verifier.ts infra/cloudflare/scripts/test-runner.ts infra/cloudflare/tests/unit/env-bindings.test.ts infra/cloudflare/tests/unit/route-manifest.test.ts` -> exited 0 but matched zero files, recorded here as a historical/non-proving command result

## No-claim boundary

- This workpack does not claim payment readiness, account authority, trusted-device authority, storage operations readiness, or deployment readiness.
- This workpack does not claim runtime completeness from placeholder directories, docs, or script presence alone.
- This workpack does not claim validation-green status while module lint is blocked and the architecture helper is currently skipping the requested file set.
