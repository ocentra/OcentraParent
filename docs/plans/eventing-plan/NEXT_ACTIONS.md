# Reusable Rust Eventing Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Reusable Rust Eventing Plan Next Actions`
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
3. Open that workpack and exact checklist rows only.
4. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by route dependency

1. [WP01 Source Boundary And Semantics Audit](workpacks/01-source-boundary-and-semantics-audit.md) - required before crate implementation claims.
2. [WP02 Crate Contract And Type Boundary](workpacks/02-crate-contract-and-type-boundary.md) - required before runtime/consumer work.
3. [WP03 Dispatch Runtime And Lifecycle](workpacks/03-dispatch-runtime-and-lifecycle.md) - required before queue/request/journal proof.
4. [WP04 Queue Idempotency Dead Letter](workpacks/04-queue-idempotency-dead-letter.md), [WP05 Request Response Contracts](workpacks/05-request-response-contracts.md), and [WP06 Journal Replay And Lineage](workpacks/06-journal-replay-and-lineage.md) - runtime proof layers.
5. [WP07 Parent Protocol Event Contracts](workpacks/07-parent-protocol-event-contracts.md) through [WP10 LAN Household Mesh Consumer](workpacks/10-lan-household-mesh-consumer.md) - consumer handoffs; do not claim product behavior from eventing-only proof.
6. [WP11 Type Safety And Ownership Hardening](workpacks/11-type-safety-and-ownership-hardening.md) and [WP12 Rollout Proof And PR Gate](workpacks/12-rollout-proof-and-pr-gate.md) - hardening and reporting gates.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
