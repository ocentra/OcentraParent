# 14 Tauri Build And Dev Scripts

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Managed scripts exist for local dev and package checks. They must stay safe for
parallel lanes.

## Where We Want To Be

Tauri build/dev/package scripts use repo defaults and lane-specific ports without
taking over unrelated Ocentra processes.

## Requirement Checklist

- [ ] Use managed repo scripts.
- [ ] Respect lane agent/portal ports.
- [ ] Avoid generic port assumptions.
- [ ] Document useful commands in README.
- [ ] Validate script changes.

## Acceptance And Proof

Script validation passes and reports mention exact commands.

## Parallel Ownership Notes

Do not alter Ocentra Games or unrelated project ports/processes.
