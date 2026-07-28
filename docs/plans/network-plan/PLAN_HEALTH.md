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
- Current proof roots: the closed shim-cleanup receipt remains preserved, and `docs/proof/network-plan/01-foundation-contracts-and-eventing.md` is the active tracked WP01 contract/eventing validation receipt.

## Consistency warnings

- Previous generated index said there was no executable workpack route. That was stale because the detailed scope lived in `03-network-implementation-checklist-and-workpacks.md` and giant moved control docs. The plan now has 8 focused route workpacks under `workpacks/`.
- Checklist counts still come from the large implementation checklist. Before DONE/PR_READY, verify the assigned route workpack and exact checklist rows match current proof.
- `packages/network-domain` is package metadata/proof-consumer surface unless explicit public exports exist; canonical shared network shapes live in `crates/schema` or the owning Rust crate. `packages/schema-domain` is temporary generated-validation or edge-decoder surface only where migration is still incomplete.
- Current active proof is bounded to the reviewed WP01 contract/runtime slice and does not close broader WP01 or WP02-WP08.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.
- Do not claim READY from checklist count.
- Do not claim READY from shim-cleanup skeleton proof.
- Do not claim READY from network-domain metadata/package proof.
- Do not claim READY from schema tests as live capture proof.
- Do not claim READY from fixture or PCAP replay as live capture proof.
- Do not claim READY from network evidence as exact URL, private content, AI runtime, policy decision, or enforcement authority proof.
- Do not claim READY from Windows lab firewall proof as production enforcement.
- Do not claim READY from control catalog or settings inventory existence as implementation.
- Do not claim feature completeness until the relevant E2E tier in `TEST_PROOF_EXPECTATIONS.md` is explicitly proven or blocked.

## Known healthy boundaries

This plan intentionally separates:

```text
network metadata evidence
contract/schema proof
Rust evidence/proof helper behavior
passive capture and replay proof
live capture proof
classification/correlation proof
cascade/parent projection proof
platform action gate proof
AI audit/risk fixture proof
control catalog reference routing
```

Do not collapse those boundaries.

## Known incomplete areas

```text
WP01 foundation contracts/eventing scope beyond the active tracked review receipt
WP02 passive capture/parser proof
WP03 classification/correlation proof
WP04 cascade/parent surface proof
WP05 adapter/platform action proof
WP06 AI audit/risk budget proof
WP07 performance/security/rollout proof
WP08 exact control catalog route proof for selected controls
live platform capture proof where claimed
mobile authority proof where claimed
production rollout proof
```

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes DNS, connection metadata, request boundary, domain reputation, network policy signal, and network read-model work here.
- Scope split: metadata and policy signals stay here. Decrypted payload content, browser URL truth, screen analysis, AI runtime, policy authority, and enforcement execution stay out unless an assigned handoff names them.
- Minimum read set: one assigned workpack, exact checklist row, `TEST_PROOF_EXPECTATIONS.md`, `WORKPACK_FAMILIES.md` only when owner/proof family is unclear, and only source/security docs named by the row.
- Test/proof decision: require schema fuzzing, request-boundary checks, rate-limit/DoS, privacy/redaction, stale/partial evidence proof, platform proof, and no-claim boundaries where touched.
- DONE blocker: no network claim may imply content inspection, exact URL truth, AI runtime, or enforcement authority unless the owning plan provides separate proof.

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `network-plan`.
- Ownership path: this plan is coordinated via `network-plan/AGENTS.md`, `network-plan/PLAN_STATE.md`, and `network-plan/NEXT_ACTIONS.md` plus selected workpack files.

### State

- Current state: route and schema hygiene are present, but implementation/closure proof remains incomplete until checklist and workpack evidence are updated.
- Current action: keep this file and `network-plan/PLAN_STATE.md` aligned before any DONE/PR_READY claim.

### Decision routes and failure controls

- Decision route: follow this plan's AGENTS landing decision, the selected workpack path, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, proof artifacts named in this file, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and the assigned plan workpacks.
