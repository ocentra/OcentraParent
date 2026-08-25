# WP01 Protected Capability Custody Boundary

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Workpack: `01-protected-capability-custody-foundation`
> Kind: neutral production-source route.
> Proves: the intended boundary and current gaps only.
> Does not prove: implementation, tests, proof, platform readiness, or DONE.

<!-- /agent-capsule -->

## Intent

Provide the one neutral protected-capability custody boundary that other owners
can consume without obtaining or manufacturing protected authority. The
fail-closed core, neutral protocol, broker, client, Windows FFI mechanics, and
private core Windows adapter are reviewed and integrated at canonical
`9375b0e10` from source branch `8df832f2d`. The graph records 99 implementation
files, 0 tests, and no Cargo workspace requirement gaps for the accepted
core/FFI packages; the planned provisioner manifest, workspace member, and BIN
target remain missing. The broker remains deliberately unavailable before state
creation because installer-owned TPM policy/non-exportable handle authority is
still unavailable.

The installer-side enrollment boundary is deliberately split. Parent Client
Runtime Distribution WP12 owns the parent Windows MSI/WiX package, the
installer-only BIN provisioner invocation, build wiring, and
upgrade/rollback/uninstall lifecycle. This workpack owns the private core/FFI
acceptance of the installer-provisioned record, TPM policy, non-exportable
handle validation, and opaque protected outcome. Neither WP12 nor any parent
caller may submit a raw `authValue`, TPM index/policy, SID, path, generation,
lease, capability, or success assertion; the TPM authorization secret stays
behind the approved non-exportable handle and policy.

The accepted source packet is implementation evidence, not operational
completion: focused host/Windows checks and Enforcer/architecture/guard checks
passed, while the installer, caller, 13 expected tests, proof, CI, READY, and
DONE remain open.

## Existing production source

The reviewed live roots are the core manifest and its source files/directories:

```text
crates/protected-capability-custody-core/Cargo.toml
crates/protected-capability-custody-core/src/lib.rs
crates/protected-capability-custody-core/src/authority.rs
crates/protected-capability-custody-core/src/broker_admission.rs
crates/protected-capability-custody-core/src/broker_admission/
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
crates/protected-capability-custody-protocol/Cargo.toml
crates/protected-capability-custody-protocol/src/
crates/protected-capability-custody-broker/Cargo.toml
crates/protected-capability-custody-broker/src/
crates/protected-capability-custody-client/Cargo.toml
crates/protected-capability-custody-client/src/
crates/ocentra-protected-capability-custody-windows-ffi/Cargo.toml
crates/ocentra-protected-capability-custody-windows-ffi/src/lib.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/
```

The 2026-08-25 source packet was independently reviewed on branch
`8df832f2d` and integrated at canonical `9375b0e10`; the graph maps 99
implementation files and 0 tests with no workspace requirement gaps for the
accepted core/FFI packages. The planned provisioner manifest, workspace member,
and BIN target remain missing. The packet adds the isolated-process/package
boundary, one shared bounded wire protocol,
opaque request/result types, startup preflight, dynamic enrolled-client pipe
ACL construction, split immutable-enrollment/runtime registry custody, and a
narrow core-owned broker entry plus the raw Windows FFI and private core adapter
modules. Internal authority constructors remain private.
The service fails with `DeploymentRequired` before opening storage, registry,
journal, listener, bootstrap, or service-ready state when protected admission is
unavailable because the required TPM policy/non-exportable handle authority is
not available. SQLite remains a checked replica; it cannot become the authority
merely because it is durable. Focused host/Windows checks and Enforcer,
architecture, source-shape, and guard checks passed; tests, proof, CI, PR,
READY, and DONE remain open.

## Required source boundary

The source topology now contains the separate Windows broker, client, neutral
`protected-capability-custody-protocol`, and Windows FFI packages as active
Cargo workspace members, with private `cfg(windows)` adapter modules inside the
core package. This is source presence, not operating custody. The remaining
production boundary is the installer-owned TPM policy and
non-exportable handle, the WP01-owned BIN-only provisioner source and WP12
package invocation/lifecycle, and a real enrolled caller. The expected
provisioner source roots are:

```text
crates/ocentra-protected-capability-custody-provisioner/Cargo.toml
crates/ocentra-protected-capability-custody-provisioner/src/main.rs
crates/ocentra-protected-capability-custody-provisioner/src/provisioning/
```

Until those owners exist, startup must remain unavailable and must not create
or mutate custody state.

The core now exposes only the narrow, explicitly reviewed
`src/broker_admission.rs` facade seam. That facade accepts broker-owned inputs
and returns typed opaque outcomes, while retaining `CustodyAdmission`,
`CurrentBindingPort`, `PlatformCustodyOwner`, and `PlatformDatabaseGuard` behind
the sealed/core-private boundary. The broker package cannot implement those
traits from outside the core or receive a caller-mintable constructor.

The broker must be a separate OS process. Same-process DPAPI, an in-process
broker, mutex/file-lock custody, caller attestation, caller-selected key or
capability, fake success, and no-op adapters are forbidden. The client may
receive only typed opaque results bound to the authenticated owner, target,
action, generation, and broker state.

## ADR-PCC-002 implementation-only repair route

The accepted source repair is one Windows front-door process: the existing
`ocentra-protected-capability-custody-broker` continues depending on core and
protocol, while core depends on one tiny FFI package and owns the safe adapter
privately. It does not add a helper process, a second protocol, or a public
adapter crate. The external seam remains one broker dispatch/open-session
path; the broker derives identity from authenticated Windows peer observation,
not from caller fields. This source route was reviewed on `8df832f2d` and
integrated at canonical `9375b0e10`; the graph records 99 implementation files,
0 tests, and no workspace requirement gaps for the accepted core/FFI packages.
The planned provisioner manifest, workspace member, and BIN target remain
missing.

The integrated FFI package is limited to raw Win32/TBS/TPM calls and safe
owned-handle RAII wrappers. Its manifest uses package-local lint tables, not
`[lints] workspace = true`, set `unsafe_code = "allow"` and
`unsafe_op_in_unsafe_fn = "deny"`, and manually mirror every workspace
Rust/Clippy deny except `unsafe_code`. The safe adapter is private core code;
core and broker continue inheriting `[lints] workspace = true`. Only the core
and the exact WP01-owned BIN-only provisioner package may depend on this FFI
crate; broker, client, and all other consumers remain prohibited from depending
on it. The
private modules preserve construction of `BrokerPeerAdmissionObservation` and
`BrokerAuthorizedClientTranscript`, the `pub(crate)` sealed platform
traits/guards, `BrokerPlatformOwner`, and the existing broker-facing runtime
methods. No raw handle, identity constructor, attestation, key selector, or
capability minting crosses to a caller.

Installer-only immutable enrollment must pre-provision the broker image/SCM
identity, enrolled client SID/image, protected registry owner and DACL/ACE
ancestor chain, and TPM2 NV counter/index through an elevated owner/MDM/OEM
ceremony. The broker retains the pipe stream/handle for the request lifetime;
core retains process/token/image observations and their handles. Pipe
process/session IDs are re-queried immediately before transcript authorization.
The private adapter must retain and revalidate SID/integrity/session, image+SCM
identity, exact registry custody, nonce/expiry/replay, and TPM2 NV/TBS
monotonic generation before session or custody state is emitted. TPM reset,
missing/deleted NV index, TBS failure, or enrollment mismatch fails closed and
requires re-pair; disk, JSON, SQLite, and rollback state cannot restore the
generation.

The source implementation order is complete through the raw owned-handle/TBS/
TPM wrappers, private core enrollment/peer/SCM/monotonic modules, and existing
core runtime seam. The remaining order is the WP01-owned BIN-only provisioner
source and installer/SCM provisioning, WP12 package invocation/lifecycle, and
then a real production caller. This route remains in validation; tests, proof,
PR, merge, READY, and DONE remain open.

## Expected test source

The complete 13-test wave is intentionally deferred until the source packet is
stable. Internal core behavior must use core-owned unit-test modules under
`src/` so private storage/path/authority state is tested without making those
constructors public. The two planned Windows tests are:

```text
crates/protected-capability-custody-core/src/broker_admission/platform/windows_adapter_test.rs
crates/protected-capability-custody-core/src/broker_admission/platform/tpm_nv_counter_test.rs
```

Public protocol tests belong to the protocol package; broker process/race/
Windows custody tests belong to the broker package; and client admission/
IPC-authentication tests belong to the client package. No test may import
private source with path tricks, use a mock/in-process broker, or create a
dependency cycle merely to reach private state.

## Consumers and unlocks

| Consumer | Graph relationship | Boundary |
| --- | --- | --- |
| Account WP05A Runtime Effect Fencing Coordinator | implementation dependency | Consumes opaque owner reservation outcomes; does not own broker authority or SQLite custody. |
| Device Trust WP01 Device Trust Source of Truth | implementation dependency | Consumes neutral protected custody for owner-participant persistence; retains device identity/currentness and signer authority. |
| Device Trust WP03 Parent Step-Up Auth | implementation dependency | Consumes neutral protected custody for ceremony/participant handoff; retains parent/device authority and ceremony semantics. |
| Browser WP06 Managed Profile Store | blocked implementation dependency | May consume only future authenticated opaque outcomes; it cannot use persisted JSON/path state as protected authority. |

These are downstream source-order relationships only. They do not make any
consumer runtime-ready and do not remove their existing Account, Device Trust,
Cloudflare, platform, caller, test, or proof blockers.

## Acceptance gates

Keep WP01 open until the integrated source is joined to installer-owned TPM
policy/non-exportable handle enrollment, WP12 provisioning, and a real
production caller; all 13 expected test roots are written and run; the Windows
process/IPC and owner-bound custody negative cases are retained;
Enforcer/architecture checks pass; and proof/checklist state is current.
Independently reviewed source consolidation does not change normal READY,
PR_READY, CI, merge, or DONE state.
