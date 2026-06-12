# Parent Desktop Runtime Package Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `Parent Desktop Runtime Package Plan Next Actions`
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
3. Open that workpack and exact checklist rows only.
4. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by unchecked boxes

- [01 Tauri Shell Contract Boundary](workpacks/01-tauri-shell-contract-boundary.md): 5 open of 5 boxes.
- [02 Local Service Connection Command](workpacks/02-local-service-connection-command.md): 5 open of 5 boxes.
- [03 LAN Route And Controller State](workpacks/03-lan-route-and-controller-state.md): 5 open of 5 boxes.
- [05 Custody And Source Labels](workpacks/05-custody-and-source-labels.md): 5 open of 5 boxes.
- [07 Windows Installer And Preview](workpacks/07-windows-installer-and-preview.md): 5 open of 5 boxes.
- [08 Cross-Platform Package Preview Matrix](workpacks/08-cross-platform-package-preview-matrix.md): 5 open of 6 boxes.
- [13 Desktop Launch Smoke](workpacks/13-desktop-launch-smoke.md): 5 open of 5 boxes.
- [14 Tauri Build And Dev Scripts](workpacks/14-tauri-build-and-dev-scripts.md): 5 open of 5 boxes.
- [20 PR, CI, And Rollout Gate](workpacks/20-pr-ci-rollout-gate.md): 3 open of 5 boxes.
- [19 Product Checklist And Feature Doc Sync](workpacks/19-product-checklist-feature-doc-sync.md): 2 open of 5 boxes.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
