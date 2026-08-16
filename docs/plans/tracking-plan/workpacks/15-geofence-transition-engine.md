# WP15 Geofence Transition Engine

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP15 Geofence Transition Engine`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Evaluate enter, exit, dwell, low-accuracy ambiguity, stale rejection, grace, and
evidence/rule refs.

## Source Inputs

- `docs/expectations/location-geofence.md`
- `docs/plans/tracking-plan/v0-5-location-test-blueprint.md`
- `docs/plans/tracking-plan/workpacks/14-geofence-rule-model.md`

## Target State

Geofence transitions cite location evidence and geofence rule refs, preserve
confidence, reject stale samples, and avoid false alerts near boundaries.

## Tests And Proof

Proof root: `output/tracking-plan-proof/15-geofence-transition-engine/`

- `05-geofence-transition-proof.json`
- `06-expected-place-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Test enter home, exit home, dwell school.
- [ ] Test low accuracy near boundary returns ambiguous.
- [ ] Test stale location does not trigger transition.
- [ ] Test grace period prevents false alert.
- [ ] Cite evidence and rule refs in every transition.

## Where We Are

This workpack has P0 contract proof and P1 fixture proof for deterministic
enter/exit/dwell/ambiguous geofence transition evaluation from
`codex/tracking-plan-full-scope` under the proof root below. Android/iOS
physical geofence delivery, alert policy integration, and UI behavior are not
claimed beyond the proof state recorded in `proof-summary.json`,
`05-geofence-transition-proof.json`, and the implementation checklist.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/15-geofence-transition-engine.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/15-geofence-transition-engine/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Android/iOS physical geofence behavior, UI, and alert integration claims
  remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [ ] Touched files: tracking contract/runtime files, proof scripts, tracking
      plan docs, checklist, and this workpack doc.
- [ ] Validation commands and results:
      `node scripts/test/tracking-plan-runtime-proof.mjs` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/15-geofence-transition-engine/`, including
      `05-geofence-transition-proof.json`.
- [ ] Product doc/checklist updates: tracking plan/checklist updated; central
      product capability checklist reconciliation remains pending while that
      shared doc is actively locked outside this lane.
- [ ] Known gaps/manual-required states: Android/iOS physical geofence proof,
      alert policy integration, provider delivery, notifications, and UI remain
      proof-gated as applicable.
