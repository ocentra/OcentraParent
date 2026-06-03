# WP10 Android Battery Connectivity And Status Adapter

## Purpose

Capture Android battery, charging, low power, radio/connectivity, app killed,
offline, and pending-upload states that affect tracking.

## Source Inputs

- `docs/tracking-control-settings-inventory.md`
- `docs/plans/tracking-plan/v0-5-location-platform-deep-dive.md`
- `docs/expectations/platforms.md`

## Target State

Android device status can explain why location is fresh, delayed, throttled,
offline, or pending upload.

## Tests And Proof

Proof root: `output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/`

- `04-device-status-proof.json`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Capture battery percentage and charging state where available.
- [ ] Capture low-power/battery-saver degraded state.
- [ ] Capture offline/network unreachable state.
- [ ] Capture app killed/restarted behavior.
- [ ] Keep pending upload count auditable.

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

- docs/plans/tracking-plan/workpacks/10-android-battery-connectivity-and-status-adapter.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
