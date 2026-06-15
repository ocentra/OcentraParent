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
4. Open only that workpack.
5. Use `TEST_PROOF_EXPECTATIONS.md` and `PROOF_INDEX.md` for proof obligations.
6. Implement narrow source/docs/test changes.
7. Update `CHECKLIST_INDEX.md`, selected workpack, and `PLAN_STATE.md` only after proof exists.

## Highest-priority queue

### 1. WP01 Auth Provider Decision

Expected result:

```text
source-backed provider decision record
Cloudflare-owned account/family custody decision
Firebase/Auth.js boundaries accepted/rejected or explicitly staged
no product family data in IdP/custom claims
provider outage and migration path documented
```

Do this before runtime account/session work.

### 2. WP02 Identity Household Role Model

Expected result:

```text
account user, household, membership, role, child profile, and device authority shapes
role/action/resource matrix
support/admin minimized actor state
cross-family negative proof
```

### 3. WP03 Session Token Lifecycle

Expected result:

```text
credential type matrix
session/refresh/logout/global revoke lifecycle
device credential and controller lease separation
expiry, replay, CSRF/origin, stale-token proof
redacted session audit model
```

### 4. WP04 Invites Recovery Lifecycle

Expected result:

```text
invite state machine
co-parent/observer/child-device invite scopes
single-use/expiry/revocation/replay proof
recovery flows for forgotten login, lost parent device, compromised account, child reinstall, transfer
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

### 7. WP06 Security Proof and Route Gate

Expected result:

```text
rollout proof pack consumes WP01-WP05/WP07 proof
route sync confirms adjacent plans consume account identity without owning it
remaining manual-required gaps listed
```

## Blocked execution rules

- Runtime implementation is blocked until WP01 provider/custody decision is complete or the selected workpack explicitly implements that decision.
- UI implementation is blocked until the required contract shape exists or a stub/blocker proof is written.
- Payment/policy/remote/device-trust integration is blocked until WP06 route gate proof exists.
- Any claim involving secure auth/session requires WP03 and WP06 proof.

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
