# Workpack 06: Rollout Proof And Route Gate

Goal: define the proof package required before policy control plane can be described as ready for downstream handoff.

Owns: route/index sync, proof manifest, exact test IDs, proof artifact inventory, skipped-risk notes, manual-required gap register, and product-status wording.

## Ownership boundary

```text
WP06 aggregates policy-control-plane-plan proof roots only.
Portal, account, device-trust, data-custody, eventing, domain, AI, notification, and enforcement owners remain separate unless their handoff proof is explicitly accepted.
Proof manifest presence supports routing only; it does not replace workpack closeout artifacts.
```

## Required proof pack

```text
source-of-truth proof
authoring/preview proof
schedule/time/DST proof
conflict precedence proof
domain compiler proof
delivery/ack/audit proof
ask-parent/override proof
authZ/assistant-confirmation proof
rollback proof
route/index sync
manual-required gap register
```

## Required rollout fields

The selected rollout proof must name, at minimum:

```text
rollout_gate_id
accepted_proof_roots
missing_proof_roots
carried_blockers
source_truth_state
authoring_preview_state
schedule_conflict_state
compiler_state
delivery_ack_state
ask_parent_override_state
event_replay_state
account_authority_state
device_trust_state
data_custody_state
enforcement_handoff_state
claims_allowed
claims_blocked
manual_required_gaps
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Required proof IDs

- `policy-rollout.proof-pack-complete`
- `policy-rollout.source-proof`
- `policy-rollout.preview-proof`
- `policy-rollout.schedule-proof`
- `policy-rollout.compiler-proof`
- `policy-rollout.delivery-proof`
- `policy-rollout.override-proof`
- `policy-rollout.authz-negative-proof`
- `policy-rollout.rollback-proof`
- `policy-rollout.route-sync`
- `policy-rollout.manual-required-gap-register`
- `policy-rollout.no-overclaim`

## Required proof artifacts

- `docs/proof/policy-control-plane-plan/06-rollout-proof-pack.md`
- `docs/proof/policy-control-plane-plan/06-route-sync-proof.md`
- `docs/proof/policy-control-plane-plan/06-no-overclaim-proof.md`
- `docs/proof/policy-control-plane-plan/06-manual-required-gap-register.md`
- `docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md`

## Failure examples

```text
only portal UI screenshot
only happy-path schedule
no offline child or device state
no rollback or audit proof
route/index out of sync
policy ready claimed from compiler-only proof
proof manifest used as workpack proof
WP02/WP05 omitted without carried blockers
```

## Route rules

- Parent policy truth routes here before domain-specific policy effects.
- Domain plans own compiled effects only after this plan defines source truth and handoff.
- `portal-ux-household-surfaces-plan` proves rendered UI only; it does not prove policy delivery.
- `v0-8-enforcement-control-plan` proves enforcement authority only after policy delivery and ack are established.

## Required report contents

The report must name:

```text
policy source document/version
authoring/preview state
schedule/timezone/DST proof
domain compiler outputs
delivery/ack state
ask-parent/override state if touched
enforcement/audit handoff
unresolved Sujan decisions
remaining WP02/WP05 state
```
