<!-- agent-capsule -->

> Agent Capsule
> Plan: `remote-access-plan`
> Doc: AGENTS
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after a global route selects it.
> Stop rule: Choose one workpack; open adjacent plans only when named by the workpack.
> Proves: local routing and ownership only.

<!-- /agent-capsule -->

# Remote Access Plan Agent Route

Task: define safe parent-owned remote live access beyond local LAN.
Context: remote access is not screen capture and not LAN pairing. It is a high-risk capability fabric using identity, device trust, transport, initial pairing, standing access, relay, authorization, observability, and revocation.
Scope: remote live screen/view, relay sessions, capability grants, initial pairing, standing access until revoke or device removal, abuse limits, and proof.
Current pass: live view first. Pairing creates standing access until revoke or device removal; repeated permission prompts are not part of the current model.
Out of scope: raw capture adapter details, local LAN discovery, account login implementation, package signing, general portal styling, and remote input/control in the current pass.

## High-Density Execution Contract

- Route first from `PLAN_STATE.md`; this plan owns remote live-access authority and must not merge with local LAN or local capture implementation claims.
- Select exactly one workpack and matching proof/checklist rows per session; avoid opening all workpacks by default.
- Every accepted claim must include pairing model, standing-access lifecycle evidence, revocation/remove-device evidence, relay fallback behavior, and abuse/risk mitigation proof.
- Stop condition: no DONE/PR_READY claims for live view capabilities without explicit pairing scope, standing-access visibility, and negative-path validation. Remote input/control claims are deferred.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.

## Ownership, Import, And Boundary Contract

This plan owns remote live-view capability authority, standing grant semantics, remote relay session semantics, pairing/grant lifecycle, relay abuse controls, and proof routing. It does not own local screen capture primitives, local LAN transport, account/session authority, trusted-device bootstrap, data retention/export/delete, portal rendering, remote input/control in the current pass, or enforcement execution.

Module roles:

```text
remote-access-plan: remote capability grants, standing access, relay session semantics, pairing/revocation/remove-device lifecycle, abuse controls, and rollout proof.
screen-plan and screen-domain: capture primitives, protected-surface behavior, screenshot custody, local screen retention settings, and screen-specific disclosure.
lan-plan and lan-domain: local pairing, LAN transport, local peer discovery, and LAN-only state.
account-identity-family-plan: account, household, role, session, parent actor, and selected-device authority.
device-trust-bootstrap-plan: parent presence proof, trusted-device bootstrap, and step-up gating for remote grants.
data-custody-storage-plan: retention, export, deletion, privacy, and custody for remote artifacts or diagnostics.
portal-ux-household-surfaces-plan: rendered remote state, child/parent visible status, and UI proof once remote read models exist.
eventing-plan: reusable event idempotency, replay, journal, request/response, and audit linkage mechanics.
agent-protocol-domain, agent-protocol, and agent-service: protocol/service seams only when selected by a workpack.
```

Direct imports are allowed only for explicit public helper surfaces:

```text
canonical schema/protocol contracts for remote capability, session, relay, or grant shapes
screen-domain public capture/disclosure/retention contracts when selected
agent-protocol-domain public read models or commands when selected
account/device-trust/data-custody/portal public handoff contracts only when the selected workpack names them
neutral event/evidence/logging helpers that do not own remote product behavior
```

Forbidden direct imports and claims:

```text
screen capture internals imported to claim remote access readiness
LAN pairing or transport proof upgraded into relay-backed remote access
local screenshot proof upgraded into live remote view proof
relay route existence upgraded into remote product readiness
standing access without revoke/remove-device proof
live-view proof upgraded into remote control/input proof
support/admin relay path hidden behind parent access
relay diagnostics retaining raw screen/input/child-private payloads by default
UI-only proof upgraded into remote product proof
remote access route bypassing account/device-trust authority
```

If remote access needs capture, LAN, identity, device trust, custody, portal rendering, event replay, support/admin behavior, or future control behavior, it must use typed handoffs, proof roots, and explicit no-claim boundaries. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

## Research Gate

This plan is execution-grade. Before DONE or PR_READY, the assigned agent must inspect existing remote architecture docs, screen-plan code/docs, LAN code/docs, portal-domain route state, and any local service remote capability code. Keep any control-oriented follow-up separate and deferred; do not fold it into the current live-view pass.

## Decision Tree

| If the task is about...                         | Open                                             |
| ----------------------------------------------- | ------------------------------------------------ |
| Capability fabric and route/session model       | `workpacks/01-remote-capability-fabric.md`       |
| Remote screen/live view relay                   | `workpacks/02-live-screen-relay.md`              |
| Remote input/control/desktop authority          | `workpacks/03-remote-input-control-authority.md` |
| Session grants, pairing, disclosure, revocation | `workpacks/04-session-pairing-grants.md`         |
| Relay abuse/security/availability               | `workpacks/05-relay-security-abuse-controls.md`  |
| Proof, route sync, rollout gate                 | `workpacks/06-rollout-proof-and-route-gate.md`   |

## Handoffs

- `screen-plan` owns capture primitives, screenshot custody, protected-surface detection, and local screen retention settings.
- `lan-plan` owns local pairing and LAN transport.
- `account-identity-family-plan` owns user, household, role, session, and device authority.
- `device-trust-bootstrap-plan` owns parent presence proof, trusted-device bootstrap, and step-up gating for remote grants.
- `data-custody-storage-plan` owns retention/export/delete for remote artifacts.
- `portal-ux-household-surfaces-plan` owns rendered parent UI once remote state exists.
- Workpack 03 is retained for future control expansion and is not part of the current live-view pass.

## Failure Conditions

- Do not claim remote access from local screen proof alone.
- Do not claim remote control in the current pass.
- Do not treat relay availability as permission to retain raw screen data.
- Do not claim standing access without revoke/remove-device proof.
- Do not claim remote readiness without account and device-trust handoff proof.
