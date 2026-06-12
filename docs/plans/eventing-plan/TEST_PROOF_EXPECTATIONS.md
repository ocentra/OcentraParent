# Reusable Rust Eventing Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Reusable Rust Eventing Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this after the eventing task/checklist row is known. Eventing proof must stay about reusable local bus semantics unless a consumer plan explicitly owns transport/product behavior.

## Where tests should live

When the eventing crate/package test tree exists, tests belong under that crate's tests and proof output under its proof folder. Consumer-specific tests belong in the consumer plan implementation crate/package.

## Decision Tree

| If the assigned work is...           | Read next                                    | Expected tests or proof                                                                                              |
| ------------------------------------ | -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| Envelope/type contract               | `CHECKLIST_INDEX.md`, source-boundary flow   | schema/serialization tests, version skew, invalid envelope negatives.                                                |
| Idempotency, TTL, retry, dead-letter | exact checklist row                          | replay/idempotency, TTL expiry boundary, retry storm, dead-letter ordering proof.                                    |
| Aggregate ordering/request-response  | exact checklist row                          | ordering invariants, duplicate prevention, cancellation, timeout, partial response proof.                            |
| Journal/replay                       | exact checklist row                          | append/replay, corruption handling, migration/rollback if format changes.                                            |
| External transport boundary          | exact checklist row and owning consumer plan | route-decision proof only unless consumer implements live transport; no cross-device product claim from crate alone. |
| Consumer integration                 | consumer plan `AGENTS.md`                    | local bus publish/consume proof plus consumer-specific auth/policy/UI tests.                                         |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `eventing.envelope.version-skew`: envelope decode handles old/new/unknown versions without silent corruption.
- `eventing.malformed.reject-safe`: malformed events fail closed with safe diagnostics.
- `eventing.idempotency.replay-duplicate`: duplicate and replayed events do not duplicate downstream state.
- `eventing.ttl.expiry-boundary`: TTL and clock-skew boundaries drop expired work predictably.
- `eventing.ordering.aggregate`: aggregate ordering, cancellation, timeout, and partial response behavior is deterministic.
- `eventing.retry.dead-letter`: retry storms are bounded and failed events reach dead-letter proof.
- `eventing.journal.recover-corruption`: journal replay handles corruption, migration, and rollback boundaries.
- `eventing.consumer.no-product-claim`: reusable crate proof does not claim consumer/product behavior.

## Required proof contents

- Command logs from Rust tests/checks.
- Negative cases for duplicate, expired, malformed, and out-of-order events.
- Explicit statement when a product claim is outside reusable crate scope.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
