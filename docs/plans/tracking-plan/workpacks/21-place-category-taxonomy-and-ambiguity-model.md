# WP21 Place-Risk Taxonomy And Ambiguity Model

## Purpose

Type nearby place categories and ambiguity states without turning category
evidence into accusations.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-ai-safety-analysis-plan.md`
- `docs/plans/tracking-plan/ui-ux-requirements-guide.md`
- `docs/expectations/policy.md`

## Target State

Place categories include school, home, hospital, clinic, police, cinema, mall,
bar, nightclub, liquor store, casino, hotel, transit, park, friend area, sports
activity, restaurant, out-of-town, highway/travel, remote area, and unknown.
Ambiguity states are visible.

## Tests And Proof

Proof root: `output/tracking-plan-proof/21-place-category-taxonomy-and-ambiguity-model/`

- `01-contract-proof.log`
- `07-nearby-place-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Add no-accusation copy validator cases.
- [ ] Test low accuracy and multiple-place ambiguity.
- [ ] Keep category as policy input, not action.
- [ ] Let parent-defined safe/restricted zones override only through policy.

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

- docs/plans/tracking-plan/workpacks/21-place-category-taxonomy-and-ambiguity-model.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/21-place-category-taxonomy-and-ambiguity-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/21-place-category-taxonomy-and-ambiguity-model/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
