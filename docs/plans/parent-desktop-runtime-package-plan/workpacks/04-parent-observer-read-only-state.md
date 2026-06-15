# 04 Parent Observer Read-Only State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `04 Parent Observer Read-Only State`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Parent-controller and observer roles are part of the platform/LAN state. Package
proof must preserve write authority boundaries.

## Where We Want To Be

Observer state can view permitted read models but cannot write policy, approve,
revoke, or take controller authority.

## Requirement Checklist

- [ ] Represent observer read-only state.
- [ ] Reject or disable write actions for observer role.
- [ ] Show why an action is unavailable.
- [ ] Include support/status refs where available.
- [ ] Test observer output.

## Acceptance And Proof

Observer state cannot be mistaken for controller authority in package proof.

Current proof: `packages/parent-domain/src/parent-desktop-release-support.ts`
and `packages/parent-domain/tests/parent-desktop-release-support.test.ts`
accept read-only service/route reads and reject observer policy writes,
approval decisions, and controller takeover.

## Parallel Ownership Notes

Portal UX may render this state. D owns shell/runtime package representation.
