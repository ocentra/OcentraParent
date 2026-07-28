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

## Completion

- Status: blocked / proof-required for WP06 only; no Cloudflare runtime-ready, deployment-ready, or payment-ready claim is made.
- Proof root: expected output path `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/`
- Runtime/source owner: `infra/cloudflare/src/env.ts`
- Owned test surface: `infra/cloudflare/tests/unit/env-bindings.test.ts`

## What is actually proved

- Durable Object ownership is explicit for `BILLING_DO`, `REFERRAL_DO`, and `ENTITLEMENT_SNAPSHOT_DO`, each with one owner, one purpose, and explicit child-data prohibition.
- D1 ownership is explicit for `BILLING_D1` as billing/support/reconciliation read-model storage only.
- KV ownership is explicit for `BILLING_RATE_LIMIT_KV` and `BILLING_CONFIG_KV`, with child-data and provider-secret boundaries kept explicit.
- Queue ownership is explicit for `BILLING_RECONCILIATION_QUEUE` and `BILLING_DEAD_LETTER_QUEUE`, including paired dead-letter responsibility.
- Optional `BILLING_AUDIT_R2` stays `manual-required` and is explicitly rejected as a telemetry dump or general-purpose child-data store.
- Placeholder Wrangler binding names and IDs are treated as non-proof and non-runtime-ready.

## Blocked truth

- `test:unit`, `test:integration`, and `test:property` passed at source base `2aab6310c`; the compact retained receipt records their exact local-only boundary.
- WP06 remains proof-required because its retained receipt is not accepted as a closure root and live queue/retry/dead-letter/D1/KV/R2 operations proof remains absent. Proof-file absence is a custody gap, not a reason to relabel a passed command as blocked.

## Proof artifacts

- `00-scope-summary.md`
- `01-negative-case-proof.md`
- `02-rollback-or-teardown-proof.md`
- `16-validation-commands.log`

## Focused validation truth

| Command | Current evidenced state |
| --- | --- |
| `npm --prefix infra/cloudflare run test:unit` | Passed 49/49 at source base `2aab6310c`; local unit behavior only. |
| `npm --prefix infra/cloudflare run test:integration` | Passed at source base `2aab6310c`; local Worker behavior only, not live storage/queue operations proof. |
| `npm --prefix infra/cloudflare run test:property` | Passed 9/9 at source base `2aab6310c`; property behavior only. |
| `npm run lint:architecture -- --files infra/cloudflare/src/env.ts infra/cloudflare/tests/unit/env-bindings.test.ts` | Required before a source/test change; this receipt adds no source/test edit. |

## No-claim boundary

- No claim is made that real binding IDs are configured.
- No claim is made that the Cloudflare worker boots successfully in this worktree.
- No claim is made that queue retries, dead-letter replay, D1 writes, KV writes, or R2 writes executed live.
