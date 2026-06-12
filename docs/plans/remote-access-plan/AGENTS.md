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

Task: define safe parent-owned remote access beyond local LAN.
Context: remote access is not screen capture and not LAN pairing. It is a high-risk capability fabric using identity, device trust, transport, consent, relay, authorization, observability, and revocation.
Scope: remote live screen, remote desktop/control, remote input authority, relay sessions, capability grants, consent/disclosure, abuse limits, and proof.
Out of scope: raw capture adapter details, local LAN discovery, account login implementation, package signing, and general portal styling.

## High-Density Execution Contract

- Route first from `PLAN_STATE.md`; this plan owns remote access authority and must not merge with local LAN or local capture implementation claims.
- Select exactly one workpack and matching proof/checklist rows per session; avoid opening all workpacks by default.
- Every accepted claim must include consent model, session lifecycle evidence, revocation evidence, relay fallback behavior, and abuse/risk mitigation proof.
- Stop condition: no DONE/PR_READY claims for remote input/control, or live view capabilities without explicit consent, scope, and negative-path validation.

## Research Gate

This plan is first-pass. Before implementation, DONE, or PR_READY, the assigned agent must inspect existing remote architecture docs, screen-plan code/docs, LAN code/docs, portal-domain route state, and any local service remote capability code, then discuss unresolved safety/product choices with Sujan. Do not treat this first-pass plan as final architecture.

## Decision Tree

| If the task is about...                         | Open                                             |
| ----------------------------------------------- | ------------------------------------------------ |
| Capability fabric and route/session model       | `workpacks/01-remote-capability-fabric.md`       |
| Remote screen/live view relay                   | `workpacks/02-live-screen-relay.md`              |
| Remote input/control/desktop authority          | `workpacks/03-remote-input-control-authority.md` |
| Session grants, consent, disclosure, revocation | `workpacks/04-session-consent-grants.md`         |
| Relay abuse/security/availability               | `workpacks/05-relay-security-abuse-controls.md`  |
| Proof, route sync, rollout gate                 | `workpacks/06-rollout-proof-and-route-gate.md`   |

## Handoffs

- `screen-plan` owns capture primitives, screenshot custody, protected-surface detection, and local screen retention settings.
- `lan-plan` owns local pairing and LAN transport.
- `account-identity-family-plan` owns user, household, role, session, and device authority.
- `data-custody-storage-plan` owns retention/export/delete for remote artifacts.
- `portal-ux-household-surfaces-plan` owns rendered parent UI once remote state exists.

## Failure Conditions

- Do not claim remote access from local screen proof alone.
- Do not claim remote control without explicit parent authority, child disclosure where required, revocation, audit, and abuse limits.
- Do not treat relay availability as permission to retain raw screen data.
