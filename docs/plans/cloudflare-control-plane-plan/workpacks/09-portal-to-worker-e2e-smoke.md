# Workpack 09: Portal To Worker E2E Smoke

## Goal

Define the first consumer-side smoke that proves a parent portal route can talk to the shared worker boundary safely.

## First-touch surface

- `infra/cloudflare/tests/e2e/portal-to-worker-billing-status.test.ts`

## Read inputs

- [ROUTE_MANIFEST_MODEL.md](../ROUTE_MANIFEST_MODEL.md)
- [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)
- [REQUIRED_TEST_ASSERTION_MATRIX.md](../REQUIRED_TEST_ASSERTION_MATRIX.md)

## Output files

- `infra/cloudflare/tests/e2e/`
- `output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/`

## Acceptance

- Consumer scope is explicit.
- Secret or private data leakage is explicitly denied.
- The smoke can block payment if absent.

## Status

- `proof-required`
- Proof root: expected output path `output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/`

## Execution truth

- The owned e2e family stays explicitly scoped to `tests/e2e/portal-to-worker-billing-status.test.ts`.
- The test contract stays explicitly scoped to `/auth/billing/status`.
- The owned e2e family is green in the current execution record; the remaining gap is the absent proof artifact in this checkout.
- Secret/private leakage assumptions remain denied by the owned test contract, and runtime success is recorded rather than invented.

## Exact blocker set

- `cmd /c npm --prefix infra/cloudflare run test:e2e` is green in the current execution record; the remaining blocker is the absent proof artifact

## Validations run

- `cmd /c npm --prefix infra/cloudflare run test:e2e` -> green in the current execution record

## No-claim boundary

- This workpack does not prove the proof artifact.
- This workpack does not prove checkout, admin, or provider flows.
- This workpack does not prove payment readiness or deployment readiness.

## Failure conditions

- Reject portal consumer assumptions that skip auth or redaction.
- Do not use portal docs as proof that the worker route works.
