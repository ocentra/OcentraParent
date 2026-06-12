<!-- agent-capsule -->

> Agent Capsule
> Plan: `policy-control-plane-plan`
> Doc: AGENTS
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after a global route selects it.
> Stop rule: Choose one workpack; open domain plans only when a selected workpack names a handoff.
> Proves: local routing and ownership only.

<!-- /agent-capsule -->

# Policy Control Plane Plan Agent Route

Task: define how parent intent becomes safe, typed, delivered, auditable policy.
Context: policy is currently scattered across portal, app/game, browser, network, tracking, AI, and enforcement. This plan prevents ad hoc policy paths and duplicate truth.
Scope: policy source of truth, parent authoring, templates, schedule/timezone/DST, preview, approval, ask-parent flow, domain compiler contracts, delivery/ack, conflict resolution, and audit.
Out of scope: domain-specific adapter implementation, UI component styling, account login, and storage internals.

## Research Gate

This plan is first-pass. Before implementation, DONE, or PR_READY, the assigned agent must inspect existing policy docs, portal policy UI, domain packages, enforcement handoff code/docs, and plan-local policy workpacks, then discuss unresolved source-of-truth and UX choices with Sujan. Do not treat this first-pass plan as final architecture.

## Decision Tree

| If the task is about...                               | Open                                           |
| ----------------------------------------------------- | ---------------------------------------------- |
| Source of truth, typed policy model, versions         | `workpacks/01-policy-source-of-truth.md`       |
| Parent authoring, templates, preview, nontechnical UX | `workpacks/02-parent-authoring-preview.md`     |
| Domain compilers and handoff contracts                | `workpacks/03-domain-policy-compilers.md`      |
| Delivery, acknowledgement, conflict, audit            | `workpacks/04-delivery-ack-audit.md`           |
| Ask-parent, overrides, bonus time, approvals          | `workpacks/05-ask-parent-overrides.md`         |
| Proof, route sync, rollout gate                       | `workpacks/06-rollout-proof-and-route-gate.md` |

## Handoffs

- `portal-ux-household-surfaces-plan` owns rendered policy UI.
- `account-identity-family-plan` owns actor/role/session authority.
- `data-custody-storage-plan` owns policy export/delete/sync custody.
- `v0-8-enforcement-control-plan` owns enforcement action authority and rollback.
- Domain plans own app/game, browser, network, tracking, screen, and AI policy effects.

## Failure Conditions

- Do not let each domain invent its own parent policy truth.
- Do not claim policy ready without preview, conflict, schedule boundary, delivery ack, and audit proof.
- Do not allow AI or assistant output to write policy without typed action, parent confirmation, and validation.
