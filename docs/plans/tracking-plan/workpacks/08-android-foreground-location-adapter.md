# WP08 Android Foreground Location Adapter

## Purpose

Prove Android foreground current, fused, and last-known location evidence with
permission, accuracy, freshness, and degraded state.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-platform-deep-dive.md`
- `docs/expectations/platforms.md`
- Android official location update docs

## Target State

Android foreground location produces contract-valid location evidence or an
explicit unavailable/degraded state.

## Tests And Proof

Proof root: `output/tracking-plan-proof/08-android-foreground-location-adapter/`

- `02-platform-permission-proof.md`
- `03-runtime-location-evidence.json`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Prove foreground permission UX.
- [ ] Prove fused/current sample.
- [ ] Prove last-known sample or unavailable state.
- [ ] Preserve accuracy and timestamp from provider.
- [ ] Document device, OS version, app build, and proof commands.

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

- docs/plans/tracking-plan/workpacks/08-android-foreground-location-adapter.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/08-android-foreground-location-adapter/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/08-android-foreground-location-adapter/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
