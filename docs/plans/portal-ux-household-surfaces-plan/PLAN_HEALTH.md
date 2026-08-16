# Portal UX Household Surfaces Plan Health Report

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Plan Health Report`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file records documentation health and consistency checks for the plan. It is generated from the existing docs and should be updated manually when the plan state is cleaned further.

## Status sources

- Short README: `README.md`
- Preserved full README: `README_FULL_ORIGINAL.md`
- Current snapshot: `missing`
- Implementation checklist present: false
- Workpacks indexed: 20

## Consistency warnings

- No current snapshot file was found. Treat workpack/checklist indexes as routing state and create/update a snapshot when product state changes.
- Checked shell/device/LAN rows do not close open policy, assistant, reports, degraded, accessibility, no-fake-data, screenshot, mobile, docs, or manual-review rows.
- `portal-domain` has many public projection contracts, but exports are not rendered UX proof by themselves.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.
- Do not claim READY from visual screenshots alone.
- Do not claim READY from fixture-only data.
- Do not claim READY from portal-local replacement read models.
- Do not claim READY from route existence.
- Do not claim READY from happy-path UI tests only.
- Do not claim READY from service-backed parser tests without rendered UX proof.
- Do not claim READY from domain proof without portal projection proof.
- Do not claim READY from portal projection as domain runtime proof.
- Do not claim READY until required degraded/error/manual-required, no-fake-data, screenshot/console, and no-claim proof exists for the selected slice.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes portal/household UX, parent dashboard, child surfaces, explanation views, and evidence/report presentation here.
- Scope split: UX state, read-model presentation, accessibility, empty/error/degraded states, and proof screenshots stay here. Capture, policy, enforcement, AI, transport, billing, setup, device trust, and custody logic stay in their owning plans unless the UI workpack names the contract.
- Minimum read set: assigned UX workpack, exact checklist row, `TEST_PROOF_EXPECTATIONS.md`, `WORKPACK_FAMILIES.md` only when owner/proof family is unclear, owning feature doc, and the specific read-model contract used by the view.
- Test/proof decision: require Playwright/e2e, accessibility, authZ-visible-state, stale data, error/degraded/empty state, logging/tracing, screenshot, and no-direct-source-read proof where touched.
- DONE blocker: no portal row may claim product status unless the UI consumes the approved read model and proof shows the user-facing state under normal and degraded conditions.

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `portal-ux-household-surfaces-plan`.
- Ownership path: this plan is coordinated via `portal-ux-household-surfaces-plan/AGENTS.md`, `portal-ux-household-surfaces-plan/PLAN_STATE.md`, `portal-ux-household-surfaces-plan/NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md`, plus selected workpack files.

### State

- Current state: route and schema hygiene are present, but implementation/closure proof remains incomplete until checklist and workpack evidence are updated.
- Current action: keep this file and `portal-ux-household-surfaces-plan/PLAN_STATE.md` aligned before any DONE/PR_READY claim.

### Decision routes and failure controls

- Decision route: follow this plan's AGENTS landing decision, the selected workpack path, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, proof artifacts named in this file, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, and the assigned plan workpack.

## PR-ready rule

The whole plan is PR-ready only when all open workpacks either close with proof or carry exact blockers, and the manual review gate records user-visible route/artifact review state.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, negative cases, no-claim language, and remaining open workpacks listed.
