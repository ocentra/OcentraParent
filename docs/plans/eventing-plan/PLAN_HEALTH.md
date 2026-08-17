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
- `docs/proof/eventing-plan/` contains the hand-authored WP06 durable manifest;
  the WP12 generated route bundle is absent.

## Consistency warnings

- Previous generated index said there was no workpack route. That was stale because the detailed workpack plan lived in `05-implementation-workpacks.md`.
- Checklist counts still come from the large implementation checklist. Before DONE/PR_READY, verify the assigned route workpack and exact checklist rows match the current proof.
- Historical proof and rollout rows currently overclaim checkout state; cited `output/eventing-plan-proof/*` roots outside WP06/WP11/WP12/WP13 are still incomplete for open workpacks. WP06 is locally evidenced as the generic enforcement prerequisite; WP10 remains open.
- WP11 is implementation-ready but open: the unconstrained `EventEnvelope<E>`
  boundary, missing aggregate/idempotency revalidation, negative tests, and
  retained proof remain unresolved.
- WP12 lacks its declared harness and canonical root. WP13's moved test layout
  is code-complete, but current validation/proof remains open and must include
  the `contract` harness.
- Do not treat focused tests or stale checked rows as plan closure; WP06 is the
  only closed selectable workpack, while WP09-WP13 remain open.
- The route/proof split must stay explicit: `output/eventing-plan-proof/<workpack>/` for raw/generated implementation output; `docs/proof/eventing-plan/` for the hand-authored WP06 durable manifest. WP12's generated route bundle is absent.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.
- Do not claim READY from crate tests alone.
- Do not claim READY from event-domain metadata alone.
- Do not claim READY from protocol shape proof as service delivery.
- Do not claim READY from local bus proof as cross-device transport.
- Do not claim READY from NDJSON journal proof as production durability, retention, deletion, or export proof.
- Do not claim READY from consumer route docs as LAN/remote delivery proof.
- Do not claim READY from WP12/WP13 while WP10 remains open.
- Do not claim feature completeness until the relevant E2E tier in `TEST_PROOF_EXPECTATIONS.md` is explicitly proven or blocked.

## Current rollout note

- WP11 source-boundary implementation is present but proof is open. WP12 route
  reconciliation is blocked by its missing harness/root, and WP13 requires
  current revalidation/proof.
- WP06 retains a hand-authored durable manifest for generic journal/topology/
  replay mechanics and the typed enforcement WP11 handoff under
  `docs/proof/eventing-plan/`; raw/generated output remains ignored. WP10
  remains open until household mesh import/export proof and owning LAN/remote-access handoff verification exist.

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

- Current state: route and schema hygiene are improved, WP06 locally evidences
  its generic handoff, WP09 remains integration-open, WP10 is blocked on LAN
  WP26, WP11 is implementation-ready/open, and WP12/WP13 require validation and
  retained proof.
- Current action: keep this file and `eventing-plan/PLAN_STATE.md` aligned while WP10 is handled; do not promote the local Eventing handoff into enforcement action proof.

### Decision routes and failure controls

- Decision route: follow this plan's AGENTS landing decision, the selected workpack path, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, proof artifacts named in this file, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and the assigned plan workpacks.
