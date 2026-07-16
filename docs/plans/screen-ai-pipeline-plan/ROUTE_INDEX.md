# Screen AI Pipeline Plan Route Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Plan Route Index`
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
screen -> AI -> policy/action integration proof
scenario proof routing
screen evidence to AI context handoff
AI result schema validation and degradation boundary
policy handoff no-direct-authority boundary
policy dry-run and action-boundary proof
journal/read-model/portal integration proof
custody/delete proof for pipeline artifacts
live-operator proof and artifact gate non-claims
performance/cadence/backpressure proof
final rollout proof and PR no-claim boundary
```

## Boundary split

```text
screen-plan owns raw capture mechanics, protected surfaces, disclosure, and local screen settings.
screen-domain owns screen evidence/OCR/VLM/disclosure/settings contracts.
ai-plan owns provider/runtime/evidence-context/model behavior when selected.
schema-domain owns canonical shared AI contracts; ai-domain is package identity/focused tests, not canonical contract owner.
policy-control-plane-plan owns policy authority and parent-rule precedence.
v0-8-enforcement-control-plan owns adapter execution and rollback.
data-custody-storage-plan owns retention/export/delete/custody policy.
portal-ux-household-surfaces-plan owns rendered parent UI and screenshot proof.
browser/app-game/network/tracking plans own their trigger/source truth.
agent-protocol/agent-service/agent-core own selected protocol/service/journal seams.
```

## Handoff rule

Open a sibling plan only when the selected workpack names the exact handoff, owner path, expected proof, and no-claim boundary.

## Proof-root rule

The current proof root is scenario-based:

```text
output/screen-ai-pipeline-proof/
```

A slice-close claim also needs a supporting manifest under:

```text
docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md
```

## No-claim rule

Do not claim screen-AI readiness from missing proof roots, source-only proof, mock-only proof, happy-path-only tests, local capture alone, AI result alone, policy decision alone, dry-run as enforcement, live-operator artifact-gate as rerun, or custody proof without deletion/retention evidence.
