# 01 Source Index And Doc Reconciliation

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `01 Source Index And Doc Reconciliation`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Existing feature, expectation, architecture, schema, capability, inventory, AI, custody, and policy docs are source-indexed.

## Current State

Top-level source index exists in this plan. It still needs reconciliation whenever implementation changes status or proof.

## Checklist

- [ ] Compare plan against `docs/features/screen-evidence-analysis.md`.
- [ ] Compare plan against `docs/features/screen-visibility-live-view.md`.
- [ ] Compare plan against `docs/expectations/screen-evidence.md`.
- [ ] Compare plan against `docs/architecture/local-screen-evidence-analysis-queue.md`.
- [ ] Reconcile schema proposal/capability guide/settings inventory.
- [ ] Update `docs/product-capability-checklist.md` when status/proof changes.

## Proof

- Updated `docs/plans/screen-plan/source-index.md`.
- `git diff -- docs/plans/screen-plan docs/features docs/product-capability-checklist.md`.
