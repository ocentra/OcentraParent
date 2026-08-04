# Workpack 06: Storage DO D1 KV R2 Queue Bindings

> **2026-07-28 correction:** The later missing-private-billing-import blocker text is historical. `infra/cloudflare` now imports module-local generated billing contracts. This workpack remains open because it has no tracked proof bundle; rerun after installing dependencies and record the actual result.

## Goal

Freeze storage and coordination ownership for Durable Objects, D1, KV, queues, and optional R2.

## First-touch surface

- `infra/cloudflare/src/env.ts`

## Read inputs

- [STORAGE_BINDING_MODEL.md](../STORAGE_BINDING_MODEL.md)
- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)
- `docs/plans/account-identity-family-plan/workpacks/08-rust-schema-workers-d1-runtime-migration.md` for the Rust-owned account/family contract handoff only

## Output files

- `infra/cloudflare/src/env.ts`
- [STORAGE_BINDING_MODEL.md](../STORAGE_BINDING_MODEL.md)
- `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/`

## Acceptance

- Each binding has one owner and one purpose.
- No child-data storage drift is allowed.
- Queue and dead-letter ownership is explicit.
- Account-identity D1/DO/KV bindings, adapter, and migration consume Account WP08's canonical Rust contract without redefining family authority.
- The retained storage result or exact blocker is linked for Cloudflare WP08 runner proof and Account WP06 aggregation.

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
- Migration validation after the selected binding and migration exist: `npm --prefix infra/cloudflare exec -- wrangler d1 migrations apply <account-identity-d1-database> --local`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare/src/env.ts`

## Negative cases

- Reject optional R2 as a telemetry dump.
- Reject D1/KV claims without privacy boundaries.
- Reject a Cloudflare adapter, migration, or test double as a redefinition of the Account WP08 Rust authority contract.

## Failure conditions

- Do not imply real binding IDs or runtime success from placeholder config.

## Completion

- Status: blocked / proof-present for WP06 only; no Cloudflare runtime-ready, deployment-ready, or payment-ready claim is made.
- Proof root: `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/`
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

- `npm --prefix infra/cloudflare run test:unit`, `test:integration`, and `test:property` all remain blocked before worker boot because `infra/cloudflare/src/index.ts` cannot import `packages/billing-domain/src/billing-checkout-portal-boundary.js`.
- That billing-domain boundary import is outside the WP06 storage-binding slice, so the blocker is carried rather than fixed here.

## Proof artifacts

- `00-scope-summary.md`
- `01-negative-case-proof.md`
- `02-rollback-or-teardown-proof.md`
- `16-validation-commands.log`

## Focused validations

- `node --import tsx --test infra/cloudflare/tests/unit/env-bindings.test.ts`
- `npm --prefix infra/cloudflare run test:unit` blocked on `packages/billing-domain/src/billing-checkout-portal-boundary.js`
- `npm --prefix infra/cloudflare run test:integration` blocked on the same import
- `npm --prefix infra/cloudflare run test:property` blocked on the same import
- `npm run lint:architecture -- --files infra/cloudflare/src/env.ts infra/cloudflare/tests/unit/env-bindings.test.ts`

## No-claim boundary

- No claim is made that real binding IDs are configured.
- No claim is made that the Cloudflare worker boots successfully in this worktree.
- No claim is made that queue retries, dead-letter replay, D1 writes, KV writes, or R2 writes executed live.
- No claim is made that Account WP08 or this packet alone completes account authority; Cloudflare WP08 runner proof and Account WP06 aggregation remain separate required handoffs.
