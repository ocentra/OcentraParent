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

## What is implemented / exercised in this checkout

- `crates/ocentra-eventing` exists and the focused crate harnesses currently
  pass: `unit`, `contract`, `journal_replay`, `integration`, and
  `version_skew`.
- `packages/event-domain` mirror tests and `type-check` pass in this checkout.
- Focused downstream contract mirrors also pass:
  `@ocentra-parent/agent-protocol-domain`
  `network-runtime-events.test.ts` plus `contracts.test.ts`, and
  `cargo test -p ocentra-parent-agent-protocol child_domain_runtime_events --quiet`.
- A fresh WP13 regression proof now exists at
  `output/eventing-plan-proof/13-test-folder-layout-regression-audit/proof-summary.json`
  plus `test-results/eventing-test-folder-layout-regression-audit/proof.json`.
- A fresh WP12 route-proof bundle is restored locally at
  `docs/proof/eventing-plan/`, `output/eventing-plan-proof/rollout-proof/`, and
  `test-results/eventing-rollout-proof/`.
- A scoped 2026-06-17 WP11 regeneration pass now restores the eventing proof
  roots at `output/eventing-plan-proof/63-type-safety-source-gate/`,
  `66-76-source-safety/`, `67-lock-await/`, and `68-fixture-parity/`.
- Package-wide
  `npm run type-check --workspace @ocentra-parent/agent-protocol-domain`
  now passes again after the scoped WP11 source-boundary hardening in
  `src/contracts.ts`, `src/policy-control-audit-redaction.ts`, and
  `src/policy-control-delivery-read-model.ts`.
- Focused
  `policy-control-audit-redaction.test.ts`,
  `policy-control-delivery-read-model.test.ts`, `contracts.test.ts`, and the
  touched-file `npm run lint:architecture -- --files ...` gate now pass for the
  WP11 follow-up.

## Open gaps / missing product runtime

- Parent protocol event payloads and service read-model bridges still need their own feature proofs before they can claim full parent/controller or child-agent product eventing completion.
- Cross-process parent-to-child, external transport/relay, LAN/relay, and service transport delivery are not implemented by the reusable crate. They must publish into a local bus on each side after typed transport/API boundaries.
- Household AI Provider Mesh is consumer-layer work. The reusable eventing crate supplies local bus semantics, typed envelopes, idempotency, TTL, retry, dead-letter, aggregate ordering, request/response, journal/replay, and topology proof. It does not provide cross-device transport, peer discovery, provider trust, job authority, payload custody, policy behavior, or enforcement behavior.
- Network AI classification, policy decisions, enforcement commands, adapter side effects, audit storage, and portal rendering remain network/service/UI consumer work, not event bus responsibilities.
- External transport delivery currently proves local queue/idempotency/dead-letter semantics and route-decision requirements only. A live transport/relay delivery implementation remains a separate workpack.
- The NDJSON journal is the reusable append/replay proof layer. Production durability requirements such as fsync policy, SQLite projections, remote replication, or retention/deletion enforcement remain consumer/platform decisions.
- The fresh regression audit work itself is locally proved: no eventing test
  modules remain under `crates/ocentra-eventing/src/`, the focused crate suites
  still pass, and the proof root is recorded under
  `output/eventing-plan-proof/13-test-folder-layout-regression-audit/`.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).
- Historical checklist rows remain marked complete in
  `implementation-checklist.md`, but those checkmarks are not current proof
  truth for this checkout.
- `CHECKLIST_INDEX.md` is the current tracker for the recent WP11/WP12/WP13
  local proof closure and the remaining WP10 open work.

## Workpack summary

- Workpacks indexed: 13 route workpacks.
- Workpack source: `05-implementation-workpacks.md` rows split into focused files under `workpacks/`.
- Historical route docs describe prior closure for WP01-WP11, but the cited
  proof bundle is not present in this checkout.
- Workpacks open in truth: WP10 consumer-boundary handoff only.
- Current meaning: implementation surfaces exist across the crate and its
  mirrors, WP11/WP12/WP13 are locally proved, but the plan remains open because
  WP10 still lacks complete proof.

### Active/open workpacks

- [10 LAN Household Mesh Consumer](workpacks/10-lan-household-mesh-consumer.md)

## Validation reality

- Focused reusable crate validation passes in this checkout:
  `cargo test -p ocentra-eventing --test unit`,
  `--test contract`, `--test journal_replay`, `--test integration`, and
  `--test version_skew`, plus
  `cargo lint-architecture crates/ocentra-eventing/src crates/ocentra-eventing/tests`.
- Post-WP13 cleanup validation also passes:
  `cargo test -p ocentra-eventing --tests`, with the resulting proof recorded
  at
  `output/eventing-plan-proof/13-test-folder-layout-regression-audit/proof-summary.json`.
- WP12 route-proof validation is restored locally through
  `node scripts/test/eventing-rollout-proof.mjs`, with the resulting proof
  bundle recorded under `output/eventing-plan-proof/rollout-proof/`.
- Shared TypeScript contract mirror validation passes in this checkout:
  `npm run test --workspace @ocentra-parent/event-domain` and
  `npm run type-check --workspace @ocentra-parent/event-domain`.
- Focused downstream mirror validation also passes:
  `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- network-runtime-events.test.ts contracts.test.ts`
  and `cargo test -p ocentra-parent-agent-protocol child_domain_runtime_events --quiet`.
- Scoped WP11 proof regeneration now passes through the reusable eventing
  surface and writes proof artifacts under
  `output/eventing-plan-proof/63-type-safety-source-gate/`,
  `66-76-source-safety/`, `67-lock-await/`, and `68-fixture-parity/`.
- `npm run type-check --workspace @ocentra-parent/agent-protocol-domain` now
  passes again, and the focused policy-control/contract tests plus touched-file
  architecture gate pass for the scoped WP11 source files.
- Current interpretation: the reusable runtime behavior is partially exercised,
  WP11/WP12/WP13 are locally proved, but the plan remains open because WP10
  still lacks its local proof roots and consumer-plan handoff verification.

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
