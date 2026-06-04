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
- Local emulator proof command:
  `npm run test:tracking-plan-android-emulator-proof`
- Pre-device plan:
  `output/tracking-plan-proof/pre-device-gap-closure/android-studio-local-proof-plan.json`

## AI Worker Checklist

- [x] Capture emulator battery percentage and charging state where available.
- [ ] Capture low-power/battery-saver degraded state.
- [x] Capture emulator connectivity dump and active-network summary.
- [ ] Capture app killed/restarted behavior.
- [ ] Keep pending upload count auditable.
- [x] Capture Android package launch, foreground-service scaffold, UI tree,
      screenshot, and logcat for local status proof.
- [x] Generate the Android Studio/local and physical-device proof artifact plan
      before device work.

## Where We Are

This workpack now has pre-device proof routing from
`node scripts/test/tracking-plan-pre-device-proof.mjs`. The generated Android
Studio/local and physical-device plans list the device metadata, battery/status,
offline/pending-upload, killed/reboot, screenshot, and logcat artifacts needed
before any Android status claim. Runtime/product-complete behavior is still not
claimed.

`npm run test:tracking-plan-android-emulator-proof` now fills the first local
status layer. The generated `04-device-status-proof.json` captures emulator
device metadata, foreground service state, battery dump, connectivity summary,
UI tree, screenshot path, and logcat findings. This is emulator scaffold proof
only; offline, killed/restarted, pending-upload, physical-device, and
production tracking freshness claims remain pending.

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
