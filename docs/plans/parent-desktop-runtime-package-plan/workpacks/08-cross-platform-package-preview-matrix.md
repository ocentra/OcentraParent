# 08 Cross-Platform Package Preview Matrix

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Package preview states exist but can be confused with platform runtime support.

## Where We Want To Be

Windows, macOS, Linux, Android parent, iOS parent, Android child, and iOS child
states are separated by proof level.

## Requirement Checklist

- [ ] Add or update platform matrix output.
- [ ] Split package, parent shell, child agent, signing, store, and relay rows.
- [ ] Mark scaffold/manual-required honestly.
- [ ] Test row stability.
- [ ] Reference matrix in PR reports.

## Acceptance And Proof

One platform preview cannot upgrade another platform or child-agent claim.

## Parallel Ownership Notes

A owns enforcement capability rows; D owns package/platform rows.
