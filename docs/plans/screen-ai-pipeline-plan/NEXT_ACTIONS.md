# Screen AI Pipeline Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from the audited checkbox and proof state and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
4. Open that workpack and exact checklist rows only.
5. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by unchecked boxes

- [08 - Live Operator Proof Gate](workpacks/08-live-operator-proof-gate.md): 11 open of 11 boxes.
- [02 - Real Trigger To Capture Gate](workpacks/02-real-trigger-to-capture-gate.md): 9 open of 9 boxes.
- [10 - Final Rollout And PR Gate](workpacks/10-final-rollout-and-pr-gate.md): 8 open of 8 boxes.
- [05 - Policy Action Dry-Run Gate](workpacks/05-policy-action-dry-run-gate.md): 7 open of 7 boxes.
- [07 - Deletion Retention And Custody Gate](workpacks/07-deletion-retention-and-custody-gate.md): 7 open of 7 boxes.

## Audit-first blockers before any new checked status

- Restore retained proof under `output/screen-ai-pipeline-proof/`; the root does not currently exist in this checkout.
- Add `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` before any slice-level closure claim; it is currently missing.
- Reconcile proof artifact shape for the assigned workpack: current docs mix scenario-local `proof-summary.json` artifacts with the richer numbered bundle defined in `pipeline-proof-matrix.md`.
- Do not ignore the current architecture gate failure on `packages/screen-domain/src/screen-evidence.ts`, `packages/portal-domain/src/contracts.ts`, and `packages/parent-domain/src/local-ai-runtime.ts`.
- Keep source-only, mock-only, happy-path-only, and artifact-gate-only evidence out of DONE/PR_READY claims.

## Ordered proof families

```text
WP01 prerequisite gate -> WP02 trigger-to-capture -> WP03 capture-to-analysis -> WP04 AI-result-to-policy -> WP05 dry-run/action boundary -> WP06 journal/read-model/portal -> WP07 custody/delete -> WP08 live-operator -> WP09 performance/backpressure -> WP10 rollout gate
```

A selected workpack can run independently only when its prerequisites are already proven or explicitly carried as blockers in the proof bundle.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale or unretained.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Classify selected owner/proof family through `WORKPACK_FAMILIES.md` when unclear.
- [ ] Pick one assigned workpack and state the exact scenario directory or directories it owns under `output/screen-ai-pipeline-proof/`.
- [ ] Run real focused validation on Windows, Android, and/or WSL/Linux where the assigned workpack requires it; treat macOS/iOS only as an external-platform constraint from this Windows host.
- [ ] Retain proof artifacts and a command log before updating any workpack or checklist row.
- [ ] Record failure conditions, skipped checks, evidence paths, proof-shape choice, and no-claim boundaries in `PLAN_STATE.md` and `TEST_PROOF_EXPECTATIONS.md` for every claimed progress.
