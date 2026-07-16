# WP02 Current Tracking Snapshot And Gap Map

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP02 Current Tracking Snapshot And Gap Map`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Maintain the live gap map before implementation starts or status changes.

## Source Inputs

- `docs/plans/tracking-plan/current-tracking-snapshot.md`
- `docs/product-capability-checklist.md`
- `docs/features/location-geofence-device-status.md`
- `docs/tracking-control-settings-inventory.md`

## Target State

Current artifacts, missing contracts, manual-required gaps, no-claim boundaries,
and product claim gates are explicit and current.

## Tests And Proof

Proof root: `output/tracking-plan-proof/02-current-tracking-snapshot-and-gap-map/`

- `00-source-snapshot.md`
- `16-validation-commands.log`

## Manual-Required Gaps

- Android/iOS real-device proof.
- Desktop precise location proof.
- Remote sync and remote AI proof.
- Emergency/critical escalation proof.

## AI Worker Checklist

- [ ] Confirm repo state before changing status.
- [ ] Keep "planning exists" separate from "runtime proof exists".
- [ ] Update the feature doc and capability checklist when proof changes.
- [ ] Do not use old checkpoint wording to override current docs.

## Where We Are

This workpack now has current snapshot and gap-map proof from
`codex/tracking-plan-full-continuation-a` under the proof root below. The proof
checks source docs, the refreshed current snapshot, and product-readiness
closure blockers while keeping runtime/product-complete behavior unclaimed.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/02-current-tracking-snapshot-and-gap-map.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/02-current-tracking-snapshot-and-gap-map/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: current snapshot, source index, implementation checklist,
      WP01/WP02 docs, source reconciliation proof script, generated WP01/WP02 proof
      roots, and product-checklist delta queue.
- [ ] Validation commands and results:
      `node scripts/test/tracking-source-reconciliation-gap-map-proof.mjs` passed
      after regenerating the product-readiness closure proof.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/02-current-tracking-snapshot-and-gap-map/`.
- [ ] Product doc/checklist updates: current snapshot, source index,
      implementation checklist, and owning feature doc/checklist delta queue; the
      shared capability checklist file was not edited by this lane.
- [ ] Known gaps/manual-required states: Android/iOS physical-device proof,
      actual child-device runtime, full product parent/child UI, authority,
      provider delivery/receipt runtime, retention product runtime, production
      workers, and product-ready tracking remain proof-gated.
