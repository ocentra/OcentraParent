# Workpack 06: Storage DO D1 KV R2 Queue Bindings

> **2026-07-28 correction:** `infra/cloudflare` imports module-local generated billing contracts. This workpack remains open because it has no tracked account-storage proof bundle; restore the module dependency environment, then record the actual result.

## Production billing read-model boundary - 2026-08-16

The Worker read-model path no longer auto-seeds or falls back to fixture
billing data outside an explicit `local`/`test`/`development` environment
using `local-safe-fixture`. Production and preview reads require real owned
bindings and durable data; missing required rows return the typed
`billing-read-model-manual-required` boundary and the Worker emits the
manual-required route response. Invoice lookup uses the durable ledger rather
than demo subjects, and license approval uses the stored entitlement snapshot.
This slice does not add provider authority or claim migration/deployment
readiness; tests and retained proof remain deferred.

## Production reachability audit - 2026-08-16

The retained store is not a production handoff. `ACCOUNT_IDENTITY_D1`, the
ordered migrations, Firebase verification, and the read adapter are real
source, but the Worker has no authoritative Account create/update/revoke/
currentness/CAS owner or shipped provider-to-sealed-authority caller.
Placeholder database IDs are not deployed authority. Account WP08 supplies the
sealed contract and local repository; Account WP02 must first correct target-
aware action authority. This audit authorizes the subsequent WP06 source route,
not validation, migration execution, proof, or runtime/deployment claims.

## Bounded source adapter/auth chain accepted - 2026-08-17

Independent source review accepts the following implementation-phase packet;
this does not close normal WP06:

- `infra/cloudflare/src/storage/account-identity-authority-store.ts` is the
  narrow D1 adapter for the Account WP08 `v0.7` handoff. It validates the
  canonical schema and rejects inactive mappings, mapping/account mismatches,
  unsafe generations, inactive or revoked bindings, invalid rows, and missing
  migrations without accepting caller-supplied authority.
- `infra/cloudflare/src/auth/verifier.ts` can call that adapter after Firebase
  verification. It does not compose target-aware sealed Account authority or
  own authoritative Account writes/currentness, so the production authority
  path remains `503` / `manual-required` and fixture headers cannot authorize
  production.
- `infra/cloudflare/src/env.ts` rejects `local-safe-fixture` outside
  local/test/development and requires `INTERNAL_QUEUE_SHARED_SECRET` in
  production. `infra/cloudflare/migrations/account-identity/0001_account_identity_authority.sql`
  remains source-only and unapplied.
- `infra/cloudflare/package.json` builds the canonical schema-domain contract
  before Cloudflare contract-consuming commands. Generated schema-domain
  `dist` output is ignored and is not retained proof.

Account WP02 target-aware authority, authoritative write/update/revocation/CAS,
the shipped provider caller, migration application, focused tests, retained
proof, deployment, and runtime reachability remain open. Normal WP06 is blocked
and is not `DONE`.

## Goal

Freeze storage and coordination ownership for Durable Objects, D1, KV, queues, and optional R2.

## Device Trust current-authority bridge

Cloudflare WP06 is the ordered durable bridge after Account Identity WP08 and
target-aware Account WP02, and before Device Trust WP03. Its production path
must authoritatively create, update, revoke, compare-and-swap, and resolve the
canonical household/child/device/pairing/install/route/lifecycle binding, then
compose a verified Firebase/provider subject into the sealed Account authority
without inventing household or signer authority. The current read adapter is
unreachable as live authority, so WP03 cannot consume it yet. No Worker
fixture, provider-subject mapping, caller-supplied selector, LAN registry, or
typed receipt may authorize `RegisterLanSignerAnchor`; WP03 owns that ceremony
after this bridge is real. This route has no reverse dependency on WP03.

## First-touch surfaces

- `infra/cloudflare/src/env.ts` for the optional account-identity D1 declaration and ownership boundary; account DO/KV remain absent and manual-required
- `infra/cloudflare/wrangler.toml` and `wrangler.production.toml` for the selected account-identity D1 binding and binding-specific migration-directory configuration
- `infra/cloudflare/src/storage/account-identity-store.ts` for the narrow migrated-schema consumer and D1 mapping custody; it currently has no production route caller
- `infra/cloudflare/src/storage/account-identity-authority-store.ts` for the canonical Account WP08 `v0.7` binding adapter
- planned `infra/cloudflare/src/storage/account-identity-authority-writer.ts` for authoritative create/update/revoke/currentness/CAS ownership
- planned `infra/cloudflare/src/auth/account-identity-authority-caller.ts` for shipped Firebase/provider-to-sealed-authority composition
- `infra/cloudflare/src/auth/verifier.ts` for the provider-gated Worker-owned caller and fail-closed manual-required path
- `infra/cloudflare/package.json` for schema-domain contract build ordering before Cloudflare consumers
- `infra/cloudflare/migrations/account-identity/0001_account_identity_authority.sql` for the isolated account-identity D1 schema/migration
- `infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts` remains the deferred migration/store integration surface

## Read inputs

- [STORAGE_BINDING_MODEL.md](../STORAGE_BINDING_MODEL.md)
- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)
- `docs/plans/account-identity-family-plan/workpacks/08-rust-schema-workers-d1-runtime-migration.md` for the Rust-owned account/family contract handoff only

## Output files

- `infra/cloudflare/src/env.ts`
- `infra/cloudflare/wrangler.toml`
- `infra/cloudflare/wrangler.production.toml`
- `infra/cloudflare/src/storage/account-identity-store.ts`
- `infra/cloudflare/src/storage/account-identity-authority-store.ts`
- `infra/cloudflare/src/storage/account-identity-authority-writer.ts` (planned)
- `infra/cloudflare/src/auth/account-identity-authority-caller.ts` (planned)
- `infra/cloudflare/src/auth/verifier.ts`
- `infra/cloudflare/package.json`
- `infra/cloudflare/migrations/account-identity/0001_account_identity_authority.sql`
- `infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts` (deferred)
- [STORAGE_BINDING_MODEL.md](../STORAGE_BINDING_MODEL.md)
- `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/`

## Acceptance

- Each binding has one owner and one purpose.
- No child-data storage drift is allowed.
- Queue and dead-letter ownership is explicit.
- The account-identity D1 binding, isolated migrations, read adapter, planned authoritative writer, and planned provider caller preserve the Account WP08/WP02 handoff boundary without redefining family authority. The provider caller must consume verified Firebase identity and target-aware sealed Account authority; until it and authoritative currentness exist, the Worker remains manual-required. Account DO/KV remain manual-required.
- The account D1 migration directory is binding-specific, so account migration application cannot target `BILLING_D1`.
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
- Account-identity migration/store validation after the selected test is registered in the module runner: `npm --prefix infra/cloudflare run test:integration`
- Required direct migration-test validation: `cd infra/cloudflare && npm exec -c "node --import tsx --test tests/integration/account-identity-d1-migration.test.ts"`; retain its result separately so the aggregate integration script cannot omit it.
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare/src/env.ts infra/cloudflare/src/storage/account-identity-store.ts infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts`

## Negative cases

- Reject optional R2 as a telemetry dump.
- Reject D1/KV claims without privacy boundaries.
- Reject a Cloudflare adapter, migration, or test double as a redefinition of the Account WP08 Rust authority contract.

## Failure conditions

- Do not imply real binding IDs or runtime success from placeholder config.

## Completion

- Status: read-side adapter/auth source retained; blocked on Account WP02 target authority and the missing authoritative writer/provider caller. Migration execution, expected tests, focused validation, retained proof, deployment, and runtime reachability remain deferred. No Cloudflare runtime-ready, deployment-ready, provider-authenticated, payment-ready, or `DONE` claim is made.
- Proof root: `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/`
- Runtime/source owner: `infra/cloudflare/src/env.ts`
- Account D1 and isolated migration configuration: `infra/cloudflare/wrangler.toml`, `wrangler.production.toml`, `src/env.ts`, and `package.json`; account DO/KV declarations remain absent and no `BILLING_D1` substitution is allowed
- Owned migrated-schema surfaces: existing reads in `infra/cloudflare/src/storage/account-identity-authority-store.ts` and `infra/cloudflare/src/storage/account-identity-store.ts`; planned authoritative writes/currentness in `infra/cloudflare/src/storage/account-identity-authority-writer.ts`; ordered `infra/cloudflare/migrations/account-identity/0001`-`0004` source
- Owned auth handoff surface: planned `infra/cloudflare/src/auth/account-identity-authority-caller.ts` consuming the existing Firebase verifier and sealed Account authority; production configuration remains manual-required.
- Owned test surfaces: `infra/cloudflare/tests/unit/env-bindings.test.ts`; `infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts`

## What is actually proved

- Durable Object ownership is explicit for `BILLING_DO`, `REFERRAL_DO`, and `ENTITLEMENT_SNAPSHOT_DO`, each with one owner, one purpose, and explicit child-data prohibition.
- D1 ownership is explicit for `BILLING_D1` as billing/support/reconciliation read-model storage only.
- KV ownership is explicit for `BILLING_RATE_LIMIT_KV` and `BILLING_CONFIG_KV`, with child-data and provider-secret boundaries kept explicit.
- Queue ownership is explicit for `BILLING_RECONCILIATION_QUEUE` and `BILLING_DEAD_LETTER_QUEUE`, including paired dead-letter responsibility.
- Optional `BILLING_AUDIT_R2` stays `manual-required` and is explicitly rejected as a telemetry dump or general-purpose child-data store.
- Placeholder Wrangler binding names and IDs are treated as non-proof and non-runtime-ready.
- Account-identity D1 is now declared with an isolated migration directory in both Wrangler configs, but no migration command or proof has run; account DO/KV remain absent and manual-required, and billing bindings cannot serve that role.

## Blocked truth

- Independent source review accepts WP01's current module dependency/runtime scaffold as implementation-phase evidence. This does not substitute for rerunning module tests or retaining proof in the later validation phase.
- Account WP08's `v0.7` Rust/TypeScript contract, local repository, Firebase verifier, and bounded Cloudflare read adapter are source evidence. Account WP02 target-aware authority and the planned Cloudflare writer/provider caller remain missing, so migration execution, direct integration-test artifacts, retained proof, and runtime storage handoff remain open.
- `infra/cloudflare/wrangler.toml`, `wrangler.production.toml`, and `src/env.ts` declare the optional account D1 binding with a binding-specific `migrations_dir`; account DO/KV are intentionally not declared. WP06 must not run the account migration command against `BILLING_D1`.
- The store no longer creates the account table opportunistically. If the isolated migration has not been applied, reads and writes return `manual-required` for the missing account schema; other D1 errors remain fail-closed errors.
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
- `npm --prefix infra/cloudflare run test:unit`, `test:integration`, and `test:property` are deferred until WP01 restores the module dependency tree; any later failure records its then-current exact blocker
- `npm run lint:architecture -- --files infra/cloudflare/src/env.ts infra/cloudflare/src/storage/account-identity-store.ts infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts`

## No-claim boundary

- No claim is made that real binding IDs are configured.
- No claim is made that the Cloudflare worker boots successfully in this worktree.
- No claim is made that queue retries, dead-letter replay, D1 writes, KV writes, or R2 writes executed live.
- No claim is made that Account WP08 or this packet alone completes account authority; Cloudflare WP08 runner proof and Account WP06 aggregation remain separate required handoffs.
- The account-identity focused integration test, migration command, and proof remain deferred. The D1 read adapter is not authoritative without Account WP02 target resolution, the planned writer/currentness owner, and the provider-to-Account caller; production configuration and runtime authority are manual-required.
