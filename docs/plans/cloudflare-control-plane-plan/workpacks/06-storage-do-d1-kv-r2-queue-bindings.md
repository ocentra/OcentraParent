# Workpack 06: Storage DO D1 KV R2 Queue Bindings

## Goal

Freeze storage and coordination ownership for Durable Objects, D1, KV, queues, and optional R2.

## First-touch surface

- `infra/cloudflare/src/env.ts`

## Read inputs

- [STORAGE_BINDING_MODEL.md](../STORAGE_BINDING_MODEL.md)
- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)

## Output files

- `infra/cloudflare/src/env.ts`
- [STORAGE_BINDING_MODEL.md](../STORAGE_BINDING_MODEL.md)
- `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/`

## Acceptance

- Each binding has one owner and one purpose.
- No child-data storage drift is allowed.
- Queue and dead-letter ownership is explicit.

## Proof IDs

- `cloudflare-control.do-bindings`
- `cloudflare-control.d1-bindings`
- `cloudflare-control.queue-bindings`
- `cloudflare-control.kv-bindings`
- `cloudflare-control.r2-audit-binding-manual-required`

## Validation

- Scoped validation: `npm --prefix infra/cloudflare run test:unit`
- Scoped validation: `npm --prefix infra/cloudflare run test:integration`
- Scoped validation: `npm --prefix infra/cloudflare run test:property`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare/src/env.ts`

## Negative cases

- Reject optional R2 as a telemetry dump.
- Reject D1/KV claims without privacy boundaries.

## Failure conditions

- Do not imply real binding IDs or runtime success from placeholder config.
