# Browser Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from open workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Read `workpacks/00-owner-boundary-proof-gate.md` to apply current browser owner/import/proof rules.
4. If the workpack owner path is unclear, classify it with `WORKPACK_FAMILIES.md`.
5. Open that workpack and exact checklist rows only.
6. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by route dependency

1. Use `WORKPACK_INDEX.md` to pick the exact assigned browser control/reference row.
2. If the task names a settings inventory, coverage matrix, schema proposal, questionnaire forest, or managed/unmanaged browser boundary, open only that reference workpack.
3. If the task is implementation or proof, route from the selected reference row into the owning browser implementation workpack/checklist row before making claims.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Apply `workpacks/00-owner-boundary-proof-gate.md` to the selected workpack before source changes.
- [ ] Classify ambiguous/reference workpacks with `WORKPACK_FAMILIES.md` before opening source.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
