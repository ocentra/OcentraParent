# Protected Capability Custody Foundation Plan Workpack Index

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan Workpack Index
> Kind: workpack selector.
> Proves: local route selection only.
> Does not prove: broker operation, tests, proof, PR readiness, or DONE.

<!-- /agent-capsule -->

Choose exactly one workpack. The source map and graph-native workspace
requirements are deliberately separate: the core/protocol/broker/client/FFI,
private core Windows production roots, and the WP01-owned BIN-only provisioner
preflight are live, reviewed topology at canonical `cbd974291`. The preflight
only revalidates existing enrollment and always fails with
`ExternalProvisioningRequired`; Parent Runtime WP12 package/lifecycle
invocation, production callers, and 13 test roots remain open. A focused
2026-08-26 audit also confirmed that all 21 WP04 client/broker anchor roots are
present and compile on Linux and the Windows target; this is source truth only,
not operational readiness or completion evidence.

The implementation-only repair route is governed by
[ADR-PCC-002](adr/ADR-PCC-002.md). It selects one existing Rust Windows
front-door process with a private `cfg(windows)` safe adapter inside core; it
does not authorize a second helper process, helper protocol, caller-supplied
identity, public proof construction, or fake authority.

| Status | Workpack | Source boundary | Required proof tier | Open condition |
| --- | --- | --- | --- | --- |
| validation / source accepted; runtime and test closure open | [01 Protected Capability Custody Boundary](workpacks/01-protected-capability-custody-foundation.md) | Active fail-closed core, neutral protocol, isolated Windows broker, client, private FFI mechanics, private core Windows adapter, and read-only BIN provisioner preflight at reviewed canonical `a6d7d9adf` (114 implementation files / 0 tests) | P0 security/persistence/platform | External OEM/firmware/MDM `TPM_RH_PLATFORM` + NV lifecycle, authenticated owner handoff, protected registry/SCM mutation, independent current observations, monotonic provider, real transport caller, 13 expected tests, proof, and runtime availability remain absent. |
| blocked / source-order route only; no implementation authorization while external owner is absent | [02 Windows Enrollment Owner Handoff](workpacks/02-windows-enrollment-owner-handoff.md) | Protected Enrollment/SCM/TPM owner transaction and provisioner handoff | P0 security/platform | External OEM/firmware/MDM authority is a prerequisite. The fixed transaction, owner caller, and two expected test roots are absent; no caller-minted enrollment or READY/DONE route is authorized. |
| blocked / implementation route remains separate from operational readiness | [03 Monotonic Anti-Rollback Provider](workpacks/03-monotonic-anti-rollback-provider.md) | Core Windows monotonic provider and platform anti-rollback boundary | P0 security/persistence | WP01 and the WP02 owner transaction remain prerequisites. The TPM NV counter test is absent; disk, SQLite, or caller counters cannot substitute for hardware-backed currentness. |
| planned / bounded source present; normal completion pending operational prerequisites | [04 Client Broker Anchor Transport](workpacks/04-client-broker-anchor-transport.md) | Protected client admission, fixed pipe, OS-derived broker anchor, and bounded Windows process/token/service-SID observations | P0 security/IPC | The 21-root fail-closed source packet is present and its focused host/Windows checks pass; no bounded dependency-legal source gap remains. WP01/WP02/WP03 and Parent WP12 remain hard completion prerequisites, while the three transport tests, real owner-bound caller, operational enrollment/anchor, proof, and DONE remain absent. |
| planned / bounded source-order route; normal completion dependency-gated | [05 Account Issuer Key and Store Custody](workpacks/05-account-issuer-key-and-store-custody.md) | Account-owned TPM-native ECDSA P-256 v2 self-contained inner/outer issuer owner and typed broker boundary | P0 security/cryptography/persistence | Envelope kinds 6/7 are AccountIssuerRequest/AccountIssuerResponse carrying operations IssueCurrentAuthority/AcknowledgeReceipt; Verify is owner-local. Rust schema is crates/schema/src/account_identity_authority_producer_v2.rs; family retains its existing authority DTO and v1 historical parse/verifier path; v2 has no v1 inner signature. Service-specific ACL is runtime-blocked: external provisioning must create/set it, while broker revalidates only; the shared WP04 TokenGroups and LookupAccountNameW service-SID observations are now present, while CNG security-descriptor revalidation remains WP05-owned. ring 0.17.14 is the v2 verifier and sha2 is key-ID hashing only; no p256/ecdsa dependency. Tests, proof, provider/ACL/runtime/binding gaps, and DONE remain open.

WP02 has no implementation authorization until the external OEM/firmware/MDM
owner transaction is available. WP04's bounded source packet is now present;
WP05 remains planned/source-authorable. Their implementation-independent edges
do not alter normal completion, which remains blocked on the operational
predecessors. WP05 does not open a second Account database connection or merge
with `custody.sqlite`: the broker mounts the owner for service lifetime and
retains protected signer custody, family-core retains the Account authority and
single transaction host, and the owner crate receives opaque Account-specific
capabilities. No row above is READY or DONE evidence.

## Ownership and dependency rules

- The core owns typed custody state, binding validation, path security, storage
  replica checks, and fail-closed transitions.
- The neutral protocol package owns the one shared broker/client wire contract;
  neither the broker binary nor the client may duplicate it.
- The graph records each package manifest, required `lib`/`main` target, and
  active workspace membership. The root Cargo workspace now activates the real
  packages; activation remains verified with `cargo metadata --no-deps`, not
  inferred from file presence or comments.
- The core's `broker_admission.rs` facade is the only cross-crate seam. It
  retains sealed owner traits and opaque admission inside the core; its private
  Windows modules construct `BrokerPeerAdmissionObservation` and transcript
  proofs while preserving `pub(crate)` platform guards and `BrokerPlatformOwner`.
  It does not make `CustodyAdmission`, platform guards, or authority
  implementations public.
- The broker source is a separate process boundary and its client consumes only
  typed opaque results. The FFI mechanics, private core adapter, and WP04
  fixed-pipe anchor/peer composition are now present, but successful protected
  admission remains disabled until the TPM policy/non-exportable handle,
  installer authority, and owner-bound runtime caller are available.
- ADR-PCC-002 records the integrated `ocentra-protected-capability-custody-
  windows-ffi` member as the only package allowed to contain raw unsafe
  Win32/TBS/TPM wrappers. Its manifest uses package-local lint tables, not
  `[lints] workspace = true`, and the safe adapter remains private
  `cfg(windows)` core modules. Core is the sole current FFI consumer; the
  WP01-owned BIN-only provisioner is the only other permitted consumer. The
  broker and client continue depending on core/protocol only.
- WP01 owns the separate expected BIN-only provisioner package at its Cargo
  manifest, `src/main.rs`, and private `src/provisioning/` directory. It has no
  library or public API, performs only the fixed installer-owned operation, and
  may not accept caller/MSI-provided path, index, policy, auth, identity, or
  success values. The preflight cannot establish enrollment and leaves startup
  fail-closed. WP12 only invokes/packages this binary and owns its MSI/WiX,
  build, and lifecycle roots; the WP12 package source remains absent.
- The current private core adapter verifies retained pipe/process/token handles, SID,
  integrity, session, image/SCM identity, exact registry owner/protected
  DACL/ACE/ancestor chain, nonce/expiry/replay, and TPM2 NV/TBS monotonic
  generation. The broker retains the pipe stream/handle for the request;
  process/session IDs are re-queried immediately before transcript
  authorization. TPM reset, missing/deleted NV index, or enrollment mismatch
  is fail closed and requires re-pair; disk state cannot restore generation.
- Parent Client Runtime Distribution WP12 owns only the parent-side MSI/WiX
  package, elevated custom-action/provisioner invocation, build/release wiring,
  and upgrade/rollback/uninstall lifecycle. It invokes/packages the WP01-owned
  binary but does not own its Cargo source. It may not expose or accept raw
  `authValue`, TPM index/policy, SID, path, image identity, generation, lease,
  capability, or success input. Protected WP01 remains the sole owner of the
  private core/FFI enrollment and TPM-policy acceptance boundary.
- Account WP05A is the coordinator consumer. Device Trust WP01 and WP03 are
  owner-participant/ceremony consumers. Their graph edges are implementation
  ordering only; their source, tests, proof, and DONE rows remain open.
- Browser WP06 is also a blocked downstream consumer; it cannot treat a
  persisted profile or path record as protected authority.
- No plan may copy the core's private constructors, mint an opaque handle, or
  replace the broker with an in-process lock/DPAPI/file path.
