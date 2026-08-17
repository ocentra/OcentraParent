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
  is the active bounded production packet. The reviewed agent-core/agent-service
  ingestion-time publish, deterministic identity, durable network journal,
  startup recovery, fail-closed reconciliation, projection-only reads, and
  expected focused tests are implemented and locally green. AI and portal
  direct-enforcement mutation negatives are now real code/tests. Local ignored
  evidence exists at
  `output/eventing-plan-proof/09-network-consumer-event-chain/`; normal
  pre-commit and accepted commits are pushed, while whole-plan integration,
  CI, review, and merge remain open.
- [10 LAN Household Mesh Consumer](workpacks/10-lan-household-mesh-consumer.md)
  is open because the expected local proof roots remain absent and the
  LAN/remote-access consumer handoff still needs exact verification.
- WP06 is locally proved at
  `docs/proof/eventing-plan/`: the typed
  `00-enforcement-wp11-handoff.md`, journal/replay proof, topology/lineage
  proof, and compact validation log are retained. This is a generic Eventing
  prerequisite only; enforcement retains authority/action/rollback proof.
- WP11 is implementation-ready but open: the live/stored envelope boundary is
  not fully constrained, stored decode lacks aggregate/idempotency revalidation,
  and the negative/audit proof roots are absent.
- WP12 is open because `scripts/test/eventing-rollout-proof.mjs` and
  `output/eventing-plan-proof/12-rollout-proof-and-pr-gate/` are absent.
- WP13 is code-complete for the moved test layout, but current validation/proof
  is open and must include `cargo test -p ocentra-eventing --test contract`.
- Next slice: route LAN WP26 authority/transport composition for WP10, then
  regenerate the canonical WP10 proof root before changing its status.

## WP09 production-route boundary

- Contracts in `agent-protocol` are not production event-chain proof.
- The current production diff now captures the exact source observation at
  ingestion, publishes deterministic phase IDs through the network runtime,
  and persists through a `ProductionFileEventJournal` recovered before the
  listener. Startup and recurring reconciliation retry retained observations;
  read and stream APIs consume projection only and do not republish.
- `TEST_*` phase refs and test-created journal paths remain proof/test material;
  they do not establish acceptance. The production spine registers no phase
  subscribers or handlers and emits only `FlowObserved`, `DomainObserved` when
  present, and `ActivityClassified`. Downstream AI, policy, enforcement, audit,
  and portal consumers remain blocked/fail-closed; no completion event is
  synthesized for them.
- Queue/drain and local request-response helpers have no shipped network
  consumer caller. Do not wire them solely to satisfy an obsolete proof row.
- WP09 Phase 1 production code and expected focused tests are written. Focused
  Eventing, protocol, core, ActivityStore, service, parent-runtime, and portal
  tests plus changed-file architecture/Enforcer gates pass locally. The local
  proof manifest is regenerable but ignored. Do not mark the workpack done,
  claim live capture/enforcement, or use it to unblock Network WP04 until the
  final integration gates land; nested fixture/prove/`TEST_*` runtime files are
  not shipped production topology.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [x] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [x] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [x] Record the rollout-proof reconciliation and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
