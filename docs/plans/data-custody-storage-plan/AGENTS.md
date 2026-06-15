<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `AGENTS.md`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX or FEATURE_ROUTE_INDEX selects the plan.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Route changes require PLAN_STATE.md, ROUTE_INDEX.md, WORKPACK_INDEX.md, PLAN_INDEX.md, and FEATURE_ROUTE_INDEX.md to stay aligned.

<!-- /agent-capsule -->

# Data Custody Storage Plan Agent Route

Use this plan for data custody guarantees, encrypted storage, evidence retention, export/import/restore, sync, deletion/tombstones, no-stolen-data boundaries, cloud/relay custody, report/query custody, and parent storage settings/apply flow.

## High-Density Execution Contract

Task: work only the assigned slice for this plan.
Context: [PLAN_STATE.md](PLAN_STATE.md) is current state; [NEXT_ACTIONS.md](NEXT_ACTIONS.md) is the resume queue; [WORKPACK_INDEX.md](WORKPACK_INDEX.md) chooses one workpack; [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) defines required proof.
Scope rule: one plan, one workpack, exact proof rows. Adjacent plans are closed until the selected workpack names a handoff.
Execution rule: architecture decisions live in [DECISIONS.md](DECISIONS.md), [DATA_CLASSIFICATION.md](DATA_CLASSIFICATION.md), [KEY_CUSTODY_MODEL.md](KEY_CUSTODY_MODEL.md), [PARENT_STORAGE_PROVIDER_MATRIX.md](PARENT_STORAGE_PROVIDER_MATRIX.md), [BUNDLE_PROTOCOL.md](BUNDLE_PROTOCOL.md), [EVENT_MODEL.md](EVENT_MODEL.md), [UI_EXPECTATIONS.md](UI_EXPECTATIONS.md), [PLATFORM_KEY_CUSTODY_MATRIX.md](PLATFORM_KEY_CUSTODY_MATRIX.md), [PARENT_SAVE_RETRIEVE_APPLY_FLOW.md](PARENT_SAVE_RETRIEVE_APPLY_FLOW.md), and [RESEARCH_AND_UI_GUIDANCE.md](RESEARCH_AND_UI_GUIDANCE.md). Workpacks must reference those decisions instead of re-litigating them.
Implementation rule: docs define outcome, boundary, shape, validation, and proof. They do not prescribe implementation code.
Proof rule: proof must include command/log evidence, negative cases, artifact paths, updated rows, and skipped-risk notes when applicable.
Failure condition: no DONE/PR_READY when expected proof is missing, only happy-path evidence exists, or this plan is used to claim adjacent implementation completion.

## Local Decision Tree

- If the assignment names a workpack, open only that workpack.
- If the assignment names a checklist row but no workpack, use [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md), then choose one workpack from [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
- If the assignment changes product status, read [DOC_INDEX.md](DOC_INDEX.md), [PLAN_STATE.md](PLAN_STATE.md), and [PROOF_INDEX.md](PROOF_INDEX.md) only for named rows.
- If the assignment touches adjacent implementation ownership, open only the adjacent plan named by the selected workpack.
- If the assignment is DONE/PR_READY, read [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md), [PROOF_INDEX.md](PROOF_INDEX.md), [PLAN_HEALTH.md](PLAN_HEALTH.md), then [../../agent/PR_DONE_FLOW.md](../../agent/PR_DONE_FLOW.md).

## Required Read Order

1. [PLAN_STATE.md](PLAN_STATE.md)
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md)
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md)
4. One assigned workpack under workpacks/
5. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md)
6. [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) and [PROOF_INDEX.md](PROOF_INDEX.md) only for named rows/artifacts

## Product Sources

- Architecture docs: DECISIONS.md, DATA_CLASSIFICATION.md, KEY_CUSTODY_MODEL.md, PARENT_STORAGE_PROVIDER_MATRIX.md, BUNDLE_PROTOCOL.md, EVENT_MODEL.md, UI_EXPECTATIONS.md, PLATFORM_KEY_CUSTODY_MATRIX.md, PARENT_SAVE_RETRIEVE_APPLY_FLOW.md, RESEARCH_AND_UI_GUIDANCE.md
- Feature docs: evidence-store-query.md, reports-notifications-sync.md
- Expectation docs: data-custody.md, evidence-storage.md, sync-export.md, cloud.md
- Adjacent plans: eventing-plan, portal-ux-household-surfaces-plan, parent-client-runtime-distribution-plan, account-identity-family-plan, device-trust-bootstrap-plan, payment-subscription-plan
