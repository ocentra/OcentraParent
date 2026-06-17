# Reusable Rust Eventing Plan Health Report

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Reusable Rust Eventing Plan Health Report`
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
- Current snapshot: `current-eventing-snapshot.md`
- Implementation checklist present: true
- Workpacks indexed: 13 route workpacks
- `docs/proof/eventing-plan/` now exists as the local WP12 route-proof bundle.

## Consistency warnings

- Previous generated index said there was no workpack route. That was stale because the detailed workpack plan lived in `05-implementation-workpacks.md`.
- Checklist counts still come from the large implementation checklist. Before DONE/PR_READY, verify the assigned route workpack and exact checklist rows match the current proof.
- Historical proof and rollout rows currently overclaim checkout state; cited
  `output/eventing-plan-proof/*` roots outside WP12/WP13 are still incomplete
  for open workpacks.
- WP11 now has focused local proof roots plus passing package-wide
  `@ocentra-parent/agent-protocol-domain` type-check, focused
  policy-control/contract tests, and a passing touched-file architecture gate.
- WP12 now has a focused local route-proof bundle and WP13 has a focused local
  regression proof root.
- Do not treat focused `cargo test` passes or stale checked rows as plan
  closure; WP11/WP12/WP13 are locally proved, but the full plan still remains
  open because WP10 lacks current local proof.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.

## Current rollout note

- WP11 source-boundary hardening is locally proved. WP12 route-proof
  reconciliation is locally restored, and WP13 also has a local proof root and
  focused revalidation.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes reusable Rust event bus/envelope/replay/consumer contract work here; product-specific consumers route to their owning plan after the eventing contract is known.
- Scope split: local bus semantics, envelopes, ordering, idempotency, TTL, retry, dead-letter, journal/replay, and request/response stay here. Network/LAN transport or product UI belongs to consumer plans.
- Minimum read set: exact checklist row or assigned task, `TEST_PROOF_EXPECTATIONS.md`, source-boundary flow, and consumer plan docs only when consumer behavior is part of the assignment.
- Test/proof decision: require serialization/version-skew, malformed-envelope, duplicate/replay, expiry, ordering, cancellation, retry-storm, dead-letter, corruption, and migration/rollback proof where touched.
- DONE blocker: no reusable eventing work may claim cross-device or product behavior without a separate consumer-plan proof path.

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `eventing-plan`.
- Ownership path: this plan is coordinated via `eventing-plan/AGENTS.md`, `eventing-plan/PLAN_STATE.md`, and `eventing-plan/NEXT_ACTIONS.md` plus selected workpack files.

### State

- Current state: route and schema hygiene are improved, WP13 local proof is
  present, WP12 local route proof is restored, and WP11 is locally proved
  through its proof roots plus focused package validation.
- Current action: keep this file and `eventing-plan/PLAN_STATE.md` aligned while
  the remaining WP10 gap is handled.

### Decision routes and failure controls

- Decision route: follow this plan�s AGENTS landing decision, the selected workpack path, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, proof artifacts named in this file, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and the assigned plan workpacks.
