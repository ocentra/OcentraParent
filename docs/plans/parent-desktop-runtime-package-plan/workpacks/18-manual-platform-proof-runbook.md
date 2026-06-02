# 18 Manual Platform Proof Runbook

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

CI can prove repeatable mechanics. Real hosts/devices are required for signing,
stores, OS permissions, and package install behavior.

## Where We Want To Be

Manual proof records name commit, platform, package/app version, command or UI
action, permission state, logs/screenshots, and observed result.

## Requirement Checklist

- [ ] Define manual proof artifacts per platform.
- [ ] Include distinct parent/child and package/runtime claims.
- [ ] Record signing/store/entitlement gaps.
- [ ] Keep artifacts out of source unless intentionally tracked.
- [ ] Update proof matrix after manual checks.

## Acceptance And Proof

Manual proof requirements are explicit before any production claim is made.

## Parallel Ownership Notes

The user/manual tester may execute this gate after branch/CI readiness.
