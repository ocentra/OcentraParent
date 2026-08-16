# 07 Parent Requests And Approvals

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `07 Parent Requests And Approvals`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

Approval/override audit proof exists in runtime work, but the parent-facing flow
is not complete.

## Where We Want To Be

Pending requests, approvals, denials, expiry, bonus time, and overrides are
clear, actionable, and tied to child/device context.

## Requirement Checklist

- [ ] Show request target, child, device, time, reason, and expiry.
- [ ] Show approve, deny, bonus time, and expired states.
- [ ] Render delivery/manual-required state honestly.
- [ ] Add audit/history visibility.
- [ ] Test pending and expired states.

## Acceptance And Proof

Parent actions render service-returned pending/result state and cannot mark
approval successful locally.

## Parallel Ownership Notes

Notification provider delivery is not owned by C.
