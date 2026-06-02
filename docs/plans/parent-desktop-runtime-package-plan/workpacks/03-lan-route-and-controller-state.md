# 03 LAN Route And Controller State

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

LAN route and controller states exist in product docs and runtime proof. Desktop
packaging must present them honestly.

## Where We Want To Be

Parent desktop can show local, LAN, relay, cache, stale, offline, controller, and
observer route state without becoming a LAN implementation.

## Requirement Checklist

- [ ] Read route state from service/protocol output.
- [ ] Show controller lease versus observer read-only.
- [ ] Label relay/cache unavailable states.
- [ ] Avoid fallback to another child silently.
- [ ] Test route-state serialization.

## Acceptance And Proof

Package proof can explain whether the desktop is live local, LAN, stale, or
unavailable.

## Parallel Ownership Notes

B owns LAN discovery/pairing implementation.
