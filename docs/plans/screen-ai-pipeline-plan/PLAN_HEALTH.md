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

This file records documentation health and consistency checks for the plan. Update it manually when the plan state, proof route, or blocker set changes.

## Status sources

- Short README: `README.md`
- Preserved full README: `README_FULL_ORIGINAL.md`
- Current snapshot: `missing`
- Implementation checklist present: true
- Workpacks indexed: 10
- Retained proof root present: false (`output/screen-ai-pipeline-proof/`)
- Plan proof manifest present: false (`docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md`)

## Consistency warnings

- No current snapshot file was found. Treat workpack/checklist indexes as routing state and create/update a snapshot when product state changes.
- Workpack/checklist truth drift was present before this audit repair: `PLAN_STATE.md` and `WORKPACK_INDEX.md` claimed 8 checked workpacks and 134 checked checklist rows, while the current recount shows 10 open workpacks and 134 open checklist rows.
- Proof-root drift was present before this audit repair: `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, and `PLAN_EXECUTION_BLUEPRINT.md` used `output/screen-ai-pipeline-plan-proof/`, while the workpacks, `implementation-checklist.md`, and `pipeline-proof-matrix.md` use `output/screen-ai-pipeline-proof/`.
- Proof-shape drift still remains: many workpacks/checklist rows cite scenario-local `proof-summary.json`, while `pipeline-proof-matrix.md` defines a richer numbered scenario bundle.
- No retained proof currently exists under `output/screen-ai-pipeline-proof/` in this checkout.
- Scoped architecture validation is currently red on `packages/screen-domain/src/screen-evidence.ts`, `packages/portal-domain/src/contracts.ts`, and `packages/parent-domain/src/local-ai-runtime.ts`.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, and `TEST_PROOF_EXPECTATIONS.md` if current state changes.
- Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
- Retain the proof artifacts the assigned workpack names under `output/screen-ai-pipeline-proof/`.
- Add or update `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` if the slice claims closure.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.
- Do not claim READY while the retained proof root is missing.
- Do not claim READY while `PLAN_PROOF_MANIFEST.md` is missing for a slice-close claim.
- Do not claim READY from mock-only proof, source-only proof, or happy-path-only proof.
- Do not claim READY from local capture without AI/context/result proof.
- Do not claim READY from AI result without schema/policy gate proof.
- Do not claim READY from policy decision without dry-run/action-boundary proof.
- Do not claim READY from dry-run proof as enforcement proof.
- Do not claim READY from live-operator artifact gate as a live rerun.
- Do not claim READY from custody proof without deletion/retention evidence.
- Do not claim feature completeness until the relevant E2E tier in `TEST_PROOF_EXPECTATIONS.md` is explicitly proven or blocked.

## Agent route walkthrough

- Landing decision: root `AGENTS.md` routes OCR/VLM pipeline, screen-to-AI evidence handoff, model result validation, and safety evaluation work here.
- Scope split: AI pipeline processing, output contracts, redaction, result validation, prompt safety, and proof datasets stay here. Raw capture mechanics stay in screen-plan; shared AI provider/runtime stays in ai-plan when named.
- Minimum read set: assigned pipeline workpack, exact checklist row, `WORKPACK_FAMILIES.md` only when owner/proof family is unclear, `TEST_PROOF_EXPECTATIONS.md`, and only the AI/screen bridge docs named by that workpack.
- Test/proof decision: require OCR/VLM output invariants, prompt-injection, hallucination regression, temperature sensitivity, redaction/custody, safety-boundary, schema rejection, and degraded model proof where touched.
- DONE blocker: no screen-AI claim may move unless proof shows validated outputs, no direct policy authority, no unredacted leak, and deterministic handoff to the owning policy boundary.

## Host-scoped proof stance

- Windows proof is expected where the assigned workpack touches Windows-owned runtime or portal behavior.
- Android proof is expected where the assigned workpack includes Android scope; use the emulator and the already-synced Samsung device when needed.
- Linux proof via WSL is expected where the assigned workpack includes Linux scope; missing Docker CLI on PATH is a local execution gap if Docker-backed proof is required.
- Real macOS and iOS proof are external-platform constraints from this Windows host and must not be misreported as local completion.

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `screen-ai-pipeline-plan`.
- Ownership path: this plan is coordinated via `screen-ai-pipeline-plan/AGENTS.md`, `screen-ai-pipeline-plan/PLAN_STATE.md`, `screen-ai-pipeline-plan/NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md`, and selected workpack files.

### State

- Current state: source and test surfaces exist, but retained proof, manifest coverage, and completion bookkeeping remain incomplete.
- Current action: keep this file and `screen-ai-pipeline-plan/PLAN_STATE.md` aligned before any DONE or PR readiness claim.

### Decision routes and failure controls

- Decision route: follow this plan's AGENTS landing decision, the selected workpack path, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when retained proof is missing, proof shape/root expectations diverge, checklist/workpack states diverge, architecture gates are red without blocker notes, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, retained proof artifacts under `output/screen-ai-pipeline-proof/`, a plan proof manifest when slice closure is claimed, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, `PLAN_HEALTH.md`, and the assigned plan workpack.

## PR-ready rule

The whole plan is PR-ready only when all ten workpacks either close with retained proof or carry exact blockers, proof-shape drift is resolved per selected workpack, and WP10 aggregates the final allowed/blocked claims.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, negative cases, no-claim language, and remaining open workpacks listed.
