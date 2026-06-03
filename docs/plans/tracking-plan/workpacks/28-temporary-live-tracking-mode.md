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

- [ ] Require parent authorization and duration.
- [ ] Model cadence, max duration, and auto-stop reason.
- [ ] Preserve battery/permission degraded states.
- [ ] Audit start/update/degrade/stop.
- [ ] Test expiry and deletion/retention behavior.

## Where We Are

This workpack is planning-only until its implementation branch produces the proof root below. Existing source docs describe the intended capability, but runtime/product-complete behavior is not claimed yet.

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

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/28-temporary-live-tracking-mode/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
