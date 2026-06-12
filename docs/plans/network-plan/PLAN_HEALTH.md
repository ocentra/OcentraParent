# Network Plan Health Report

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan Health Report`
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
- Current snapshot: `current-network-snapshot.md`
- Implementation checklist present: true
- Workpacks indexed: 8 route workpacks

## Consistency warnings

- Previous generated index said there was no executable workpack route. That was stale because the detailed scope lived in `03-network-implementation-checklist-and-workpacks.md` and giant moved control docs. The plan now has 8 focused route workpacks under `workpacks/`.
- Checklist counts still come from the large implementation checklist. Before DONE/PR_READY, verify the assigned route workpack and exact checklist rows match current proof.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes DNS, connection metadata, request boundary, domain reputation, network policy signal, and network read-model work here.
- Scope split: metadata and policy signals stay here. Decrypted payload content, browser URL truth, screen analysis, and enforcement execution stay out unless an assigned handoff names them.
- Minimum read set: one assigned workpack, exact checklist row, `TEST_PROOF_EXPECTATIONS.md`, and only source/security docs named by the row.
- Test/proof decision: require schema fuzzing, CORS/origin/header/host/redirect checks, smuggling/desync/cache-poisoning where request paths are touched, rate-limit/DoS, privacy/redaction, and stale/partial evidence proof.
- DONE blocker: no network claim may imply content inspection or enforcement authority unless the owning plan provides separate proof.
