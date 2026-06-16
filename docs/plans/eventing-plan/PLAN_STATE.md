# Reusable Rust Eventing Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Reusable Rust Eventing Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `eventing-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the implementation plan for a reusable Rust event bus inspired by the Ocentra Games `@ocentra/eventing-domain` package, but rebuilt as a Rust-first crate that can be used by Ocentra Parent and other Ocentra Rust projects.

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Open only the assigned workpack.
5. Use `CHECKLIST_INDEX.md` for exact checklist sections.
6. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- Snapshot: [current-eventing-snapshot.md](current-eventing-snapshot.md)

## What is already present / proved

- Household Mesh consumer proof is complete: `output/eventing-plan-proof/12-household-mesh-consumer/proof-summary.json`.
- Reusable runtime envelope/version and source-safety proof is complete: `output/eventing-plan-proof/14-24-runtime-lifecycle/proof-summary.json`, `output/eventing-plan-proof/63-type-safety-source-gate/proof-summary.json`, `output/eventing-plan-proof/reusable-eventing-runtime/proof-summary.json`.
- Journal/replay proof is complete: `output/eventing-plan-proof/36-41-journal-replay/proof-summary.json`.
- Network consumer proof linkage is complete: `output/eventing-plan-proof/62-network-proof-links/proof-summary.json`, `output/network-plan-proof/10-service-event-chain-stream/proof-summary.json`.
- Type-safety and ownership hardening proof is complete: `output/eventing-plan-proof/63-type-safety-source-gate/proof-summary.json`.
- WP12 rollout proof and PR gate is complete: `output/eventing-plan-proof/rollout-proof/proof-summary.json`, `test-results/eventing-rollout-proof/proof.json`, and `output/eventing-plan-proof/rollout-proof/pr-done-report.md` record the final reconciliation.

## Open gaps / missing product runtime

- Parent protocol event payloads and service read-model bridges still need their own feature proofs before they can claim full parent/controller or child-agent product eventing completion.
- Cross-process parent-to-child, external transport/relay, LAN/relay, and service transport delivery are not implemented by the reusable crate. They must publish into a local bus on each side after typed transport/API boundaries.
- Household AI Provider Mesh is consumer-layer work. The reusable eventing crate supplies local bus semantics, typed envelopes, idempotency, TTL, retry, dead-letter, aggregate ordering, request/response, journal/replay, and topology proof. It does not provide cross-device transport, peer discovery, provider trust, job authority, payload custody, policy behavior, or enforcement behavior.
- Network AI classification, policy decisions, enforcement commands, adapter side effects, audit storage, and portal rendering remain network/service/UI consumer work, not event bus responsibilities.
- External transport delivery currently proves local queue/idempotency/dead-letter semantics and route-decision requirements only. A live transport/relay delivery implementation remains a separate workpack.
- The NDJSON journal is the reusable append/replay proof layer. Production durability requirements such as fsync policy, SQLite projections, remote replication, or retention/deletion enforcement remain consumer/platform decisions.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checkbox rows detected: 138 total, 138 checked, 0 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks indexed: 12 route workpacks.
- Workpack source: `05-implementation-workpacks.md` rows split into focused files under `workpacks/`.
- Workpacks with implementation proof complete: route proofs exist for the reusable runtime, parent/child, network, and household mesh slices.
- Workpacks open: none. WP12 rollout proof and PR gate reconciliation is complete.
- Current meaning: the route proofs are present, the rollout gate is fully closed, and the proof checklist is synchronized.

### Active/open workpacks

- None.

## Validation reality

- `cargo test -p ocentra-eventing` passes for the reusable Rust crate.
- `npm test` in `packages/event-domain` passes for the shared TypeScript contract mirror.
- The reusable runtime, journal/replay, and consumer-boundary proof packs are now attached to their own local proof docs.
- Current interpretation: the reusable runtime behavior is test-verified, and the rollout proof report closes the plan gate with the checklist fully reconciled.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/eventing-plan/.
- Required proof manifest names:
  - docs/proof/eventing-plan/slice-01-\*.md
  - docs/proof/eventing-plan/slice-02-\*.md
  - docs/proof/eventing-plan/slice-03-\*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
