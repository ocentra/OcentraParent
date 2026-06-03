# WP22 Local Parent-Defined Place Database

## Purpose

Let parents define home, school, friend, activity, safe, restricted, and custom
places as parent-owned evidence and policy inputs.

## Source Inputs

- `docs/device-location-tracking-schema-proposal.md`
- `docs/expectations/data-custody.md`
- `docs/plans/tracking-plan/v0-5-location-tracking-full-scope-plan.md`

## Target State

Parent-defined places are local/parent-owned, auditable, exportable/deletable,
and usable by geofence, expected-place, nearby-place, and policy compilers.

## Tests And Proof

Proof root: `output/tracking-plan-proof/22-local-parent-defined-place-database/`

- `01-contract-proof.log`
- `07-nearby-place-proof.json`
- `14-retention-delete-proof.json`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Keep parent-defined place data out of Ocentra-hosted default storage.
- [ ] Add CRUD/import/export/delete proof.
- [ ] Add audit refs for parent edits.
- [ ] Test safe-zone/restricted-zone policy behavior.

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

- docs/plans/tracking-plan/workpacks/22-local-parent-defined-place-database.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/22-local-parent-defined-place-database/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/22-local-parent-defined-place-database/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
