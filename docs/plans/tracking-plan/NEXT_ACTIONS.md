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
3. Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
4. Open that workpack and exact checklist rows only.
5. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Audit-priority next actions

- [WP01 Source Index And Repo Reconciliation](workpacks/01-source-index-and-repo-reconciliation.md): source ownership drift must stay aligned with `packages/tracking-domain`, `crates/tracking-core`, `schema-domain`, and current proof scripts.
- [WP02 Current Tracking Snapshot And Gap Map](workpacks/02-current-tracking-snapshot-and-gap-map.md): reconcile snapshot/PLAN_STATE proof-chain wording before any broad claim.
- [WP33 Proof Gates Fixtures Rollout And PR Gate](workpacks/33-proof-gates-fixtures-rollout-and-pr-gate.md): checked state is false green until proof scripts rerun cleanly or blockers are carried.
- [WP34 Tracking Event Contracts And Protocol Constants](workpacks/34-tracking-event-contracts-and-protocol-constants.md) through [WP39 Tracking Portal Event Read-Model Proof](workpacks/39-tracking-portal-event-read-model-proof.md): these on-disk event workpacks are active scope.
- [WP25 Policy Compiler For Tracking Rules](workpacks/25-policy-compiler-for-tracking-rules.md), [WP27 Escalation Engine](workpacks/27-escalation-engine.md), [WP28 Temporary Live Tracking Mode](workpacks/28-temporary-live-tracking-mode.md), and [WP29 Missing-Device Mode](workpacks/29-missing-device-mode.md): reopen because box closure is ahead of proof truth.

## Central schema first gate

Before editing tracking source for any selected workpack, answer this in the proof note:

```text
Does this shape cross a package, crate, plan, protocol, event, portal, policy, notification, custody, or proof boundary?
If yes: schema-domain or neutral protocol/event/evidence boundary owns it.
If no: tracking-domain/tracking-core may keep it private/internal.
```

## First implementation/proof blockers

- Restore or carry blocker rows for missing upstream proof inputs required by tracking closure and source-reconciliation proof scripts.
- Remove forbidden re-export/barrel patterns in Rust or TS tracking surfaces when the selected workpack touches them.
- Keep `@ocentra-parent/parent-domain` tracking proof dependencies out of authority unless the build/proof path is green or explicitly carried as a blocker.
- Regenerate or block in order: claim audit, product-readiness closure, source reconciliation/gap map.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Classify owner/proof family through `WORKPACK_FAMILIES.md` when unclear.
- [ ] Record canonical schema owner before accepting a cross-boundary tracking shape.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, evidence path, schema-owner state, and no-claim boundaries in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
- [ ] Do not trust prior checked rows until audit-reopened workpacks and proof chain rerun cleanly or blockers are carried.
