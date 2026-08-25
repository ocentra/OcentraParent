# Protected Capability Custody Foundation Plan State

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan State
> Kind: current state and open gaps.
> Proves: only the current protected-custody route and its stated boundaries.
> Does not prove: implementation, tests, proof, runtime reachability, or release.

<!-- /agent-capsule -->

## Current status

## Source-consolidation checkpoint — 2026-08-25

The fail-closed four-package source packet is integrated through
`1b46b5935` (source branch `origin/codex/protected-capability-custody-repair-round3-aug24`
at `3d8231e796`). Core, protocol, broker, and client source are present, but
safe pinned Windows process/token observation, protected registry owner/DACL
verification, installer/SCM enrollment, and a real production caller remain
missing. All 11 expected core/protocol/broker/client test roots are absent.
The broker therefore remains unavailable before custody state creation. No
tests, proof, CI, PR, READY, or DONE claim changes here.

The plan is an active neutral foundation route with one workpack in
validation. Independently reviewed production source now includes the core,
neutral protocol, isolated broker process, client, and narrow broker-admission
facade as active Cargo workspace members. It is still not an operating custody
system:

- admission/open/platform authority is sealed inside the crate;
- the SQLite state is a checked replica, not the authority for protected
  capability custody;
- the separate broker/client/protocol boundary and bounded wire source exist,
  but successful peer admission remains unavailable before state creation;
- no safe pinned Windows process/token observation, exact protected registry
  owner/DACL/parent-chain verifier, non-restorable monotonic provider, immutable
  broker/SCM anchor, installer/SCM enrollment, or production caller exists;
- the expected unit, security, recovery, concurrency, and Windows integration
  tests are not present;
- no current proof, pre-commit, CI, PR, or merge claim exists for this route.

The graph records the route as `validation`. Its graph-owned workspace
requirements now observe the root manifest, each package manifest, required
`lib`/`bin` targets, and active Cargo workspace membership; `cargo metadata
--no-deps` remains authoritative. Source presence and focused compilation do
not derive ordinary READY or DONE while the protected adapters, tests, caller,
proof, and release gates remain open.

## ADR-PCC-002 routing checkpoint — 2026-08-25

ADR-PCC-002 selects one Rust Windows front-door process: the existing protected
broker continues to depend on core and protocol, while core depends on one tiny
Windows FFI crate. The safe adapter is private `cfg(windows)` core modules;
there is no second helper process, helper protocol, or public adapter crate. The
graph now records these absent planned production roots and workspace
obligations:

```text
crates/ocentra-protected-capability-custody-windows-ffi/Cargo.toml
crates/ocentra-protected-capability-custody-windows-ffi/src/lib.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/enrollment.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/peer.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/scm.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/monotonic.rs
```

The FFI package is limited to raw Win32/TBS/TPM calls and safe owned-handle RAII
wrappers. Its manifest must use package-local lint tables, not
`[lints] workspace = true`, set `unsafe_code = "allow"` and
`unsafe_op_in_unsafe_fn = "deny"`, and manually mirror every workspace
Rust/Clippy deny except `unsafe_code`. Core and broker continue inheriting
`[lints] workspace = true`. The private core modules preserve construction of
`BrokerPeerAdmissionObservation` and `BrokerAuthorizedClientTranscript`, the
`pub(crate)` sealed platform traits/guards, `BrokerPlatformOwner`, and the
existing broker-facing runtime methods. Installer-only immutable enrollment
must pin broker/client image+SCM identity, exact protected registry
owner/DACL/ACE/ancestor chain, and a TPM2 NV counter/index reached via TBS.
The broker retains the pipe stream/handle for the request lifetime; core retains
process/token/image observations, and pipe process/session IDs are re-queried
immediately before transcript authorization. TPM reset, missing/deleted NV
index, TBS failure, or enrollment mismatch fails closed and requires re-pair;
disk state never restores the generation.

This is graph implementation-phase routing only. If the graph derives
implementation authorization, it authorizes only the missing production repair
roots; the normal WP01 lifecycle remains `validation`, not READY or DONE. The
current broker/client stubs remain blocked until the replacement exists. The
11 existing expected test roots remain absent, with these core-private
adapter/TPM test expectations recorded as absent planned tests only:

```text
crates/protected-capability-custody-core/src/broker_admission/platform/windows_adapter_test.rs
crates/protected-capability-custody-core/src/broker_admission/platform/tpm_nv_counter_test.rs
```

Account WP05A, Device Trust WP01/WP03, and Browser WP06 remain blocked
downstream consumers.

## Owning boundary

The neutral owner is the protected-custody plan and its future broker/client
surface. Account, Device Trust, Data Custody, Cloudflare, policy, and provider
owners may consume typed opaque results; none may mint authority, open the
broker, select a key, or treat a SQLite row as authoritative.

The existing production roots are mapped in `docs/engineering-graph/code-map.json`:

```text
crates/protected-capability-custody-core/Cargo.toml
crates/protected-capability-custody-core/src/lib.rs
crates/protected-capability-custody-core/src/authority.rs
crates/protected-capability-custody-core/src/binding.rs
crates/protected-capability-custody-core/src/binding/
crates/protected-capability-custody-core/src/custody.rs
crates/protected-capability-custody-core/src/custody/
crates/protected-capability-custody-core/src/path_security.rs
crates/protected-capability-custody-core/src/path_security/
crates/protected-capability-custody-core/src/platform.rs
crates/protected-capability-custody-core/src/platform/
crates/protected-capability-custody-core/src/storage.rs
crates/protected-capability-custody-core/src/storage/
```

The accepted source packet maps and activates this concrete package topology:

```text
crates/protected-capability-custody-core/src/broker_admission.rs
crates/protected-capability-custody-protocol/Cargo.toml
crates/protected-capability-custody-protocol/src/lib.rs
crates/protected-capability-custody-broker/Cargo.toml
crates/protected-capability-custody-broker/src/lib.rs
crates/protected-capability-custody-broker/src/main.rs
crates/protected-capability-custody-broker/src/windows_ipc.rs
crates/protected-capability-custody-broker/src/authority.rs
crates/protected-capability-custody-broker/src/custody.rs
crates/protected-capability-custody-client/Cargo.toml
crates/protected-capability-custody-client/src/lib.rs
crates/protected-capability-custody-client/src/windows_ipc.rs
crates/protected-capability-custody-client/src/admission.rs
```

The planned implementation roots are one FFI manifest/lib and the private
`cfg(windows)` core module tree named in the ADR. No second adapter manifest,
package, public target, or workspace obligation is present.

`broker_admission.rs` is a narrow core-owned facade seam: it may expose only
typed broker-entry/request/result operations after authenticated process/IPC
validation. Its private Windows modules construct the opaque peer observation
and transcript while retaining the `pub(crate)` sealed traits/guards and
`BrokerPlatformOwner`; they must not expose `CustodyAdmission`,
`CurrentBindingPort`, `PlatformCustodyOwner`, `PlatformDatabaseGuard`, or any
constructor that lets a caller implement or mint those sealed owner traits.
The neutral protocol crate owns the wire contract consumed by both broker and
client; the broker binary does not own a client-visible copy of that contract.

## Consumer routing

The reviewed graph records Account WP05A and Device Trust WP01/WP03 as
downstream consumers of this neutral boundary. These are source-order/unlock
relationships only. They do not transfer ownership, create a caller, or close
their existing authority and platform gaps. No edge points from this neutral
plan back to Account or Device Trust, so the route does not create a dependency
cycle. Browser WP06 is likewise downstream and remains blocked on this owner;
its persisted profile/path state cannot become protected authority.

## Exit conditions

The workpack can leave validation only after the real FFI crate, private core
Windows adapter modules, installer/SCM enrollment, and a production caller
exist; focused source and
boundary validation are green; all expected tests are written and executed;
the broker owns the protected operation; restart/recovery and concurrent
reservations are covered; and retained proof/checklist/merge evidence is
current. Until then, keep the state open and report the exact missing adapter
or caller.
