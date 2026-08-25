<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: after PLAN_STATE.md.
> Stop rule: pick one workpack; do not broaden into sibling plans.
> Proves: next-action routing only.
> Does not prove: implementation completion or PR readiness.
> Proof rule: update this file only when queue state changes.

<!-- /agent-capsule -->

# Account Identity Family Plan Next Actions

## How to use

1. Confirm branch and assignment.
2. Open `WORKPACK_INDEX.md`.
3. Select exactly one workpack.
4. Read `workpacks/00-owner-boundary-proof-gate.md` to apply current owner/import/proof rules.
5. Open only the selected workpack.
6. Use `TEST_PROOF_EXPECTATIONS.md` and `PROOF_INDEX.md` for proof obligations.
7. Implement narrow source/docs/test changes.
8. Update `CHECKLIST_INDEX.md`, selected workpack, and `PLAN_STATE.md` only after proof exists.

## Highest-priority queue

Audit snapshot June 17, 2026: WP01 has a docs-only provider/custody proof pack on disk; WP02, WP03, WP04, WP05, and WP07 have prior complete proof roots on disk. WP06 is reopened for a final aggregation rerun after Account WP08 plus Cloudflare WP06/WP08; PR-ready remains false because browser request-safety is still an explicit blocker artifact and the remaining runtime/schema/adjacent execution gaps stay manual-required.

Current routing note: independent source review accepts the replacement Account packet at `35edb2830`, integrated through `e69acf279`. WP08 schema validation, sealed current authority, and local SQLite CAS repository are present. Reviewed source at `86caae334` and `7934fb41b` adds the target-aware WP02 action resolver, preserves parent actor versus child/profile/device target identity, derives current account/household/member/device/role identity from the opaque Account binding, rejects caller-supplied authority, and migrates the real storage-custody consumer. WP05's runtime composer/revalidation path remains a fail-closed base authority-consumer boundary; the durable Account WP05A multi-owner coordinator/fence, owner participants, and exact-idempotent replay/recovery handoff are not present. The existing WP08 current-authority CAS/repository and mutation-effect rows remain source inputs, not the WP05A owner. Data Custody WP08 confirmation staging/consume depends on the WP05A-owned typed Account handoff. The source wave deliberately did not run tests or claim proof. The next coherent source chain is Account WP05 base authority consumption plus the WP05A durable coordinator/recovery packet, then the already ordered Cloudflare WP06 authoritative D1 writer/currentness/revocation/CAS plus its shipped provider caller, then Device Trust WP03 live ceremony composition; WP02/WP05/WP05A expected tests follow only after their source owners exist.

Do not revive the rejected `ac03afee3a` WP02-WP05 record packet. Its public
serde records were disconnected from production and accepted caller-mintable
authority/lifecycle facts. The dependency-first production sequence is:

```text
accepted WP08 schema + sealed authority + local repository/CAS
  -> WP02 target-aware actor/target resolver with unavailable capability/lease/step-up actions fail-closed
  -> Account WP05 base authority consumer and typed downstream handoff boundary
  -> Account WP05A durable opaque-effect coordinator/fence/schema/recovery owner
     (remote-view/remote-control capability and controller-lease reservations;
      consume the existing WP08 Account repository/read/CAS seam)
  -> Data Custody WP08 confirmation staging/consume may consume the typed Account handoff
  -> Cloudflare WP06 authoritative D1 writer/currentness/revocation/CAS and provider-to-Account caller
  -> Device Trust WP03 RegisterLanSignerAnchor actor/target composition
  -> mount reviewed WP04 repository behind real identity/membership/support/Data owners and write its six expected test roots
  -> complete WP05 Device Trust/remote/export/delete step-up consumers
  -> write the full WP02-WP05/WP08 expected-test wave
  -> run focused tests and focused Enforcer only after test source is complete
  -> Cloudflare WP08 runner proof and Account WP06 aggregation
```

### 0a. WP08 Account producer transport handoff

The Rust-only producer transport is source-present at `c5ed3ce5c` and mapped
to WP08. It is not a public authority factory: issuance is crate-private,
authority-bearing fields come from `VerifiedAccountIdentityAuthority`, and
missing signer/key custody returns typed unavailable. The next source owner is
an authenticated Account producer adapter and durable signer/key registry;
Cloudflare must not consume D1 rows, Firebase claims, request headers, or a
serialized handoff as authority. Expected parser/canonical/signature/time
tests remain unwritten and no proof or completion claim is made.

Source packets must be reachable from a shipped caller, derive trust from owned
state, keep terminal transitions monotonic, and fail closed. A new DTO, enum, or
test-only constructor with no production caller is not progress.

PR #607 is closed without merge. Do not rebase its TypeScript Cloudflare
adapter/D1-test-double slice into this plan. Start with Rust-owned account
schema authority. Then route the contract through Account WP09 issuer/key
custody and authenticated producer binding, then hand the contract to
Cloudflare WP06 for D1/DO/KV persistence/migration and Cloudflare WP08 for
runner/integration proof.

### 0. WP08 Rust Schema And Account Authority

Expected result:

```text
Rust-owned canonical account/family authority schema
canonical household-child-device binding includes pairing, install, selected route, lifecycle, revocation, and authority generation
family-owned trusted read boundary fails closed and never treats a request DTO as authority
Rust account-authority parity across household, role, device, invite/recovery, and session semantics
cross-household, stale/revoked, malformed, duplicate, and schema-incompatible negatives
redacted correlated authority proof and retained focused Rust command log
explicit handoff to Cloudflare WP06 then WP08; no worker-runtime claim
```

### 0b. WP09 Account Issuer Key Custody And Cloudflare Handoff

Reviewed core result: canonical `4f6245e51` contains durable issuer/key lineage,
strict startup recovery, and a household-scoped receipt/wire outbox over the
typed WP08 handoff. First accept Protected Custody WP01's isolated broker/client
and opaque protected admission; Account must consume that boundary and must not
recreate in-process key custody. Live caller tracing found no protected signer,
binding authenticator, delivery-owner implementation, or production lifecycle
caller. Then write the coherent Account-owned adapter/runtime packet; it must
deliver the outer wire plus an authenticated current public-key record and
accept only an exactly bound Cloudflare acknowledgement. Cloudflare WP06 then
owns its private consumer/mount. The later test wave must write all seven
expected custody/registry/adapter/runtime roots before focused execution. Retained proof
comes only after code and tests converge. Do not add Account WP02, Account WP05A,
Device Trust, or Cloudflare source ownership here, duplicate the WP08 schema/wire
contract, permit caller-selected keys, or use mock/no-op/in-memory custody.

### 1. WP01 Auth Provider Decision

Expected result:

```text
accepted provider/custody decision with exact D1/DO/KV/R2 ownership split
explicit IdP/Auth.js adapter boundary
degraded/manual-required behavior for provider outage
replacement/migration path
proof root and command log
```

### 2. WP02 Identity Household Role Model

Expected result:

```text
typed account/household/membership/role/device references
parent-controller actor device resolved separately from target child/profile/device
same-family identity derived from owned authority, never caller booleans
capability, controller lease, and step-up derived from their owning authority or rejected as unavailable
ParentOwner/CoParent/Observer ViewChildStatus preserved as a parent action over an independently resolved target
role/action/resource matrix with cross-family denial proof
observer/support/admin boundaries
proof root and command log
```

### 3. WP03 Session Token Lifecycle

Expected result:

```text
credential type matrix
browser session lifecycle, replay-safe rotation, revoke/logout, expiry/skew handling
sensitive-action freshness gate
proof root and command log
```

2026-08-18 candidate source boundary: the Cloudflare runtime composition is
reachable through the final WP06 provider caller and Account current-authority
capability. Historical migrations `0005_account_browser_session_custody.sql`
and `0006_account_browser_session_refresh_custody.sql`, plus forward
`0007_account_browser_session_custody_hardening.sql`, the opaque session store,
refresh-family CAS/replay custody, refresh-bound logout/revoke routes, exact
CSRF plus origin/fetch-metadata checks, redacted milestones, and `__Host-`
cookies are source-present. The store captures trusted time internally and the
forward migration sentinel/row decoder fail closed on malformed schema values
or an unapplied/mismatched custody version. Independent coordinator re-review
accepted the repaired source boundary; this is implementation acceptance only,
not test, migration, deployment, proof, or DONE acceptance.
The expected route/store/request-safety tests are not
present yet and remain deferred to the test/proof phase:
`infra/cloudflare/tests/unit/account-browser-session-store.test.ts`,
`infra/cloudflare/tests/unit/account-browser-session-routes.test.ts`,
`infra/cloudflare/tests/security/account-browser-session-request-safety.test.ts`,
and `infra/cloudflare/tests/integration/account-browser-session-real.test.ts`.
Applied migrations, deployment, retained proof, and DONE remain open.

### 3a. WP03 live production truth correction — 2026-08-25

The accepted Cloudflare files are implementation topology only. WP03 remains
`BLOCKED`/`REPAIR` and must stay unavailable/manual-required for provider-only
login and every trusted-device route: `verifier.ts` projects a verified
provider bearer to `trustedDevice: true` without a request-bound owner/device
credential. Provider identity is not physical parent-device trust; the trusted
device value must be derived from an owner-issued credential matched to current
device/session authority before enablement.

The session contracts/routes are registered but unbound and the Worker returns
HTTP 501/manual-required before Account session dispatch. Production auth mode
is `account-auth-adapter-manual-required`; the Account D1 binding is
optional/placeholder, and migration application, live D1/Worker mounting, and
startup/provider composition are not proven. The codec/store/Firebase/JWKS,
ordered migration, and Rust session repository sources remain positive
evidence, but the Rust repository has no non-owner production caller and the
Cloudflare runtime composition is blocked behind WP06 and the unbound route
contracts.

The four expected Cloudflare runtime test/proof roots remain absent:

```text
infra/cloudflare/tests/unit/account-browser-session-store.test.ts
infra/cloudflare/tests/unit/account-browser-session-routes.test.ts
infra/cloudflare/tests/security/account-browser-session-request-safety.test.ts
infra/cloudflare/tests/integration/account-browser-session-real.test.ts
```

Keep the exact hard dependencies on Account WP02 and Cloudflare WP06. The next
legal actions are owner-side request-bound device credential/currentness
composition, mounted auth/D1/migrations, and the four focused runtime tests
with retained proof. Do not add source, tests, proof, CI, PR, READY, or DONE in
this truth packet.

### 4. WP04 Invites Recovery Lifecycle

Expected result:

```text
invite state machine
co-parent/observer/child-device invite scopes
single-use/expiry/revocation/replay proof
recovery flows for forgotten login, lost parent device, compromised-account, child reinstall, transfer
data custody handoff for delete/export
```

### 5. WP05 Device Ownership AuthZ

Expected result:

```text
actor/household/role/device/session/capability authZ matrix
wrong-household and stale/revoked device denial proof
remote view/control capability separation
export/delete and billing owner gates
durable Account-owned opaque-effect CAS/recovery owner with exact-idempotent replay and crash/restart recovery
typed handoff required before Data Custody WP08 confirmation staging/consume can advance
```

### 5A. WP05A Runtime Effect Fencing Coordinator

Before source implementation, route and review the owner-specific protocol:

```text
Account WP02/WP08 -> sealed Account authority source; WP05A private Account participant adapter
Account WP03 -> session freshness/revocation participant
Device Trust WP01 -> trusted-device currentness participant
Device Trust WP03 -> parent-step-up reservation participant
WP05A -> coordinator/recovery plus private Account participant and
         remote-view/remote-control capability/controller-lease reservations
         (Protected Custody WP01 admission is a direct prerequisite)
WP05 -> Account authorization consumer and typed downstream handoffs
```

The coordinator must use private prepare/commit/abort/recover handles, exact
target/generation binding, durable idempotency, and fail-closed restart
uncertainty. Do not implement an Account-local snapshot CAS or move Device
Trust/step-up truth into this plan. Data Custody WP08/WP09/WP10/WP11 remain
blocked until the owner participants and coordinator recovery source are
reviewed.

### 6. WP07 Parent Account Family Setup UI

Expected result:

```text
first-run setup state machine
create/join household, child profile, device pair, co-parent/observer invites
honest source/custody/degraded/manual-required UI labels
portal tests or explicit missing UI blocker
```

### 7. WP06 Security Proof And Route Gate

Expected result:

```text
rollout proof pack consumes WP01-WP05/WP07 proof
reopened final gate consumes Account WP08 authority proof plus Cloudflare WP06 storage and Cloudflare WP08 runner/proof handoffs
route sync confirms adjacent plans consume account identity without owning it
remaining manual-required gaps listed
explicit request-safety blocker carried forward without fake-green closure
```

## Blocked execution rules

- Runtime implementation is blocked until WP01 provider/custody decision is complete or the selected workpack explicitly implements that decision.
- UI implementation is blocked until the required contract shape exists or a stub/blocker proof is written.
- Payment/policy/remote/device-trust integration remains blocked until the reopened WP06 route gate consumes green Account WP08 plus Cloudflare WP06/WP08 proof. A precise blocker is recorded for audit only and does not release dependent scheduling.
- Any claim involving secure auth/session requires WP03 and WP06 proof.
- Any selected workpack that conflicts with `workpacks/00-owner-boundary-proof-gate.md` must be updated or blocked before source changes.

## PR readiness guard

A partial PR may be acceptable only when one workpack is fully closed and the report lists remaining open workpacks.

Do not create PR-ready claims from:

```text
docs-only provider discussion without accepted decision
happy-path login flow only
role matrix without cross-family negatives
UI mock without typed contract/source-state proof
invite/recovery flow without replay/rate-limit/enumeration proof
```

## 2026-08-17 WP02 review reopening

Reopen WP02 implementation review before downstream custody composition. The
first source correction must resolve target child/device identity separately
from the actor parent-controller device, reject capability/lease/step-up
actions until their owned authority sources are composed, remove the
`same_family` hardcode and caller-supplied trust facts, preserve the correct
parent `ViewChildStatus` mapping, and keep the sealed current-authority
boundary. Cloudflare WP06, not WP02, owns the provider
caller and authoritative D1 write/currentness/revocation/CAS path. Do not close
the row from the existing evaluator tests or historical proof; the production
caller and new negative/positive expected-test wave remain open.
