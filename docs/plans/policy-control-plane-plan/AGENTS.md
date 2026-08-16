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
Scope: policy source of truth, parent authoring, templates, schedule/timezone/DST, preview, approval, ask-parent flow, domain compiler contracts, delivery/ack, conflict resolution, event model, and audit.
Out of scope: domain-specific adapter implementation, UI component styling, account login, storage internals, and enforcement execution.

## High-Density Execution Contract

- Route first from `PLAN_STATE.md`; this plan owns policy control-plane truth and must not duplicate domain-owned policy implementation.
- Keep work constrained to the selected workpack and exact proof/checklist rows; update adjacent plans only through explicit handoff entries.
- Every completion claim must include decision model, precedence/conflict resolution proof, delivery acknowledgement, rollback/supersede evidence, event-idempotency evidence, and audit closure evidence.
- Stop condition: do not close this plan as complete until schedule, conflict, override, delivery, event, and rollback paths are proven and testable.
- This plan is execution-grade architecture, not a policy-settings placeholder. Workpack execution remains open until proof says otherwise.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.

## Ownership, Import, And Boundary Contract

This plan owns the policy control plane: source truth, decision/precedence semantics, schedule and time budget semantics, compiler handoff contracts, delivery/ack/audit contracts, ask-parent/override semantics, event model, and proof route. It does not own rendered UI implementation, account/session authority, device-trust step-up, data custody, domain runtime effects, AI output, notification delivery, or enforcement action execution.

Module roles:

```text
crates/schema: canonical shared policy shapes when contracts cross package, app, crate, or plan boundaries.
policy-domain: legacy/package anchor or TypeScript proof-consumer surface unless explicit public exports exist.
policy-control-core: Rust source, authority, compiler, conflict, delivery, event, preview, request, and source helper crate.
agent-protocol and agent-protocol-domain: delivery/read-model/audit/assistant seams only when selected.
portal-domain/apps/portal: rendered authoring, preview, conflict, approval, and audit UI surfaces only when selected.
eventing-plan: reusable local event bus, idempotency, replay, journal, and request/response semantics only.
account-identity-family-plan: actor, role, session, parent authority, and household authority owner.
device-trust-bootstrap-plan: parent presence, step-up, and trusted-device gating owner.
data-custody-storage-plan: policy export/delete/sync/retention custody owner.
v0-8-enforcement-control-plan: enforcement action authority, execution, rollback, and adapter behavior owner.
domain plans: app/game, browser, network, tracking, screen, and AI runtime effects after typed compiler handoff.
```

Direct imports are allowed only for explicit public helper surfaces:

```text
crates/schema canonical policy contracts and shared shapes
policy-control-core public Rust helpers when the selected workpack owns Rust control proof
policy-domain public exports when they actually exist and are selected
agent-protocol/agent-protocol-domain public delivery/read-model/audit/assistant contracts when selected
eventing public event/replay/idempotency contracts when selected
portal/account/device-trust/data-custody/enforcement/domain public handoff contracts only when the selected workpack names them
neutral evidence/logging/protocol helpers that do not own feature behavior
```

Forbidden direct imports and claims:

```text
portal, account, device-trust, data-custody, enforcement, AI, notification, or domain runtime internals imported to bypass typed policy handoffs
UI preview upgraded into applied policy proof
compiler output upgraded into source truth
compiler tests upgraded into domain runtime effect proof
event model proof upgraded into delivery proof
assistant draft upgraded into parent approval
child request upgraded into parent approval
policy delivery upgraded into enforcement authority
single-domain ack upgraded into global active policy
focused contract passes upgraded into full plan completion
policy-domain package identity upgraded into canonical policy source truth
```

If policy work needs rendered UI, account/role/session authority, device step-up, custody, notification, AI drafting, domain runtime effects, or enforcement actions, it must use typed handoffs, proof roots, and explicit no-claim boundaries. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

## Decision Tree

| If the task is about...                               | Open                                           |
| ----------------------------------------------------- | ---------------------------------------------- |
| Source of truth, typed policy model, versions         | `workpacks/01-policy-source-of-truth.md`       |
| Schedule, timezone, DST, time budgets, conflict       | `workpacks/07-schedule-time-budget-conflict-model.md` |
| Parent authoring, templates, preview, nontechnical UX | `workpacks/02-parent-authoring-preview.md`     |
| Domain compilers and handoff contracts                | `workpacks/03-domain-policy-compilers.md`      |
| Event family, idempotency, replay, audit linkage      | `workpacks/08-policy-event-model.md`           |
| Delivery, acknowledgement, conflict, audit            | `workpacks/04-delivery-ack-audit.md`           |
| Ask-parent, overrides, bonus time, approvals          | `workpacks/05-ask-parent-overrides.md`         |
| Proof, route sync, rollout gate                       | `workpacks/06-rollout-proof-and-route-gate.md` |

## Handoffs

- `portal-ux-household-surfaces-plan` owns rendered policy UI.
- `account-identity-family-plan` owns actor/role/session authority.
- `device-trust-bootstrap-plan` owns parent presence proof and trusted-device step-up gating for high-risk policy changes.
- `data-custody-storage-plan` owns policy export/delete/sync custody.
- `v0-8-enforcement-control-plan` owns enforcement action authority and rollback.
- Domain plans own app/game, browser, network, tracking, screen, and AI policy effects.

## State

- Current state: execution-grade architecture docs exist; implementation and proof remain open until workpack evidence is closed in `PLAN_STATE.md`.
- Do not move to DONE/PR_READY until decision precedence, schedule boundaries, delivery acknowledgements, rollback behavior, event-idempotency, and cross-plan proof links are explicit and green in proofs.

## Failure Conditions

- Do not let each domain invent its own parent policy truth.
- Do not claim policy ready without preview, conflict, schedule boundary, delivery ack, event, rollback, and audit proof.
- Do not allow AI or assistant output to write policy without typed action, parent confirmation, and validation.
