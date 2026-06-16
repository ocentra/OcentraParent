# Execution Blueprint

Status: active proof reconciliation.

## Execution objective

Keep the code, test, proof, and rollout docs synchronized for `eventing-plan`.

## Current proof slices

| Slice | Proof doc | Primary artifacts | Result |
| --- | --- | --- | --- |
| 01 | [`docs/proof/eventing-plan/slice-01-envelope-version.md`](../../proof/eventing-plan/slice-01-envelope-version.md) | `output/eventing-plan-proof/14-24-runtime-lifecycle/proof-summary.json`, `output/eventing-plan-proof/63-type-safety-source-gate/proof-summary.json`, `output/eventing-plan-proof/reusable-eventing-runtime/proof-summary.json` | pass |
| 02 | [`docs/proof/eventing-plan/slice-02-ordering-replay.md`](../../proof/eventing-plan/slice-02-ordering-replay.md) | `output/eventing-plan-proof/36-41-journal-replay/proof-summary.json`, `output/eventing-plan-proof/reusable-eventing-runtime/proof-summary.json` | pass |
| 03 | [`docs/proof/eventing-plan/slice-03-consumer-boundary.md`](../../proof/eventing-plan/slice-03-consumer-boundary.md) | `output/eventing-plan-proof/62-network-proof-links/proof-summary.json`, `output/network-plan-proof/10-service-event-chain-stream/proof-summary.json`, `output/eventing-plan-proof/12-household-mesh-consumer/proof-summary.json` | pass |

## Execution stages

1. Implement or update code.
2. Write or update tests.
3. Compile and validate the touched code.
4. Run the tests.
5. Run the smallest relevant package/crate validation.
6. Collect proof in the designated local artifact path.
7. Record the proof pointer outside the plan folder.
8. Reconcile the workpack, checklist, and rollout docs.

## Proof storage

Proof artifacts live in the designated local artifact path for the workpack or crate, not in this plan folder.

## Notes

- Full-plan proof is still available at `output/eventing-plan-proof/full-eventing-plan/proof-summary.json`.
- Reusable-runtime proof is still available at `output/eventing-plan-proof/reusable-eventing-runtime/proof-summary.json`.
- Rollout reconciliation is aligned across the plan state, workpack index, proof checklist, proof manifest, and PR/DONE report.
