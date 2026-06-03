# WP18 Child Check-In Flow

## Purpose

Provide a safe parent-requested and child-initiated check-in flow with optional
location sample, no-shame copy, audit refs, and escalation handoff.

## Source Inputs

- `docs/expectations/notifications.md`
- `docs/expectations/location-geofence.md`
- `docs/plans/tracking-plan/ui-ux-requirements-guide.md`

## Target State

Child check-ins support safe/help/share-location/call-parent responses. Unclear
or missing check-ins remain rule-based escalation inputs, not accusations.

## Tests And Proof

Proof root: `output/tracking-plan-proof/18-child-check-in-flow/`

- `01-contract-proof.log`
- `09-policy-alert-proof.json`
- `11-ui-snapshots/`
- `12-playwright-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Use calm child-safe copy.
- [ ] Support optional current location sample where permission allows.
- [ ] Audit every prompt and response.
- [ ] Resolve or update alert state from child response.
- [ ] Test unresolved check-in escalation by rule only.

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

- docs/plans/tracking-plan/workpacks/18-child-check-in-flow.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/18-child-check-in-flow/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [x] Proof artifacts under `output/tracking-plan-proof/18-child-check-in-flow/`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
