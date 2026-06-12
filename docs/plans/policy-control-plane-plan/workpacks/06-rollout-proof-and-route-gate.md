# Workpack 06: Rollout Proof and Route Gate

Goal: define proof required before policy control plane is called ready.

Expected proof:

- Source-of-truth schema proof.
- Authoring/preview proof.
- Domain compiler proof.
- Delivery/ack/audit proof.
- Ask-parent/override proof if touched.
- Route/index sync.

Failure: PR_READY without negative tests for schedule, authZ, replay, delivery, and conflict handling.

## Execution Detail

Required proof pack:

- Source-of-truth schema/contract proof.
- Parent authoring preview/conflict proof.
- Domain compiler fixture proof.
- Delivery/ack/audit proof.
- Ask-parent/override proof if touched.
- AuthZ and assistant-confirmation proof.
- Route/index sync.

Expected tests/proof names:

- `policy.rollout.source-proof`
- `policy.rollout.preview-proof`
- `policy.rollout.compiler-proof`
- `policy.rollout.delivery-proof`
- `policy.rollout.authz-negative-proof`
- `policy.rollout.route-sync`

Failure examples:

- Only portal UI screenshot.
- Only happy-path schedule.
- No offline child/device state.
- No rollback/audit proof.

## Research Gate

This rollout gate cannot be closed from first-pass docs. The assigned agent must inspect existing portal policy UI, parent-domain policy/read-model code, app-game/browser/network/tracking/screen/AI policy paths, and enforcement handoff docs. Source-of-truth, schedule semantics, assistant action authority, and parent UX decisions must be discussed with Sujan before product status moves.

## Required Route Updates

- Parent policy truth routes here before domain-specific policy effects.
- Domain plans own compiled effects only after this plan defines source truth and handoff.
- `portal-ux-household-surfaces-plan` proves rendered UI only; it does not prove policy delivery.
- `v0-8-enforcement-control-plan` proves enforcement authority only after policy delivery/ack is established.

## Minimum DONE Report

The report must name:

- policy source document/version.
- authoring/preview state.
- schedule/timezone/DST proof.
- domain compiler outputs.
- delivery/ack state.
- ask-parent/override state if touched.
- enforcement/audit handoff.
- unresolved Sujan decisions.
