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
`crates/network-core`, `crates/agent-protocol`, and `crates/agent-core` are the selected Rust domain, protocol, and runtime owners for this workpack.
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
- Do not introduce a TypeScript-owned network schema or a private network event bus. Rust-owned contract proof and `ocentra-eventing` integration are required.
- Do not use schema proof as Rust parity, eventing, service runtime, or platform proof.

## Current slice note

- The 2026-06-17 `network-foundation-shim-cleanup` slice is limited to the parent-domain TypeScript shim boundary and initial proof-root creation.
- In this slice, `packages/parent-domain/src/network-flow.ts` and `packages/parent-domain/src/network-contracts.ts` can be cleaned up without widening scope.
- The historical `./network-control-catalog` parent-domain contradiction is resolved: the unpublished compatibility surface and shim family were retired, while the selected `network-domain` subpaths remain canonical for that boundary.
- This does not close the broader WP01 outcome: Rust contract/eventing validation and the workpack's capture, platform, and enforcement no-claim boundaries remain independently required.
