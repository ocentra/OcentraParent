# AI Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from open workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Read `workpacks/00-owner-boundary-proof-gate.md` to apply current AI owner/import/proof rules.
4. Open that workpack and exact checklist rows only.
5. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Dependency-first Phase 1 queue

Checkbox count is no longer used to choose the next code slice. The retained
source audit is [CODE_AUDIT.md](CODE_AUDIT.md). Work in this order:

1. Complete the WP03 source-preserving migration from the reviewed
   `crates/schema` packet at `83382d67b` into the neutral leaf crate
   `crates/ai-contracts` (`ocentra-ai-contracts`). Keep schema and
   agent-protocol as direct consumers, preserve the generated TypeScript owner,
   and add no public re-export or authority constructor.
2. Write WP03's owned contract/parity tests together after the move:
   `crates/ai-contracts/tests/contract/ai_contracts.rs`,
   `crates/ai-contracts/tests/contract/ai_contracts_negative.rs`, and
   `packages/schema-domain/tests/contract/ai-contracts.test.ts`. Establish and
   review a real production caller before focused execution. These tests remain
   WP03-owned; they do not close the WP04 adapter route.
3. Implement the WP04 explicit agent-protocol adapter over the leaf crate and
   write its separately owned contract test:
   `crates/agent-protocol/tests/contract/ai_contracts.rs`. WP04 must consume
   the leaf through an explicit wire adapter and must not duplicate WP03's
   leaf/TypeScript tests or schema ownership. Source migration, tests, and
   adapter authorization do not imply READY, proof, or DONE.
4. WP07 general durable AI work lifecycle.
5. WP09 context builder and WP12 prompt registry.
6. WP14-WP17 local text execution, parser, and degraded boundary.
7. WP19 AI result journal/SQLite ingest, then WP18/WP20 integration.
8. WP21-WP23/WP25 memory and graph closure.
9. WP40-WP42 artifact/runtime governance.
10. WP30/WP31/WP38 screen OCR/VLM execution.
11. WP33-WP36 feature bridges, then WP43-WP47 product/security/performance.
12. WP48 rollout only after all Phase 1 gaps are closed.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Apply `workpacks/00-owner-boundary-proof-gate.md` to the selected workpack before source changes.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
