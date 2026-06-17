# Workpack 03: Worker Entrypoint Runtime Guards

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

## Proof IDs

- `cloudflare-control.worker-entrypoint`
- `cloudflare-control.env-validation`
- `cloudflare-control.cors-fail-fast`
- `cloudflare-control.request-size-limit`
- `cloudflare-control.kill-switch`

## Validation

- Scoped validation: `npm --prefix infra/cloudflare run test:unit`
- Scoped validation: `npm --prefix infra/cloudflare run test:integration`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare/src/index.ts`

## Negative cases

- Reject dispatch before guard checks.
- Reject error bodies that imply raw secret or stack leakage.

## Failure conditions

- Do not treat a fail-safe stub as a proven runtime implementation.
