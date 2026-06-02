# 01 Tauri Shell Contract Boundary

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

The Tauri app is the parent desktop shell candidate. It must not drift into
child-device authority.

## Where We Want To Be

The desktop shell owns packaging, launch, connection, and display of typed
service state only.

## Requirement Checklist

- [ ] Keep capture, AI, policy, enforcement, and timers out of Tauri commands.
- [ ] Use typed service/protocol output.
- [ ] Document shell ownership in README/docs.
- [ ] Test command output boundaries.
- [ ] Label unavailable service states.

## Acceptance And Proof

Tests and docs show the shell connects to child-agent state instead of executing
child-agent work.

## Parallel Ownership Notes

D owns this boundary. A/B own enforcement/LAN runtime claims.
