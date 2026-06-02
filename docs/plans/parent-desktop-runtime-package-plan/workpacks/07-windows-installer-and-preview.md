# 07 Windows Installer And Preview

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Windows MSI/updater scaffolding and package preview mechanics exist. Production
signing is not claimed.

## Where We Want To Be

Windows package preview can be built and smoke-checked while signing and release
states remain explicit.

## Requirement Checklist

- [ ] Build or verify Windows package preview where available.
- [ ] Smoke launch the parent shell where feasible.
- [ ] Label unsigned/dev preview.
- [ ] Keep production release boundary explicit.
- [ ] Record artifacts/commands in reports.

## Acceptance And Proof

Package proof shows preview mechanics without claiming signed production
installer readiness.

## Parallel Ownership Notes

Primary watches CI before merge. D owns branch fixes.
