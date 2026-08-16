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

- [09 Network Consumer Event Chain](workpacks/09-network-consumer-event-chain.md)
  is the single legal READY code packet. Implement only the reviewed
  agent-core/agent-service ingestion-time publish, deterministic identity,
  durable network-journal, startup-recovery, and read-side-effect removal
  boundary; tests, proof, CI, review, and
  merge remain later gates.
- [10 LAN Household Mesh Consumer](workpacks/10-lan-household-mesh-consumer.md)
  is open because the expected local proof roots remain absent and the
  LAN/remote-access consumer handoff still needs exact verification.
- WP06 is locally proved at
  `docs/proof/eventing-plan/`: the typed
  `00-enforcement-wp11-handoff.md`, journal/replay proof, topology/lineage
  proof, and compact validation log are retained. This is a generic Eventing
  prerequisite only; enforcement retains authority/action/rollback proof.
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

## WP09 production-route boundary

- Contracts in `agent-protocol` are not production event-chain proof.
- The current capture path does not publish once at ingestion into a durable
  network consumer chain; the service read API republishes rows through an
  in-memory `OnceCell`/`EventBus::new` spine, and phase subscribers are no-op
  routing surfaces.
- `TEST_*` phase refs and test-created journal paths do not establish a
  production network journal or startup recovery before readiness.
- WP09 is READY for implementation only. Do not mark it done, claim live
  capture/enforcement, or use its future code slice to unblock Network WP04
  until tests, retained proof, and the consumer handoff exist. AI, policy,
  enforcement, audit, and portal remain downstream blocked/fail-closed; nested
  fixture/prove/`TEST_*` runtime files are not shipped production topology.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [x] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [x] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [x] Record the rollout-proof reconciliation and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
