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

## AI Worker Checklist

- [ ] Prove When In Use authorization UX.
- [ ] Prove current location sample.
- [ ] Prove denied/restricted and services-disabled states.
- [ ] Preserve accuracy/freshness.
- [ ] Do not claim Always/background from this workpack.

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

- docs/plans/tracking-plan/workpacks/11-ios-core-location-foreground-adapter.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/11-ios-core-location-foreground-adapter/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/11-ios-core-location-foreground-adapter/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
