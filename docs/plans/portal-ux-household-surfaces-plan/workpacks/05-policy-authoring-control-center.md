# 05 Policy Authoring Control Center

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `05 Policy Authoring Control Center`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

Policy preview and control states exist in pieces. Complete nontechnical policy
authoring remains incomplete.

## Where We Want To Be

Parents can scan, create, preview, and understand rules by child, target,
schedule, action, proof level, and last result.

## Decision Tree

| If the assignment touches...        | Read next                                       | Required proof                              |
| ----------------------------------- | ----------------------------------------------- | ------------------------------------------- |
| Policy source of truth or compiler  | `../../policy-control-plane-plan/AGENTS.md`     | typed policy/version/conflict proof         |
| Enforcement-ready state             | `../../v0-8-enforcement-control-plan/AGENTS.md` | adapter authority and rollback proof        |
| App/browser/network/tracking target | owning domain plan AGENTS                       | target compiler/read-model proof            |
| Parent UI authoring                 | this workpack and exact route/source            | create/preview/confirm UI proof             |
| Assistant-proposed policy           | WP11 assistant action preview flow              | typed preview and parent confirmation proof |

## Required Policy States

- Draft intent: not saved, not delivered, no enforcement claim.
- Preview: typed dry-run result with target, schedule, child/device scope, conflict, and proof tier.
- Pending approval: parent confirmation required or co-parent approval required.
- Delivered: child/service acknowledged receipt, not necessarily enforced.
- Active: policy is within schedule and domain adapter has authority.
- Blocked/manual-required: platform permission, adapter authority, account role, conflict, or stale route prevents action.
- Rollback/recovery: previous state and audit ref are visible.

## Requirement Checklist

- [ ] Use typed intents for rule changes.
- [ ] Show dry-run/observe/enforcement-eligible states.
- [ ] Show conflict and unavailable reasons.
- [ ] Keep policy evaluation out of the portal.
- [ ] Test create/preview UI paths where backed by service state.
- [ ] Require parent confirmation before writes.
- [ ] Show delivery/ack/audit status separately from active enforcement.
- [ ] Prove authZ matrix for observer, co-parent, and controller roles.

## Acceptance And Proof

UI actions produce typed request/preview state and render service response.

Expected proof names:

- `portal.policy.draft-preview-confirm`
- `portal.policy.conflict-unavailable`
- `portal.policy.delivery-ack-audit`
- `portal.policy.authz-role-matrix`
- `portal.policy.no-evaluation-in-portal-negative`
- `portal.policy.rollback-visibility`

Proof must include screenshots/DOM snapshots, typed intent/preview fixture or live response, denied-role case, and audit/proof refs.

## Failure Conditions

- Do not let portal UI compile, evaluate, or enforce policy by itself.
- Do not equate delivered policy with active enforcement.
- Do not allow AI/assistant output to write policy without typed preview and parent confirmation.

## Parallel Ownership Notes

A owns enforcement action truth. C owns the authoring workflow and visual model.
