# Workpack 01: Cloudflare Module Scaffold

## Goal

Keep the repo-local `infra/cloudflare/` module shape honest without overclaiming runtime completion.

## Current status

`scoped acceptance complete / source-and-validation-evidence`

## First-touch surface

- `infra/cloudflare/package.json`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PARENT_CLOUDFLARE_MODULE_SPEC.md](../PARENT_CLOUDFLARE_MODULE_SPEC.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)

## Output files

- `infra/cloudflare/`
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- This workpack and `PLAN_STATE.md` retain the scoped evidence; generated `output/` artifacts are ignored and not committed.

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
- Current source and validation evidence is recorded in this workpack and `PLAN_STATE.md`; generated `output/` artifacts are intentionally ignored and not treated as tracked source proof.
- Current-code audit on 2026-07-28 found the previous missing billing-domain import blocker stale: `infra/cloudflare` now consumes its local generated billing contracts. `npm --prefix infra/cloudflare run lint`, `npm --prefix infra/cloudflare run test:unit` (49 tests), and the module-directory architecture gate all pass.

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

- `npm --prefix infra/cloudflare run lint` -> passed
- `npm --prefix infra/cloudflare run test:unit` -> passed (49 tests across 7 suites)
- `npm run lint:architecture -- --files infra/cloudflare` -> passed (architecture-policy and generated-artifacts)

## No-claim boundary

- This workpack does not claim payment readiness, account authority, trusted-device authority, storage operations readiness, or deployment readiness.
- This workpack does not claim runtime completeness from placeholder directories, docs, or script presence alone.
- This workpack claims only its scaffold/package-script acceptance and its scoped validation. It does not claim broader Cloudflare runtime readiness.
