# V0.8 Windows Adapter Artifact Ingestion Proof

Date: 2026-05-29

## Scope

This checkpoint covers the non-visual V0.8 Windows adapter artifact ingestion and custody proof. The slice adds Rust protocol and service read-model coverage for accepting or rejecting adapter artifact records before they can feed the artifact gate.

## Product Truth

- Artifact records must include a non-empty artifact id, target subject, artifact subject, and audit custody event id.
- Artifact records must match the target subject and use an artifact kind valid for the requested surface.
- App, domain/network, managed-browser, and rollback/audit artifacts can feed the gate only after ingestion accepts their custody and subject relationship.
- Unmanaged-browser exact URL and unsupported OS artifacts remain refused; this proof does not upgrade those surfaces.
- Accepted artifacts can make a gate entry ready for manual review, but `claimUpgradeAllowed` stays false.

## Evidence

- Protocol: `crates/agent-protocol/src/windows_adapter_artifact_ingestion.rs`
- Protocol tests: `crates/agent-protocol/src/windows_adapter_artifact_ingestion_tests.rs`
- Service read model: `crates/agent-service/src/windows_adapter_artifact_ingestion_read_model.rs`
- Service tests: `crates/agent-service/src/windows_adapter_artifact_ingestion_read_model_tests.rs`
- Proof harness: `scripts/test/v0-8-windows-adapter-artifact-ingestion-proof.mjs`
- Proof artifact: `test-results/v0-8-windows-adapter-artifact-ingestion-proof/proof.json`

## Known Gaps

- This is a protocol/service ingestion proof, not a real Windows host adapter run.
- No broad app blocking, network/domain blocking, managed exact URL enforcement, unmanaged browser exact URL evidence, privileged admin hardening, anti-tamper, bypass resistance, or real OS apply/rollback is claimed here.
- Local validation intentionally omits browser, Playwright, full `npm run validate`, package previews, Android device-owner proof, and iOS entitlement proof per the no-focus-stealing local validation policy.
