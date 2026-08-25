# Protected Capability Custody Foundation Test and Proof Expectations

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Test and Proof Expectations
> Kind: scoped test/proof selector.
> Proves: required test and proof shape only.
> Does not prove: source or runtime completion before commands and artifacts exist.

<!-- /agent-capsule -->

## Expected test roots

The following roots are obligations for WP01 and are currently absent. The
core-owned roots are unit modules under `src/` because storage, path, and
authority internals are intentionally private. The package-level roots test
only their public boundaries.

Core-owned unit modules:

- `crates/protected-capability-custody-core/src/binding_test.rs`
- `crates/protected-capability-custody-core/src/storage_schema_test.rs`
- `crates/protected-capability-custody-core/src/custody_transition_test.rs`
- `crates/protected-capability-custody-core/src/path_security_test.rs`
- `crates/protected-capability-custody-core/src/custody_reconciliation_test.rs`

Protocol, broker, and client package tests:

- `crates/protected-capability-custody-protocol/tests/wire_contract.rs`
- `crates/protected-capability-custody-broker/tests/authority.rs`
- `crates/protected-capability-custody-broker/tests/reservation_races.rs`
- `crates/protected-capability-custody-broker/tests/windows_broker_custody.rs`
- `crates/protected-capability-custody-client/tests/admission.rs`
- `crates/protected-capability-custody-client/tests/windows_ipc_authentication.rs`

ADR-PCC-002 adds these planned adapter tests. They are absent at the current
baseline and must not be created by the graph/routing packet:

- `crates/ocentra-protected-capability-custody-windows/tests/windows_adapter_custody.rs`
- `crates/ocentra-protected-capability-custody-windows/tests/tpm_nv_counter.rs`

## Required coverage

Tests must exercise real public boundaries and real failure states: malformed or
cross-household bindings, schema/object/index drift, path traversal and replica
tampering, generation/revocation/replay, restart reconciliation, uncertain
prepared state, concurrent reservation races, broker authentication and client
identity mismatch, ACL/path/key ownership, watermark/lease monotonicity, and
Windows process restart. The adapter tests must additionally cover retained
pipe/process/token handles, SID/integrity/session, image+SCM identity, exact
registry owner/protected DACL/ACE/ancestor chain, installer-only enrollment,
nonce/expiry/replay, TBS-backed TPM2 NV generation, TPM reset, missing/deleted
NV index, and required re-pair outcomes. A fixture, mock broker, same-process DPAPI helper,
mutex/file-lock substitute, private-source path import, or caller-provided
attestation is not product proof.

The integration test may be Windows-only. Unsupported platforms must report the
typed manual-required/unavailable state rather than silently substituting an
in-process implementation.

## Focused validation profile

After production source and test source exist, choose the smallest commands that
cover the touched crate and broker/client packages, then run:

```text
cargo check -p ocentra-protected-capability-custody-core
cargo check -p ocentra-protected-capability-custody-protocol
cargo check -p ocentra-protected-capability-custody-broker
cargo check -p ocentra-protected-capability-custody-client
cargo check -p ocentra-protected-capability-custody-windows-ffi
cargo check -p ocentra-protected-capability-custody-windows
cargo test -p ocentra-protected-capability-custody-core --lib
cargo test -p ocentra-protected-capability-custody-protocol --tests
cargo test -p ocentra-protected-capability-custody-broker --tests
cargo test -p ocentra-protected-capability-custody-client --tests
cargo test -p ocentra-protected-capability-custody-windows --tests
npm run lint:architecture -- --files crates/protected-capability-custody-core crates/protected-capability-custody-protocol crates/protected-capability-custody-broker crates/protected-capability-custody-client
npm run hub:guard -- --paths <exact-touched-paths> --operation commit
```

The package commands become runnable only after the real manifests and targets
are added and activated in the workspace. Do not run the repo-wide gate from
this docs-only route.

## Proof requirements

Retain a command log, negative-case evidence, restart/reconciliation evidence,
Windows broker/IPC evidence, no-claim boundaries, and a checklist update under
`output/protected-capability-custody-foundation-plan-proof/01-protected-capability-custody-foundation/`.
Proof is generated after tests and validation; its planned path is not evidence.
