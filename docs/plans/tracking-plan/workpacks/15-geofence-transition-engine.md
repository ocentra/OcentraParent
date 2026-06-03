# WP15 Geofence Transition Engine

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

This workpack has focused contract proof from `codex/tracking-plan-full-scope` under the proof root below. Runtime, platform, provider, and UI behavior is not claimed beyond the proof state recorded in `proof-summary.json` and the implementation checklist.

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

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [x] Proof artifacts under `output/tracking-plan-proof/15-geofence-transition-engine/`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
