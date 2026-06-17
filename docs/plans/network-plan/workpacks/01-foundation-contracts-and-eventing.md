# WP01 Foundation Contracts And Eventing

Scope: establish network evidence contracts, Rust protocol parity, evidence grade, policy action capability, and reusable eventing integration.

Source rows: `03-network-implementation-checklist-and-workpacks.md` rows 1-10.

Read next:

- `../01-network-evidence-and-intervention-full-scope-plan.md`
- `../source-index.md`
- `../../eventing-plan/AGENTS.md` only for reusable eventing obligations
- `../TEST_PROOF_EXPECTATIONS.md`

Expected outcome:

- NetworkFlowEvidence, NetworkDomainEvidence, NetworkActivityClassification, NetworkEvidenceGrade, and NetworkPolicyAction boundaries are schema-backed and versioned.
- Rust protocol parity is defined where network contracts cross the child service boundary.
- Network events consume reusable typed eventing instead of inventing `NetworkEventBus`, private dispatch registry, retry queue, or request registry.
- Authority limits are explicit: network evidence can inform policy but cannot directly enforce.

Expected tests/proof:

- `network.contract.schema-unit`
- `network.contract.schema-fuzz`
- `network.rust-protocol.parity`
- `network.evidence-grade.boundary-negative`
- `network.eventing.integration-contract`
- Proof includes eventing workpack reference, schema fixture path, and no-private-bus audit.

Failure conditions:

- Do not claim exact content, exact URL, exact video, or private message truth from network-only evidence.
- Do not allow AI/network observations to publish enforcement commands.
- Do not use the giant settings inventory as implementation proof.

## Current slice note

- The 2026-06-17 `network-foundation-shim-cleanup` slice is limited to the parent-domain TypeScript shim boundary and initial proof-root creation.
- In this slice, `packages/parent-domain/src/network-flow.ts` and `packages/parent-domain/src/network-contracts.ts` can be cleaned up without widening scope.
- The remaining exact contradiction for WP01 is the public `./network-control-catalog` surface, which must be resolved before the control-catalog shim family can be removed honestly.
