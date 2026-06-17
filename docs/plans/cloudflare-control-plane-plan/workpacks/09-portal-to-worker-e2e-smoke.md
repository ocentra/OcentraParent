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

## Proof IDs

- `cloudflare-control.portal-to-worker-smoke`

## Validation

- `npm --prefix infra/cloudflare run test:e2e`

## Negative cases

- Reject portal consumer assumptions that skip auth or redaction.

## Failure conditions

- Do not use portal docs as proof that the worker route works.
