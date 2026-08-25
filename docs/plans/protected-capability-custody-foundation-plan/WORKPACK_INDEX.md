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
while the protected Windows adapters, production caller, and test roots remain
expected and missing.

The implementation-only repair route is governed by
[ADR-PCC-002](adr/ADR-PCC-002.md). It selects one existing Rust Windows
front-door process with an in-process safe adapter; it does not authorize a
second helper process, helper protocol, caller-supplied identity, or fake
authority.

| Status | Workpack | Source boundary | Required proof tier | Open condition |
| --- | --- | --- | --- | --- |
| validation / implementation-only repair authorized by graph; normal state remains validation | [01 Protected Capability Custody Boundary](workpacks/01-protected-capability-custody-foundation.md) | Active fail-closed core, neutral protocol, isolated Windows broker, client, narrow core facade, plus planned Windows FFI/adapter manifests and targets | P0 security/persistence/platform | Safe pinned process/token and registry ACL owners, non-restorable monotonic authority, installer/SCM enrollment, a real caller, expected tests, proof, and runtime availability remain absent. |

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
  retains sealed owner traits and opaque admission inside the core; it
  does not make `CustodyAdmission`, platform guards, or authority implementations
  public.
- The broker source is a separate process boundary and its client consumes only
  typed opaque results. Successful protected admission remains disabled until
  the missing OS/installer authority adapters exist.
- ADR-PCC-002 adds two planned, absent workspace members: the tiny
  `ocentra-protected-capability-custody-windows-ffi` raw-wrapper module and the
  safe `ocentra-protected-capability-custody-windows` adapter module. The FFI
  package is the only package allowed to contain its scoped unsafe Win32/TBS/TPM
  wrappers; its manifest must use package-local lint tables, not
  `[lints] workspace = true`, set `unsafe_code = "allow"` and
  `unsafe_op_in_unsafe_fn = "deny"`, and manually mirror every workspace
  Rust/Clippy deny except `unsafe_code`. The safe adapter and broker continue
  inheriting `[lints] workspace = true`; the safe adapter is the only
  in-process Windows seam used by the broker. Their manifests and `lib` targets
  are graph obligations, not source presence.
- The future adapter must verify retained pipe/process/token handles, SID,
  integrity, session, image/SCM identity, exact registry owner/protected
  DACL/ACE/ancestor chain, nonce/expiry/replay, and TPM2 NV/TBS monotonic
  generation. TPM reset, missing/deleted NV index, or enrollment mismatch is
  fail closed and requires re-pair; disk state cannot restore the generation.
- Account WP05A is the coordinator consumer. Device Trust WP01 and WP03 are
  owner-participant/ceremony consumers. Their graph edges are implementation
  ordering only; their source, tests, proof, and DONE rows remain open.
- Browser WP06 is also a blocked downstream consumer; it cannot treat a
  persisted profile or path record as protected authority.
- No plan may copy the core's private constructors, mint an opaque handle, or
  replace the broker with an in-process lock/DPAPI/file path.
