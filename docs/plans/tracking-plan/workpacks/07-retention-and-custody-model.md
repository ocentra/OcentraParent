# WP07 Retention And Custody Model

## Purpose

Make location retention, deletion, export, and custody explicit before storing
or showing location history.

## Source Inputs

- `docs/expectations/data-custody.md`
- `docs/expectations/location-geofence.md`
- `docs/device-location-tracking-capability-guide.md`

## Target State

Retention supports last-known-only, 24h, 7d, 30d, custom, parent export, and
delete-after-alert-resolved. Custody labels distinguish child-device local,
parent-device local, family LAN relay, parent-owned storage, and
parent-approved cloud without default Ocentra-hosted storage.

## Tests And Proof

Proof root: `output/tracking-plan-proof/07-retention-and-custody-model/`

- `01-contract-proof.log`
- `10-journal-sqlite-proof.json`
- `14-retention-delete-proof.json`
- `17-retention-export-proof.json`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Default remote sync off.
- [ ] Default remote AI off.
- [ ] Add delete/tombstone/export tests.
- [ ] Ensure deleted points cannot stay visible in map UI.
- [ ] Keep notification providers out of location evidence custody.

## Where We Are

This workpack has P0 contract proof plus P1 fixture proof for retention delete
read-model filtering, parent-owned retention export, and UI-visible
deleted-history hiding from `codex/tracking-plan-full-scope` under the proof
root below. Platform behavior, live service-backed retention UI, and product
claim readiness are not claimed beyond the proof state recorded in
`proof-summary.json`, `14-retention-delete-proof.json`,
`17-retention-export-proof.json`, the WP30 UI fixture artifact, and the
implementation checklist.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/07-retention-and-custody-model.md
- docs/plans/tracking-plan/implementation-checklist.md
- `packages/activity-domain/src/tracking-retention-runtime.ts`
- `output/tracking-plan-proof/07-retention-and-custody-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, live service-backed retention UI, and product claims remain
  manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract/runtime files, proof scripts, tracking
      plan docs, checklist, and this workpack doc.
- [x] Validation commands and results:
      `node scripts/test/tracking-plan-runtime-proof.mjs` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/07-retention-and-custody-model/`, including
      `14-retention-delete-proof.json` and
      `17-retention-export-proof.json`.
- [x] Product doc/checklist updates: feature doc, implementation checklist,
      product capability checklist, README, and this workpack updated.
- [x] Known gaps/manual-required states: Android/iOS physical proof, precise
      desktop, provider delivery, notifications, live service-backed retention
      UI, and full UI remain proof-gated as applicable.
