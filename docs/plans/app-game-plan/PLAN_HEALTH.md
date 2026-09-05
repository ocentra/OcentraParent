# App + Game Plan Health Report

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App + Game Plan Health Report`
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
- Current snapshot: `current-app-game-snapshot.md`
- Implementation checklist present: true
- Workpacks indexed: 220
- Reviewed workpack code/test maps: 220/220
- Phase 1 source/test status: 172 code+test complete, 20 reviewed no-code,
  28 with concrete writing gaps

## Consistency warnings

- No high-level checklist/workpack contradiction detected by the generated health check. Still verify the assigned workpack and checklist rows before DONE/PR_READY.
- Current execution source is the assigned workpack plus proof root. Historical checked rows and generated long-name handoff rows do not override the selected workpack scope.
- Central app/game contracts belong in `schema-domain` when the shape crosses package, crate, app, or plan boundaries. Do not repair stale app/game work by recreating shared contracts in sibling feature domains.
- Direct runtime coupling from app/game to AI, policy, enforcement, notification, portal, child-runtime, LAN, remote, setup, payment, or data-custody owners is unhealthy unless the selected workpack records a temporary compatibility reason and a handoff replacement route.
- Use `WORKPACK_FAMILIES.md` to classify the selected workpack owner path when the `WORKPACK_INDEX.md` row is generated, long, or ambiguous.
- Historical workpack source lists still name removed `activity-domain`,
  `parent-domain`, `agent-protocol-domain`, `text-domain`, and missing
  `scripts/test/app-game-*` runners. `CODE_AUDIT.md` plus the executable graph
  is the current replacement ownership map.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.
- Do not claim PR_READY from checked generated handoff rows alone.
- Do not claim PR_READY from staged journal/read-model proof alone.
- Do not claim PR_READY from portal rows without source, service, protocol, or runtime proof when the claim needs those layers.
- Do not claim PR_READY from policy dry-run without adapter proof or explicit manual-required proof.
- Do not claim platform parity from platform preflight alone.
- Do not claim feature completeness until the relevant E2E tier in `TEST_PROOF_EXPECTATIONS.md` is explicitly proven or blocked.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes native app/game evidence work here; browser games/cloud gaming route to browser-plan, and app-only legacy reconciliation routes only when the assigned row says so.
- Scope split: installed app/game identity, inventory, runtime, foreground, launcher, session, app/game policy targets, platform authority, and parent/child UX stay here. URL/page/screen/network content stays out unless referenced as evidence handoff.
- Minimum read set: one assigned workpack, its exact checklist row, `WORKPACK_FAMILIES.md` only when owner path is unclear, `source-index.md` only when source ownership is unclear, and `TEST_PROOF_EXPECTATIONS.md` before DONE.
- Test/proof decision: require contract, invariant, foreground-not-content, runtime-not-foreground, launcher ambiguity, authZ, replay/idempotency, platform-capability, degraded-state UI, and no-fake-green proof where applicable.
- DONE blocker: no product row may claim app/game control until the proof separates observation, classification, policy decision, adapter authority, and manual-required platform states.

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `app-game-plan`.
- Ownership path: this plan is coordinated via `app-game-plan/AGENTS.md`, `app-game-plan/PLAN_STATE.md`, and `app-game-plan/NEXT_ACTIONS.md` plus selected workpack files.

### State

- Current state: route and schema hygiene are present, but implementation/closure proof remains incomplete until checklist and workpack evidence are updated.
- Current action: keep this file and `app-game-plan/PLAN_STATE.md` aligned before any DONE/PR_READY claim.

### Decision routes and failure controls

- Decision route: follow this plan's AGENTS landing decision, the selected workpack path, `WORKPACK_FAMILIES.md` when owner path is unclear, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, proof artifacts named in this file, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and the assigned plan workpacks.
