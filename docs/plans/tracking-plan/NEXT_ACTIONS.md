# Tracking Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Next Actions`
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

## Audit-priority next actions

- [WP01 Source Index And Repo Reconciliation](workpacks/01-source-index-and-repo-reconciliation.md): source ownership drift must stay aligned with `packages/tracking-domain`, `crates/tracking-core`, and the current proof scripts.
- [WP02 Current Tracking Snapshot And Gap Map](workpacks/02-current-tracking-snapshot-and-gap-map.md): rerun only after the closure artifact is regenerated from a restored pre-device/runtime/service/mobile proof chain.
- [WP33 Proof Gates Fixtures Rollout And PR Gate](workpacks/33-proof-gates-fixtures-rollout-and-pr-gate.md): checked state is false green until proof scripts rerun cleanly.
- [WP34 Tracking Event Contracts And Protocol Constants](workpacks/34-tracking-event-contracts-and-protocol-constants.md) through [WP39 Tracking Portal Event Read-Model Proof](workpacks/39-tracking-portal-event-read-model-proof.md): these on-disk workpacks must be treated as active scope, not omitted backlog.
- [WP25 Policy Compiler For Tracking Rules](workpacks/25-policy-compiler-for-tracking-rules.md), [WP27 Escalation Engine](workpacks/27-escalation-engine.md), [WP28 Temporary Live Tracking Mode](workpacks/28-temporary-live-tracking-mode.md), and [WP29 Missing-Device Mode](workpacks/29-missing-device-mode.md): reopen because box closure is ahead of proof truth.

## First implementation/proof blockers

- Restore the missing pre-device/runtime/service/mobile proof inputs required by `tracking-plan-pre-device-proof.mjs` and `tracking-product-readiness-closure-proof.mjs`.
- Remove forbidden Rust public re-exports in `crates/tracking-core/src/lib.rs` and `crates/agent-protocol/src/tracking/mod.rs`.
- Restore `@ocentra-parent/parent-domain` build health before treating the remaining parent-domain-based tracking proof scripts as authoritative.
- Regenerate `tracking-plan-pre-device-proof`, then `tracking-product-readiness-closure-proof`, then `tracking-source-reconciliation-gap-map-proof`.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
- [ ] Do not trust prior checked rows until the audit-reopened workpacks and proof chain rerun cleanly.
- [ ] Do not treat `tracking-claim-audit-proof.mjs` as the active blocker anymore; it now reruns cleanly from `tracking-domain` and the blocker has moved to missing upstream proof artifacts.
