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
WP07 First-Run Setup UI And State Machine: accepted Rust fail-closed source is integrated through `ca230550b`; it renders 15 explicit unavailable/manual-required authority rows with evaluator `not-run`, preserves LAN diagnostic state, owns the canonical 13-command LAN classifier, rejects LAN commands on non-LAN routes, and makes no readiness/progression claim. Existing Rust/portal/E2E tests are stale; tests, builds, proof, precommit, CI, and PR remain deferred until the source wave closes.
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

## Current phase order

```text
production source: accepted fail-closed WP07 boundary; real owner-backed setup progression still blocked
expected-test source: repair the four stale setup test families after the source wave closes
focused run/fix: deferred
proof/checklist closeout: deferred
```

## WP07 source-wave handoff

The accepted source packet is intentionally narrow. The Rust Start panel exposes
all 15 required authorities as unavailable/manual-required, consumes LAN
selected-device/pairing/reachability values only as observations, and never
invokes provisioning evaluation or action planning. `AgentCommandName::is_lan_command`
is the canonical 13-variant classifier, and generic parent dispatch rejects LAN
commands on non-LAN routes (LAN discovery is likewise rejected outside
LAN-owned routes).

Deferred test work is exact and remains open: update
`crates/parent-runtime-core/tests/integration/parent_ui_bridge/snapshot_and_dispatch_tests.rs`
for the new Start snapshot and non-LAN rejection behavior; add the canonical
13-command classifier coverage; then update the existing portal setup route
unit/E2E fixtures and run the workpack's focused commands. No test, build, proof,
CI, or PR result is implied by this source handoff.

The missing authenticated composition remains account/session/household,
signed parent package, child package/service/permission, device trust, trusted
LAN pairing, custody sync, policy baseline, network reachability, and recovery
owner inputs. These are blockers to progression, not permission to synthesize
readiness.

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
