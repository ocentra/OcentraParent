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

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes portal/household UX, parent dashboard, child surfaces, explanation views, and evidence/report presentation here.
- Scope split: UX state, read-model presentation, accessibility, empty/error/degraded states, and proof screenshots stay here. Capture, policy, enforcement, AI, and transport logic stay in their owning plans unless the UI workpack names the contract.
- Minimum read set: assigned UX workpack, exact checklist row, `TEST_PROOF_EXPECTATIONS.md`, owning feature doc, and the specific read-model contract used by the view.
- Test/proof decision: require Playwright/e2e, accessibility, authZ-visible-state, stale data, error/degraded/empty state, logging/tracing, screenshot, and no-direct-source-read proof where touched.
- DONE blocker: no portal row may claim product status unless the UI consumes the approved read model and proof shows the user-facing state under normal and degraded conditions.
