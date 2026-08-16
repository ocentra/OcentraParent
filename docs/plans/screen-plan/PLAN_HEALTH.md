# Screen Plan Health Report

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan Health Report`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file records documentation health and consistency checks for the plan. It is generated from the existing docs and should be updated manually when the plan state is cleaned further.

## Status sources

- Short README: `README.md`
- Preserved full README: `README_FULL_ORIGINAL.md`
- Current snapshot: `current-screen-snapshot.md`
- Implementation checklist present: true
- Workpacks indexed: 40

## Consistency warnings

- Possible status mismatch: implementation checklist is 100/100 checked, but 22/40 workpacks still contain open boxes. Do not claim the plan complete until the assigned workpack/checklist source is reconciled.
- Current snapshot has substantial retained proof, but each proof row carries an explicit non-claim. Do not strip the non-claims when summarizing.
- Large reference workpacks are not execution assignments unless a route explicitly names them.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, and `TEST_PROOF_EXPECTATIONS.md` if the current state changed.
- Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.
- Do not claim READY from the 100/100 checklist count while workpack rows remain open.
- Do not claim READY from a checked workpack that does not own the selected scope.
- Do not claim READY from mock screenshots, fixture-only proof, or portal screenshots alone.
- Do not claim READY from local capture proof as screen-AI pipeline completion.
- Do not claim READY from OCR/VLM candidate proof as final model quality unless selected proof says so.
- Do not claim READY from screen summary/evidence proof as policy authority.
- Do not claim READY from policy dry-run proof as enforcement adapter execution.
- Do not claim READY from live-view preflight, loopback, relay/cache harness, worker-startup gate, or UI persistence as product live-view readiness.
- Do not claim READY from redacted summary export as raw screenshot remote upload.
- Do not claim READY without deletion/custody, redaction, platform/manual-required, and no-claim proof for the selected slice.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes screen capture, window/surface inventory, screenshot custody, screen evidence storage, and capture proof here.
- Scope split: capture and custody stay here. OCR/VLM inference routes to screen-ai-pipeline-plan or ai-plan only when the assigned row names that processing boundary.
- Minimum read set: assigned screen workpack, exact checklist row, `WORKPACK_FAMILIES.md` only when owner/proof family is unclear, `TEST_PROOF_EXPECTATIONS.md`, platform/source docs named by the row, and proof index for screenshots/logs.
- Test/proof decision: require permission/manual-required, redaction, custody, no-content-claim, timing/clock-boundary, privacy, platform adapter, storage deletion, and screenshot/log proof where touched.
- DONE blocker: no screen row may claim content understanding, AI safety, policy authority, enforcement execution, product live-view readiness, raw remote upload, or remote-access readiness without separate proof from the owning plan.

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `screen-plan`.
- Ownership path: this plan is coordinated via `screen-plan/AGENTS.md`, `screen-plan/PLAN_STATE.md`, `screen-plan/NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md`, and selected workpack files.

### State

- Current state: route and schema hygiene are present, retained screen proof exists for many slices, but plan completion remains blocked by open workpack rows and explicit non-claims.
- Current action: keep this file and `screen-plan/PLAN_STATE.md` aligned before any DONE/PR_READY claim.

### Decision routes and failure controls

- Decision route: follow this plan's AGENTS landing decision, the selected workpack path, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, proof artifacts are absent, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, proof artifacts named in this file, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, `PLAN_HEALTH.md`, and the assigned plan workpack.

## PR-ready rule

The whole plan is PR-ready only when open workpacks either close with retained proof or carry exact blockers, and the closure report preserves platform, model-quality, live-view, raw-retention, remote-boundary, privacy/legal, policy, enforcement, and screen-AI pipeline non-claims.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, negative cases, no-claim language, and remaining open workpacks listed.
