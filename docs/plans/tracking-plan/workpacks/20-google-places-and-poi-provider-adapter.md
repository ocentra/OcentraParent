# WP20 Google Places And POI Provider Adapter

## Purpose

Keep Google Places and other POI provider integration separate from the generic
nearby-place abstraction so provider-specific query limits, field masks, mapping,
and failure behavior are testable.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-ai-safety-analysis-plan.md`
- `docs/plans/tracking-plan/v0-5-location-test-blueprint.md`
- Google Places Nearby Search docs
- future Apple MapKit/OpenStreetMap provider docs when implemented

## Target State

Provider adapters use bounded location restrictions, minimal production field
masks, safe category mapping, distance calculation, provider failure
degradation, and no wildcard/broad place data requests in production.

## Tests And Proof

Proof root: `output/tracking-plan-proof/20-google-places-and-poi-provider-adapter/`

- `01-contract-proof.log`
- `07-nearby-place-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [x] Keep provider adapter code behind the nearby-place abstraction.
- [x] Test bounded radius/location restriction.
- [x] Test minimal field mask and reject wildcard production masks.
- [x] Test provider response mapping to category, distance, confidence, and
      ambiguity.
- [x] Test provider failure degrades gracefully.
- [x] Keep API credentials and provider terms out of core policy logic.

## Where We Are

This workpack now has P1 fixture/runtime helper proof from
`codex/tracking-poi-provider-adapter-proof`. The proof is local and contract
backed: it does not call Google Places, store API credentials, prove production
billing or terms setup, prove provider delivery, render UI, or prove mobile
physical-device background behavior.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/20-google-places-and-poi-provider-adapter.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/20-google-places-and-poi-provider-adapter/`
- `packages/activity-domain/src/tracking-poi-provider-adapter.ts`
- `packages/activity-domain/src/tracking-poi-provider-category.ts`
- `packages/activity-domain/tests/tracking-poi-provider-adapter.test.ts`
- `scripts/test/tracking-plan-poi-provider-adapter-proof.mjs`

## Manual-Required Gaps

- Live provider credentials, production provider setup, provider delivery, UI,
  retention, platform, and physical-device claims remain manual-required until
  their own proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch:
      `20-google-places-and-poi-provider-adapter`;
      `codex/tracking-poi-provider-adapter-proof`.
- [x] Touched files: activity-domain tracking adapter/test, proof script,
      package script, this workpack, implementation checklist, owning feature
      doc, activity-domain README, and generated proof artifacts.
- [x] Validation commands and results:
      `npm run test:tracking-plan-poi-provider-adapter-proof` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/20-google-places-and-poi-provider-adapter/`.
- [x] Product doc/checklist updates: owning feature doc and implementation
      checklist updated; central product checklist delta queued through hub
      DOC_DELTA policy instead of editing `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: live Google Places credentials,
      production provider setup, provider delivery, UI, hosted accessibility,
      Android/iOS physical background location, and production persistence
      remain unclaimed.
