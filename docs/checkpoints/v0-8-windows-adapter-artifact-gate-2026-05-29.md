<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 Windows Adapter Artifact Gate
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.8 Windows Adapter Artifact Gate

Date: 2026-05-29

## Scope

This checkpoint covers the non-visual V0.8 Windows adapter artifact gate. The slice adds Rust protocol and service read-model proof that capability/readiness rows cannot upgrade app, domain/network, managed-browser, unmanaged-browser, unsupported OS, rollback, or audit claims unless the required host artifact references are present.

## Product Truth

- App target upgrades require same-identity app or package evidence, apply result, rollback result, and audit custody event references.
- Domain and network upgrades require filter apply result, filter rollback result, and audit custody event references.
- Managed-browser exact URL upgrades require managed exact URL evidence and audit custody event references.
- Unmanaged-browser support remains process-only and cannot upgrade into exact URL control from this gate.
- Unsupported OS targets refuse Windows adapter claim upgrades.
- Even complete artifact references only make a target ready for manual review; they do not set product-ready broad blocking.

## Evidence

- Protocol: `crates/agent-protocol/src/windows_adapter_artifact_gate.rs`
- Protocol tests: `crates/agent-protocol/src/windows_adapter_artifact_gate_tests.rs`
- Service read model: `crates/agent-service/src/windows_adapter_artifact_gate_read_model.rs`
- Service tests: `crates/agent-service/src/windows_adapter_artifact_gate_read_model_tests.rs`
- Proof harness: `scripts/test/v0-8-windows-adapter-artifact-gate.mjs`
- Proof artifact: `test-results/v0-8-windows-adapter-artifact-gate/proof.json`

## Known Gaps

- This is a CI-mechanical protocol/service gate, not a real Windows host adapter run.
- No broad app blocking, network/domain blocking, managed exact URL enforcement, unmanaged browser exact URL evidence, admin hardening, anti-tamper, rollback enforcement, Android child enforcement, or iOS child enforcement is claimed here.
- Local validation intentionally omits browser, Playwright, full `npm run validate`, package previews, Android device-owner proof, and iOS entitlement proof per the no-focus-stealing local validation policy.
