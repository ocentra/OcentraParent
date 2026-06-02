# 15 Platform Capability Matrix

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Platform expectations require exact status per platform and capability.

## Where We Want To Be

Matrix rows match package, parent shell, child agent, signing, store, relay, and
support states across platforms.

## Requirement Checklist

- [ ] Split implemented, scaffold, unavailable, degraded, and manual-required.
- [ ] Include parent desktop/mobile and child-agent rows separately.
- [ ] Include signing/store/relay rows.
- [ ] Generate or test deterministic output.
- [ ] Sync docs/checklist when rows change.

## Acceptance And Proof

Platform claim matrix is reviewed with the branch before PR/merge.

## Parallel Ownership Notes

This matrix should align with A and B proof matrices without duplicating them.
