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

At the 2026-08-16 checkpoint, the retained store was not a production handoff.
`ACCOUNT_IDENTITY_D1`, the ordered migrations, Firebase verification, and the
read adapter were real source, but the Worker had no authoritative Account
create/update/revoke/currentness/CAS owner or provider-to-sealed-authority
caller. The 2026-08-18 review below supersedes this missing-source finding.
Placeholder database IDs are not deployed authority. Account WP08 supplies the
sealed contract and local repository; Account WP02 target-aware action authority
is now independently reviewed source. This audit authorizes the WP06
implementation-only source route,
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

Account WP02 target-aware authority is reviewed source. Authoritative
write/update/revocation/CAS and verified-provider current-authority caller
source are now reviewed. The verifier reaches current-authority resolution,
but the Account-owned mutation producer is not mounted in the Worker
entrypoint. Migration application, focused tests, retained proof, deployment,
mutation runtime reachability, and normal WP06 completion remain open. WP06 is
not `DONE`.

## Authoritative source packet reviewed - 2026-08-18

- `infra/cloudflare/src/storage/account-identity-authority-writer.ts` owns the
  Account-authorized create, current-authority, compare-and-swap, and revoke
  operations. Compare-and-swap binds authority generation, session generation,
  current session identity, and a rotated next session identity.
- `infra/cloudflare/src/auth/account-identity-authority-caller.ts` consumes
  verified provider identity and an Account-owned authority producer. It does
  not accept caller-provided household, role, device, session, capability,
  controller-lease, or step-up authority.
- `infra/cloudflare/src/auth/verifier.ts` reaches the provider/current-authority
  read path. The runtime exposes no create/CAS/revoke API; its parameterless
  mutation-readiness result remains manual-required until the Account-owned
  producer transport is verified.
- Commits `f9cfc9070` and `dfa8181b1` close the missing source-owner gap only.
  The complete expected-test packet, focused execution, migration application,
  retained proof, deployment, and runtime-composition gate are still open.

## 2026-08-18 producer transport audit

The Account-owned producer required for Cloudflare mutation composition is not
present in this repository. `ocentra-family-identity-core` keeps
`VerifiedAccountIdentityAuthority` crate-private and non-serializable; no
signed/sealed Account-to-Worker transport or verifier is exposed to
`infra/cloudflare`. The previous exported
`createAccountOwnedAuthorityProducer(resolveCurrentAuthority)` factory has
therefore been removed: a private symbol brand alone did not prevent any
caller holding the factory from minting a producer around an arbitrary
closure.

`infra/cloudflare/src/auth/account-identity-authority-runtime.ts` now routes
verified-provider reads through the existing bounded D1 read adapter, while
its parameterless `getMutationAuthorityReadiness()` returns
`account-identity-authority-source-unavailable` and emits no mutation. The
writer's create/compare-and-swap/revoke methods remain gated by the
unconstructible `AccountOwnedAuthorityProducer`; the exact owner route that
must be supplied before mutation can be mounted is:
`account-identity-family-plan` WP02/WP08 -> Account-owned signed/sealed
current-authority transport and Worker verifier -> Cloudflare WP06 runtime.
Until that route exists, D1 evidence, provider claims, request headers, and
serialized handoffs remain insufficient to mint Account authority.

## Goal

Freeze storage and coordination ownership for Durable Objects, D1, KV, queues, and optional R2.

## Device Trust current-authority bridge

Cloudflare WP06 is the ordered durable bridge after Account Identity WP08 and
target-aware Account WP02, and before Device Trust WP03. Its production path
must authoritatively create, update, revoke, compare-and-swap, and resolve the
canonical household/child/device/pairing/install/route/lifecycle binding, then
compose a verified Firebase/provider subject into the sealed Account authority
without inventing household or signer authority. Current-authority resolution
is reachable through the verifier, but the Account-owned mutation producer is
not mounted, so WP03 cannot consume a fully composed authority lifecycle yet. No Worker
fixture, provider-subject mapping, caller-supplied selector, LAN registry, or
typed receipt may authorize `RegisterLanSignerAnchor`; WP03 owns that ceremony
after this bridge is real. This route has no reverse dependency on WP03.

## First-touch surfaces

- `infra/cloudflare/src/env.ts` for the optional account-identity D1 declaration and ownership boundary; account DO/KV remain absent and manual-required
- `infra/cloudflare/wrangler.toml` and `wrangler.production.toml` for the selected account-identity D1 binding and binding-specific migration-directory configuration
- `infra/cloudflare/src/storage/account-identity-store.ts` for the narrow migrated-schema consumer and D1 mapping custody; it currently has no production route caller
- `infra/cloudflare/src/storage/account-identity-authority-store.ts` for the canonical Account WP08 `v0.7` binding adapter
- `infra/cloudflare/src/storage/account-identity-authority-writer.ts` for authoritative create/update/revoke/currentness/CAS ownership; source exists, mutation runtime composition does not
- `infra/cloudflare/src/auth/account-identity-authority-caller.ts` for verified Firebase/provider-to-sealed-authority composition; current-authority resolution is reachable through the verifier
- `infra/cloudflare/src/auth/account-identity-authority-runtime.ts` for the safe verified-provider read/manual boundary; no mutation scalar API is exposed before the trusted producer route exists
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
- `infra/cloudflare/src/storage/account-identity-authority-writer.ts`
- `infra/cloudflare/src/auth/account-identity-authority-caller.ts`
- `infra/cloudflare/src/auth/account-identity-authority-runtime.ts`
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
- The account-identity D1 binding, isolated migrations, read adapter, authoritative writer, and provider caller preserve the Account WP08/WP02 handoff boundary without redefining family authority. The provider caller consumes verified Firebase identity and Account-owned authority; caller trust facts remain rejected. Current-authority resolution is reachable, while the runtime exposes only parameterless mutation readiness and no create/CAS/revoke scalar seam. The writer operations remain gated by an unconstructible `AccountOwnedAuthorityProducer` until Account WP02/WP08 supplies the signed/sealed producer transport and Worker verifier. Account DO/KV remain manual-required.
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

- Status: read/current-authority adapter, Account-owned writer, and provider caller source reviewed; Account WP02 target identity reviewed. The verifier reaches current-authority resolution, but the mutation producer is not mounted. Runtime mutation composition, migration execution, expected tests, focused validation, retained proof, and deployment remain deferred. Normal workpack readiness remains blocked. No Cloudflare runtime-ready, deployment-ready, payment-ready, or `DONE` claim is made.
- Proof root: `output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/`
- Runtime/source owner: `infra/cloudflare/src/env.ts`
- Account D1 and isolated migration configuration: `infra/cloudflare/wrangler.toml`, `wrangler.production.toml`, `src/env.ts`, and `package.json`; account DO/KV declarations remain absent and no `BILLING_D1` substitution is allowed
- Owned migrated-schema surfaces: existing reads in `infra/cloudflare/src/storage/account-identity-authority-store.ts` and `infra/cloudflare/src/storage/account-identity-store.ts`; authoritative writes/currentness source in `infra/cloudflare/src/storage/account-identity-authority-writer.ts`; ordered `infra/cloudflare/migrations/account-identity/0001`-`0004` source
- Owned auth handoff surface: `infra/cloudflare/src/auth/account-identity-authority-caller.ts` consuming verified Firebase identity plus Account-owned authority. Current-authority resolution is mounted through the verifier; mutation composition remains manual-required.
- Owned test surfaces: `infra/cloudflare/tests/unit/env-bindings.test.ts`; `infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts`

## 2026-08-19 Account producer transport mapping

Account WP08 now supplies a Rust-owned, crate-private producer transport at
canonical source `c5ed3ce5c`. Its authority-bearing fields derive from the
sealed `VerifiedAccountIdentityAuthority`; signer/key custody and an
authenticated producer adapter are still absent, so the Account issuer is
typed unavailable. WP06 therefore remains limited to the existing verified-
provider read/manual boundary. The missing Cloudflare source is a private
transport verifier/service-binding consumer at the existing caller/runtime
boundary plus a D1 currentness, revocation, and CAS recheck before writer
mutations. The exact helper split is a decision blocker, not permission to
invent a public route or arbitrary module. Expected subject, signature/time,
currentness, migration, reachability, restart, and concurrency tests are
mapped but not written or run.

## 2026-08-19 WP06 producer-consumer source contract

The next source packet is a private consumer at the existing
`account-identity-authority-caller.ts` / `account-identity-authority-runtime.ts`
seam. It is not a public Worker route and does not add a browser, admin,
support, Firebase, or request-selected authority path.

### Deep interface and trust route

Account WP02/WP08 must provide an authenticated Cloudflare service-binding
adapter owned by Account. The adapter accepts only the provider and subject
already returned by the Worker provider verifier as a lookup key and returns
either a bounded signed producer wire or a typed unavailable result. It must
never accept household, member, device, role, session, generation, target,
lease, capability, or receipt scalars from the Worker request.

The Worker must obtain the verification key by `key_id` from an Account-owned,
durable, versioned public-key registry over that authenticated service binding.
The registry entry must be checked against `sha256:<public-key>` before use;
the key is not an environment variable, Firebase key, request header, D1 row
supplied by the caller, or hard-coded fixture. Until Account supplies durable
signer/key custody and authenticated registry distribution, the adapter and
consumer return `account-identity-authority-source-unavailable` and no writer
operation is mounted. Unknown, revoked, expired, or rotated-out key IDs remain
manual-required rather than falling back to another key.

The private Worker verifier owns the bounded-wire checks: exact domain
separator, schema/audience/environment/algorithm, field and payload limits,
canonical payload bytes, strict timestamp form and lifetime/skew, key-id
derivation, and Ed25519 signature. It yields only the validated Account
handoff to the existing server-owned caller; it does not yield a reusable
authority token to a route caller.

Before create, compare-and-swap, revoke, or any future mutation, the existing
D1 writer must re-read the provider mapping in the same transaction and
compare the verified handoff with durable provider/account/household/member/
device/session/authority-generation state, active mapping/revocation,
pairing/install/lifecycle state, and the expected session identity and
generation. A stale, revoked, mismatched, replayed, or unavailable result
fails closed; only the guarded atomic write/CAS result is returned.

### Exact source and test roots for the packet

Source remains bounded to these owning surfaces; no global graph or unrelated
Cloudflare route is implied:

```text
infra/cloudflare/src/auth/account-identity-authority-producer-transport.ts  (new private binding adapter/verifier seam)
infra/cloudflare/src/auth/account-identity-authority-caller.ts               (mount only after verified handoff)
infra/cloudflare/src/auth/account-identity-authority-runtime.ts              (retain parameterless manual-required gate)
infra/cloudflare/src/storage/account-identity-authority-writer.ts            (transactional recheck and guarded mutation)
infra/cloudflare/src/storage/account-identity-authority-store.ts             (durable currentness read)
infra/cloudflare/src/env.ts                                                   (private binding declaration only, when Account route exists)
infra/cloudflare/wrangler.toml                                                (binding shape only; no placeholder readiness claim)
infra/cloudflare/wrangler.production.toml                                     (binding shape only; no placeholder readiness claim)
infra/cloudflare/migrations/account-identity/0001_account_identity_authority.sql
```

Expected tests are written later in the plan-wide test phase, through the
same interface used by production:

```text
infra/cloudflare/tests/unit/account-identity-authority-producer-transport.test.ts
infra/cloudflare/tests/unit/account-identity-authority-caller.test.ts
infra/cloudflare/tests/unit/account-identity-authority-runtime.test.ts
infra/cloudflare/tests/integration/account-identity-authority-currentness.test.ts
infra/cloudflare/tests/integration/account-identity-authority-restart-cas.test.ts
infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts
```

The test adapter must be an in-memory implementation of the same private
service-binding/key-registry interface, not a mock or fake authority source.
Required negatives include unavailable signer/registry/service binding,
unknown or hash-mismatched key, key rotation/revocation, malformed/tampered/
non-canonical/oversize wire, invalid signature, expiry/future skew, provider
subject mismatch, revoked or stale D1 mapping, generation/session conflict,
restart/replay, concurrent CAS, and proof that no public scalar mutation route
exists. No test, proof, migration application, runtime readiness, or DONE claim
is made by this contract update.

### Hard dependencies and stop condition

The packet depends on Account WP02/WP08 durable signer/key custody,
authenticated producer issuance, and authenticated public-key registry
distribution. Cloudflare WP06 owns verification, durable recheck, and guarded
storage only. Account WP08 owns the Rust wire; Account WP02/WP08 own issuer and
key custody; Device Trust WP03 remains downstream. If the Account service
binding or registry cannot be made authenticated and durable, keep the current
manual-required runtime and do not invent a verifier, key, endpoint, or
caller-supplied substitute.

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
- Account WP08's `v0.7` Rust/TypeScript contract, local repository, Firebase verifier, bounded Cloudflare read adapter, Account-owned writer/provider caller, and Account WP02 target-aware identity are reviewed source evidence. Mutation runtime composition, migration execution, direct integration-test artifacts, retained proof, and the full runtime storage handoff remain open.
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
- The account-identity focused integration test, migration command, and proof remain deferred. Account WP02 target resolution and the D1 writer/provider caller are reviewed source, but create/CAS/revoke are not runtime-reachable until an owning route composes the mutation producer; production configuration and full runtime authority remain manual-required.
