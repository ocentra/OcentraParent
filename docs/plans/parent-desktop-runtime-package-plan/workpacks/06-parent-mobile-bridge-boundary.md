# 06 Parent Mobile Bridge Boundary

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

## Parallel Ownership Notes

Future mobile work may become its own lane. D keeps current package boundary.
