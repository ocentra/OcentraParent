<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `WP06 Rollout Proof And Route Gate`
> Kind: final proof aggregation and route-sync workpack.
> Read when: selected by WORKPACK_INDEX.md or when making DONE/PR_READY claims for this plan.
> Stop rule: do not implement new feature scope here; consume prior proof roots, close route gaps, and record blockers.
> Proves: setup-install route readiness only after prior proof roots and this gate pass.
> Does not prove: account, package, LAN, device trust, data custody, policy, payment, or portal shell readiness.
> Proof rule: before DONE/PR_READY, write all WP06 proof artifacts and command log.

<!-- /agent-capsule -->

# WP06 Rollout Proof And Route Gate

## Goal

Define and verify the proof package required before setup-install-provisioning can claim execution-grade readiness or downstream handoff readiness.

## Ownership boundary

```text
WP06 aggregates setup-install-provisioning proof roots and records blockers.
It does not implement or accept sibling proof for account, runtime distribution, LAN, device trust, custody, policy, payment, or broad portal UX unless those plans provide explicit proof artifacts.
```

## Required inputs

```text
PROOF_INDEX.md
TEST_PROOF_EXPECTATIONS.md
CHECKLIST_INDEX.md
output/setup-install-provisioning-plan-proof/01-family-web-info-site/
output/setup-install-provisioning-plan-proof/02-registration-login-entry/
output/setup-install-provisioning-plan-proof/03-parent-install-journey/
output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/
output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/
output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/
```

If any prior proof root is missing, this workpack records a blocker and does not claim readiness.

## Required proof pack

```text
public site boundary proof
registration/account handoff proof
parent install journey proof
child install/permission journey proof
pairing/readiness/recovery proof
first-run UI/state-machine proof
platform readiness matrix
public/private boundary proof
manual-required gap register
route/index sync proof
```

## Required rollout fields

The selected rollout proof must name, at minimum:

```text
rollout_gate_id
accepted_proof_roots
missing_proof_roots
carried_blockers
wp01_state
wp02_state
wp03_state
wp04_state
wp05_state
wp07_state
account_owner_state
parent_runtime_owner_state
child_runtime_owner_state
device_trust_owner_state
lan_owner_state
data_custody_owner_state
policy_owner_state
payment_owner_state
portal_ux_owner_state
platform_readiness_matrix_state
public_private_boundary_state
manual_required_gap_state
product_status_wording_state
route_index_sync_state
claims_allowed
claims_blocked
pr_ready_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Route sync boundaries

This plan exports setup state to these plans; it does not absorb their implementation:

```text
account-identity-family-plan owns account, session, household, roles, invites, recovery authority.
parent-client-runtime-distribution-plan owns parent package/sign/update/rollback proof.
child-agent-runtime-distribution-plan owns child package artifacts.
app-plan owns child local service/runtime/platform adapter behavior.
device-trust-bootstrap-plan owns trusted-device bootstrap/key-sealing/approval proof.
lan-plan owns discovery, pairing protocol, signed local transport, and physical LAN proof.
portal-ux-household-surfaces-plan owns broader rendered portal shell and household surfaces.
data-custody-storage-plan owns storage/export/delete/sync/custody guarantees.
payment-subscription-plan owns subscription/entitlement after setup/account authority.
policy-control-plane-plan owns policy baseline after account/setup authority.
```

## Required rollout states

```text
notImplemented
previewOnly
manualRequired
readyForTest
productionReady
blocked
```

## Required proof root

```text
output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/
```

Required artifacts:

```text
00-rollout-proof-pack.md
01-route-sync-proof.md
02-platform-readiness-matrix.md
03-public-private-boundary-proof.md
04-manual-required-gap-register.md
05-product-status-safe-wording-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] WP01 proof root exists or blocker recorded.
- [ ] WP02 proof root exists or blocker recorded.
- [ ] WP03 proof root exists or blocker recorded.
- [ ] WP04 proof root exists or blocker recorded.
- [ ] WP05 proof root exists or blocker recorded.
- [ ] WP07 proof root exists or blocker recorded.
- [ ] Rollout proof pack is written.
- [ ] Route/index sync proof is written.
- [ ] Platform readiness matrix is written.
- [ ] Public/private boundary proof is written.
- [ ] Manual-required gap register is written.
- [ ] Product-status wording is safe.
- [ ] Focused commands pass or blockers are recorded.
- [ ] PLAN_STATE and CHECKLIST_INDEX reflect actual proof state.

## Focused commands

```bash
node -e "console.log('setup-rollout-route-gate')"
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan
```

If UI/source routes changed:

```bash
npm run test --workspace @ocentra-parent/portal -- setup
npm run test:e2e --workspace @ocentra-parent/portal -- setup
```

If policy/eventing lanes are active, do not route-sync those plan files directly. Record a route-sync blocker or queued handoff instead.

## Negative states

- PR_READY without named proof roots.
- Website-only proof called onboarding complete.
- Parent install proof called child setup complete.
- Child install proof called pairing/trust/policy complete.
- Pairing proof called product setup complete.
- Production-ready wording without package/platform/support proof.
- Public page route claims private activity custody.
- Aggregation erases sibling-owner blockers.

## Manual-required gaps

Any adjacent plan not yet updated must be listed in `04-manual-required-gap-register.md`. A route-sync gap blocks whole-plan PR_READY but does not necessarily block a single workpack PR.

## Fill before DONE / PR_READY

```text
Workpack id and branch: WP06 Rollout Proof And Route Gate / codex/tracking-plan-full-continuation-a
Prior proof roots consumed: WP01, WP02, WP03, WP04, WP05, and WP07 consumed directly.
Route-sync status: route ownership is documented, but whole-plan readiness stays blocked because sibling owner proofs are still open; the old WP03 parent-domain build blocker is removed, the WP05 redaction-ownership gap is now locally closed, and the package test wrapper still misroutes into an unrelated app-game proof harness.
Touched files: output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/00-rollout-proof-pack.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/01-route-sync-proof.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/02-platform-readiness-matrix.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/03-public-private-boundary-proof.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/04-manual-required-gap-register.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/05-product-status-safe-wording-proof.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/16-validation-commands.log, docs/plans/setup-install-provisioning-plan/workpacks/06-rollout-proof-and-route-gate.md, docs/plans/setup-install-provisioning-plan/PLAN_STATE.md, docs/plans/setup-install-provisioning-plan/WORKPACK_INDEX.md, docs/plans/setup-install-provisioning-plan/CHECKLIST_INDEX.md
Validation commands and results: exact-file readback over the claimed setup-plan truth-sync files PASS; `git diff --check` over the claimed setup-plan truth-sync files PASS; this docs-only WP06 pass consumed the immediately previous WP03 export-map repair result and did not rerun package/crate/app validation.
Proof artifacts: output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/00-rollout-proof-pack.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/01-route-sync-proof.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/02-platform-readiness-matrix.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/03-public-private-boundary-proof.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/04-manual-required-gap-register.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/05-product-status-safe-wording-proof.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/16-validation-commands.log
Manual-required gaps: WP03 proof root exists and now records a green parent-domain build plus a direct green release-support suite, but the `npm run test --workspace @ocentra-parent/parent-domain -- parent-desktop-release-support` wrapper path still detours into an unrelated app-game proof harness; account/provider/session, runtime distribution, LAN/device-trust, portal shell beyond WP07, data custody, policy baseline, and payment proofs remain owned by sibling plans.
No-claim boundaries: no PR_READY claim, no production onboarding claim, no installer readiness claim, no first-run readiness claim, and no sibling implementation claim from aggregation alone.
```
