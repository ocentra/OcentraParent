# Workpack 09: Portal To Worker E2E Smoke

## Goal

Define the first consumer-side smoke that proves a parent portal route can talk to the shared worker boundary safely.

## First-touch surface

- `docs/proof/cloudflare-control-plane-plan/wp09-portal-to-worker-e2e-smoke/`

## Read inputs

- [ROUTE_MANIFEST_MODEL.md](../ROUTE_MANIFEST_MODEL.md)
- [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)

## Output files

- `infra/cloudflare/tests/e2e/`
- `docs/proof/cloudflare-control-plane-plan/wp09-portal-to-worker-e2e-smoke/`

## Acceptance

- Consumer scope is explicit.
- Secret or private data leakage is explicitly denied.
- The smoke can block payment if absent.

## Proof IDs

- `cloudflare-control.portal-to-worker-smoke`

## Validation

- Docs or scaffold validation: `npm run format:check`

## Negative cases

- Reject portal consumer assumptions that skip auth or redaction.

## Failure conditions

- Do not use portal docs as proof that the worker route works.
