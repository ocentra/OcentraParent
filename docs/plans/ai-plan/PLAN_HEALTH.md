# AI Plan Health Report

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan Health Report`
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
- Current snapshot: `current-ai-snapshot.md`
- Implementation checklist present: true
- Workpacks indexed: 48

## Consistency warnings

- Possible status mismatch: implementation checklist is 168/169 checked, but 47/48 workpacks still contain open boxes. Do not claim the plan complete until the assigned workpack/checklist source is reconciled.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes AI/provider/model work here; this plan `AGENTS.md` routes workers through `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and `WORKPACK_INDEX.md` before any detailed doc.
- Scope split: local AI safety contracts, evidence context, provider routing, model/runtime status, memory graph, parent explanations, and household provider mesh stay here. Browser/page evidence, screen capture, network evidence, enforcement, and app/game facts stay in their owning plans unless the assigned workpack names the bridge.
- Minimum read set: one assigned workpack, its exact checklist row, `TEST_PROOF_EXPECTATIONS.md`, and only the source/feature docs named by those files.
- Test/proof decision: require output-invariant, prompt-injection, hallucination/regression, safety-boundary, redaction, provider-failure, replay/idempotency, and no-direct-enforcement proof when those boundaries are touched.
- DONE blocker: no AI claim may move unless proof shows the AI is evidence-only, schema-validated, custody-aware, and unable to bypass deterministic parent policy authority.
