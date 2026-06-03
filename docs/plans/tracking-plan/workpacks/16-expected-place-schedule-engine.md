# WP16 Expected-Place Schedule Engine

## Purpose

Evaluate where the child device is expected to be based on recurring schedules,
calendar events, temporary windows, geofences, place categories, and routes.

## Source Inputs

- `docs/expectations/policy.md`
- `docs/expectations/location-geofence.md`
- `docs/plans/tracking-plan/v0-5-location-tracking-full-scope-plan.md`

## Target State

Expected-place decisions cite schedule refs, expected-place rule refs, tolerance,
grace, evidence refs, and exception state.

## Tests And Proof

Proof root: `output/tracking-plan-proof/16-expected-place-schedule-engine/`

- `06-expected-place-proof.json`
- `09-policy-alert-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Test school, home-at-night, activity, calendar, and temporary trip rules.
- [ ] Test early/late/exit grace and distance tolerance.
- [ ] Test holiday exception suppression.
- [ ] Test low accuracy maps to check-in/ambiguous, not accusation.
- [ ] Keep parent policy as final action authority.

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

- docs/plans/tracking-plan/workpacks/16-expected-place-schedule-engine.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/16-expected-place-schedule-engine/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/16-expected-place-schedule-engine/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
