# Cloudflare Control Plane Plan State

## Production billing read-model boundary - 2026-08-16

- `infra/cloudflare/src/billing-binding-read-model.ts` now permits fixture
  seeding and fixture fallback only when `ENVIRONMENT` is `local`, `test`, or
  `development` and `AUTH_ADAPTER_MODE` is exactly `local-safe-fixture`.
  Production, preview, and provider modes never create billing schema rows or
  populate D1/KV/R2 from `fixtures.ts`.
- Production reads require their owned D1/KV/R2 binding and durable row/object
  where a single record is required. Missing billing status, referral, or
  entitlement rows return a typed `billing-read-model-manual-required` error;
  empty durable collections remain empty rather than becoming demo rows.
- Production invoice-subject lookup now queries the stored invoice ledger; it
  no longer scans the fixture-only demo subject list. License approval is
  derived from the stored entitlement snapshot rather than a fixture account.
- `infra/cloudflare/src/index.ts` maps this typed unavailable result to the
  existing manual-required route shape with HTTP 503. Local-safe fixture mode
  remains explicit for local/dev/test harnesses and cannot be selected by
  production or preview environments.
- This is a production-code draft only. No provider verification, migration,
  deployment, tests, proof, or runtime readiness claim is made.

## Production reachability audit - 2026-08-16

- WP06 source was re-audited after removal of the dead D1 adapter. The only
  production definition/reference for `createAccountIdentityStore` is
  `infra/cloudflare/src/storage/account-identity-store.ts`; its callers are
  the store's unit tests only. `infra/cloudflare/src/index.ts` and
  `src/routes.ts` do not import or dispatch an account-identity store route.
- `ACCOUNT_IDENTITY_D1` is declared in `src/env.ts` and both Wrangler files,
  but both database IDs remain placeholders and the isolated migration has no
  recorded application. Binding/configuration and migration source therefore
  do not constitute a deployed runtime owner.
- The reachable Worker auth path in `src/index.ts` calls `verifyAuthState`.
  `src/auth/verifier.ts` has no cryptographic provider verifier: the
  `local-safe-fixture` branch is a fixture-only normalization path, while
  account-adapter modes return `manual-required`. No provider-verified input
  can legally reach the D1 store today.
- Independent source review accepts WP01 only as bounded implementation-phase
  scaffold evidence. It does not authorize deployed bindings, provider
  verification, account authority, production storage, runtime readiness,
  proof freshness, or DONE.
- Account WP08's independently accepted `v0.7` contract now permits WP06's
  next source-only packet: a real durable adapter and production caller that
  consumes the sealed account-binding boundary. Provider verification and live
  deployment remain fail-closed/manual-required. WP08 remains test/proof work
  after WP06; WP07 remains local-dev/proof-only; WP11 remains deployment work.
- The graph records reviewed-implementation gates for WP01 and Account WP08.
  These gates authorize WP06 source work only; normal READY/DONE semantics
  still require dependency completion and all tests/proof contracts.

## WP06 production-code follow-up - 2026-08-16

- Account-identity D1 is now declared in `infra/cloudflare/src/env.ts`,
  `wrangler.toml`, and `wrangler.production.toml`, with the binding-specific
  `migrations/account-identity/0001_account_identity_authority.sql` migration.
- `src/storage/account-identity-store.ts` consumes the migrated table and no
  longer creates production schema inline. An unapplied table returns typed
  `manual-required`; other D1 failures remain fail-closed. Account DO/KV are
  absent by design in this slice. The store has no production route caller;
  provider verification and the runtime-owned persistence handoff remain
  manual-required.
- This is code drafted only. Migration execution, tests, validation, proof,
  provider verification/login routes, public account routes, and runtime or
  deployment readiness remain deferred.

## WP06 bounded source adapter/auth chain - 2026-08-17

- Independent source review accepts the bounded WP06 packet as implementation-
  phase evidence. `infra/cloudflare/src/storage/account-identity-authority-store.ts`
  reads the Account WP08 `v0.7` handoff from the isolated account D1 schema,
  validates the canonical contract, and fails closed for inactive mappings,
  account mismatches, unsafe generations, inactive/revoked bindings, invalid
  rows, and an unapplied migration.
- `infra/cloudflare/src/auth/verifier.ts` exposes only a narrow Worker-owned
  caller after a provider-verification result. The current Worker has no
  provider verifier, so provider-bound requests remain `503` /
  `manual-required`; caller-supplied account authority and fixture headers
  cannot authorize production. `infra/cloudflare/src/env.ts` also rejects
  local-safe fixtures outside local/test/development and requires the internal
  queue secret in production.
- `infra/cloudflare/migrations/account-identity/0001_account_identity_authority.sql`
  is source-only. The Cloudflare package build-order hook regenerates the
  canonical schema-domain contract before contract-consuming Cloudflare
  commands; generated `packages/schema-domain/dist` remains ignored and is not
  a retained proof artifact.
- Migration application, provider verification, tests, retained proof,
  deployment, and runtime reachability remain open. Normal WP06 is blocked and
  is not `DONE` or runtime-ready.

Status: engineering-grade Cloudflare control-plane spec is complete; WP00 games infra parity extraction remains validation-blocked by repo-wide `npm run format:check` drift outside its packet; WP01 Cloudflare module scaffold now has a clean Wrangler/Workers Types graph plus focused local validation and a retained receipt, complete only for its narrow scaffold acceptance; the repo-local module is largely implemented. WP06 has an independently accepted bounded source adapter/auth chain, with provider verification absent and the Worker remaining `503` / `manual-required`; migration application, tests, validation, proof, deployment, and runtime reachability remain open, so normal WP06 is blocked and not `DONE`. Account DO/KV remain absent and manual-required. PR #608 merged to `main` as `5af4a1a92` after fresh full CI passed its product, security, and platform jobs, but it proves only the local WP07 dev/seed/proof boundary. PR #604 is closed without merge: its overlapping branch/evidence are preserved, but it is superseded/conflicting and must not be rebased into the current tree. WP02 through WP12 retain their own blocked-state or handoff evidence and remain open; WP01/WP07 do not imply runtime, deployment, authority, payment, or workpack closure.

Research status: aligned against the current Parent repo and a direct inspection of the reusable games Cloudflare module, summarized in `GAMES_INFRA_PARITY_MAP.md`, including its package scripts, wrangler config, route manifest, auth middleware, payment flows, `PaymentDO`, test runner, and module docs. Parent keeps the module and testing patterns; Parent strips game-only economy, Solana, matchmaking, social, AI proxy, and asset-delivery concerns.

## Current ownership interpretation

```text
infra/cloudflare:
  Shared Worker module, env guards, route manifest, auth adapter boundary, DO/D1/KV/R2/Queue bindings, local dev/seeding, test runner, deployment proof, observability/redaction boundary, and consumer handoff gates.

billing-domain and payment-subscription-plan:
  Billing route contracts, product math, provider semantics, subscription lifecycle, invoice/grace/referral qualification, and payment runtime readiness.

account-identity-family-plan:
  Parent session, household, guardian/admin/support role authority, and account-provider selection.

device-trust-bootstrap-plan:
  Trusted parent device proof and device trust material for sensitive routes.

portal-ux-household-surfaces-plan:
  Parent shell and Cloudflare consumer UI only.

setup-install-provisioning-plan:
  Setup/bootstrap callers and install journey entrypoints.

data-custody-storage-plan:
  Retention, export, deletion, and sensitive-data custody policy.

schema-domain or domain public packages:
  Canonical shared contracts when Cloudflare route shapes cross package or plan boundaries.
```

## Current coupling risks

```text
- Implementation-present is not completion-ready.
- Billing handler presence is not payment semantics readiness.
- Auth verifier presence is not production account/session or trusted-device authority.
- Wrangler placeholder config is not deploy readiness.
- Local tests and local dev proof are not production promotion proof.
- Route manifest presence is not consumer handoff readiness.
- Binding presence is not operations readiness.
- Optional R2 remains support-safe audit/export only and must not become child telemetry or raw child-data storage.
- Private source imports from domain packages are migration-sensitive unless the package intentionally exposes them as public contract surfaces.
- Payment remains blocked until WP12 handoff is proven and downstream payment-plan consumption acknowledges what is accepted and what remains blocked.
```

## Current proof interpretation

```text
Source presence is not runtime proof.
Scaffold directory presence is not module readiness.
Route manifest proof is not auth proof.
Auth adapter proof is not account or trusted-device authority proof.
Wrangler config proof is not deployed environment proof.
Local dev/seed proof is not production promotion proof.
Storage binding proof is not queue/dead-letter/reconciliation operations proof.
Portal smoke proof is not portal UX completion or payment readiness.
WP12 can aggregate only accepted proof roots plus exact carried blockers.
```

Current Parent direction:

- `git ls-tree -r HEAD -- output/cloudflare-control-plane-plan-proof` is empty. Earlier workpack proof-root and command-result narratives are not retained evidence in this checkout; every checklist row remains unchecked until its selected workpack reruns and retains its own bundle.
- PR #608 is merged to `main` as `5af4a1a92` with fresh full CI passed across product, security, and platform jobs. Its local-dev seed/proof hardening is integrated, but WP07 remains local-only because no tracked workpack proof bundle exists. PR #604 is closed without merge as superseded/conflicting; preserve its branch/evidence but do not rebase or merge it.
- A current source audit confirms `infra/cloudflare` imports module-local generated billing contracts, not the formerly cited private `packages/billing-domain/src/*` modules. That former import blocker must not be carried into WP02-WP12 routing.
- WP01's selected graph now resolves normally (`wrangler@4.115.0` + `@cloudflare/workers-types@5.20260804.1`) after moving the declared Workers Types range to `^5.20260722.1`; current focused lint, unit `49/49`, contract `14/14`, integration `70/70`, and Cloudflare architecture checks pass. The tracked receipt and ignored detailed proof bundle record the exact local-only result.
- The WP07 follow-up is a proof-only successor from current source, not a revival of PR #604. Its local-dev/seed proof remains a separate workpack decision; this WP01 dependency refresh does not close or start WP07.
- `infra/cloudflare/` is now a separate repo-local module, not a payment-owned subfolder.
- The module includes package scripts, Wrangler dev/prod configs, concrete env and secret names, a real worker entrypoint, a real route manifest, a real auth verifier boundary, real local seed helpers, and real unit, integration, e2e, contract, security, property, and fuzz suites.
- Several planned source subdirectories remain scaffold-only `README.md` surfaces while most runtime behavior currently lives in `src/index.ts`, `src/billing-binding-read-model.ts`, and `src/fixtures.ts`.
- The test/proof spec includes an exhaustive required test-file inventory, a file-level assertion matrix, explicit redaction and observability boundaries, and proof-shape requirements for execution agents that must now match real module truth.
- Cloudflare Worker entrypoint owns env validation, CORS fail-fast, request-size limits, emergency kill switch, route-manifest dispatch, safe error handling, redacted logging, and scheduled reconciliation hooks.
- Auth is an adapter boundary with explicit states: `public`, `parent-session-required`, `trusted-parent-device-required`, `admin-required`, `support-required`, `provider-webhook-signature-required`, and `internal-queue-only`.
- Durable Objects coordinate per-account, per-household, and per-ledger idempotent writes.
- D1 owns queryable billing/support/reconciliation ledgers and read models.
- KV owns rate limits, short-lived caches, and idempotency helpers where needed.
- Queues own retry, dead-letter, provider polling, and reconciliation jobs.
- Optional R2 is limited to support-safe audit/export artifacts; it must not become child-data storage.
- Payment work cannot claim runtime readiness until the Cloudflare handoff proof exists.

## Decision records

| Record                    | Status                                                       | Gap                                                                                                                                                | Closure criteria                                                                                                               |
| ------------------------- | ------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| CFCP-001 through CFCP-021 | source-present / retained-proof-absent | Worker/module/config/test source exists, but no Cloudflare output proof root is tracked at HEAD. The prior private billing-module import blocker is resolved in source; runtime, deployment, authority, and payment behavior have not been rerun or retained. | Select one workpack, install its declared dependencies, execute its focused validation, retain its proof bundle, and only then update its individual status. |

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then the assigned workpack and `NEXT_ACTIONS.md`.
  - do not treat file presence, empty scaffolds, or stale docs as runtime correctness.
- Before any checked update, attach:
  - exact runtime or blocker state for the touched slice,
  - a proof pointer under `output/cloudflare-control-plane-plan-proof/`.
- Failure rule: no PR-ready claim until the shared module, auth boundary, storage bindings, and test pyramid each have proof or exact blockers.

## Overclaim boundary

This plan is implementation-present but not completion-ready. Cloudflare correctness remains incomplete until scoped validation stays green, proof bundles live under `output/cloudflare-control-plane-plan-proof/`, and the WP12 handoff bundle no longer carries open external billing-boundary blockers, WP02 lint debt, manual-required authority and deployment states, or downstream payment-plan consumption gaps.
