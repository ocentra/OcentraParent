# WP03 Contract Boundary And Effect Schemas

## Purpose

Create schema-backed tracking contracts before runtime code consumes location,
status, geofence, alert, retention, AI, or acknowledgement data.

## Source Inputs

- `docs/expectations/location-geofence.md`
- `docs/device-location-tracking-schema-proposal.md`
- `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`
- `docs/plans/tracking-plan/v0-5-location-tracking-full-scope-plan.md`

## Target State

Effect Schema contracts exist for every tracking contract family, with branded
ids and decode helpers. No Zod, no manual string brands, and no raw app/runtime
string literals.

## Tests And Proof

Proof root: `output/tracking-plan-proof/03-contract-boundary-and-effect-schemas/`

- `00-source-snapshot.md`
- `01-contract-proof.log`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Add TypeScript contracts before runtime consumers.
- [ ] Add parser/brand tests and invalid-state tests.
- [ ] Mirror Rust only after TypeScript contracts are explicit and tested.
- [ ] Keep external input as `unknown` until parsed.
- [ ] Update docs/checklist rows if proof status changes.

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

- docs/plans/tracking-plan/workpacks/03-contract-boundary-and-effect-schemas.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/03-contract-boundary-and-effect-schemas/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/03-contract-boundary-and-effect-schemas/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
