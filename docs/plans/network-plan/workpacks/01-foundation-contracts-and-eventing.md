# WP01 Foundation Contracts And Eventing

Scope: establish network evidence contracts, Rust protocol parity, evidence grade, policy action capability, and reusable eventing integration.

Source rows: `03-network-implementation-checklist-and-workpacks.md` rows 1-10.

Read next:

- `../01-network-evidence-and-intervention-full-scope-plan.md`
- `../source-index.md`
- `../../eventing-plan/AGENTS.md` only for reusable eventing obligations
- `../TEST_PROOF_EXPECTATIONS.md`

## Ownership boundary

```text
crates/schema or the owning Rust crate owns canonical shared network contracts and generated DTOs. `packages/schema-domain` is temporary generated-validation or edge-decoder surface only where migration is still incomplete.
network-domain is metadata/proof-consumer unless explicit public exports exist.
ocentra-network-evidence owns Rust evidence/proof helper logic.
agent-protocol, agent-core, and agent-service own protocol/runtime/service proof only when selected.
eventing-plan owns reusable local event bus semantics only.
policy-control-plane-plan owns policy decision semantics.
v0-8-enforcement-control-plan owns enforcement authority and action execution.
```

## Expected outcome

- NetworkFlowEvidence, NetworkDomainEvidence, NetworkActivityClassification, NetworkEvidenceGrade, and NetworkPolicyAction boundaries are schema-backed and versioned.
- Rust protocol parity is defined where network contracts cross the child service boundary.
- Network events consume reusable typed eventing instead of inventing `NetworkEventBus`, private dispatch registry, retry queue, or request registry.
- Authority limits are explicit: network evidence can inform policy but cannot directly enforce.

## Required proof fields

The selected proof must name, at minimum:

```text
schema_owner
rust_owner
protocol_owner
eventing_owner
evidence_grade_state
policy_handoff_state
enforcement_authority_state
private_bus_state
schema_fixture_ref
rust_parity_ref
eventing_workpack_ref
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Expected tests/proof

- `network.contract.schema-unit`
- `network.contract.schema-fuzz`
- `network.rust-protocol.parity`
- `network.evidence-grade.boundary-negative`
- `network.eventing.integration-contract`
- Proof includes eventing workpack reference, schema fixture path, and no-private-bus audit.

## Failure conditions

- Do not claim exact content, exact URL, exact video, search text, or private message truth from network-only evidence.
- Do not allow AI/network observations to publish enforcement commands.
- Do not use the giant settings inventory as implementation proof.
- Do not use `network-domain` package metadata as canonical schema ownership proof. Rust-owned schema proof is required for canonical network contract authority.
- Do not use schema proof as Rust parity, eventing, service runtime, or platform proof.

## Current slice note

- The 2026-06-17 `network-foundation-shim-cleanup` slice is limited to the parent-domain TypeScript shim boundary and initial proof-root creation.
- In this slice, `packages/parent-domain/src/network-flow.ts` and `packages/parent-domain/src/network-contracts.ts` can be cleaned up without widening scope.
- The remaining exact contradiction for WP01 is the public `./network-control-catalog` surface, which must be resolved before the control-catalog shim family can be removed honestly.

## Current typed-eventing sub-slice

- `NetworkFlowObservedEvent` now consumes the reusable `ocentra-eventing` `DomainEvent` and `EventEnvelope` boundary directly under the distinct `network.flow.eventing.observed` contract. The existing `network.flow.observed` runtime stream remains reserved for `NetworkRuntimeEventPayload` subscribers. The focused proof records stored-envelope round trip, blank-device-reference rejection, canonical schema enforcement, and a length-prefixed device/flow idempotency key that cannot collapse when hyphenated component boundaries differ.
- Proof routing: `docs/proof/network-plan/01-network-foundation-eventing-contract.md` records this WP01 sub-slice against source rows 1-10's reusable-eventing integration obligation, with `docs/plans/eventing-plan/workpacks/09-network-consumer-event-chain.md` as the reusable-eventing handoff reference. The proof explicitly keeps all untouched WP01 obligations and skipped risk surfaces open.
- This is only a foundation contract handoff. WP01 remains open: it does not prove the remaining schema parity, evidence-grade, policy-action, private-bus audit, service runtime, or platform rows.
