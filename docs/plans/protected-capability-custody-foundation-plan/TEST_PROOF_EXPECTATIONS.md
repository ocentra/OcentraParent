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

ADR-PCC-002 adds these planned core-private adapter tests. They are absent at
the current baseline and must not be created by the graph/routing packet:

- `crates/protected-capability-custody-core/tests/unit/windows_adapter.rs`
- `crates/protected-capability-custody-core/tests/security/tpm_nv_counter.rs`

## Required coverage

Tests must exercise real public boundaries and real failure states: malformed or
cross-household bindings, schema/object/index drift, path traversal and replica
tampering, generation/revocation/replay, restart reconciliation, uncertain
prepared state, concurrent reservation races, broker authentication and client
identity mismatch, ACL/path/key ownership, watermark/lease monotonicity, and
Windows process restart. The core-private adapter tests must additionally cover
retained pipe/process/token handles, SID/integrity/session, image+SCM identity,
exact registry owner/protected DACL/ACE/ancestor chain, installer-only
enrollment, nonce/expiry/replay, TBS-backed TPM2 NV generation, TPM reset,
missing/deleted NV index, immediate pipe-ID re-query, and required re-pair
outcomes. A fixture, mock broker, same-process DPAPI helper,
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
cargo test -p ocentra-protected-capability-custody-core --lib
cargo test -p ocentra-protected-capability-custody-protocol --tests
cargo test -p ocentra-protected-capability-custody-broker --tests
cargo test -p ocentra-protected-capability-custody-client --tests
npm run lint:architecture -- --files crates/protected-capability-custody-core crates/protected-capability-custody-protocol crates/protected-capability-custody-broker crates/protected-capability-custody-client crates/ocentra-protected-capability-custody-windows-ffi
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

## Split workpack expected source — 2026-08-25

All paths below are expected source, not present test evidence. No test files
are created by this routing packet.

### WP02 Windows Enrollment Owner Handoff

Production roots:

```text
crates/protected-capability-custody-core/src/broker_admission/platform/windows.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/enrollment.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/scm.rs
crates/ocentra-protected-capability-custody-provisioner/Cargo.toml
crates/ocentra-protected-capability-custody-provisioner/src/main.rs
crates/ocentra-protected-capability-custody-provisioner/src/provisioning/mod.rs
crates/ocentra-protected-capability-custody-provisioner/src/provisioning/owner_handoff.rs
```

Expected tests:

```text
crates/protected-capability-custody-core/tests/unit/windows_adapter.rs
crates/ocentra-protected-capability-custody-provisioner/tests/integration/owner_handoff.rs
```

The tests must exercise the external owner transaction and fail closed when
OEM/firmware/MDM authority is absent; a setup or MSI assertion is not evidence.

### WP03 Monotonic Anti-Rollback Provider

Production roots:

```text
crates/protected-capability-custody-core/src/broker_admission/platform/windows/monotonic.rs
crates/protected-capability-custody-core/src/platform/anti_rollback.rs
```

Expected test:

```text
crates/protected-capability-custody-core/tests/security/tpm_nv_counter.rs
```

The test must cover reset, missing/deleted NV state, stale generation, and
fail-closed re-pair behavior using the real hardware-backed owner boundary.

### WP04 Client Broker Anchor Transport

Production roots:

```text
crates/protected-capability-custody-core/src/broker_admission/platform/windows/client_anchor.rs
crates/protected-capability-custody-client/src/lib.rs
crates/protected-capability-custody-client/src/windows_ipc.rs
crates/protected-capability-custody-client/src/admission.rs
```

Expected tests:

```text
crates/protected-capability-custody-broker/tests/windows_broker_custody.rs
crates/protected-capability-custody-client/tests/admission.rs
crates/protected-capability-custody-client/tests/windows_ipc_authentication.rs
```

Coverage must retain the fixed pipe and OS-derived server process/token/image,
SCM, and enrollment anchor. `sysinfo` and caller identity are not authority;
source-order review does not make these tests or runtime evidence present. The
route uses the existing interprocess PID/session and RAII owner observations;
it does not add a handshake redesign, spawned child, stdin bootstrap, nonce
pipe, caller PID/SID/path, or `dunce` fallback. Account issuer signing and
store-lease coverage belongs to distinct WP05.

### WP05 Account Issuer Key and Store Custody

Expected roots are the Account-owned `crates/account-issuer-owner` contract,
repository, currentness, key-registry, outbox, delivery, startup, recovery,
signing, and RPC modules; the exact v2 schema, family producer/parser/envelope,
distinct protected protocol/client/broker `account_issuer` contract/RPC
facades and v2 codec, dedicated Windows CNG/PCP P-256 lifecycle, capability,
export, and sign modules; the family Account client facade; Parent Runtime
composition; and the Cloudflare v2 contract/transport plus D1 currentness
migration. The first source packet freezes producer
`ocentra.account-authority-producer.v2`, inner domain
`ocentra.account-authority-producer.signing.v2\0`, audience
`ocentra.account.authority.v2`, algorithm `ecdsa-p256-sha256-p1363`, outer
domain `ocentra.account-issuer.transport.v2\0`, service
`ocentra.account-authority-producer.cloudflare.v2`, algorithm-aware
`sha256:ecdsa-p256:<hex>` key IDs with a v2 derivation domain, canonical
65-byte SEC1 public keys, exact 64-byte low-S P1363 signatures, and authority/
key-generation/enrollment binding before parallel code. The initial protected
protocol envelope message kinds 6 and 7 are AccountIssuerRequest and AccountIssuerResponse carrying the inner operations IssueCurrentAuthority and AcknowledgeReceipt; Verify
stays owner-local. Tests must prove that the broker mounts one
Account owner for service lifetime while family-core retains
`VerifiedAccountIdentityAuthority`, its authority repository/source of truth,
and one opaque `BEGIN IMMEDIATE` transaction/currentness host. The existing
family-owned handoff contract remains a separate historical/input boundary
and is never embedded, re-signed, or duplicated inside P-256 v2; no second
Account connection or `custody.sqlite` merge is allowed.

Service-specific key custody is currently REJECT/runtime-blocked: the existing
key ACL is SYSTEM GenericAll, SCM exposes only SID type, token observation
lacks TokenGroups, and LookupAccountNameW service-SID resolution is absent.
External provisioning must create/set the service-specific ACL; the broker only
opens and revalidates the descriptor and token/service observations. Caller
SDDL/SID and broad SYSTEM/BA grants are forbidden. WP04 owns the shared
service-SID and TokenGroups FFI/core observation roots; WP05 owns CNG
security-descriptor revalidation. Rust v2 verification uses locked ring 0.17.14
ECDSA_P256_SHA256_FIXED after explicit low-S precheck; sha2 is only for key-ID
hashing and no p256/ecdsa dependency is allowed.

P-256 v2 tests must cover the self-contained P-256 inner and outer wire without
wrapping a newly signed Ed25519-v1 inner wire, runtime
`NCryptIsAlgSupported`/`EnumAlgorithms`, a
unique non-exportable signing-only PCP key with service ACL,
`BCRYPT_ECCPUBLIC_BLOB` to canonical 65-byte SEC1, exact low-S 64-byte P1363
`r||s` over SHA-256 of original canonical bytes, FFI low-S canonicalization,
Rust/Cloudflare high-S rejection, algorithm-aware key IDs, schema/family/
protocol/Cloudflare v2 contracts, Account SQLite authoritative key/receipt/
outbox custody, D1 public verifier currentness/CAS plus inbound idempotency
receipt only, and no duplicate D1 outbox. Unsupported TPM/manual enrollment
fail-closed behavior, binding/attestation, rotation, recovery, and
cross-binding lineage remain open. Ed25519 v1 is historical parse/verify only;
no new v1 signing, fallback, software/wrapped fallback, silent downgrade,
generic lifecycle operation bytes, direct SQLite sharing, mock, or
caller-supplied key/path/private material is acceptable.
