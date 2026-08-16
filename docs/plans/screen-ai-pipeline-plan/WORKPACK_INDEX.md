# Screen AI Pipeline Plan Workpack Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Plan Workpack Index`
> Kind: workpack selector; use before opening any workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Use this index to open exactly one assigned workpack. Do not read every file in `workpacks/`.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not use it as permission to scan multiple workpacks.

Audit recount on 2026-06-16 found all ten workpacks open. Do not trust older `checked` labels elsewhere in this folder without retained proof.

| Status | Workpack                                                                                                    |  Size | Boxes                 |
| ------ | ----------------------------------------------------------------------------------------------------------- | ----: | --------------------- |
| open   | [01 - Prerequisite Merge And Branch Gate](workpacks/01-prerequisite-merge-and-branch-gate.md)               |   729 | 0/5 checked; 5 open   |
| open   | [02 - Real Trigger To Capture Gate](workpacks/02-real-trigger-to-capture-gate.md)                           | 2,270 | 0/9 checked; 9 open   |
| open   | [03 - Capture To AI Analysis Gate](workpacks/03-capture-to-ai-analysis-gate.md)                             | 1,520 | 0/6 checked; 6 open   |
| open   | [04 - AI Result To Policy Gate](workpacks/04-ai-result-to-policy-gate.md)                                   |   573 | 0/5 checked; 5 open   |
| open   | [05 - Policy Action Dry-Run Gate](workpacks/05-policy-action-dry-run-gate.md)                               |   835 | 0/7 checked; 7 open   |
| open   | [06 - Journal Read Model And Portal Gate](workpacks/06-journal-read-model-and-portal-gate.md)               |   763 | 0/6 checked; 6 open   |
| open   | [07 - Deletion Retention And Custody Gate](workpacks/07-deletion-retention-and-custody-gate.md)             | 1,413 | 0/7 checked; 7 open   |
| open   | [08 - Live Operator Proof Gate](workpacks/08-live-operator-proof-gate.md)                                   | 5,543 | 0/11 checked; 11 open |
| open   | [09 - Performance Cadence And Backpressure Gate](workpacks/09-performance-cadence-and-backpressure-gate.md) | 1,564 | 0/6 checked; 6 open   |
| open   | [10 - Final Rollout And PR Gate](workpacks/10-final-rollout-and-pr-gate.md)                                 | 1,470 | 0/8 checked; 8 open   |

## Selection rules

- Choose exactly one workpack.
- If owner/proof family is unclear, classify through `WORKPACK_FAMILIES.md`; do not scan every family.
- Do not use source-only proof as product proof.
- Do not use mock-only or placeholder proof to check a row.
- Do not use local capture proof as AI analysis proof.
- Do not use AI result proof as policy authority or enforcement proof.
- Do not use policy dry-run proof as adapter execution proof.
- Do not use live-operator artifact-gate proof as a live capture rerun.
- Do not claim PR_READY while `output/screen-ai-pipeline-proof/` or `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` is missing.
