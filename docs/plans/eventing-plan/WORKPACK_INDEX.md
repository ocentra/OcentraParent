# Reusable Rust Eventing Plan Workpack Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Reusable Rust Eventing Plan Workpack Index`
> Kind: workpack selector; use before opening any workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Choose one row, then open only that workpack plus the exact checklist/proof/test rows it names.
> Proves: local eventing workpack routing only.
> Does not prove: implementation correctness, product integration, cross-device delivery, PR readiness, or broad DONE.
> Proof rule: If this index changes status, update `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `PLAN_HEALTH.md`, and any affected checklist/proof rows.

<!-- /agent-capsule -->

Use this file to select exactly one eventing slice. Do not read all eventing plan docs, the full checklist, or all workpacks.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not use it as permission to scan multiple workpacks.

Source truth for detailed historical scope is [05-implementation-workpacks.md](05-implementation-workpacks.md). The files below are token-efficient execution routes split from that source; they do not replace proof.

Status key: `done` means this checkout contains focused local proof for the slice. `historical` means source/tests are present but the cited proof bundle is absent in this checkout. `open` means current route work still remains.

| Status | Workpack                                                                                      | Source rows  | Expected proof tier            | Open condition                                                                                      |
| ------ | --------------------------------------------------------------------------------------------- | ------------ | ------------------------------ | --------------------------------------------------------------------------------------------------- |
| historical | [01 Source Boundary And Semantics Audit](workpacks/01-source-boundary-and-semantics-audit.md) | 1-5          | P0_DOCS_PLAN                   | Historical docs-only slice; source semantics are documented, but current closure still depends on the missing route proof bundle.             |
| historical | [02 Crate Contract And Type Boundary](workpacks/02-crate-contract-and-type-boundary.md)       | 6-10         | P1_GENERIC_CRATE_CONTRACT      | The reusable crate exists in source, but the historical proof bundle cited by this plan is absent in this checkout.                          |
| historical | [03 Dispatch Runtime And Lifecycle](workpacks/03-dispatch-runtime-and-lifecycle.md)           | 11-24        | P2_GENERIC_CRATE_RUNTIME       | Dispatch/runtime source and focused tests exist, but the route proof pack is absent in this checkout.                                       |
| historical | [04 Queue Idempotency Dead Letter](workpacks/04-queue-idempotency-dead-letter.md)             | 25-30        | P2_GENERIC_CRATE_RUNTIME       | Queue/dead-letter behaviors are exercised by focused crate tests, but the cited proof artifacts are absent in this checkout.                |
| historical | [05 Request Response Contracts](workpacks/05-request-response-contracts.md)                   | 31-35        | P2_GENERIC_CRATE_RUNTIME       | Request/response behavior is exercised in focused crate tests, but the cited proof artifacts are absent in this checkout.                   |
| done | [06 Journal Replay And Lineage](workpacks/06-journal-replay-and-lineage.md)                         | 36-41, 69-78 | P3_GENERIC_JOURNAL_REPLAY      | Proof complete: the hand-authored WP06 manifest under `docs/proof/eventing-plan/` retains the typed WP11 handoff, journal/replay proof, topology/lineage proof, and compact validation log. This proves generic local mechanics only; enforcement action/authority and WP10 remain outside the row. |
| historical | [07 Parent Protocol Event Contracts](workpacks/07-parent-protocol-event-contracts.md)         | 42-50        | P4_PARENT_PROTOCOL_INTEGRATION | Focused Rust/TS protocol contract tests pass, but the cited parent/network protocol proof bundle is absent in this checkout.                |
| blocked — fail-closed ingress source integrated; functional owners missing | [08 Parent Runtime Integration](workpacks/08-parent-runtime-integration.md) | 51-56 | P5_PARENT_RUNTIME_INTEGRATION | The service accepts only a typed intent marker and returns rejected/manual-required with every authority, journal, publish, event-id, and child-transport claim unclaimed. Functional parent-runtime composition is blocked on Account WP03, Tracking WP40, Policy WP03/WP04/WP08, Enforcement WP11, and Child Runtime WP10; the planned runtime owner and two expected test roots are absent. |
| Phase 1/tests/local proof written; integration open | [09 Network Consumer Event Chain](workpacks/09-network-consumer-event-chain.md) | 57-62 | P6_NETWORK_CONSUMER_READY | Exact-source ingestion, deterministic phase identity/idempotency, recovered ProductionFile journaling before listener readiness, startup/recurring reconciliation, fail-closed persisted-row validation, projection-only reads, and AI/portal direct-command authority negatives are written. Focused tests, local ignored proof, architecture/Enforcer, normal pre-commit, accepted commits, and push pass. Whole-plan integration, CI, review, and merge remain open; no downstream consumer execution or Network WP04 readiness is claimed. |
| open — code drafted; tests/validation/proof deferred | [10 LAN Household Mesh Consumer](workpacks/10-lan-household-mesh-consumer.md)                 | 79-87        | P6_NETWORK_CONSUMER_READY      | Runtime authorization is fail-closed and unavailable pending LAN WP26's real ingress, durable custody, selected non-revoked route composition, and private authority handoff. |
| validation — source/test source integrated; execution/proof open | [11 Type Safety And Ownership Hardening](workpacks/11-type-safety-and-ownership-hardening.md) | 63-68        | P2/P3 hardening                | Through `fa1230661` and canonical test-source commit `ac5d41322`, live/stored decode revalidates identity, request completion is associated-response typed, unsupported journal idempotency fails closed, action replay consumes a journal-minted non-cloneable authority, and the routed test sources are integrated. The three target harnesses compile with `--no-run`; actual test execution, retained proof, checklist rows 63-68, and completion review remain open. |
| blocked — missing harness/root and prerequisite acceptance | [12 Rollout Proof And PR Gate](workpacks/12-rollout-proof-and-pr-gate.md)                     | Main gates   | route gate                     | `scripts/test/eventing-rollout-proof.mjs` and canonical proof root `output/eventing-plan-proof/12-rollout-proof-and-pr-gate/` are absent; WP09 integration acceptance, WP10 authority/consumer handoff, and WP13 validation/proof remain incomplete. |
| validation — code complete; validation/proof open | [13 Test Folder Layout Regression Audit](workpacks/13-test-folder-layout-regression-audit.md) | fresh audit  | P2_GENERIC_CRATE_RUNTIME       | Moved external test layout is present, but fresh validation/proof is absent; the `contract` Cargo harness is required. |

## Selection Rules

- If the task names a numbered row from `05-implementation-workpacks.md`, choose the workpack whose source-row range contains it.
- If the task is consumer-specific, choose the consumer eventing workpack only long enough to identify eventing obligations, then route to the owning consumer plan for product behavior.
- If the selected workpack owner/proof family is unclear, classify it through `WORKPACK_FAMILIES.md`; do not scan every family.
- If no workpack owns the task, update this index before implementation claims.
- If a selected workpack expects a crate/test folder that does not exist, record the missing location and keep the workpack open.
- Do not use crate-local proof to claim consumer transport or product behavior.
- Do not use WP12 or WP13 proof to close WP10.
