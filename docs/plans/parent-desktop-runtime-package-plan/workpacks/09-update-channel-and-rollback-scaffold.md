# 09 Update Channel And Rollback Scaffold

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `09 Update Channel And Rollback Scaffold`
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

Updater scaffolding exists, but production update signing and rollback behavior
are not fully proved.

## Where We Want To Be

Update state distinguishes scaffold, preview, signature-required, rollback
available, rollback unavailable, and production boundary.

## Requirement Checklist

- [ ] Represent update channel and rollback states.
- [ ] Keep unsigned preview labels.
- [ ] Reject production claims without signing proof.
- [ ] Add tests for matrix/proof output.
- [ ] Update release docs when status changes.

## Acceptance And Proof

Parents/support can tell whether update behavior is production, preview, or
manual-required.

Current proof: scaffold, unsigned-preview, signature-required, production
promotion, and rollback-unavailable states are parsed and tested. Rollback
available is rejected for unsigned preview or unpromoted production state.

## Parallel Ownership Notes

Do not bypass update signature rules in production code.
