# Protected Capability Custody Foundation Test and Proof Expectations

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Test and Proof Expectations
> Kind: scoped test/proof selector.
> Proves: required test and proof shape only.
> Does not prove: source or runtime completion before commands and artifacts exist.

<!-- /agent-capsule -->

## Expected test roots

The following roots are obligations for WP01 and are currently absent:

- `crates/protected-capability-custody-core/tests/unit/binding.rs`
- `crates/protected-capability-custody-core/tests/unit/storage_schema.rs`
- `crates/protected-capability-custody-core/tests/unit/transition_state.rs`
- `crates/protected-capability-custody-core/tests/security/path_and_replica_integrity.rs`
- `crates/protected-capability-custody-core/tests/recovery/custody_reconciliation.rs`
- `crates/protected-capability-custody-core/tests/concurrency/broker_reservation_races.rs`
- `crates/protected-capability-custody-core/tests/integration/windows_broker_custody.rs`

## Required coverage

Tests must exercise real public boundaries and real failure states: malformed or
cross-household bindings, schema/object/index drift, path traversal and replica
tampering, generation/revocation/replay, restart reconciliation, uncertain
prepared state, concurrent reservation races, broker authentication and client
identity mismatch, ACL/path/key ownership, watermark/lease monotonicity, and
Windows process restart. A fixture, mock broker, same-process DPAPI helper,
mutex/file-lock substitute, or caller-provided attestation is not product proof.

The integration test may be Windows-only. Unsupported platforms must report the
typed manual-required/unavailable state rather than silently substituting an
in-process implementation.

## Focused validation profile

After production source and test source exist, choose the smallest commands that
cover the touched crate and broker/client packages, then run:

```text
cargo check -p ocentra-protected-capability-custody-core
cargo test -p ocentra-protected-capability-custody-core --tests
npm run lint:architecture -- --files crates/protected-capability-custody-core
npm run hub:guard -- --paths <exact-touched-paths> --operation commit
```

The exact broker/client package commands are selected when those crates exist.
Do not run the repo-wide gate from this docs-only route.

## Proof requirements

Retain a command log, negative-case evidence, restart/reconciliation evidence,
Windows broker/IPC evidence, no-claim boundaries, and a checklist update under
`output/protected-capability-custody-foundation-plan-proof/01-protected-capability-custody-foundation/`.
Proof is generated after tests and validation; its planned path is not evidence.
