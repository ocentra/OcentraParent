# Remote Access Route Index

Use [AGENTS.md](AGENTS.md), then [PLAN_STATE.md](PLAN_STATE.md), [NEXT_ACTIONS.md](NEXT_ACTIONS.md), and [WORKPACK_INDEX.md](WORKPACK_INDEX.md). Use [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when the selected workpack owner/proof family is unclear.

## Owns

```text
remote live-view capability authority
standing access after pairing
remote relay session semantics
remote capability grants
pairing, disclosure, revocation, and remove-device lifecycle
relay abuse controls and degraded states
remote proof and rollout gate
```

## Boundary split

```text
screen-plan owns capture primitives, protected surfaces, screenshot custody, and local screen retention settings.
lan-plan owns local pairing and LAN-only transport.
account-identity-family-plan owns account, household, role, session, and device authority.
device-trust-bootstrap-plan owns trusted-device and parent-presence step-up.
data-custody-storage-plan owns retention/export/delete/privacy for remote artifacts and diagnostics.
portal-ux-household-surfaces-plan owns rendered parent/child-visible remote state.
eventing-plan owns reusable idempotency/replay/journal mechanics.
remote input/control is deferred in WP03 and is not part of the current live-view pass.
```

## Handoff rule

Open a sibling plan only when the selected workpack names the exact handoff, owner path, expected proof, and no-claim boundary.

## No-claim rule

Do not claim remote readiness from local screen capture proof, local LAN proof, UI-only proof, relay route existence, legacy docs, or live-view proof as control. Remote product claims require selected proof roots under `output/remote-access-plan-proof/<workpack>/` or exact carried blockers.
