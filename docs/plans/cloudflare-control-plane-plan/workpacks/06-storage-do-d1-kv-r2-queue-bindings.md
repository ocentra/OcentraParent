# Workpack 06: Storage DO D1 KV R2 Queue Bindings

> **2026-07-28 correction:** `infra/cloudflare` imports module-local generated billing contracts. This workpack remains open because it has no tracked account-storage proof bundle; restore the module dependency environment, then record the actual result.

## Goal

Freeze storage and coordination ownership for Durable Objects, D1, KV, queues, and optional R2.

## First-touch surfaces

- `infra/cloudflare/src/env.ts` for the required account-identity D1/DO/KV declarations and ownership boundary (currently absent)
- `infra/cloudflare/wrangler.toml` for the selected account-identity D1 binding and binding-specific migration-directory configuration (currently absent)
- `infra/cloudflare/src/account-identity-d1-adapter.ts` for the Cloudflare-owned adapter that consumes, but does not define, the Account WP08 contract
- `infra/cloudflare/migrations/0001_account_identity_authority.sql` for the selected account-identity D1 schema/migration
- `infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts` for the migration/adapter integration surface

## Read inputs

- [STORAGE_BINDING_MODEL.md](../STORAGE_BINDING_MODEL.md)
- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)
- `docs/plans/account-identity-family-plan/workpacks/08-rust-schema-workers-d1-runtime-migration.md` for the Rust-owned account/family contract handoff only

## Output files

- `infra/cloudflare/src/env.ts`
- `infra/cloudflare/wrangler.toml`
- `infra/cloudflare/src/account-identity-d1-adapter.ts`
- `infra/cloudflare/migrations/0001_account_identity_authority.sql`
- `infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts`
- [STORAGE_BINDING_MODEL.md](../STORAGE_BINDING_MODEL.md)
- `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/`

## Acceptance

- Each binding has one owner and one purpose.
- No child-data storage drift is allowed.
- Queue and dead-letter ownership is explicit.
- Account-identity D1/DO/KV bindings, adapter, and migration consume Account WP08's canonical Rust contract without redefining family authority.
- The account D1 migration directory is binding-specific (or has an equivalent proven mapping), so account migration application cannot target `BILLING_D1`.
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
- Migration validation only after the selected account binding has a binding-specific migration directory (or equivalent isolated mapping): `cd infra/cloudflare && npm exec -c "wrangler d1 migrations apply <account-identity-d1-database> --local"`
- Account-identity migration/adapter validation after the selected test is registered in the module runner: `npm --prefix infra/cloudflare run test:integration`
- Required direct migration-test validation: `cd infra/cloudflare && npm exec -c "node --import tsx --test tests/integration/account-identity-d1-migration.test.ts"`; retain its result separately so the aggregate integration script cannot omit it.
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare/src/env.ts infra/cloudflare/src/account-identity-d1-adapter.ts infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts`

## Negative cases

- Reject optional R2 as a telemetry dump.
- Reject D1/KV claims without privacy boundaries.
- Reject a Cloudflare adapter, migration, or test double as a redefinition of the Account WP08 Rust authority contract.

## Failure conditions

- Do not imply real binding IDs or runtime success from placeholder config.

## Completion

- Status: blocked / no current account-storage proof; no Cloudflare runtime-ready, deployment-ready, or payment-ready claim is made.
- Proof root: `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/`
- Runtime/source owner: `infra/cloudflare/src/env.ts`
- Required Account D1/DO/KV and isolated account-migration configuration: `infra/cloudflare/wrangler.toml` plus `src/env.ts` (currently absent; no `BILLING_D1` substitution)
- Owned adapter/migration surfaces: `infra/cloudflare/src/account-identity-d1-adapter.ts`; `infra/cloudflare/migrations/0001_account_identity_authority.sql`
- Owned test surfaces: `infra/cloudflare/tests/unit/env-bindings.test.ts`; `infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts`

## What is actually proved

- Durable Object ownership is explicit for `BILLING_DO`, `REFERRAL_DO`, and `ENTITLEMENT_SNAPSHOT_DO`, each with one owner, one purpose, and explicit child-data prohibition.
- D1 ownership is explicit for `BILLING_D1` as billing/support/reconciliation read-model storage only.
- KV ownership is explicit for `BILLING_RATE_LIMIT_KV` and `BILLING_CONFIG_KV`, with child-data and provider-secret boundaries kept explicit.
- Queue ownership is explicit for `BILLING_RECONCILIATION_QUEUE` and `BILLING_DEAD_LETTER_QUEUE`, including paired dead-letter responsibility.
- Optional `BILLING_AUDIT_R2` stays `manual-required` and is explicitly rejected as a telemetry dump or general-purpose child-data store.
- Placeholder Wrangler binding names and IDs are treated as non-proof and non-runtime-ready.
- No account-identity D1/DO/KV binding or account migration directory is currently proved; this packet does not claim that billing bindings can serve that role.

## Blocked truth

- WP01 retains a pinned resolver graph: `npm --prefix infra/cloudflare ls wrangler @cloudflare/workers-types` resolves `wrangler@4.118.0` with deduped `@cloudflare/workers-types@5.20260804.1`. That prerequisite is satisfied; it does not substitute for WP06's account-storage proof.
- Account WP08's Rust contract and the selected WP06 D1 binding, adapter, migration, and direct integration-test artifacts are not yet retained. Until they exist, this packet cannot produce a storage handoff for Cloudflare WP08 or Account WP06.
- `infra/cloudflare/wrangler.toml` and `src/env.ts` currently declare only billing storage bindings: no account D1/DO/KV declaration or binding-specific account `migrations_dir`/equivalent mapping exists. WP06 must not run the account migration command against `BILLING_D1`.
- `infra/cloudflare/src/index.ts` imports `./generated/billing-contracts.js`, backed by the checked-in module-local generated artifact. Obsolete `packages/billing-domain/src/*` imports are not WP06 blockers and must not be revived.

## Proof artifacts

- `00-scope-summary.md`
- `01-negative-case-proof.md`
- `02-rollback-or-teardown-proof.md`
- `03-account-identity-d1-migration-test.md`
- `16-validation-commands.log`

## Focused validations

- `node --import tsx --test infra/cloudflare/tests/unit/env-bindings.test.ts`
- `cd infra/cloudflare && npm exec -c "node --import tsx --test tests/integration/account-identity-d1-migration.test.ts"` required direct test; retain its result in `03-account-identity-d1-migration-test.md` and do not let `test:integration` substitute for it
- `npm --prefix infra/cloudflare run test:unit`, `test:integration`, and `test:property` remain blocked on the missing account binding, adapter, migration, and direct integration-test surfaces; any later failure records its then-current exact blocker
- `npm run lint:architecture -- --files infra/cloudflare/src/env.ts infra/cloudflare/src/account-identity-d1-adapter.ts infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts`

## No-claim boundary

- No claim is made that real binding IDs are configured.
- No claim is made that the Cloudflare worker boots successfully in this worktree.
- No claim is made that queue retries, dead-letter replay, D1 writes, KV writes, or R2 writes executed live.
- No claim is made that Account WP08 or this packet alone completes account authority; Cloudflare WP08 runner proof and Account WP06 aggregation remain separate required handoffs.
- The account-identity adapter, migration, focused integration test, and Wrangler D1 binding configuration are required first-touch/output surfaces for this WP06 packet; listing them does not claim they exist or have run in this checkout.
