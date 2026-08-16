# Portal UX Household Surfaces Plan Route Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Plan Route Index`
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

## Owns

```text
portal route composition
parent-facing UX presentation
read-model projection
visible loading/empty/error/degraded/manual-required states
source/custody labels
accessibility/responsive/keyboard behavior
screenshot/browser proof
manual review route pack
no-fake-data contract
```

## Boundary split

```text
apps/portal owns rendered runtime view composition.
portal-domain owns public portal/panel/projection contracts.
schema-domain and domain packages own typed source/read-model contracts.
policy plan owns policy source truth and approval semantics.
AI plan owns assistant runtime and model behavior.
enforcement plan owns action authority and rollback.
setup/account/device-trust/LAN/browser/app-game/network/screen/tracking/payment/data-custody plans own their domain truth.
```

## Handoff rule

Open a sibling plan only when the selected workpack names the exact handoff, owner path, expected proof, and no-claim boundary.

## No-claim rule

Do not claim product readiness from visual screenshots, fixture data, route existence, portal-local replacement models, happy-path UI tests, service parser tests, or checked workpack rows alone. Portal proof shows user-facing projection only; domain runtime truth remains in the owning plan.
