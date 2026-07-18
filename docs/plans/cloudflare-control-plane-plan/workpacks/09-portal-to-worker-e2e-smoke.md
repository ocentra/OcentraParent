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

- `blocked / proof-required`
- Proof root: expected output path `output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/`

## Execution truth

- The owned e2e family stays explicitly scoped to `tests/e2e/portal-to-worker-billing-status.test.ts`.
- The test contract stays explicitly scoped to `/auth/billing/status`.
- The owned command blocked before route execution because the rerun set against the live generated billing-contract source surface has not been refreshed here.
- Secret/private leakage assumptions remain denied by the owned test contract, but runtime success is not proven because the worker never booted.

## Exact blocker set

- `cmd /c npm --prefix infra/cloudflare run test:e2e` remains blocked on the live generated billing-contract source surface rerun gap

## Validations run

- `cmd /c npm --prefix infra/cloudflare run test:e2e`

## No-claim boundary

- This workpack does not prove a green portal-to-worker smoke.
- This workpack does not prove checkout, admin, or provider flows.
- This workpack does not prove payment readiness or deployment readiness.

## Failure conditions

- Reject portal consumer assumptions that skip auth or redaction.
- Do not use portal docs as proof that the worker route works.
