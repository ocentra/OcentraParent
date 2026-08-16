# Screen Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from open workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
4. Open that workpack and exact checklist rows only.
5. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by unchecked boxes

- [22 Deletion And Retention Proof](workpacks/22-deletion-and-retention-proof.md): 6 open of 6 boxes; reopen after independent review found unsafe retention, non-atomic queue mutation, and fabricated proof boundaries.

- [03 Contract Boundary And Effect Schemas](workpacks/03-contract-boundary-and-effect-schemas.md): 8 open of 8 boxes.
- [05 Capability Status Contract](workpacks/05-capability-status-contract.md): 8 open of 8 boxes.
- [09 Windows Capture Adapter Plan Proof](workpacks/09-windows-capture-adapter-plan-proof.md): 7 open of 7 boxes.
- [18 Screen Analysis Result Schema](workpacks/18-screen-analysis-result-schema.md): 7 open of 7 boxes.
- [20 Result Validator And Invalid Output Handling](workpacks/20-result-validator-and-invalid-output-handling.md): 7 open of 7 boxes.
- [01 Source Index And Doc Reconciliation](workpacks/01-source-index-and-doc-reconciliation.md): 6 open of 6 boxes.
- [02 Current Screen Snapshot And Gap Map](workpacks/02-current-screen-snapshot-and-gap-map.md): 6 open of 6 boxes.
- [06 Capture Scope Model](workpacks/06-capture-scope-model.md): 6 open of 6 boxes.
- [07 Capture Trigger Model](workpacks/07-capture-trigger-model.md): 6 open of 6 boxes.
- [08 Platform Adapter Abstraction](workpacks/08-platform-adapter-abstraction.md): 6 open of 6 boxes.
- [14 Protected Surface Detector](workpacks/14-protected-surface-detector.md): 6 open of 6 boxes.
- [15 Encrypted Temporary Image Queue](workpacks/15-encrypted-temporary-image-queue.md): 6 open of 6 boxes.
- [16 Queue Scheduler And Debouncer](workpacks/16-queue-scheduler-and-debouncer.md): 6 open of 6 boxes.
- [21 Journal And SQLite Ingest](workpacks/21-journal-and-sqlite-ingest.md): 6 open of 6 boxes.
- [23 Policy Compiler For Screen Derived Evidence](workpacks/23-policy-compiler-for-screen-derived-evidence.md): 6 open of 6 boxes.
- [29 Proof Tiers And Proof Packs](workpacks/29-proof-tiers-and-proof-packs.md): 6 open of 6 boxes.
- [12 Android MediaProjection Adapter Plan Proof](workpacks/12-android-mediaprojection-adapter-plan-proof.md): 2 open of 9 boxes.
- [13 iOS ReplayKit Adapter Plan Proof](workpacks/13-ios-replaykit-adapter-plan-proof.md): 2 open of 7 boxes.
- [28 Live View Optional Mode](workpacks/28-live-view-optional-mode.md): 2 open of 14 boxes.

## Current status warnings

- Treat the full checklist's 100/100 checked state as incomplete while `WORKPACK_INDEX.md` still lists open workpacks.
- Use `current-screen-snapshot.md` for retained proof context, but preserve each proof row's explicit non-claim.
- Do not claim product live view from live-view preflight, local loopback, relay/cache harness, or worker-startup gate proof.
- Do not claim screen-AI pipeline, AI quality, policy authority, enforcement execution, remote access, or product custody from screen-local proof.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Classify selected owner/proof family through `WORKPACK_FAMILIES.md` when unclear.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, accepted proof path, explicit no-claims, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
