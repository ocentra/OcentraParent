# Browser Plan Route Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Plan Route Index`
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
| Need checklist status        | `CHECKLIST_INDEX.md`; open `implementation-checklist.md` only at named row/section |
| Need proof validation        | `PROOF_INDEX.md` and exact proof file                                              |
| Need source ownership        | `DOC_INDEX.md` then `source-index.md` if necessary                                 |
| Need original full narrative | `README_FULL_ORIGINAL.md` only after current state/indexes are insufficient        |
