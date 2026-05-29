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
