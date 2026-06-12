# Screen AI Pipeline Plan Health Report

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Plan Health Report`
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
- Implementation checklist present: true
- Workpacks indexed: 10

## Consistency warnings

- No current snapshot file was found. Treat workpack/checklist indexes as routing state and create/update a snapshot when product state changes.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes OCR/VLM pipeline, screen-to-AI evidence handoff, model result validation, and safety evaluation work here.
- Scope split: AI pipeline processing, output contracts, redaction, result validation, prompt safety, and proof datasets stay here. Raw capture mechanics stay in screen-plan; shared AI provider/runtime stays in ai-plan when named.
- Minimum read set: assigned pipeline workpack, exact checklist row, `TEST_PROOF_EXPECTATIONS.md`, and only the AI/screen bridge docs named by that workpack.
- Test/proof decision: require OCR/VLM output invariants, prompt-injection, hallucination regression, temperature sensitivity, redaction/custody, safety-boundary, schema rejection, and degraded model proof where touched.
- DONE blocker: no screen-AI claim may move unless proof shows validated outputs, no direct policy authority, no unredacted leak, and deterministic handoff to the owning policy boundary.

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `screen-ai-pipeline-plan`.
- Ownership path: this plan is coordinated via `screen-ai-pipeline-plan/AGENTS.md`, `screen-ai-pipeline-plan/PLAN_STATE.md`, and `screen-ai-pipeline-plan/NEXT_ACTIONS.md` plus selected workpack files.

### State

- Current state: route and schema hygiene are present, but implementation/closure proof remains incomplete until checklist and workpack evidence are updated.
- Current action: keep this file and `screen-ai-pipeline-plan/PLAN_STATE.md` aligned before any DONE/PR_READY claim.

### Decision routes and failure controls

- Decision route: follow this plan�s AGENTS landing decision, the selected workpack path, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, proof artifacts named in this file, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and the assigned plan workpacks.
