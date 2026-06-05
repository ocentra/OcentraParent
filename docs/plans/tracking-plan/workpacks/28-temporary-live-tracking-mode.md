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
- [x] Audit lifecycle/degrade/stop refs.
- [x] Test expiry and deletion/retention behavior.

## Where We Are

This workpack now has P1 fixture-simulation proof from
`codex/tracking-temporary-live-readiness-proof` under the proof root below. The
proof derives ready, active time-boxed, expired auto-stop, revoked,
unavailable, and duration-policy manual states from the existing tracking
policy read model. It preserves cadence, max duration, expiry, retention
action, audit refs, and degraded platform/battery proof requirements while
keeping live location runtime, background location, battery runtime, parent and
child UI, remote sync, provider delivery, and physical-device proof unclaimed.

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

- [x] Workpack id and branch:
      `codex/tracking-temporary-live-readiness-proof`.
- [x] Touched files: parent-domain temporary live readiness proof contract,
      focused test, proof script, owning feature doc, implementation checklist,
      this workpack doc, proof output, and queued capability-checklist doc
      delta.
- [x] Validation commands and results:
      `node scripts/test/tracking-temporary-live-readiness-proof.mjs` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/28-temporary-live-tracking-mode/` and
      `test-results/tracking-temporary-live-readiness-proof/`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and capability-checklist delta queued while
      the central checklist remains sequenced through hub locks.
- [x] Known gaps/manual-required states: live location runtime, background
      location, battery runtime, child disclosure UI, parent live UI, remote
      sync, provider delivery, physical-device proof, production retention
      deletion, Android/iOS physical proof, and full runtime execution remain
      proof-gated.
