# Workpack 06: Rollout Proof And Route Gate

Goal: define the proof package required before policy control plane can be described as ready for downstream handoff.

Owns: route/index sync, proof manifest, exact test IDs, proof artifact inventory, skipped-risk notes, manual-required gap register, and product-status wording.

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
```
