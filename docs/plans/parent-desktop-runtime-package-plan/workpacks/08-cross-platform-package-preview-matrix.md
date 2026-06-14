# 08 Cross-Platform Package Preview Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `08 Cross-Platform Package Preview Matrix`
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

Package preview states exist but can be confused with platform runtime support.
Current CI separates parent desktop package previews, parent Android/iOS mobile
scaffold previews, and child Android/iOS agent previews.

## Where We Want To Be

Windows, macOS, Linux, Android parent, iOS parent, Android child, and iOS child
states are separated by proof level.

## Requirement Checklist

- [ ] Add or update platform matrix output.
- [ ] Split package, parent shell, child agent, signing, store, and relay rows.
- [ ] Add separate parent Android and parent iOS package-preview targets; do
      not reuse child-agent package previews as parent mobile proof.
- [ ] Mark scaffold/manual-required honestly.
- [ ] Test row stability.
- [ ] Reference matrix in PR reports.

## Acceptance And Proof

One platform preview cannot upgrade another platform or child-agent claim.

## Parallel Ownership Notes

A owns enforcement capability rows; D owns package/platform rows.
