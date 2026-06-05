# WP28 Temporary Live Tracking Mode

## Purpose

Support parent-approved temporary live tracking with duration, cadence,
permission, disclosure, battery, expiry, audit, and retention behavior.

## Source Inputs

- `docs/device-location-tracking-capability-guide.md`
- `docs/expectations/location-geofence.md`
- `docs/expectations/data-custody.md`

## Target State

Temporary live tracking is time-boxed, capability-gated, visible to parent,
safe for child disclosure requirements, and auto-expiring.

## Tests And Proof

Proof root: `output/tracking-plan-proof/28-temporary-live-tracking-mode/`

- `03-runtime-location-evidence.json`
- `09-policy-alert-proof.json`
- `14-retention-delete-proof.json`
- `16-validation-commands.log`

## AI Worker Checklist

- [x] Require parent authorization and duration.
- [x] Model cadence, max duration, and auto-stop reason.
- [x] Preserve battery/permission degraded states.
- [ ] Audit start/update/degrade/stop. P1 runtime state now carries parent
      approval/stop audit refs and degradation reason codes; live update
      delivery audit remains pending.
- [ ] Test expiry and deletion/retention behavior. P1 expiry and retention-mode
      state proof exists; platform deletion and live retention settings UI
      remain pending.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope`
under the proof root below. Branch
`codex/tracking-temporary-live-runtime-proof` adds P1 runtime proof through
`npm run test:tracking-plan-temporary-live-runtime-proof`: parent authorization
refs, bounded duration/cadence, expiry, parent stop, low-power degraded state,
retention mode, and `productClaimReady=false` are proved with real
activity-domain contracts and runtime helpers. Platform live sampling, provider
delivery, notification delivery, child-device runtime UI, full product UI, and
physical-device behavior are not claimed.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/28-temporary-live-tracking-mode.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/28-temporary-live-tracking-mode/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [x] Proof artifacts under `output/tracking-plan-proof/28-temporary-live-tracking-mode/`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
- [x] Workpack id and branch:
      `codex/tracking-temporary-live-runtime-proof`.
- [x] Touched files: activity-domain temporary-live runtime contracts/tests,
      proof script, root script wiring, tracking feature doc, implementation
      checklist, this workpack doc, and generated WP28 proof artifacts.
- [x] Validation commands and results:
      `npm run test:tracking-plan-temporary-live-runtime-proof` passed locally.
- [x] Proof artifacts under
      `output/tracking-plan-proof/28-temporary-live-tracking-mode/`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack updated; central capability row delta queued
      through the hub instead of editing `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: Android/iOS live sampling,
      child-device runtime UI, full product UI, provider delivery,
      notification delivery, physical-device proof, authority, and production
      proof remain proof-gated.
