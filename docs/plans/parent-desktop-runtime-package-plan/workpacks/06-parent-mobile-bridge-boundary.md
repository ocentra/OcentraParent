# 06 Parent Mobile Bridge Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `06 Parent Mobile Bridge Boundary`
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

Parent mobile scaffold proof exists separately from Android/iOS child-agent
proof.

## Where We Want To Be

Parent mobile shell states can reuse route/provider contracts while child mobile
agent claims remain scaffold/manual-required.

## Requirement Checklist

- [ ] Split parent mobile from child Android/iOS agent rows.
- [ ] Label mobile package scaffold/proof level.
- [ ] Avoid claiming Device Owner, Family Controls, VPN/DNS, or entitlements.
- [ ] Keep mobile service provider routing optional/degraded.
- [ ] Update platform docs when status changes.

## Acceptance And Proof

Reports never say "mobile support" without naming parent versus child and proof
level.

Current proof: the release-support matrix has separate `parent-mobile`,
`child-android`, and `child-ios` rows. It rejects child mobile agent parity
claims from the parent desktop release-support proof.

## Parallel Ownership Notes

Future mobile work may become its own lane. D keeps current package boundary.
