# Protected Capability Custody Foundation Plan Workpack Index

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan Workpack Index
> Kind: workpack selector.
> Proves: local route selection only.
> Does not prove: broker operation, tests, proof, PR readiness, or DONE.

<!-- /agent-capsule -->

Choose exactly one workpack. The source map and graph-native workspace
requirements are deliberately
separate: the core/protocol/broker/client production roots are live topology,
while the one Windows FFI crate, private core Windows modules, installer-side
Parent Runtime WP12 package, production caller, and test roots remain expected
and missing.

The implementation-only repair route is governed by
[ADR-PCC-002](adr/ADR-PCC-002.md). It selects one existing Rust Windows
front-door process with a private `cfg(windows)` safe adapter inside core; it
does not authorize a second helper process, helper protocol, caller-supplied
identity, public proof construction, or fake authority.

| Status | Workpack | Source boundary | Required proof tier | Open condition |
| --- | --- | --- | --- | --- |
| validation / implementation-only repair authorized by graph; normal state remains validation | [01 Protected Capability Custody Boundary](workpacks/01-protected-capability-custody-foundation.md) | Active fail-closed core, neutral protocol, isolated Windows broker, client, narrow core facade, one planned Windows FFI manifest/lib, and private core Windows module targets | P0 security/persistence/platform | Safe pinned process/token and registry ACL owners, non-restorable monotonic authority, installer/SCM enrollment, a real caller, expected tests, proof, and runtime availability remain absent. |

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
  typed opaque results. Successful protected admission remains disabled until
  the missing OS/installer authority adapters exist.
- ADR-PCC-002 adds one planned, absent workspace member:
  `ocentra-protected-capability-custody-windows-ffi`, with only a manifest and
  `lib` target. It is the only package allowed to contain raw unsafe Win32/TBS/
  TPM wrappers; its manifest must use package-local lint tables, not
  `[lints] workspace = true`, set `unsafe_code = "allow"` and
  `unsafe_op_in_unsafe_fn = "deny"`, and manually mirror every workspace
  Rust/Clippy deny except `unsafe_code`. The safe adapter is private
  `cfg(windows)` core modules, and core plus broker continue inheriting
  `[lints] workspace = true`. Core depends on the FFI package; the broker
  continues depending on core and protocol. All planned roots are routing, not
  source presence.
- The future core adapter must verify retained pipe/process/token handles, SID,
  integrity, session, image/SCM identity, exact registry owner/protected
  DACL/ACE/ancestor chain, nonce/expiry/replay, and TPM2 NV/TBS monotonic
  generation. The broker retains the pipe stream/handle for the request;
  process/session IDs are re-queried immediately before transcript
  authorization. TPM reset, missing/deleted NV index, or enrollment mismatch
  is fail closed and requires re-pair; disk state cannot restore generation.
- Parent Client Runtime Distribution WP12 owns only the parent-side MSI/WiX
  package, elevated custom-action/provisioner invocation, build/release wiring,
  and upgrade/rollback/uninstall lifecycle. It may not expose or accept raw
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
