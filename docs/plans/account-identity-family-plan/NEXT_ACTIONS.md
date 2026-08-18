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

Current routing note: independent source review accepts the replacement Account packet at `35edb2830`, integrated through `e69acf279`. WP08 schema validation, sealed current authority, and local SQLite CAS repository are present. Reviewed source at `86caae334` and `7934fb41b` adds the target-aware WP02 action resolver, preserves parent actor versus child/profile/device target identity, derives current account/household/member/device/role identity from the opaque Account binding, rejects caller-supplied authority, and migrates the real storage-custody consumer. Capability, controller-lease, and step-up actions remain fail-closed because those authority sources are not present. The source wave deliberately did not run tests or claim proof. The next coherent source chain is Cloudflare WP06 authoritative D1 writer/currentness/revocation/CAS plus its shipped provider caller, then Device Trust WP03 live ceremony composition; WP02 expected tests follow in the later complete test-source wave.

Do not revive the rejected `ac03afee3a` WP02-WP05 record packet. Its public
serde records were disconnected from production and accepted caller-mintable
authority/lifecycle facts. The dependency-first production sequence is:

```text
accepted WP08 schema + sealed authority + local repository/CAS
  -> WP02 target-aware actor/target resolver with unavailable capability/lease/step-up actions fail-closed
  -> Cloudflare WP06 authoritative D1 writer/currentness/revocation/CAS and provider-to-Account caller
  -> Device Trust WP03 RegisterLanSignerAnchor actor/target composition
  -> complete WP04 atomic invite/recovery orchestration and typed custody handoff
  -> complete WP05 Device Trust/remote/export/delete step-up consumers
  -> write the full WP02-WP05/WP08 expected-test wave
  -> run focused tests and focused Enforcer only after test source is complete
  -> Cloudflare WP08 runner proof and Account WP06 aggregation
```

Source packets must be reachable from a shipped caller, derive trust from owned
state, keep terminal transitions monotonic, and fail closed. A new DTO, enum, or
test-only constructor with no production caller is not progress.

PR #607 is closed without merge. Do not rebase its TypeScript Cloudflare
adapter/D1-test-double slice into this plan. Start with Rust-owned account
schema authority. Then hand the contract to Cloudflare WP06 for D1/DO/KV
persistence/migration and Cloudflare WP08 for runner/integration proof.

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
or an unapplied/mismatched custody version. Independent review remains open.
The expected route/store/request-safety tests are not
present yet and remain deferred to the test/proof phase:
`infra/cloudflare/tests/unit/account-browser-session-store.test.ts`,
`infra/cloudflare/tests/unit/account-browser-session-routes.test.ts`,
`infra/cloudflare/tests/security/account-browser-session-request-safety.test.ts`,
and `infra/cloudflare/tests/integration/account-browser-session-real.test.ts`.
Applied migrations, deployment, retained proof, and DONE remain open.

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
```

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
