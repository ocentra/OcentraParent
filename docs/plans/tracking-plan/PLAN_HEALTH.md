# Tracking Plan Health Report

> **Current authority (2026-08-15):** this older documentation-health report is
> superseded for code/test status by `PLAN_STATE.md`, `source-index.md`, and
> `CODE_AUDIT.md`. In particular, `packages/tracking-domain` and the named
> `scripts/test/tracking-*.mjs` proof suite are absent from the checkout.

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Health Report`
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
- Current snapshot: `current-tracking-snapshot.md`
- Implementation checklist present: true
- Workpacks on disk: 39
- Older generated index coverage: 33

## Consistency warnings

- `WORKPACK_INDEX.md` previously omitted `WP34-WP39`; treat them as active on-disk workpacks.
- `WP25`, `WP27`, `WP28`, `WP29`, and `WP33` require audit reopen even though their checkbox rows are fully checked.
- Source ownership drift exists in older plan text that still points to `packages/activity-domain` or `packages/parent-domain` instead of the current `packages/tracking-domain`, `schema-domain`, and `crates/tracking-core` ownership surfaces.
- Snapshot and PLAN_STATE can differ on proof-chain freshness; trust the selected workpack proof root plus current blockers, not old checked rows.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, and `TEST_PROOF_EXPECTATIONS.md` if current state changed.
- Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.
- Do not treat omitted on-disk workpacks as out of scope.
- Do not accept tracking-local schema authority for cross-boundary contracts.
- Do not claim completion from hosted screenshots, local fixtures, emulator/simulator artifacts, or proof-file presence alone.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes location/activity tracking, device movement evidence, geofence/session read models, and tracking proof work here.
- Scope split: tracking evidence, location/session semantics, geofence transitions, device/platform adapter status, and parent-visible tracking reports stay here. Enforcement, AI interpretation, and app/browser/network signals stay out unless named as handoffs.
- Minimum read set: this plan `AGENTS.md`, `PLAN_STATE.md`, assigned workpack from `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md` only when owner/proof family is unclear, exact checklist row, `TEST_PROOF_EXPECTATIONS.md`, and proof index only for required artifacts.
- Test/proof decision: require unit, integration, property/invariant, clock skew/DST, migration/rollback, authZ, replay/idempotency, concurrency/race, rate-limit/abuse, location accuracy/degraded-state, logging/metrics/trace, and UI screenshot proof where touched.
- DONE blocker: no tracking row may claim reliable location/session/geofence behavior without negative cases, stale/missing signal handling, platform limitation notes, canonical schema ownership, and proof artifact paths.

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `tracking-plan`.
- Ownership path: this plan is coordinated via `tracking-plan/AGENTS.md`, `tracking-plan/PLAN_STATE.md`, `tracking-plan/NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md`, and selected workpack files.

### State

- Current state: route and schema hygiene are present, but product completion remains blocked by audit-reopened workpacks, physical/runtime/provider/production proof gaps, and event-chain proof gaps.
- Current action: keep this file and `tracking-plan/PLAN_STATE.md` aligned before any DONE/PR_READY claim.

### Decision routes and failure controls

- Decision route: follow this plan's AGENTS landing decision, the selected workpack path, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when schema authority is local-only, handoff routes are missing, checklist/workpack states diverge, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, proof artifacts named in this file, schema-owner notes, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, `PLAN_HEALTH.md`, and the assigned plan workpack.

## PR-ready rule

The whole plan is PR-ready only when audit-reopened workpacks are either closed with retained proof or carry exact blockers, WP34-WP39 event-chain workpacks are represented, schema ownership is centralized, and product-ready claims remain false until hard proof exists.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, negative cases, no-claim language, and remaining open workpacks listed.
