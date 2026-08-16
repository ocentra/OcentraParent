# 10 Parent Approval And Override

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `10 Parent Approval And Override`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md),
[folder README](../README.md),
[policy-schedules-approvals feature](../../features/policy-schedules-approvals.md), and
[enforcement expectation](../../expectations/enforcement.md).

## Purpose

Define the typed ask-parent / approval / denial / override path so a child
request becomes an enforcement action only after parent-authorized,
expiry-aware, audited state transitions.

## Central schema boundary

```text
schema-domain owns public request, approval, denial, override, expiry, reason, and audit schemas when they cross package/crate/protocol boundaries.
policy-control-plane-plan owns parent authority, schedule/budget override semantics, and upstream policy truth.
account-identity-family-plan owns actor/role/device authority when selected.
v0-8-enforcement-control-plan owns the action-state transition, audit linkage, and no-claim boundary.
```

## Source Inputs

- `../v0-8-enforcement-control-20-step-plan.md`
- `../v0-8-enforcement-control-test-blueprint.md`
- `../../features/policy-schedules-approvals.md`
- `../../expectations/enforcement.md`

## Target State

A child request can become a parent-approved or denied action through typed,
audited, expiry-aware intents validated by the child-device agent.

## Required proof fields

```text
canonical_schema_owner_state
request_state
approval_state
denial_state
expiry_state
duplicate_state
override_state
actor_authority_state
device_route_state
audit_state
notification_delivery_state
manual_required_state
no_child_approval_claim
no_notification_claim
no_claim
```

## Tests And Proof

Proof root: `output/v0-8-enforcement-control-plan-proof/10-parent-approval-override/`

Focused validation should record:

- `npm run test --workspace @ocentra-parent/enforcement-domain -- enforcement`
- selected approval/override service or protocol tests
- selected portal tests only when parent-visible approval state changes
- selected architecture gate for touched approval/enforcement surfaces

## AI Worker Checklist

- [ ] Add request, approval, denial, expiry, duplicate, and override states.
- [ ] Validate child, device, route, actor, policy version, and target.
- [ ] Record audit events for every transition.
- [ ] Show pending and expired state in parent surfaces.
- [ ] Keep notification delivery separate unless a provider is proved.

## Where We Are

Approval and override audit-reference proof exists. Product flow still needs
typed request, response, expiry, bonus-time, and child-agent validation.

## Negative Cases

- child requests must not self-approve
- wrong-device, wrong-route, or revoked-actor requests must reject
- duplicates and expired approvals must remain explicit and audited
- approval proof must not imply notification delivery proof
- malformed target or policy-version refs must block action-ready state

## Manual-Required Gaps

- Push or out-of-band notification delivery remains a separate proof surface.
- High-risk parent presence or step-up trust handoffs remain separate.
- Cross-platform child-agent parity remains unproved where not explicitly tested.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch recorded.
- [ ] Validation commands and results recorded in `16-validation-commands.log`.
- [ ] Proof artifacts under `output/v0-8-enforcement-control-plan-proof/10-parent-approval-override/`.
- [ ] Known gaps/manual-required states listed here and in the proof note.
