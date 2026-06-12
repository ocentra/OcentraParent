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

Source truth for detailed historical scope is [05-implementation-workpacks.md](05-implementation-workpacks.md). The files below are token-efficient execution routes split from that source; they do not replace proof.

| Status | Workpack                                                                                      | Source rows  | Expected proof tier            | Open condition                                                                                      |
| ------ | --------------------------------------------------------------------------------------------- | ------------ | ------------------------------ | --------------------------------------------------------------------------------------------------- |
| open   | [01 Source Boundary And Semantics Audit](workpacks/01-source-boundary-and-semantics-audit.md) | 1-5          | P0_DOCS_PLAN                   | Until source semantics, crate boundary, and workspace decision record are proved.                   |
| open   | [02 Crate Contract And Type Boundary](workpacks/02-crate-contract-and-type-boundary.md)       | 6-10         | P1_GENERIC_CRATE_CONTRACT      | Until type grammar, IDs, envelopes, custody, and serialization/version proof exist.                 |
| open   | [03 Dispatch Runtime And Lifecycle](workpacks/03-dispatch-runtime-and-lifecycle.md)           | 11-24        | P2_GENERIC_CRATE_RUNTIME       | Until dispatch, registrar, timeout, panic, metrics, and lifecycle proof exists.                     |
| open   | [04 Queue Idempotency Dead Letter](workpacks/04-queue-idempotency-dead-letter.md)             | 25-30        | P2_GENERIC_CRATE_RUNTIME       | Until queue capacity, TTL, retry, duplicate, idempotency, and dead-letter proof exists.             |
| open   | [05 Request Response Contracts](workpacks/05-request-response-contracts.md)                   | 31-35        | P2_GENERIC_CRATE_RUNTIME       | Until typed request/response, timeout, late response, and double-completion proof exists.           |
| open   | [06 Journal Replay And Lineage](workpacks/06-journal-replay-and-lineage.md)                   | 36-41, 69-78 | P3_GENERIC_JOURNAL_REPLAY      | Until append/replay, lineage compatibility, topology, lifecycle, and no-global proof exists.        |
| open   | [07 Parent Protocol Event Contracts](workpacks/07-parent-protocol-event-contracts.md)         | 42-50        | P4_PARENT_PROTOCOL_INTEGRATION | Until parent, child, network, AI, policy, enforcement, audit, and portal event contracts are typed. |
| open   | [08 Parent Runtime Integration](workpacks/08-parent-runtime-integration.md)                   | 51-56        | P5_PARENT_RUNTIME_INTEGRATION  | Until parent/child runtime handoff, journal-before-action, audit, and read-model proof exists.      |
| open   | [09 Network Consumer Event Chain](workpacks/09-network-consumer-event-chain.md)               | 57-62        | P6_NETWORK_CONSUMER_READY      | Until network to AI to policy to enforcement event authority is proved in consumer plan.            |
| open   | [10 LAN Household Mesh Consumer](workpacks/10-lan-household-mesh-consumer.md)                 | 79-87        | P6_NETWORK_CONSUMER_READY      | Until mesh transport validates selected event import/export without remote direct publish.          |
| open   | [11 Type Safety And Ownership Hardening](workpacks/11-type-safety-and-ownership-hardening.md) | 63-68        | P2/P3 hardening                | Until mutation, await/lock, associated response, brand parity, and source gates pass.               |
| open   | [12 Rollout Proof And PR Gate](workpacks/12-rollout-proof-and-pr-gate.md)                     | Main gates   | route gate                     | Until checklist rows, proof artifacts, feature docs, and PR/DONE evidence are reconciled.           |

## Selection Rules

- If the task names a numbered row from `05-implementation-workpacks.md`, choose the workpack whose source-row range contains it.
- If the task is consumer-specific, choose the consumer eventing workpack only long enough to identify eventing obligations, then route to the owning consumer plan for product behavior.
- If no workpack owns the task, update this index before implementation claims.
- If a selected workpack expects a crate/test folder that does not exist, record the missing location and keep the workpack open.
