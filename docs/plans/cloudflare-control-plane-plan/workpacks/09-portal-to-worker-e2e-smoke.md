# Workpack 09: Portal To Worker E2E Smoke

> **2026-07-28 correction:** The later missing-private-billing-import blocker text is historical. `infra/cloudflare` now imports module-local generated billing contracts. This workpack remains open because it has no tracked proof bundle; rerun after installing dependencies and record the actual result.

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

- `blocked / proof-present`
- Proof root: `output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/`

## Execution truth

- The owned e2e family stays explicitly scoped to `tests/e2e/portal-to-worker-billing-status.test.ts`.
- The test contract stays explicitly scoped to `/auth/billing/status`.
- The owned command is deferred before route execution because WP01's module dependency tree is currently empty; the worker imports module-local generated billing contracts rather than private billing-domain source.
- Secret/private leakage assumptions remain denied by the owned test contract, but runtime success is not proven because the worker never booted.

## Exact blocker set

- Restore the WP01 resolver graph, then retain the actual portal-to-worker boot or route blocker if one remains.

## Validations run

- `cmd /c npm --prefix infra/cloudflare run test:e2e`

## No-claim boundary

- This workpack does not prove a green portal-to-worker smoke.
- This workpack does not prove checkout, admin, or provider flows.
- This workpack does not prove payment readiness or deployment readiness.

## Failure conditions

- Reject portal consumer assumptions that skip auth or redaction.
- Do not use portal docs as proof that the worker route works.
