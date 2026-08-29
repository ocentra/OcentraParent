# WP11 iOS Core Location Foreground Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP11 iOS Core Location Foreground Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

## Source Route Correction (2026-08-29)

This workpack is blocked for source work. The canonical
`platforms/ios/OcentraChildAgent` application is owned by Child Runtime
Distribution WP06 and is deliberately capability-only. It has no Core Location
producer or Rust/transport handoff. Child WP10 must first provide trusted child
startup/ingress, and Tracking WP40 must provide the trusted durable tracking
ingress that receives platform observations.

The neutral `tracking-core` location/status models and their tests are
consumer contracts, not iOS adapter implementation. The routed missing roots
are:

- `platforms/ios/OcentraChildAgent/Tracking/ChildForegroundLocationAdapter.swift`
- `platforms/ios/OcentraChildAgent/Tracking/ChildTrackingLocationIngress.swift`
- `platforms/ios/OcentraChildAgentUITests/ChildForegroundLocationRuntimeUITests.swift`
- `platforms/ios/tests/child_foreground_location_runtime.test.mjs`

Do not add a local JSON/SharedPreferences/file handoff, caller-minted identity,
or presentation-only status row. The eventual XCTest source must exercise the
real application/Core Location boundary and preserve denied, restricted,
services-disabled, stale, and no-background-claim behavior.
The Node test root must invoke the real XCTest target; source-text inspection
or a fixture-only substitute is not acceptable.

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

- [ ] Prove When In Use authorization UX. Parent-domain manual-required row now
      exists; real authorization capture remains pending.
- [ ] Prove current location sample. Parent-domain manual-required row now
      exists; real Core Location sample capture remains pending.
- [ ] Prove denied/restricted and services-disabled states. Parent-domain
      manual-required row now exists; real simulator/device state capture
      remains pending.
- [ ] Preserve accuracy/freshness.
- [ ] Do not claim Always/background from this workpack.
- [ ] Generate the iOS simulator/local proof artifact plan before device work.
- [ ] Route simulator package build/install/launch proof through the tracking
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
`node scripts/test/tracking-ios-location-manual-required-proof.mjs` now writes
parent-domain read-model proof rows for When In Use authorization, foreground
sample, and denied/restricted/services-disabled gaps under this workpack root.
Those rows attach simulator package/manual proof refs plus an explicit runtime
artifact inventory for the missing When In Use authorization state, foreground
location events, and degraded location state artifacts, and keep authorization,
sample capture, physical-device, notification, provider, authority, and
product-ready claims false.
`node scripts/test/tracking-ios-location-wp33-gate-proof.mjs` now wraps the same
WP11 foreground manual-required rows into the WP33 rollout gate artifact
`output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/27-ios-location-manual-required-proof.json`
without claiming Core Location authorization or foreground samples.

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
- The routed Swift adapter/handoff and XCTest roots listed above, only after
  Child WP06, Child WP10, and Tracking WP40 are reviewed.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files: proof script, root script wiring, package-preview CI
      upload wiring, feature doc, checklist, and this workpack doc.
- [ ] Validation commands and results: `npm run test:tracking-plan-ios-simulator-proof`
      writes local proof; macOS package-preview runs it with
      `--require-simulator` after the real iOS simulator smoke.
- [ ] Proof artifacts under `output/tracking-plan-proof/11-ios-core-location-foreground-adapter/`.
- [ ] Product doc/checklist updates or reason none were needed: feature doc and
      tracking checklist updated; central product checklist delta remains
      primary-owned through hub.
- [ ] Known gaps/manual-required states: Core Location authorization/sample,
      background/region behavior, notification delivery, signing/TestFlight,
      physical-device, and authority proof remain unclaimed.
- [ ] Parent-domain manual-required proof added:
      `test-results/tracking-ios-location-manual-required-proof/proof.json`.
- [ ] WP33 companion gate added:
      `test-results/tracking-ios-location-wp33-gate-proof/proof.json`.
