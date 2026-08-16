<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Provisioning Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: after PLAN_STATE.md.
> Stop rule: pick one workpack; do not broaden into sibling plans.
> Proves: next-action routing only.
> Does not prove: implementation completion or PR readiness.
> Proof rule: update this file only when queue state changes.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan Next Actions

## How to use

1. Confirm branch and assignment.
2. Open `WORKPACK_INDEX.md`.
3. Select exactly one workpack.
4. Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
5. Open only that workpack.
6. Use `TEST_PROOF_EXPECTATIONS.md` and `PROOF_INDEX.md` for proof obligations.
7. Update `CHECKLIST_INDEX.md`, selected workpack, and `PLAN_STATE.md` only after proof exists.

## Current owned-workpack state

```text
WP01 Family Web Info Site: local setup-owned slice done; deployment/custom-domain remains blocker-only.
WP02 Registration Login Entry: local account-entry handoff slice done; provider/session/household implementation remains account-owned.
WP03 Parent Install Journey: local parent-install journey slice done; signed package/update/rollback/store delivery remains package-owner proof.
WP04 Child Install Permission Journey: local child install/permission journey slice done; child runtime/package/platform execution remains sibling-owned.
WP05 Pairing Readiness Recovery: local setup pairing/readiness slice done; physical LAN/device-trust proof remains sibling-owned.
WP07 First-Run Setup UI And State Machine: selected Start route projection proof done; broader portal/sibling readiness remains blocked.
WP06 Rollout Proof And Route Gate: local aggregation/blocker pack done; whole-plan PR_READY remains false.
```

## Current blocker queue

```text
account/provider/session/household/invite/recovery proof -> account-identity-family-plan
parent package/signing/update/rollback/distribution proof -> parent-desktop-runtime-package-plan
child package/runtime/platform permission proof -> child-agent-runtime-distribution-plan and app/runtime owners
trusted-device/key/step-up proof -> device-trust-bootstrap-plan
LAN discovery/signed hello/pairing physical proof -> lan-plan
data custody/export/delete/sync proof -> data-custody-storage-plan
policy baseline production proof -> policy-control-plane-plan
payment/subscription/entitlement proof -> payment-subscription-plan
broader portal shell/household UX proof -> portal-ux-household-surfaces-plan
```

## If new setup work is assigned

- For public website/data boundary updates, use WP01.
- For auth-entry route labels only, use WP02; do not add account/session logic here.
- For parent install journey labels only, use WP03; do not add packaging/signing/update logic here.
- For child install/permission labels only, use WP04; do not add runtime/package/platform adapter logic here.
- For pairing/readiness/recovery journey labels only, use WP05; do not add LAN/device-trust internals here.
- For first-run setup route projection only, use WP07; do not claim sibling readiness.
- For broad status, route sync, or PR wording, use WP06.

## PR readiness guard

A partial PR may be acceptable only when one workpack is fully closed and remaining workpacks/blockers are listed.

Do not create PR-ready claims from:

```text
website-only route map
login button without account handoff proof
download button without package/distribution proof
child installer without permission/pairing/readiness proof
UI mock without source/custody/degraded labels
rollout aggregation while sibling-owner proof remains blocked
```
