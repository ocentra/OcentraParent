<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan State`
> Kind: current state and open gaps.
> Read when: immediately after AGENTS.md.
> Stop rule: use this file to choose route state, then continue only to NEXT_ACTIONS.md and WORKPACK_INDEX.md.
> Proves: only current plan state and open-gap accounting.
> Does not prove: implementation completion, security readiness, or PR readiness.
> Proof rule: if state changes, update the assigned workpack, CHECKLIST_INDEX.md, and PROOF_INDEX.md proof path.

<!-- /agent-capsule -->

# Account Identity Family Plan State

## Current status

```text
Plan route: upgraded
Execution-grade workpacks: WP01 has a provider/custody proof pack plus the retained narrow D1 storage-adapter proof at `docs/proof/account-identity-family-plan/01-auth-provider-decision/06-account-identity-storage-adapter-proof.md`; WP08 has a tracked durable Rust-authority manifest under `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/`; WP02, WP03, WP04, WP05, and WP07 have prior proof roots on disk; WP05's implementation phase is explicitly blocked on the missing Account WP05A durable multi-owner effect coordinator/recovery owner (while WP05 remains the base authority consumer); WP06 is reopened for final aggregation after WP08 plus Cloudflare WP06/WP08 handoffs
Implementation: reviewed source provides Rust-owned schema validation, a non-forgeable current account/member/household/role/device capability, durable authority and session custody, target-aware WP02 resolution, a strict WP04 SQLite invite/recovery repository, and WP09 durable issuer/key-registry/outbox core custody. Reviewed Protected Custody WP04 fixed-pipe transport and WP05 Account issuer-owner source now authorize the missing WP09 protected signer, binding, delivery, runtime, and seven expected test roots; WP09 remains fail-closed and unreachable until that packet is written and composed. WP05's runtime composer and opaque receipt are also only a fail-closed base authority-consumer boundary; the Account WP05A durable multi-owner coordinator, owner participants, and typed handoff remain missing. Data Custody WP08 confirmation staging/consume depends on that WP05A-owned handoff. Cloudflare retains the D1 current-authority read adapter and ordered Account/Payment migration source, but its private WP09 outer-wire/key-registry consumer, mutation composition, higher authority, expected tests, deployment/migration execution, proof, and DONE remain open
Proof artifacts: `output/account-identity-family-plan-proof/01-auth-provider-decision/`, `02-identity-household-role-model/`, `03-session-token-lifecycle/`, `04-invites-recovery-lifecycle/`, `05-device-ownership-authz/`, `06-security-proof-and-route-gate/`, and `07-parent-account-family-setup-ui/` are populated; WP08 uses its tracked durable manifest rather than ignored raw output; WP03 and WP06 carry request-safety as an explicit blocker note instead of a fake-green proof; `test-results/account-identity-family-plan-*` roots remain absent unless a selected workpack explicitly requires them
PR-ready: false
```

## Closed PR disposition

PR #607 is closed without merge. Its TypeScript Cloudflare account-identity
persistence/D1-test-double slice is preserved as branch evidence only; it does
not establish Rust schema authority or any Cloudflare runtime/migration proof.

## 2026-08-17 WP02-WP05 live source correction

WP02-WP05 are not production complete. Their Rust evaluators and focused tests
are real, and bounded provisioning, policy, and child-runtime consumers exist.
Legacy evaluators still accept caller-assembled authority/lifecycle flags, but
reviewed source at `86caae334` and `7934fb41b` now places the real
storage-custody consumer behind a target-aware resolver over the sealed WP08
boundary and current Account authority. The authoritative Cloudflare
writer/update/revocation/CAS owner and shipped provider-to-Account caller are
still absent. Historical checked rows prove contract/proof slices, not durable
account, session, invite/recovery, or device-authorization execution.

The attempted source packet at remote commit `ac03afee3a` was independently
reviewed and rejected. It introduced public deserializable account, session,
invite, and recovery records with no production callers or durable owner. It
also allowed callers to mint proof, replay, freshness, same-family, abuse,
timing, support, and owner-approval state and allowed non-monotonic lifecycle
rewrites. The packet remains quarantined as remote evidence and is not mapped
as implementation.

The first legal production source seam is Account WP02: resolve the actor
parent-controller device separately from the target child/profile/device and
derive same-family, capability, controller-lease, and step-up facts behind the
sealed WP08 boundary. Parent-owner/co-parent/observer `ViewChildStatus` must
remain a parent action while its target is resolved independently. Cloudflare
WP06 then owns the authoritative D1 producer/currentness/revocation/CAS path
and shipped Firebase/provider-to-Account caller. Device Trust WP03 follows by
consuming live Account and Device Trust currentness for
`RegisterLanSignerAnchor`. Only after that chain exists should downstream
runtime orchestration and the deferred concurrency/restart/security test wave
proceed.
`CHECKLIST_INDEX.md` keeps the historical proof rows checked while adding
explicit unchecked production-source and expected-test overlays.

## 2026-08-17 accepted Account source wave

The replacement packet at `origin/codex/account-wp02-source-wave` head
`35edb2830` supersedes the rejected `ac03afee3a` design. It is integrated on
`codex/eventing-wp09-production` through `e69acf279` and passed two independent
source reviews plus focused formatting, architecture, generated-contract,
single-source-contract, and Enforcer checks. No tests, build, proof, precommit,
or CI were run in this source phase.

What is now real:

- WP08 owns strict Rust schema validation and generated TypeScript parity;
- WP02 owns a sealed capability and local durable compare-and-swap repository
  for current account, household, member, role, device, session, target,
  support receipt, and authority generation state; its action-level
  actor-versus-target composition remains incorrect and reopened;
- WP03 currentness comes from persisted session identity, generation, expiry,
  freshness, and revocation state instead of request booleans;
- WP04 has a strict durable invite/recovery repository without public authority
  or owner-receipt construction; recovery remains `Approved` until a real
  downstream owner receipt is acknowledged;
- WP05 billing and support/admin consumers bind action identity to current
  repository authority and a complete support receipt, but this does not close
  the workpack: the runtime effect handoff still has no durable Account-owned
  CAS repository/fence or crash/replay recovery owner;
- Cloudflare WP06 owns the D1 adapter and the ordered undeployed source
  migrations `0001_account_identity_authority.sql`,
  `0002_account_identity_current_authority.sql`,
  `0003_provider_billing_mappings.sql`, and
  `0004_provider_billing_mappings_canonical_identity.sql`.

What remains before product completion:

- the complete WP02 expected-test packet for the reviewed target-aware action
  authority, including actor/target substitution and parent observer cases;
- a shipped Cloudflare authoritative writer/currentness/revocation/CAS owner,
  Firebase/provider-to-Account caller, and account/session route composition;
- ship identity/membership/support/Data owner adapters for the reviewed WP04
  repository and typed custody handoff;
- Device Trust step-up plus remote/export/delete consumers;
- the full expected-test wave for reload, concurrency, replay, expiry,
  revocation, cross-household, support-receipt, migration, and route negatives;
- focused execution, proof regeneration, precommit, PR, CI, and merge.

Therefore WP02-WP05 and WP08 have bounded source slices but remain open;
historical checkboxes and proof do not close their new source/test obligations.
WP05 cannot enter implementation READY until Account WP05A's durable
multi-owner coordinator/recovery packet and typed handoff exist; WP05 remains
the base Account authority consumer and cannot supply that owner handoff by
itself. Data Custody WP08 confirmation staging/consume remains a downstream
blocked handoff and cannot bypass this dependency.

## Current product direction

```text
Cloudflare-first custody for account/family authority.
D1 owns relational account, household, membership, child profile, device, invite, recovery, and session metadata when Cloudflare runtime is selected.
Durable Objects own short-lived coordination and serialized setup/session/recovery state where needed.
KV is non-authoritative cache, rate-limit, or lookup-hint state only.
R2 is excluded from account authority and may hold only explicitly encrypted artifacts if a later data-custody decision approves it.
Firebase Auth, if used, is an external IdP/token issuer only.
Auth.js or another app-owned auth layer may be used only as an adapter/session layer, not the family authority model.
```

## 2026-08-17 provider handoff

Account WP01 selects Firebase Auth as the external identity provider for the
Cloudflare Worker adapter. The provider may prove only the external user
identity: the adapter must verify Firebase RS256 ID tokens against configured
issuer, audience, JWKS, time, and non-empty subject, then return only that
verified provider subject. D1/DO and the Rust family authority remain the sole
owners of account, household, membership, role, child, device, invite,
recovery, and session product truth. Auth.js is not selected for this Worker
path and cannot become family authority.

This is a narrow handoff to Cloudflare WP05's implementation-only packet. It
does not authorize account login/session routes, D1 migration, deployment,
tests, proof, or runtime readiness; unresolved provider configuration remains
fail-closed/manual-required.

## Current repo facts already read

- `docs/features/family-setup-device-roles.md` says family setup is product foundation and not portal polish. It also states the child-device agent remains authority for device role, controller lease, revocation, stale command rejection, and local capability status.
- `docs/expectations/family-setup.md` separates parent outcome, child-device outcome, data scope, contract families, validation gates, and non-goals.
- `docs/expectations/portal.md` says portal sends typed queries/intents to the agent and must not become child-device execution authority.
- `packages/family-domain/package.json` now describes family-domain as helpers that consume canonical `schema-domain` family contracts; do not move shared canonical shapes back into family-domain.
- `packages/setup-domain/src/family-setup-bridge.ts` and `packages/setup-domain/src/registration-entry.ts` already consume the household/invite/recovery contracts.
- `crates/family-identity-core` and `crates/provisioning-core` already carry Rust parity and downstream provisioning consumers for the same authority/session/setup surfaces.

## Module ownership and linkage

```text
crates/schema or the owning Rust crate:
  Canonical shared account/family/session/device-authority schemas, brands, parsers, route/action/read-model DTOs, literals, and encoded-shape parity when shapes cross package, crate, app, or plan boundaries.

schema-domain:
  Temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.

family-domain:
  TypeScript helper/projection package for account/family authority. It consumes Rust-owned/generated contracts and exposes approved helper surfaces; it must not become a sibling-feature runtime dependency.

family-identity-core:
  Rust parity/runtime authority boundary for account, household, role, child profile, device, session, invite/recovery, and audit semantics.

setup-domain and provisioning-core:
  Setup/provisioning consumers of account/family authority, not authority owners.

portal-domain and apps/portal:
  UI projection/rendering consumers. They may prove honest state visibility but do not prove account runtime, Cloudflare persistence, device trust, LAN/remote transport, or child activity readiness.

Cloudflare control-plane runtime/schema:
  Cloudflare retains an isolated `ACCOUNT_IDENTITY_D1` configuration, ordered Account migrations, Firebase verification source, and a current-authority read adapter. It still has no authoritative Account writer/update/revocation/CAS owner, shipped provider-to-sealed-authority caller, account/session route composition, applied production migration, live Device Trust binding, or production Worker readiness. Those runtime/persistence boundaries remain open here.

2026-08-17 production reachability audit:
  Firebase verification and the Account D1 read adapter exist, but the Worker still has no target-aware provider-to-Account authority composer, authoritative Account write/update/revoke/CAS path, or account/session route. Production configuration and migration remain manual-required, so no live Account or Device Trust authority is reachable.

Adjacent plans:
  Payment, policy, data custody, device trust, LAN, remote, setup-install, and broader portal UX consume account/family authority through handoff contracts, events, requests, read models, and proof routes. They must not re-own the authority model.
```

## Current proof interpretation

```text
Workpack proof roots prove local contract/proof slices only.
Absent `test-results/account-identity-family-plan-*` roots are not automatically fatal because current proof logs live under `output/account-identity-family-plan-proof/**/16-validation-commands.log` unless a selected workpack requires a test-results artifact.
WP01 10/10 means the provider/custody decision proof pack is filled, not that runtime auth/provider implementation is complete.
WP03 and WP06 request-safety artifacts are blockers because this plan still does not own a real browser request consumer.
WP07 proves the local setup route/projection slice; it does not prove physical device trust, Cloudflare account runtime, LAN/remote transport, or custody execution.
```

## External research anchors

- Cloudflare D1 is a managed serverless SQLite-compatible database for Workers/Pages and supports relational query/storage ownership.
- Cloudflare Durable Objects provide stateful serverless coordination with compute plus durable storage and are appropriate for serialized short-lived coordination.
- Firebase custom claims are delivered through ID tokens, must be validated server-side, are size-limited, and should be used for access control only, not as a product-data store.
- Auth.js supports JWT and database session strategies; either choice must be evaluated against revocation, token size, custody, and adapter constraints.
- OWASP requires deny-by-default authorization and permission validation on every request.
- OWASP session guidance requires meaningless, unpredictable session identifiers and server-side session state.
- OWASP recovery guidance requires consistent responses, side-channel reset delivery, random single-use expiring tokens, and rate limiting.
- NIST 800-63B requires risk-appropriate authentication assurance, step-up when higher assurance is required, replay resistance at higher assurance, and reauthentication/session timeout rules.

## Open gaps

```text
- WP02 root now contains `00-identity-entity-model-proof.md`, `01-role-action-resource-matrix.md`, `02-membership-state-machine-proof.md`, `03-cross-family-negative-proof.md`, `04-observer-read-only-proof.md`, `05-support-admin-boundary-proof.md`, `06-audit-event-proof.md`, and `16-validation-commands.log`.
- WP03 root now contains `00-credential-type-matrix.md`, `01-session-lifecycle-proof.md`, `02-token-expiry-replay-proof.md`, `03-refresh-revocation-proof.md`, `04-session-freshness-proof.md`, `05-csrf-origin-proof.md`, `06-token-redaction-proof.md`, and `16-validation-commands.log`; `05-csrf-origin-proof.md` is an explicit blocker note because this slice does not own a real browser request surface.
- WP04 root now contains `00-invite-state-machine-proof.md`, `01-invite-negative-proof.md`, `02-recovery-state-machine-proof.md`, `03-recovery-abuse-proof.md`, `04-delete-export-handoff-proof.md`, `05-support-recovery-audit-proof.md`, and `16-validation-commands.log`.
- WP05 root now contains `00-device-authority-matrix.md`, `01-revoked-device-negative-proof.md`, `02-wrong-household-negative-proof.md`, `03-controller-lease-proof.md`, `04-remote-capability-proof.md`, `05-export-delete-owner-proof.md`, `06-billing-owner-proof.md`, and `16-validation-commands.log`.
- WP07 root now contains `00-first-run-ui-state-machine.md`, `01-household-setup-ui-proof.md`, `02-device-role-ui-proof.md`, `03-observer-read-only-ui-proof.md`, `04-recovery-ui-proof.md`, `05-mobile-parent-child-claim-split-proof.md`, `06-source-custody-label-proof.md`, and `16-validation-commands.log`; the portal route/test/e2e surface is now real and keeps sibling runtime ownership explicit instead of pretending setup owns Cloudflare, trust, custody, or transport execution.
- `packages/family-domain/tests/unit/setup-lifecycle.test.ts` was repaired so the direct invite/recovery suite now matches the live schema, and `packages/family-domain/src/setup-lifecycle.ts` received a local exhaustiveness repair so the WP04 build gate is green again; no further production TS/Rust changes were required for WP02-WP03 closure, and WP05 only needed owner-only test additions in shared TypeScript/Rust authority suites.
- WP08's Rust schema/account-authority implementation and focused test surface are retained by the tracked durable manifest; Cloudflare WP06/WP08 and Account WP06 final aggregation remain open.
- WP08's bounded source repair is independently accepted as implementation evidence only. `AccountIdentityAuthorityHandoff` is the exact `v0.7` canonical binding, and `family-identity-core` owns a sealed, fail-closed current-binding port plus local SQLite repository/CAS. WP02's action composer remains target-unsafe, focused Rust/TypeScript tests and retained proof stay deferred, and the prior durable manifest does not validate this follow-up.
- Cloudflare WP06 has Firebase verification source, ordered migrations, and a D1 read adapter, but no authoritative writer/update/revocation/CAS owner or shipped provider-to-sealed-authority caller. Applied migration, route reachability, live Device Trust composition, runner proof, and deployment remain manual-required.
- WP06's prior root contains `00-security-proof-pack.md`, `01-authn-negative-proof.md`, `02-authz-matrix-proof.md`, `03-token-replay-proof.md`, `04-recovery-abuse-proof.md`, `05-origin-csrf-open-redirect-proof.md`, `06-route-sync-proof.md`, `07-logging-redaction-proof.md`, `08-manual-required-gap-register.md`, and `16-validation-commands.log`; it is reopened and cannot be final-gate proof until `09-account-authority-cloudflare-storage-gate.md` aggregates green Account WP08, Cloudflare WP06, and Cloudflare WP08 proof. A blocker remains a scheduling block for payment, policy, remote, and device trust.
- Browser request-safety proof remains blocked at `output/account-identity-family-plan-proof/03-session-token-lifecycle/05-csrf-origin-proof.md` and `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/05-origin-csrf-open-redirect-proof.md` because this plan slice still does not own a real browser request consumer.
- Adjacent runtime and schema work remain manual-required: provider verification and account/session runtime routes, D1/DO/KV account-family schema and migration proof, Cloudflare worker/runtime proof, payment execution, policy execution, data-custody execution, device-trust bootstrap, LAN transport, and remote transport.
```

## No-claim boundaries

Do not claim:

```text
auth provider selected
family authority implemented
household setup implemented
secure login/session implemented
device authority implemented
invite/recovery implemented
first-run setup UI ready
payment/customer ownership ready
policy authorization ready
remote access authorization ready
device trust bootstrap ready
product-ready account/family flow
```

until the relevant workpack proof root and checklist rows prove the claim.

## 2026-08-17 live-code review correction

Account WP02 remains open for expected tests and production composition, not
for the bounded target-aware resolver source. Reviewed commits `86caae334` and
`7934fb41b` keep the parent actor separate from the target child/profile/device,
derive authorization from opaque current Account authority, preserve observer
read-only scope, and migrate the real storage-custody consumer. The raw legacy
evaluator remains diagnostic risk if fed caller-assembled facts, and the
Cloudflare provider-to-authority caller is still missing. Retain all expected-
test, focused validation, proof, PR, and DONE gates.

## 2026-08-18 Account WP03 candidate production source boundary

Account WP03 now has a candidate Cloudflare source seam for browser login, refresh,
logout, global revoke, and session custody, rebased onto Cloudflare WP06 final
head `56a4faa37`. The source uses the final provider verification result and
`createAccountIdentityAuthorityCaller(...).resolveVerifiedProviderAuthority`
for provider-bound login/current authority. Browser refresh-bound requests
re-resolve Account current authority from D1 because they have no provider
token; they do not mint or reconstruct authority from request fields.

The source-level request safety and custody boundary is now concrete: allowed
origin and same-origin/same-site fetch metadata are required, refresh CSRF is
an exact digest/session match, optional access cookies must bind to the same
session, access expiry does not prevent refresh-bound logout/revoke, and
production cookies use `__Host-` names with Secure/Path=/ when the environment
permits them. Store creation and parent-owner global revoke runtime-check the
opaque WeakSet capability; Account currentness is revalidated inside the D1
create mutation; refresh rotation uses rotate-first CAS, durable consumed
digest custody, replay-family revocation, and guarded mutation/audit batches;
global revoke advances a durable generation fence. Audit and revoke-outcome
rows retain only domain-separated digests and bounded request correlation.

The rotate-first batch deliberately aborts on a concurrent CAS loss before
consumed-refresh or audit custody can commit; the in-flight loser is therefore
manual-required rather than attempting a post-abort mutation. Later reuse of
the durable consumed digest still reaches replay-family revocation. This race
remains an expected test and operational-reconciliation gap.

The historical `0005`/`0006` files are not edited as deployed-schema repair.
Forward `0007_account_browser_session_custody_hardening.sql` rebuilds the
authority-bearing custody tables as STRICT, fails closed by aborting on invalid
legacy rows, and publishes the exact schema version sentinel only after a
complete copy. Its non-sensitive quarantine attempt is not retained when the
migration transaction rolls back. Every BrowserSessionStore read or mutation
requires that sentinel and fails closed when it is absent or malformed.

Independent coordinator re-review accepted the repaired source boundary after
the persisted-row casing fix and an exact verification of the rotate-first D1
batch bindings. The exact Cloudflare route/store/security test family is absent
and remains deferred, as
do migration application, live D1/Worker execution, retained proof, precommit,
CI, PR, and DONE. The final WP06 parameterless mutation-readiness seam remains
manual-required; this source does not fabricate provider or Account authority.

## 2026-08-25 Account WP03 live production truth packet

The independent production-source review classifies Account WP03 as
`BLOCKED` with `REPAIR` required. This packet records live source and caller
truth only; it does not add completion evidence, change the WP02/Cloudflare
WP06 dependencies, or authorize login/session or trusted-device enablement.

- **P0 — provider identity is not device trust.** The provider-bearer path in
  `infra/cloudflare/src/auth/verifier.ts` projects `trustedDevice: true`
  without a request-bound owner/device credential. A Firebase/provider bearer
  proves only the verified provider subject; it does not prove the physical
  parent device. Affected trusted-device routes and provider-only login must
  remain unavailable/manual-required until an owner-issued, request-bound
  device credential is matched to current device/session authority and the
  trusted-device value is derived from that binding.
- **P1 — registered routes are not dispatched.** Account session contracts and
  routes are registered in the Worker source, but their request/response
  bindings are unbound. The Worker returns HTTP 501/manual-required before
  Account session dispatch. Production remains
  `account-auth-adapter-manual-required`; the Account D1 binding is
  optional/placeholder, and migration application, live D1/Worker mounting,
  and runtime startup composition are not proven.
- **Positive implementation topology is not runtime proof.** The isolated
  Cloudflare codec/store/routes, Firebase verifier/JWKS inputs, ordered
  Account session migrations (`0005`, `0006`, and forward `0007`), and the
  Rust session repository/evaluator are reviewed source evidence:
  `infra/cloudflare/src/storage/account-browser-session-codec.ts`,
  `infra/cloudflare/src/storage/account-browser-session-store.ts`,
  `infra/cloudflare/src/auth/browser-session-routes.ts`,
  `infra/cloudflare/src/providers/firebase-auth.ts`,
  `infra/cloudflare/src/providers/firebase-auth-jwks.ts`,
  `infra/cloudflare/migrations/account-identity/0005_account_browser_session_custody.sql`,
  `infra/cloudflare/migrations/account-identity/0006_account_browser_session_refresh_custody.sql`,
  `infra/cloudflare/migrations/account-identity/0007_account_browser_session_custody_hardening.sql`,
  and `crates/family-identity-core/src/session_lifecycle_repository.rs`.
  The Rust repository has no non-owner production caller, and Cloudflare
  runtime composition remains blocked behind the unbound/manual-required
  route and WP06 authority/runtime seam.
- **P2 — expected runtime tests/proof are absent.** These four expected
  Cloudflare roots remain unwritten: `infra/cloudflare/tests/unit/account-browser-session-store.test.ts`,
  `infra/cloudflare/tests/unit/account-browser-session-routes.test.ts`,
  `infra/cloudflare/tests/security/account-browser-session-request-safety.test.ts`,
  and `infra/cloudflare/tests/integration/account-browser-session-real.test.ts`.
  Existing plan/source notes do not substitute for their execution, retained
  proof, CI, PR, or completion gates.

The smallest legal repair sequence is WP02/WP06 authority and caller
composition, an owner-issued request-bound device credential with current
device/session matching, mounted auth/D1/migrations, then the four focused
Cloudflare test/proof roots. Until that sequence is complete, WP03 remains
blocked and no implementation, READY, or DONE claim is made.

## Default execution order

```text
WP01 provider decision and custody boundary
WP08 Rust-owned schema, sealed authority, and local repository/CAS
    (Account WP09 issuer/key custody, authenticated producer binding, and startup reload/recovery || WP02 target-aware actor/target action authority)
Cloudflare WP06 authoritative D1 writer/currentness/revocation/CAS and provider caller
Device Trust WP03 live Account/Device Trust ceremony composition
Cloudflare WP08 runner/proof
WP03 session/token lifecycle
WP04 invite/recovery lifecycle
WP05 base Account authority consumer
WP05A durable Account-owned effect coordinator/recovery owner, then device ownership authorization
WP07 parent account/family setup UI
WP06 security proof and route gate
```

WP06 is last because it consumes proof from every earlier workpack.

## 2026-08-24 Account WP09 issuer/key custody and Cloudflare handoff

Account WP09's independently reviewed durable core is integrated through canonical `4f6245e51`. It provides SQLite-owned issuer/key lineage, strict startup schema and row validation, private binding/delivery interfaces, durable receipt/wire outbox custody, exact current-authority checks, and household-scoped reconciliation over WP08's sealed wire. A post-integration live caller trace found zero implementations of the signer, binding-authenticator, or delivery-owner traits and zero production calls to `deliver_next_pending`; Cloudflare also cannot obtain an authenticated current key record or consume the outer wire. The signer must consume the reviewed Protected Custody WP04 fixed-pipe transport and WP05 Account issuer-owner/protected-signing boundary rather than recreate key custody in Account. Those source-order gates now authorize the three missing production adapter/runtime roots and seven expected test roots; normal operational completion still requires the upstream DONE gates. No complete producer adapter, executed test, proof, checklist acceptance, runtime readiness, READY, or DONE claim is made. Cloudflare WP06 remains blocked on reviewed WP09 implementation; Account WP02, WP05A, and Device Trust remain separate authorities.
## 2026-08-18 multi-owner effect-fence routing correction

The earlier Account-only CAS wording is retired as an implementation route.
`HouseholdAuthorityRuntimeCasFence` currently receives already-resolved
snapshots and its shipped implementation is manual-required; an Account-local
ledger cannot atomically own Device Trust or parent-step-up revocation.

Account WP05A (`workpacks/05-runtime-effect-fencing-coordinator.md`) is now the
planned coordinator/recovery route. It owns only opaque operation identity,
prepare/commit/abort/recover ordering, exact-idempotent committed replay, and
the private Account participant plus capability/controller-lease reservation
adapters for `start remote view` and `start remote control`. Other action rows
retain their own owner gates. Account WP02/WP08 remain the Account source of
truth consumed through that adapter; its existing transaction-scoped seam is
`account_identity_authority_repository.rs`,
`account_identity_authority_repository_read.rs`, and
`account_identity_authority_repository_cas.rs`, owned by WP08 and not WP05A
completion. Protected Custody WP01 is a direct prerequisite for the protected
admission outcome. Device Trust WP01 and Device Trust WP03 remain separate
owner participants; no implementation completion is claimed.
The coordinator must fail closed on unavailable, mismatched, revoked, expired,
or restart-uncertain reservations and must not claim distributed transaction
atomicity. Account WP05 consumes this handoff; Data WP08/WP09/WP10/WP11 remain
blocked until reviewed owner participants exist.

## 2026-08-19 Account WP08 producer transport mapping

The integrated Account producer packet at canonical source `c5ed3ce5c` is now
mapped to WP08. Nine Rust source files define a bounded, Rust-owned,
domain-separated signed envelope and parser; issuance is crate-private and
starts only from `VerifiedAccountIdentityAuthority`. Signer/key custody and an
authenticated producer adapter are absent, so the service remains unavailable
until Account-owned custody exists. No Cloudflare verifier, service-binding
mount, D1 currentness recheck, tests, proof, runtime reachability, or DONE claim
is implied. The expected wire/parser, canonical/signature/time, and
cross-boundary subject/currentness negatives remain open.

## Health rules

- Do not start runtime implementation if WP01 provider/custody decision is open.
- Do not treat partial proof roots as completed workpacks.
- Do not add setup UI before WP02/WP03 contract shapes exist or are explicitly stubbed with blockers.
- Do not let setup, payment, policy, remote, or device-trust plans own account-family authority.
- Do not use Firebase custom claims for household membership/product data.
- Do not put child activity evidence into account/identity state.
- Do not mark rows checked without exact proof artifact names and command logs.
- Do not move canonical shared account/family shapes out of `crates/schema` or the owning Rust crate into sibling feature owners.
- Do not claim E2E readiness from a local workpack proof root; use the E2E tiers in `TEST_PROOF_EXPECTATIONS.md`.
