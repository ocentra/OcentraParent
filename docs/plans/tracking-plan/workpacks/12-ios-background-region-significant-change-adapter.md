# WP12 iOS Background Region Significant-Change Adapter

## Purpose

Proof-gate iOS Always authorization, region monitoring, significant-change,
visits, background modes, low-power, and terminated/relaunch states.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-platform-deep-dive.md`
- `docs/expectations/platforms.md`
- Apple Core Location docs

## Target State

iOS background/region behavior is claimed only with real-device proof and
degraded/manual-required states where behavior is unavailable.

## Tests And Proof

Proof root: `output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/`

- `02-platform-permission-proof.md`
- `05-geofence-transition-proof.json`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`
- `18-ios-simulator-proof.json`
- Pre-device plan:
  `output/tracking-plan-proof/pre-device-gap-closure/ios-simulator-local-proof-plan.json`
- Proof command:
  `npm run test:tracking-plan-ios-simulator-proof`

## AI Worker Checklist

- [ ] Prove Always authorization UX. Parent-domain manual-required row now
      exists; real authorization/entitlement capture remains pending.
- [ ] Prove region enter/exit where claimed. Parent-domain manual-required row
      now exists; real region transition capture remains pending.
- [ ] Prove significant-change and visit events where claimed. Parent-domain
      manual-required row now exists; real event capture remains pending.
- [ ] Prove background/terminated degraded behavior. Parent-domain
      manual-required row now exists; real background delivery and relaunch proof
      remain pending.
- [ ] Document App Store/privacy disclosure implications before release claims.
- [x] Generate the iOS simulator/local and physical-device proof artifact plan
      before device work.
- [x] Route simulator package build/install/launch proof through the tracking
      proof harness while preserving Always/background/region behavior as
      physical-device manual-required.

## Where We Are

This workpack now has pre-device proof routing from
`node scripts/test/tracking-plan-pre-device-proof.mjs`. The generated iOS
simulator/local and physical-device plans list authorization, region definition,
location event, region transition, alert decision, screenshot, Xcode log, and
privacy-disclosure artifacts required before any iOS background/region claim.
Runtime/product-complete behavior, entitlement approval, and real-device
background behavior are still not claimed.
`npm run test:tracking-plan-ios-simulator-proof` now writes package-mechanics
proof into this workpack root. It can prove the simulator package build and
install/launch path on macOS, but simulator package launch is not Always
authorization, region monitoring, significant-change, visits, low-power,
terminated/relaunch, notification, entitlement, or physical-device proof.
`node scripts/test/tracking-ios-location-manual-required-proof.mjs` now writes
parent-domain read-model proof rows for Always authorization, region
transitions, significant-change/visit events, and background
terminated/relaunch gaps under this workpack root. Those rows attach simulator
package/manual proof refs and keep Core Location background runtime,
entitlement, notification delivery, physical-device, authority, and
product-ready claims false.
`node scripts/test/tracking-ios-location-wp33-gate-proof.mjs` now wraps the same
WP12 background/region manual-required rows into the WP33 rollout gate artifact
`output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/27-ios-location-manual-required-proof.json`
without claiming Always authorization, region monitoring, significant-change,
visits, background delivery, entitlement, or physical-device behavior.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/12-ios-background-region-significant-change-adapter.md
- docs/plans/tracking-plan/implementation-checklist.md
- scripts/test/tracking-plan-ios-simulator-proof.mjs
- `output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/`
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
- [x] Proof artifacts under `output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/`.
- [x] Product doc/checklist updates or reason none were needed: feature doc and
      tracking checklist updated; central product checklist delta remains
      primary-owned through hub.
- [x] Known gaps/manual-required states: Always authorization, region
      monitoring, significant-change, visits, background delivery, low-power,
      terminated/relaunch, notification delivery, physical-device, and
      authority proof remain unclaimed.
- [x] Parent-domain manual-required proof added:
      `test-results/tracking-ios-location-manual-required-proof/proof.json`.
- [x] WP33 companion gate added:
      `test-results/tracking-ios-location-wp33-gate-proof/proof.json`.
