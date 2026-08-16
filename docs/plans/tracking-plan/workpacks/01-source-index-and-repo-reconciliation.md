# WP01 Source Index And Repo Reconciliation

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP01 Source Index And Repo Reconciliation`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Keep all tracking work grounded in the current repo sources and the two pasted
planning guides without creating a second product truth.

## Source Inputs

- `docs/features/location-geofence-device-status.md`
- `docs/expectations/location-geofence.md`
- `docs/tracking-control-settings-inventory.md`
- `docs/device-location-tracking-capability-guide.md`
- `docs/device-location-tracking-schema-proposal.md`
- `docs/plans/tracking-plan/source-index.md`
- both pasted planning attachments named in the source index

## Target State

The source index, coverage audit, feature doc, and capability checklist agree on
status, source ownership, planning-only boundaries, and manual-required gaps.

## Tests And Proof

Proof root: `output/tracking-plan-proof/01-source-index-and-repo-reconciliation/`

- `00-source-snapshot.md`
- `16-validation-commands.log`

## AI Worker Checklist

- [x] Read the source index and this workpack.
- [x] Update source references only when a current source doc changes.
- [x] Do not cite inaccessible ChatGPT share content as read.
- [x] Keep pasted drafts as planning input, not runtime contract source.
- [x] Report product-doc updates or why none were needed.

## Where We Are

The 2026-08-15 code-first audit re-inventoried the live checkout and replaced
the stale source route. `packages/tracking-domain` and the advertised
`scripts/test/tracking-*.mjs` suite are absent; current owners are the Rust
schema/protocol/tracking/runtime/policy/notification/AI/service crates and the
presentation-only portal paths listed in `source-index.md`. No product status
was advanced.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/01-source-index-and-repo-reconciliation.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/01-source-index-and-repo-reconciliation/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-plan-code-audit`.
- [ ] Touched files: source index, current snapshot, implementation checklist,
      WP01/WP02 docs, source reconciliation proof script, generated WP01/WP02 proof
      roots, and product-checklist delta queue.
- [ ] Validation commands and results: Phase 2 focused validation remains open;
      the formerly named tracking proof command does not exist in this checkout.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/01-source-index-and-repo-reconciliation/`.
- [ ] Product doc/checklist updates: source index, current snapshot,
      implementation checklist, and owning feature doc/checklist delta queue; the
      shared capability checklist file was not edited by this lane.
- [ ] Known gaps/manual-required states: physical-device, authority-enrolled,
      provider delivery/receipt, full product UI, retention product runtime,
      production workers, and product-ready tracking remain proof-gated.
