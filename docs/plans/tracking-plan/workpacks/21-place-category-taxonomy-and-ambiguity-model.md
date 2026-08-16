# WP21 Place-Risk Taxonomy And Ambiguity Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP21 Place-Risk Taxonomy And Ambiguity Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
- `17-category-ambiguity-no-accusation-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Add no-accusation copy validator cases.
- [ ] Test low accuracy and multiple-place ambiguity.
- [ ] Keep category as policy input, not action.
- [ ] Let parent-defined safe/restricted zones override only through policy.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope`
under the proof root below. This continuation adds
`node scripts/test/tracking-place-category-ambiguity-proof.mjs`, which builds
parent-domain proof rows from the existing POI provider adapter and proves
no-accusation copy, low-accuracy ambiguity, multiple-place ambiguity, category
as policy input only, and parent-defined zone override as policy-review input
only. Runtime, platform, provider delivery, physical-device, production, and
UI behavior is not claimed beyond the proof state recorded in
`proof-summary.json` and the implementation checklist.

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
- packages/parent-domain/src/tracking-place-category-ambiguity-proof.ts
- packages/parent-domain/tests/tracking-place-category-ambiguity-proof.test.ts
- scripts/test/tracking-place-category-ambiguity-proof.mjs
- `output/tracking-plan-proof/21-place-category-taxonomy-and-ambiguity-model/`
- `test-results/tracking-place-category-ambiguity-proof/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [ ] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [ ] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [ ] Proof artifacts under `output/tracking-plan-proof/21-place-category-taxonomy-and-ambiguity-model/`.
- [ ] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [ ] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
- [ ] Workpack id and branch: `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: WP21 proof source, tests, proof harness, feature doc,
      implementation checklist, WP21/WP33 docs, and generated proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-place-category-ambiguity-proof.mjs` passed
      locally.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/21-place-category-taxonomy-and-ambiguity-model/`
      and `test-results/tracking-place-category-ambiguity-proof/`.
- [ ] Product doc/checklist updates: owning tracking feature doc, tracking
      implementation checklist, WP21, and WP33 updated; central product
      checklist row remains unchanged because this does not upgrade product
      support beyond the existing local proof tier.
- [ ] Known gaps/manual-required states: live provider execution, provider
      delivery, exact-place claims, Android/iOS physical-device proof,
      authority, production, full UI, and automatic action remain unclaimed.
