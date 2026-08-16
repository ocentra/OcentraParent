# 03 LAN Route And Controller State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `03 LAN Route And Controller State`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

## Decision Tree

| If the assignment touches...         | Read next                                      | Required proof                    |
| ------------------------------------ | ---------------------------------------------- | --------------------------------- |
| LAN discovery/pairing implementation | `../../lan-plan/AGENTS.md`                     | LAN plan workpack proof           |
| Remote relay/cloud route             | `../../remote-access-plan/AGENTS.md`           | relay/session proof               |
| Account/device authority             | `../../account-identity-family-plan/AGENTS.md` | authZ matrix proof                |
| Desktop display of route state       | this workpack and nearest shell surface        | route-state render/snapshot proof |

## Expected Route States

- `localLive`: parent shell can reach the local service for the selected household/device.
- `lanLive`: selected child/device is reachable over LAN through a proved route.
- `relayLive`: remote relay route is active and authorized by remote-access proof.
- `cached`: parent shell is displaying last-known state with age and source.
- `stale`: data is too old for live claim; actions must be disabled or manual-required.
- `offline`: device/service is unavailable.
- `observerOnly`: parent can view allowed state but cannot control.
- `controllerLease`: parent has controller authority with expiry and revocation.
- `manualRequired`: platform/permission/proof gap blocks automation.

## Requirement Checklist

- [ ] Read route state from service/protocol output.
- [ ] Show controller lease versus observer read-only.
- [ ] Label relay/cache unavailable states.
- [ ] Avoid fallback to another child silently.
- [ ] Test route-state serialization.
- [ ] Prove stale/offline/cached states disable unsafe actions.
- [ ] Preserve selected household/device identity and prevent cross-family route bleed.
- [ ] Record relay/LAN/local route proof source instead of inferring it in desktop UI.

## Acceptance And Proof

Package proof can explain whether the desktop is live local, LAN, stale, or
unavailable.

Expected proof names:

- `parent-desktop.route-state.local-live`
- `parent-desktop.route-state.lan-live`
- `parent-desktop.route-state.relay-live`
- `parent-desktop.route-state.cached-stale-offline`
- `parent-desktop.route-state.observer-controller-authz`
- `parent-desktop.route-state.cross-family-negative`

Proof must include selected household/device IDs as redacted test identifiers, state snapshots, expiry/freshness values, and disabled-action evidence where relevant.

## Failure Conditions

- Do not silently switch to a different child/device when the selected route fails.
- Do not claim relay or remote access from LAN proof.
- Do not let observer/read-only state expose control actions.

## Parallel Ownership Notes

B owns LAN discovery/pairing implementation.
