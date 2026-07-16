# V0.8 Enforcement Control Plan Route Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Plan Route Index`
> Kind: route map for this plan.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Choose the smallest local route for this plan.

| If the task says...                         | Read                                                                                              |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| Start/resume this plan                      | `PLAN_STATE.md` then `NEXT_ACTIONS.md` then `WORKPACK_INDEX.md`                                  |
| Assigned a numbered workpack                | `WORKPACK_INDEX.md` then that one workpack then `TEST_PROOF_EXPECTATIONS.md`                     |
| Need owner or handoff boundary              | `WORKPACK_FAMILIES.md`, then only the named sibling owner if the selected workpack requires it   |
| Need docs/route hardening                   | `PLAN_STATE.md`, `WORKPACK_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, `PROOF_INDEX.md`, then only the named route/workpack docs |
| Need checklist status                       | `CHECKLIST_INDEX.md`; open `implementation-checklist.md` only at the named row/section          |
| Need proof validation or proof cleanup      | `PROOF_INDEX.md` and the exact proof root or proof file                                           |
| Need source ownership after workpack select | `WORKPACK_FAMILIES.md` then `DOC_INDEX.md` and `source-index.md` only if still necessary         |
| Need PR_READY or broad status claim         | `PLAN_HEALTH.md` then `PROOF_INDEX.md` then `../../agent/PR_DONE_FLOW.md`                        |
| Need original full narrative                | `README_FULL_ORIGINAL.md` only after current state/indexes are insufficient                      |

## Stop rules

- Do not read all open workpacks. Open only the assigned workpack.
- Do not read sibling plan docs unless the selected workpack names the handoff.
- Do not use evidence, UI, or AI route docs as permission to claim enforcement
  authority.
