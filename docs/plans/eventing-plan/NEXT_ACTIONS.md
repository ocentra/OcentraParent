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

- [06 Journal Replay And Lineage](workpacks/06-journal-replay-and-lineage.md)
  is reopened first for the enforcement prerequisite. Retain the P3 journal
  proof and `00-enforcement-wp11-handoff.md` under its own proof root; until
  both exist, enforcement WP11 and WP04 remain blocked/manual-required.
- [10 LAN Household Mesh Consumer](workpacks/10-lan-household-mesh-consumer.md)
  is open because the expected local proof roots remain absent and the
  LAN/remote-access consumer handoff still needs exact verification.
- WP11 is now locally proved: the scoped proof roots remain present, package-wide
  `npm run type-check --workspace @ocentra-parent/agent-protocol-domain`
  passes again, focused `policy-control-audit-redaction.test.ts`,
  `policy-control-delivery-read-model.test.ts`, and `contracts.test.ts` pass,
  and the touched-file `lint:architecture` gate is green.
- WP12 is now locally proved at
  `output/eventing-plan-proof/rollout-proof/proof-summary.json`,
  `test-results/eventing-rollout-proof/proof.json`, and
  `docs/proof/eventing-plan/`.
- WP13 is now locally proved at
  `output/eventing-plan-proof/13-test-folder-layout-regression-audit/proof-summary.json`
  and `test-results/eventing-test-folder-layout-regression-audit/proof.json`.
- Next slice: verify the owning LAN/remote-access proof handoff for WP10 before
  changing its local status.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [x] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [x] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [x] Record the rollout-proof reconciliation and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
