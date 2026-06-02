# 02 Local Service Connection Command

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Parent desktop proof needs to show real service connectivity rather than a Vite
backend assumption.

## Where We Want To Be

The Tauri command reports local service availability, route state, controller
state, and package metadata through typed output.

## Requirement Checklist

- [ ] Connect to configured Rust service path.
- [ ] Return unavailable/degraded state when service is missing.
- [ ] Include controller/observer/source state where available.
- [ ] Avoid hardcoded success responses.
- [ ] Add focused script tests.

## Acceptance And Proof

The proof script shows available and unavailable service outcomes.

## Parallel Ownership Notes

This does not prove the child service itself; it proves the parent shell bridge.
