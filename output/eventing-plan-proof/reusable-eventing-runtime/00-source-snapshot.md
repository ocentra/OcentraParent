# Reusable Eventing Runtime Source Snapshot

branch: codex/network-policy-preview-stored-flow-evidence-on-row10k
head: 987da398c1c98c68bcaeb69862d899657e24de91
origin/main: 9b2a08e0d21cbf66ae3f6276bf8c7adb56b8268c
merge-base: 466978a9b8f6ac946977169fd2e16e01b4cf6d92

## Status

```text
 M crates/agent-core/src/lib.rs
 M crates/agent-core/src/network_event_runtime.rs
 M crates/agent-core/src/network_event_runtime/remote_delivery_transport_dispatch_state.rs
 M crates/agent-protocol/README.md
 M crates/agent-protocol/src/network_flow.rs
 M crates/agent-protocol/src/network_flow_tests.rs
 M crates/agent-service/README.md
 M crates/agent-service/src/network_remote_delivery_status_payload.rs
 M crates/agent-service/src/network_remote_delivery_status_service_tests.rs
 M docs/features/network-domain-control.md
 M docs/plans/network-plan/implementation-checklist.md
 M docs/plans/network-plan/workpacks/README.md
 M output/eventing-plan-proof/18-24-handler-policy/eventing-clippy.log
 M output/eventing-plan-proof/18-24-handler-policy/eventing-tests.log
 M output/eventing-plan-proof/18-24-handler-policy/proof-summary.json
 M output/eventing-plan-proof/18-24-handler-policy/source-shape.log
 M output/eventing-plan-proof/25-30-queue-policy/eventing-clippy.log
 M output/eventing-plan-proof/25-30-queue-policy/eventing-tests.log
 M output/eventing-plan-proof/25-30-queue-policy/proof-summary.json
 M output/eventing-plan-proof/25-30-queue-policy/source-shape.log
 M output/eventing-plan-proof/31-35-request-response/eventing-clippy.log
 M output/eventing-plan-proof/31-35-request-response/eventing-tests.log
 M output/eventing-plan-proof/31-35-request-response/proof-summary.json
 M output/eventing-plan-proof/31-35-request-response/source-shape.log
 M output/eventing-plan-proof/36-41-journal-replay/eventing-clippy.log
 M output/eventing-plan-proof/36-41-journal-replay/eventing-tests.log
 M output/eventing-plan-proof/36-41-journal-replay/proof-summary.json
 M output/eventing-plan-proof/36-41-journal-replay/source-shape.log
 M output/eventing-plan-proof/42-44-parent-child-protocol-contracts/proof-summary.json
 M output/eventing-plan-proof/45-50-network-protocol-contracts/proof-summary.json
 M output/eventing-plan-proof/51-54-parent-child-runtime/proof-summary.json
 M output/eventing-plan-proof/55-56-enforcement-journal-action/proof-summary.json
 M output/eventing-plan-proof/57-network-workpack-10-reusable-crate/proof-summary.json
 M output/eventing-plan-proof/58-network-event-chain-exact-refs/proof-summary.json
 M output/eventing-plan-proof/59-weak-network-evidence-command-routing/proof-summary.json
 M output/eventing-plan-proof/60-61-command-boundary/proof-summary.json
 M output/eventing-plan-proof/62-network-proof-links/proof-summary.json
 M output/eventing-plan-proof/63-delivery-decision-proof/proof-summary.json
 M output/eventing-plan-proof/67-lock-await/eventing-tests.log
 M output/eventing-plan-proof/67-lock-await/proof-summary.json
 M output/eventing-plan-proof/67-lock-await/source-shape.log
 M output/eventing-plan-proof/68-fixture-parity/eventing-clippy.log
 M output/eventing-plan-proof/68-fixture-parity/proof-summary.json
 M output/eventing-plan-proof/68-fixture-parity/rust-fixture-parity-tests.log
 M output/eventing-plan-proof/68-fixture-parity/source-shape.log
 M output/eventing-plan-proof/69-compatibility-matrix/compatibility-matrix-example.log
 M output/eventing-plan-proof/69-compatibility-matrix/compatibility-matrix-tests.log
 M output/eventing-plan-proof/69-compatibility-matrix/eventing-clippy.log
 M output/eventing-plan-proof/69-compatibility-matrix/proof-summary.json
 M output/eventing-plan-proof/69-compatibility-matrix/source-shape.log
 M output/eventing-plan-proof/71-manual-clock/eventing-clippy.log
 M output/eventing-plan-proof/71-manual-clock/eventing-tests.log
 M output/eventing-plan-proof/71-manual-clock/proof-summary.json
 M output/eventing-plan-proof/71-manual-clock/source-shape.log
 M output/eventing-plan-proof/72-contract-registry/contract-registry-docs-example.log
 M output/eventing-plan-proof/72-contract-registry/contract-registry-tests.log
 M output/eventing-plan-proof/72-contract-registry/eventing-clippy.log
 M output/eventing-plan-proof/72-contract-registry/proof-summary.json
 M output/eventing-plan-proof/72-contract-registry/source-shape.log
 M output/eventing-plan-proof/73-duplicate-subscriber/duplicate-subscriber-test.log
 M output/eventing-plan-proof/73-duplicate-subscriber/eventing-clippy.log
 M output/eventing-plan-proof/73-duplicate-subscriber/proof-summary.json
 M output/eventing-plan-proof/73-duplicate-subscriber/source-shape.log
 M output/eventing-plan-proof/74-lifecycle-clear/eventing-clippy.log
 M output/eventing-plan-proof/74-lifecycle-clear/lifecycle-clear-tests.log
 M output/eventing-plan-proof/74-lifecycle-clear/proof-summary.json
 M output/eventing-plan-proof/74-lifecycle-clear/source-shape.log
 M output/eventing-plan-proof/74-production-shutdown/eventing-clippy.log
 M output/eventing-plan-proof/74-production-shutdown/production-shutdown-tests.log
 M output/eventing-plan-proof/74-production-shutdown/proof-summary.json
 M output/eventing-plan-proof/74-production-shutdown/source-shape.log
 M output/eventing-plan-proof/75-family-variants/eventing-clippy.log
 M output/eventing-plan-proof/75-family-variants/family-variant-tests.log
 M output/eventing-plan-proof/75-family-variants/proof-summary.json
 M output/eventing-plan-proof/75-family-variants/source-shape.log
 M output/eventing-plan-proof/delivery-semantics/eventing-clippy.log
 M output/eventing-plan-proof/delivery-semantics/eventing-delivery-tests.log
 M output/eventing-plan-proof/delivery-semantics/proof-summary.json
 M output/eventing-plan-proof/delivery-semantics/source-shape.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-command-boundary-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-enforcement-journal-action-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-network-protocol-contract-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-network-runtime-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-parent-child-protocol-contract-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-parent-child-runtime-proof.log
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/00-source-snapshot.md
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/agent-protocol-domain-remote-delivery-status-test.log
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/agent-protocol-remote-delivery-status-test.log
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/agent-service-remote-delivery-status-test.log
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/expected-remote-delivery-outbox-status-bridge.json
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/proof-summary.json
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/source-shape.log
 M output/network-plan-proof/10k-remote-delivery-transport-dispatch-state/12-validation-commands.log
 M output/network-plan-proof/10k-remote-delivery-transport-dispatch-state/expected-remote-delivery-transport-dispatch-state.json
 M output/network-plan-proof/10k-remote-delivery-transport-dispatch-state/proof-summary.json
 M output/network-plan-proof/10k-remote-delivery-transport-dispatch-state/source-shape.log
 M output/network-plan-proof/policy-preview-stored-flow-evidence/proof-summary.json
 M output/network-plan-proof/policy-preview-stored-flow-evidence/source-shape.log
 M packages/agent-protocol-domain/README.md
 M packages/agent-protocol-domain/src/defaults.ts
 M packages/agent-protocol-domain/src/network-remote-delivery-status.ts
 M packages/agent-protocol-domain/tests/network-remote-delivery-status.test.ts
 M scripts/test/network-policy-preview-stored-flow-evidence-proof.mjs
 M scripts/test/network-remote-delivery-outbox-status-bridge-proof.mjs
 M scripts/test/network-remote-delivery-transport-dispatch-state-proof.mjs
 M test-results/eventing-network-runtime-proof/proof.json
 M test-results/network-policy-preview-stored-flow-evidence-proof/proof.json
 M test-results/network-remote-delivery-outbox-status-bridge-proof/proof.json
 M test-results/network-remote-delivery-transport-dispatch-state-proof/proof.json
```

## Inspected Paths

- crates/ocentra-eventing
- docs/plans/eventing-plan
- output/eventing-plan-proof

## Phase Boundary

This proof pack validates the reusable Rust event bus/runtime only.
Network, parent/child runtime, portal, service, broker, relay-hub, policy, AI, enforcement, and platform-adapter proofs are consumer phases layered on top of this crate.
