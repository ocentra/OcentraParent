# WP20 Google Places And POI Provider Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP20 Google Places And POI Provider Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
- `08-provider-parity-readiness-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Keep provider adapter code behind the nearby-place abstraction.
- [ ] Test bounded radius/location restriction.
- [ ] Test minimal field mask and reject wildcard production masks.
- [ ] Test provider response mapping to category, distance, confidence, and
      ambiguity.
- [ ] Test provider failure degrades gracefully.
- [ ] Keep API credentials and provider terms out of core policy logic.

## Where We Are

This workpack now has focused P1 provider-adapter contract proof from
`codex/tracking-google-poi-provider-proof`. The proof builds a bounded Google
Places Nearby Search request with a production-safe field mask, maps
real-shaped provider response rows into nearby-place category, distance,
confidence, and ambiguity evidence, and records provider unavailable
degradation. The current continuation branch also records provider parity
readiness rows in
`output/tracking-plan-proof/20-google-places-and-poi-provider-adapter/08-provider-parity-readiness-proof.json`:
Google is request-mapped from the existing contract proof, while Apple MapKit
and OpenStreetMap/Nominatim remain manual-required until provider terms,
runtime, and authorization proof exist. Live Google/Apple/OSM provider
execution, credentials, exact-place claims, physical-device proof, UI, and
production persistence remain unclaimed.

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
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: WP20,
      `codex/tracking-google-poi-provider-proof`.
- [ ] Touched files: `packages/parent-domain/src/tracking-poi-provider-adapter.ts`,
      `packages/parent-domain/tests/tracking-poi-provider-adapter.test.ts`,
      `scripts/test/tracking-poi-provider-adapter-proof.mjs`, this workpack,
      the location/geofence feature doc, implementation checklist, and proof
      outputs.
- [ ] Validation commands and results:
      `node scripts/test/tracking-poi-provider-adapter-proof.mjs` passed
      parent-domain build plus Vitest tracking POI provider and tracking policy
      tests.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/20-google-places-and-poi-provider-adapter/`.
- [ ] Provider parity readiness rows preserve Google request-mapped status and
      Apple MapKit/OpenStreetMap manual-required status without claiming live
      execution.
- [ ] Product doc/checklist updates: owning feature doc and implementation
      checklist updated. Product capability checklist update is queued because
      another lane currently owns that file lock.
- [ ] Known gaps/manual-required states: live provider execution, credentials,
      provider terms/runtime auth, exact-place claims, physical-device proof,
      UI, production persistence, and Apple/OSM provider parity remain
      unclaimed.
