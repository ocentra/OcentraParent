# WP10 Android Battery Connectivity And Status Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP10 Android Battery Connectivity And Status Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
- `17-status-gap-proof.json`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`
- Focused WP10 proof command:
  `node scripts/test/tracking-android-status-proof.mjs`
- Local emulator proof command:
  `npm run test:tracking-plan-android-emulator-proof`
- Pre-device plan:
  `output/tracking-plan-proof/pre-device-gap-closure/android-studio-local-proof-plan.json`

## AI Worker Checklist

- [ ] Capture emulator battery percentage and charging state where available.
- [ ] Capture low-power/battery-saver degraded state. Proof:
      `output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/17-status-gap-proof.json`.
- [ ] Capture emulator connectivity dump and active-network summary.
- [ ] Capture app killed/restarted behavior. Proof:
      `output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/17-status-gap-proof.json`.
- [ ] Keep pending upload count auditable. Proof:
      `test-results/tracking-android-status-proof/proof.json`.
- [ ] Capture Android package launch, foreground-service scaffold, UI tree,
      screenshot, and logcat for local status proof.
- [ ] Generate the Android Studio/local and physical-device proof artifact plan
      before device work.
- [ ] Link Samsung S9 physical battery/connectivity/status artifacts into the
      WP10 parent-domain status read model. Proof:
      `test-results/tracking-android-status-proof/proof.json`.

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
UI tree, screenshot path, logcat findings, and a status-gap bridge to the local
low-power, killed/restarted, pending-upload, and manual-required rows from
`17-status-gap-proof.json`. This is emulator/local proof only; foreground
location, background/geofence transitions, notification delivery,
physical-device, offline radio behavior, and production tracking freshness
claims remain pending.

`node scripts/test/tracking-android-status-proof.mjs` now adds a focused WP10
parent-domain proof for the remaining local status gap states. It records
low-power/battery-saver degradation, app killed/restarted auditability,
pending-upload count auditability, Samsung S9 physical battery/connectivity
status evidence, and a manual-required platform-proof row under
`17-status-gap-proof.json` and
`test-results/tracking-android-status-proof/proof.json`. This proof is
parser/read-model evidence only; it does not claim foreground location samples,
background location runtime, geofence transitions, offline radio behavior,
notification delivery, device-owner authority, physical-device behavior,
production upload workers, or product-ready Android tracking.

`node scripts/test/tracking-android-physical-device-runtime-proof.mjs` now
records Samsung S9 physical-device status artifacts for the Android runtime
proof: foreground-service `ServiceRecord` with `isForeground=true`, device
metadata, battery/connectivity dumps, UI/keyguard screenshot, logcat, and 3,194
foreground-service-backed background GPS samples. These artifacts support WP10
status evidence and WP33 physical-device review accounting. The WP10 row is now
closed only for physical battery/connectivity/status evidence; it still does not
prove offline radio behavior, Android system geofence delivery, authority,
production upload workers, or product-ready Android tracking.

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

- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: Android status workpack doc, tracking implementation
      checklist, owning tracking feature doc, WP10 generated proof artifacts,
      and focused Android status proof results.
- [ ] Validation commands and results:
      `node scripts/test/tracking-android-status-proof.mjs` passed locally.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/`,
      including `04-device-status-proof.json`, `17-status-gap-proof.json`,
      `15-manual-platform-proof.md`, and `16-validation-commands.log`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack doc updated for the local Android
      low-power, killed/restarted, pending-upload, Samsung S9 physical status,
      and manual-required status proof; central capability row update remains a
      hub/primary-owned doc delta.
- [ ] Known gaps/manual-required states: foreground-service-backed physical
      background GPS samples are observed on the Samsung S9, but geofence
      transitions, Android system delivery, offline radio behavior, notification
      delivery, device-owner authority, production upload workers, and
      product-ready Android tracking remain unclaimed.
