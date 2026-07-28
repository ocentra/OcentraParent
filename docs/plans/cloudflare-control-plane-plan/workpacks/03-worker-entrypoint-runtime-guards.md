# Workpack 03: Worker Entrypoint Runtime Guards

> **2026-07-28 correction:** The later missing-private-billing-import blocker text is historical. `infra/cloudflare` now imports module-local generated billing contracts. This workpack remains open because it has no tracked proof bundle; rerun after installing dependencies and record the actual result.

## Goal

Freeze the shared guard chain for env validation, CORS, request-size limits, kill switch, safe errors, and scheduled hooks.

## First-touch surface

- `infra/cloudflare/src/index.ts`

## Read inputs

- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)
- [DEPLOYMENT_MODEL.md](../DEPLOYMENT_MODEL.md)

## Output files

- `infra/cloudflare/src/index.ts`
- `output/cloudflare-control-plane-plan-proof/03-worker-entrypoint-runtime-guards/`

## Acceptance

- The fetch path fails safe before dispatch.
- Scheduled reconciliation hook shape is explicit.
- Logging is redacted by default.

## Status

- `blocked / proof-present`
- Proof root: `output/cloudflare-control-plane-plan-proof/03-worker-entrypoint-runtime-guards/`

## Execution truth

- The shared worker entrypoint still owns env validation, CORS fail-fast, request-size limits, kill switch behavior, safe errors, and scheduled hook shape.
- Unit suites that do not require `src/index.ts` boot passed.
- Unit and integration suites that require the real worker entrypoint remain blocked before route execution because `src/index.ts` cannot import `packages/billing-domain/src/billing-checkout-portal-boundary.js`.
- The scheduled hook shape is explicit in source but not runtime-proven.

## Exact blocker set

- `packages/billing-domain/src/billing-checkout-portal-boundary.js`

## Validations run

- `cmd /c npm --prefix infra/cloudflare run test:unit`
- `cmd /c npm --prefix infra/cloudflare run test:integration`
- `npm run lint:architecture -- --files infra/cloudflare/src/index.ts`

## No-claim boundary

- This workpack does not prove a green worker boot.
- This workpack does not prove live runtime guard success through dispatch.
- This workpack does not prove deployment, portal smoke, or payment readiness.

## Failure conditions

- Reject dispatch before guard checks.
- Reject error bodies that imply raw secret or stack leakage.
- Do not treat a fail-safe stub as a proven runtime implementation.
