# WP14 Geofence Rule Model

## Purpose

Create parent-defined and system-suggested geofence rules with geometry,
schedule, grace, accuracy, transition policy, and audit refs.

## Source Inputs

- `docs/expectations/location-geofence.md`
- `docs/device-location-tracking-schema-proposal.md`
- `docs/plans/tracking-plan/v0-5-location-tracking-full-scope-plan.md`

## Target State

Geofence rules cover home, school, relative home, friend home, activity, work,
safe zone, restricted zone, temporary trip zone, and custom boundaries with
circle/polygon shapes.

## Tests And Proof

Proof root: `output/tracking-plan-proof/14-geofence-rule-model/`

- `01-contract-proof.log`
- `06-expected-place-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Require geometry, child/profile/device scope, enabled state, and audit refs.
- [ ] Represent schedule refs and transition policy.
- [ ] Add accuracy and grace fields.
- [ ] Keep AI suggestions separate from parent-created authority.

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

- docs/plans/tracking-plan/workpacks/14-geofence-rule-model.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/14-geofence-rule-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/14-geofence-rule-model/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
