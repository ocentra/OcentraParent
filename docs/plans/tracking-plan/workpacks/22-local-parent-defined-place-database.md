# WP22 Local Parent-Defined Place Database

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP22 Local Parent-Defined Place Database`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
- `17-parent-owned-export-proof.json`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Keep parent-defined place data out of Ocentra-hosted default storage.
- [ ] Add CRUD/import/export/delete proof.
- [ ] Add audit refs for parent edits.
- [ ] Test safe-zone/restricted-zone policy behavior.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope`
and P1 local parent-defined place store proof from
`codex/tracking-local-place-store-proof` under the proof root below. The P1
proof covers schema-backed local create/update/import/export/delete helper
behavior, parent-device-local default storage, parent-owned export, deletion
tombstones, and safe/restricted parent-defined nearby-place policy signals.
Platform adapters, provider delivery, live UI, hosted accessibility,
physical-device background location, and production persistence are not claimed
beyond the proof state recorded in `proof-summary.json` and the implementation
checklist.

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

- Platform, provider, UI, production persistence, or physical-device claims
  remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-local-place-store-proof`.
- [ ] Touched files: `packages/activity-domain/src/tracking-local-place-store.ts`,
      `packages/activity-domain/src/tracking.ts`,
      `packages/activity-domain/tests/tracking-local-place-store.test.ts`,
      `scripts/test/tracking-plan-local-place-store-proof.mjs`, root
      `package.json`, this workpack doc, implementation checklist, feature doc,
      activity-domain README, and WP22 proof artifacts.
- [ ] Validation commands and results:
      `npm run test:tracking-plan-local-place-store-proof` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/22-local-parent-defined-place-database/`,
      including `07-nearby-place-proof.json`,
      `14-retention-delete-proof.json`, and
      `17-parent-owned-export-proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and activity-domain README updated.
      Central product capability checklist movement is queued through the hub
      DOC_DELTA flow instead of editing `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: Android/iOS physical proof, precise
      desktop OS location, provider delivery, production persistence, live UI,
      hosted accessibility, notifications, and authority-enrolled behavior
      remain proof-gated as applicable.
