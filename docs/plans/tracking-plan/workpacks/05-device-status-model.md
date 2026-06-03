# WP05 Device Status Model

## Purpose

Model heartbeat, battery, charging, connectivity, pending upload, stale/offline,
and degraded states that shape tracking reliability.

## Source Inputs

- `docs/tracking-control-settings-inventory.md`
- `docs/device-location-tracking-capability-guide.md`
- `docs/expectations/location-geofence.md`

## Target State

Device status evidence can explain why tracking is live, stale, throttled,
offline, pending upload, service-disabled, or unavailable.

## Tests And Proof

Proof root: `output/tracking-plan-proof/05-device-status-model/`

- `01-contract-proof.log`
- `04-device-status-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Cover last heartbeat, last location sample, last parent sync, and pending
      upload count.
- [ ] Cover battery percentage, charging, low-power, radio, and connectivity.
- [ ] Keep degraded reasons explicit.
- [ ] Do not let stale/offline become live location.

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

- docs/plans/tracking-plan/workpacks/05-device-status-model.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/05-device-status-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/05-device-status-model/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
