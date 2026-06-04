# WP11 iOS Core Location Foreground Adapter

## Purpose

Model and prove iOS When In Use/current/last-known location behavior without
claiming background behavior.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-platform-deep-dive.md`
- `docs/expectations/platforms.md`
- Apple Core Location docs

## Target State

iOS foreground location evidence is schema-valid, permission-labeled,
freshness-labeled, and degraded when unavailable.

## Tests And Proof

Proof root: `output/tracking-plan-proof/11-ios-core-location-foreground-adapter/`

- `02-platform-permission-proof.md`
- `03-runtime-location-evidence.json`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`
- `18-ios-simulator-proof.json`
- Pre-device plan:
  `output/tracking-plan-proof/pre-device-gap-closure/ios-simulator-local-proof-plan.json`
- Proof command:
  `npm run test:tracking-plan-ios-simulator-proof`

## AI Worker Checklist

- [ ] Prove When In Use authorization UX.
- [ ] Prove current location sample.
- [ ] Prove denied/restricted and services-disabled states.
- [ ] Preserve accuracy/freshness.
- [ ] Do not claim Always/background from this workpack.
- [x] Generate the iOS simulator/local proof artifact plan before device work.
- [x] Route simulator package build/install/launch proof through the tracking
      proof harness and macOS package-preview artifacts without claiming Core
      Location authorization or samples.

## Where We Are

This workpack now has pre-device proof routing from
`node scripts/test/tracking-plan-pre-device-proof.mjs`. The generated iOS
simulator/local plan lists the simulator build, install/launch, authorization,
foreground location-evidence, and screenshot artifacts needed before any iOS
foreground location claim. Runtime/product-complete behavior is still not
claimed, and Always/background behavior stays out of scope here.
`npm run test:tracking-plan-ios-simulator-proof` now writes local proof under
this workpack root. On macOS it can prove the simulator package build and
install/launch path through the existing Xcode/simctl scripts; on non-macOS
hosts it writes `manual_required` output instead of pretending simulator
execution happened. This is package-mechanics proof only, not Core Location
authorization or foreground sample proof.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/11-ios-core-location-foreground-adapter.md
- docs/plans/tracking-plan/implementation-checklist.md
- scripts/test/tracking-plan-ios-simulator-proof.mjs
- `output/tracking-plan-proof/11-ios-core-location-foreground-adapter/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [x] Touched files: proof script, root script wiring, package-preview CI
      upload wiring, feature doc, checklist, and this workpack doc.
- [x] Validation commands and results: `npm run test:tracking-plan-ios-simulator-proof`
      writes local proof; macOS package-preview runs it with
      `--require-simulator` after the real iOS simulator smoke.
- [x] Proof artifacts under `output/tracking-plan-proof/11-ios-core-location-foreground-adapter/`.
- [x] Product doc/checklist updates or reason none were needed: feature doc and
      tracking checklist updated; central product checklist delta remains
      primary-owned through hub.
- [x] Known gaps/manual-required states: Core Location authorization/sample,
      background/region behavior, notification delivery, signing/TestFlight,
      physical-device, and authority proof remain unclaimed.
