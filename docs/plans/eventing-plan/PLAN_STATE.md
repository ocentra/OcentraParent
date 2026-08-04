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

## Current ownership interpretation

```text
ocentra-eventing:
  Reusable Rust local event bus, typed envelopes, event ids, idempotency keys, aggregate ordering, queue/dead-letter semantics, request/response registry, journal/replay, topology/contract registry, local dispatch lifecycle, and testkit helpers.

schema-domain:
  Canonical shared event contract shapes when event contracts cross package, crate, app, or plan boundaries.

event-domain:
  Package-boundary metadata only. Shared event contracts live in schema-domain or the owning protocol package.

agent-protocol and agent-service:
  Protocol/service consumers when selected. They prove wire, service, and read-model delivery only for their own surfaces.

LAN and remote-access plans:
  Transport, mesh, relay, pairing, route authority, and cross-device delivery owners.

Network, AI, policy, enforcement, portal, data-custody, browser, app-game, screen, tracking, setup, payment, and account plans:
  Consumer owners. They may publish or consume typed events through handoffs, but they own their domain behavior.
```

## Current coupling risks

```text
- Local bus proof is not cross-device transport proof.
- Protocol shape proof is not service delivery proof.
- NDJSON journal/replay proof is not production durability, retention, deletion, export, or remote replication proof.
- Consumer read-model proof is not reusable eventing crate readiness.
- Eventing does not own AI classification, policy decisions, enforcement actions, portal rendering, data custody policy, LAN/remote relay, or adapter side effects.
- WP12 route proof and WP13 layout proof cannot close WP10 LAN household mesh consumer proof.
- Historical checked rows and historical proof references do not override current proof-root absence.
```

## Current proof interpretation

```text
output/eventing-plan-proof/<workpack>/ is the normal raw/generated implementation/workpack proof root.
docs/proof/eventing-plan/ is accepted for the current WP12 route-proof bundle and the hand-authored WP06 durable manifest.
crate tests can prove local reusable bus behavior only.
event-domain metadata does not prove shared event contract implementation.
agent-protocol/service tests prove protocol/service handoff only when selected.
consumer plan proof is required before claiming product behavior.
WP06 now retains its exact enforcement WP11 journal handoff, journal/topology
  proof, and compact validation log in its hand-authored durable manifest under
  `docs/proof/eventing-plan/`. WP10 remains
open until its proof roots and LAN/remote-access handoff verification exist.
```

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
5. Open only the assigned workpack.
6. Use `CHECKLIST_INDEX.md` for exact checklist sections.
7. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- Snapshot: [current-eventing-snapshot.md](current-eventing-snapshot.md)

## What is implemented / exercised in this checkout

- `crates/ocentra-eventing` exists and the focused crate harnesses currently pass: `unit`, `contract`, `journal_replay`, `integration`, and `version_skew`.
- `packages/event-domain` mirror tests and `type-check` pass in this checkout.
- Focused downstream contract mirrors also pass: `@ocentra-parent/agent-protocol-domain` `network-runtime-events.test.ts` plus `contracts.test.ts`, and `cargo test -p ocentra-parent-agent-protocol child_domain_runtime_events --quiet`.
- A fresh WP13 regression proof now exists at `output/eventing-plan-proof/13-test-folder-layout-regression-audit/proof-summary.json` plus `test-results/eventing-test-folder-layout-regression-audit/proof.json`.
- A fresh WP12 route-proof bundle is restored locally at `docs/proof/eventing-plan/`, `output/eventing-plan-proof/rollout-proof/`, and `test-results/eventing-rollout-proof/`.
- A scoped 2026-06-17 WP11 regeneration pass now restores the eventing proof roots at `output/eventing-plan-proof/63-type-safety-source-gate/`, `66-76-source-safety/`, `67-lock-await/`, and `68-fixture-parity/`.
- Package-wide `npm run type-check --workspace @ocentra-parent/agent-protocol-domain` now passes again after the scoped WP11 source-boundary hardening in `src/contracts.ts`, `src/policy-control-audit-redaction.ts`, and `src/policy-control-delivery-read-model.ts`.
- Focused `policy-control-audit-redaction.test.ts`, `policy-control-delivery-read-model.test.ts`, `contracts.test.ts`, and the touched-file `npm run lint:architecture -- --files ...` gate now pass for the WP11 follow-up.

## Open gaps / missing product runtime

- Parent protocol event payloads and service read-model bridges still need their own feature proofs before they can claim full parent/controller or child-agent product eventing completion.
- Cross-process parent-to-child, external transport/relay, LAN/relay, and service transport delivery are not implemented by the reusable crate. They must publish into a local bus on each side after typed transport/API boundaries.
- Household AI Provider Mesh is consumer-layer work. The reusable eventing crate supplies local bus semantics, typed envelopes, idempotency, TTL, retry, dead-letter, aggregate ordering, request/response, journal/replay, and topology proof. It does not provide cross-device transport, peer discovery, provider trust, job authority, payload custody, policy behavior, or enforcement behavior.
- Network AI classification, policy decisions, enforcement commands, adapter side effects, audit storage, and portal rendering remain network/service/UI consumer work, not event bus responsibilities.
- External transport delivery currently proves local queue/idempotency/dead-letter semantics and route-decision requirements only. A live transport/relay delivery implementation remains a separate workpack.
- The NDJSON journal is the reusable append/replay proof layer. Production durability requirements such as fsync policy, SQLite projections, remote replication, or retention/deletion enforcement remain consumer/platform decisions.
- The fresh regression audit work itself is locally proved: no eventing test modules remain under `crates/ocentra-eventing/src/`, the focused crate suites still pass, and the proof root is recorded under `output/eventing-plan-proof/13-test-folder-layout-regression-audit/`.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).
- Historical checklist rows remain marked complete in `implementation-checklist.md`, but those checkmarks are not current proof truth for this checkout.
- `CHECKLIST_INDEX.md` is the current tracker for the recent WP11/WP12/WP13 local proof closure and the remaining WP10 open work.

## Workpack summary

- Workpacks indexed: 13 route workpacks.
- Workpack source: `05-implementation-workpacks.md` rows split into focused files under `workpacks/`.
- Historical route docs describe prior closure for WP01-WP11, but the cited proof bundle is not present in this checkout. WP06 is locally evidenced by its durable hand-authored manifest because enforcement WP11 needs its exact generic handoff.
- Workpacks open in truth: WP10 consumer-boundary handoff.
- Current meaning: implementation surfaces exist across the crate and its mirrors; WP06/WP11/WP12/WP13 are locally proved. The plan remains open because WP10 still lacks its required proof roots and consumer-plan handoff verification.

### Active/open workpacks

- [10 LAN Household Mesh Consumer](workpacks/10-lan-household-mesh-consumer.md)

## Validation reality

- Focused reusable crate validation passes in this checkout: `cargo test -p ocentra-eventing --test unit`, `--test contract`, `--test journal_replay`, `--test integration`, and `--test version_skew`, plus `cargo lint-architecture crates/ocentra-eventing/src crates/ocentra-eventing/tests`.
- Post-WP13 cleanup validation also passes: `cargo test -p ocentra-eventing --tests`, with the resulting proof recorded at `output/eventing-plan-proof/13-test-folder-layout-regression-audit/proof-summary.json`.
- WP12 route-proof validation is restored locally through `node scripts/test/eventing-rollout-proof.mjs`, with the resulting proof bundle recorded under `output/eventing-plan-proof/rollout-proof/`.
- Shared TypeScript contract mirror validation passes in this checkout: `npm run test --workspace @ocentra-parent/event-domain` and `npm run type-check --workspace @ocentra-parent/event-domain`.
- Focused downstream mirror validation also passes: `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- network-runtime-events.test.ts contracts.test.ts` and `cargo test -p ocentra-parent-agent-protocol child_domain_runtime_events --quiet`.
- Scoped WP11 proof regeneration now passes through the reusable eventing surface and writes proof artifacts under `output/eventing-plan-proof/63-type-safety-source-gate/`, `66-76-source-safety/`, `67-lock-await/`, and `68-fixture-parity/`.
- `npm run type-check --workspace @ocentra-parent/agent-protocol-domain` now passes again, and the focused policy-control/contract tests plus touched-file architecture gate pass for the scoped WP11 source files.
- WP06 focused proof now passes: journal/replay (22), topology manifest (4),
  lineage compatibility (3), runtime shutdown (5), and the scoped
  architecture gate are recorded in the durable manifest under
  `docs/proof/eventing-plan/`.
- Current interpretation: the reusable runtime behavior is partially exercised, WP06/WP11/WP12/WP13 are locally proved, but the plan remains open because WP10 lacks its local proof roots and consumer-plan handoff verification.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- `WORKPACK_FAMILIES.md` unless the selected workpack owner/proof family is unclear.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Active proof-root route:
  - use `output/eventing-plan-proof/<workpack-file-stem>/` for new raw/generated implementation/workpack output.
  - `docs/proof/eventing-plan/` is accepted for the current WP12 route-proof bundle and the hand-authored WP06 durable manifest.
  - historical `docs/proof/eventing-plan/slice-*` references do not close runtime work by themselves.
- Before any checked update, attach:
  - a real test run log or explicit blocker from the assigned implementation boundary,
  - a proof artifact under the selected proof root,
  - negative cases, no-claim language, and manual-required notes where applicable.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, consumer handoff, and rollback/teardown proofs are present or carried as exact blockers for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
