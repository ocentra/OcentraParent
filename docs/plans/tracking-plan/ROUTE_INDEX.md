# Tracking Plan Route Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Route Index`
> Kind: route map for this plan.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Choose the smallest local route for this plan.

| If the task says...          | Read                                                                               |
| ---------------------------- | ---------------------------------------------------------------------------------- |
| Start/resume this plan       | `PLAN_STATE.md` then `NEXT_ACTIONS.md` then `WORKPACK_INDEX.md`                    |
| Assigned a numbered workpack | `WORKPACK_INDEX.md` then that one workpack                                         |
| Owner/proof family unclear   | `WORKPACK_FAMILIES.md` only for selected-workpack classification                   |
| Need checklist status        | `CHECKLIST_INDEX.md`; open `implementation-checklist.md` only at named row/section |
| Need proof validation        | `PROOF_INDEX.md` and exact proof file                                              |
| Need source ownership        | `DOC_INDEX.md` then `source-index.md` if necessary                                 |
| Need original full narrative | `README_FULL_ORIGINAL.md` only after current state/indexes are insufficient        |

## Central schema route

Cross-boundary tracking shapes belong in `schema-domain` or an approved neutral protocol/event/evidence boundary. `tracking-domain` helper schemas are private unless explicitly exported and approved. `tracking-core` Rust types must mirror canonical schemas or protocol/event contracts.

## Handoff route

Event mechanics stay in eventing. Policy authority, notification runtime, custody, AI runtime, portal completion, and platform behavior stay with their owning plans unless the selected workpack names the exact handoff.

## No-claim route

Do not claim broad readiness from proof-file presence, screenshots, local fixtures, or checked boxes. The selected workpack proof root must prove the exact claim or carry the blocker.
