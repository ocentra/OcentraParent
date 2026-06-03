# WP30 Parent And Child UI/UX Surfaces

## Purpose

Render parent map/list/status, alert cards, evidence drawer, exception editor,
child check-in, live tracking, missing-device, retention, and capability states.

## Source Inputs

- `docs/plans/tracking-plan/ui-ux-requirements-guide.md`
- `docs/expectations/portal.md`
- `docs/expectations/location-geofence.md`

## Target State

UI shows source, freshness, accuracy, custody, retention, permission,
capability, ambiguity, deleted state, and safe copy across parent and child
surfaces.

## Tests And Proof

Proof root: `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`

- `11-ui-snapshots/`
- `12-playwright-proof.log`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Add Playwright coverage for all required screens and badges.
- [ ] Add no-overlap/no-overclaim screenshot proof.
- [ ] Ensure deleted history disappears.
- [ ] Ensure child copy avoids accusation.
- [ ] Keep portal as authoring/display surface, not evaluator.

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

- docs/plans/tracking-plan/workpacks/30-parent-and-child-ui-ux-surfaces.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
