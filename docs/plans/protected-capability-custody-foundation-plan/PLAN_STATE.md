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

The independently reviewed Protected WP01 source packet is integrated. The
reviewed source branch was `8df832f2d`; canonical merge commit
`9375b0e10` records the integrated FFI/private-core packet. The current graph
topology records 99 implementation files and 0 tests, with no Cargo workspace
requirement gaps for the accepted core/FFI packages. The planned provisioner
manifest, workspace member, and BIN target remain missing (`cargo metadata
--no-deps` remains authoritative). Focused host and Windows checks,
architecture/source-shape and Enforcer guards, and
the lane/hub guards passed for this source packet. This is source acceptance
evidence only: no tests, proof, pre-commit, CI, PR, READY, or DONE claim is
made here.

The plan remains an active neutral foundation route with one workpack in
`validation`. The FFI mechanics and private core Windows adapter source are
now present, but the system is still not operating protected custody:

- admission/open/platform authority remains sealed inside the core;
- SQLite remains a checked replica, not protected authority;
- the separate broker/client/protocol boundary and private FFI/core adapter
  source exist, while runtime admission remains unavailable;
- startup returns `DeploymentRequired` before DB/state/listener mutation when
  the required TPM policy and non-exportable handle authority are unavailable;
- installer/provisioner enrollment and a real production caller remain open;
- all 13 expected test roots remain absent, as do retained proof and release
  evidence.

The graph records the route as `validation`. Source acceptance and focused
compilation do not derive ordinary READY or DONE while the TPM policy/handle,
installer, caller, test, proof, and release gates remain open.

## Installer-side ownership checkpoint — 2026-08-25

Parent Client Runtime Distribution WP12
(`12-protected-broker-provisioner-package`) is now the routed owner for the
parent-side Windows MSI/WiX package, elevated custom-action/provisioner
invocation, build/release wiring, and upgrade/rollback/uninstall contract. Its
expected package roots are `scripts/release/windows/parent-protected-custody/`,
`scripts/release/windows/parent-protected-custody.wxs`, and
`scripts/release/windows/build-parent-protected-custody-package.ps1`. WP12
invokes and packages the fixed installer-owned provisioner binary but does not
own its source. The BIN-only provisioner package, its Cargo manifest, `src/main.rs`,
and private `src/provisioning/` implementation directory are Protected WP01
source roots recorded in the graph. Generated
MSI/checksum/signing outputs belong under `target/release-packages/` and are not
source truth.

Protected WP01 remains the sole owner of the private core/FFI Windows adapter,
enrollment provenance, registry/SCM/peer authority, TPM policy and
non-exportable-handle validation, opaque admission/transcript proofs, and the
fixed BIN-only provisioner source boundary. The parent package may invoke an
approved elevated provisioner but may not expose
or accept a raw TPM `authValue`, TPM index/policy, SID, path, image identity,
generation, lease, capability, or success flag from MSI properties, command
line, setup, or a production caller. This is routing only: the package roots,
installer ceremony, real caller, expected tests, proof, CI, PR, READY, and DONE
remain absent/open, and the 24-plan program count is unchanged while this plan
gains one additional workpack.

## ADR-PCC-002 routing checkpoint — 2026-08-25

ADR-PCC-002 selects one Rust Windows front-door process: the existing protected
broker continues to depend on core and protocol, while core depends on one tiny
Windows FFI crate. The safe adapter is private `cfg(windows)` core modules;
there is no second helper process, helper protocol, or public adapter crate.
The FFI/private-core source roots below are integrated at `9375b0e10` and are
reviewed source truth, not merely absent planned roots:

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
implementation authorization, it authorizes only the remaining production
roots; the normal WP01 lifecycle remains `validation`, not READY or DONE. The
broker/client/FFI/private-core source is present, but runtime remains blocked
until the TPM policy/non-exportable handle and installer/caller boundaries are
operational. All 13 expected test roots remain absent, with these core-private
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

The integrated implementation roots are one FFI manifest/lib, the private
`cfg(windows)` core module tree named in the ADR, and the expected WP01-owned
BIN-only provisioner manifest, `main` target, and private provisioning
directory. No second adapter manifest or public target is present. Parent
Runtime WP12 owns only the package invocation/lifecycle roots and consumes the
WP01-owned binary.

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

The workpack can leave validation only after the integrated source is backed by
the required TPM policy/non-exportable handle authority, installer/SCM
provisioning, and a production caller; focused source and boundary validation
are green; all 13 expected tests are written and executed;
the broker owns the protected operation; restart/recovery and concurrent
reservations are covered; and retained proof/checklist/merge evidence is
current. Until then, keep the state open and report the exact missing adapter
or caller.
