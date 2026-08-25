# ADR-PCC-002: One Windows Front-Door Custody Module

## Status

Accepted for implementation-only routing on 2026-08-25. The CNG/TPM,
FFI/private-core, and read-only provisioner source shape was independently
reviewed and integrated at canonical `a6d7d9adf`; this ADR still does not claim
installer enrollment, runtime availability, tests, proof, READY, or DONE.

## Context

The canonical implementation baseline is `a6d7d9adf`. The protected-custody
core, neutral wire package, broker process, client, Windows FFI mechanics,
private core Windows adapter, and BIN-only provisioner preflight are real Rust
source. The broker still fails closed before custody state creation, and the
preflight always returns `ExternalProvisioningRequired`, because external
installer/platform authority and owner handoff are not available. The graph
records 114 implementation files, 0 tests, and no workspace requirement gaps
for the mapped packages. Focused source/compile and
architecture/Enforcer/guard checks passed.

The actual core API already owns the protected construction boundary. Its
`BrokerPeerAdmissionObservation` and `BrokerAuthorizedClientTranscript` have
private fields and private construction; `platform::sealed` exposes only
`pub(crate)` traits and guards; and `BrokerPlatformOwner` is implemented beside
those guards. The repair must preserve those facts and the existing
`BrokerCustodyRuntime` broker-facing methods. A separate safe adapter crate
would need a public authority-construction seam and would risk either a core /
broker cycle or public authority minting.

## Decision

Keep one Rust Windows front-door process: the existing
`ocentra-protected-capability-custody-broker`, with the existing custody core
and protocol dependency shape. The integrated packet adds exactly one tiny
package-local-unsafe crate,
`ocentra-protected-capability-custody-windows-ffi`, as a dependency of core.
It contains only raw Win32/TBS/TPM calls and safe RAII wrappers for owned pipe,
process, token, TBS-context, and TPM-NV handles. It contains no custody
decisions, enrollment policy, persistence authority, transcript construction,
or caller interface. Its `src/lib.rs` may organize those wrappers into private
raw-wrapper modules, but every exported value remains a safe owned RAII value.

The safe adapter is not another crate. It is a private `cfg(windows)` module
tree inside the existing core platform boundary:

```text
crates/protected-capability-custody-core/src/broker_admission/platform/windows.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/enrollment.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/peer.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/scm.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/monotonic.rs
crates/ocentra-protected-capability-custody-provisioner/Cargo.toml
crates/ocentra-protected-capability-custody-provisioner/src/main.rs
crates/ocentra-protected-capability-custody-provisioner/src/provisioning/
```

Those private modules consume only the FFI RAII wrappers, construct the private
`BrokerPeerAdmissionObservation` and
`BrokerAuthorizedClientTranscript`, implement the `pub(crate)` sealed platform
traits/guards beside `BrokerPlatformOwner`, and retain the existing
`BrokerCustodyRuntime` methods. No public constructor, raw handle, identity
assertion, or authority minting crosses the broker boundary.

The FFI manifest must define package-local Rust and Clippy lint tables; it must
not inherit `[lints] workspace = true`. It sets `unsafe_code = "allow"` and
`unsafe_op_in_unsafe_fn = "deny"`, then manually mirrors every workspace
Rust/Clippy deny except `unsafe_code`. The core and broker continue inheriting
`[lints] workspace = true`; the safe Windows modules therefore receive the
workspace deny policy through core. Only the exact core package and the
WP01-owned BIN-only provisioner package may depend on this FFI crate. The
broker, client, and all other consumers must not depend on it.

The external seam remains one dispatch/open-session path. The broker retains
the named-pipe stream/handle for the whole request lifetime. Core's private
Windows peer module retains the process/token/image observations and their
owned handles. Pipe process/session IDs are re-queried immediately before
transcript authorization and must still match the retained observations; no
caller-supplied identity or attestation is accepted.

## Module depth and seam

The FFI crate is intentionally shallow: it hides unsafe ABI calls and handle
lifetime mechanics behind owned RAII values. The private core Windows module
tree is the deep module: its small internal seams concentrate observation,
enrollment, SCM identity, registry custody, TPM checks, monotonic validation,
and error mapping where the core can construct its private proofs. The broker
retains the request transport and calls the existing runtime methods; it does
not become a second authority. This preserves verification locality and
avoids both a dependency cycle and a public authority-construction seam.

## Enrollment and authority

Enrollment is installer-only and immutable at runtime. A pre-provisioned,
elevated owner/MDM/OEM ceremony must establish and pin the broker image and SCM
identity, the enrolled client SID/image, the protected registry root and exact
ancestor chain, and the TPM2 NV counter/index. Runtime code may verify those
records but may not self-enroll, widen them, or treat a user-writable registry
or disk record as authority. The parent-side MSI/WiX/custom-action and package
lifecycle boundary is owned by Parent Client Runtime Distribution WP12
(`12-protected-broker-provisioner-package`); this ADR owns the core-side
acceptance and opaque proof boundary, not the installer implementation.

The WP12 package may invoke an elevated, installer-only provisioner, but no
untrusted MSI property, command-line argument, setup field, or parent caller
may supply an `authValue`, TPM index, policy, SID, image/path identity,
generation, lease, capability, or success result. A raw TPM `authValue` is not
part of any package, registry, log, or caller contract. The provisioner keeps
the authorization secret behind the TPM-owned non-exportable handle and the
approved TPM policy; the private core adapter verifies only the resulting
installer-owned enrollment facts and never accepts the raw secret as input.

The fixed NV index and policy identify the expected object but are distinct from
authorization by the `TPM_RH_PLATFORM` hierarchy. LocalSystem, process
elevation, TBS, PCP signing, or an OS account does not grant platform hierarchy
authority. The ceremony must obtain that authority from the external
OEM/firmware/MDM owner path; an empty authorization or caller-supplied
authorization is never a fallback. The current provisioner only performs
readback/revalidation and returns `ExternalProvisioningRequired`.

WP12 places its committed package and lifecycle source under
`scripts/release/windows/parent-protected-custody/`, its WiX manifest at
`scripts/release/windows/parent-protected-custody.wxs`, and its build wiring at
`scripts/release/windows/build-parent-protected-custody-package.ps1`. Generated
MSI/checksum/signing outputs remain release artifacts under
`target/release-packages/`, not repository source. The installer-owned
provisioner source is WP01-owned as a separate expected BIN-only package at
`crates/ocentra-protected-capability-custody-provisioner/Cargo.toml`,
`src/main.rs`, and private `src/provisioning/` modules. It has no `lib.rs` or
public API and performs one fixed installer-owned operation; no caller or MSI
property may supply a path, TPM index/policy, `authValue`, identity, generation,
lease, capability, or success result. Existing child-agent WiX and installer
files are not this boundary.

The monotonic generation authority is a TPM2 NV counter accessed through TBS.
The private core/FFI boundary owns the TPM policy, index binding, non-exportable
handle lifetime, and fail-closed validation; the parent installer only performs
the approved elevated provisioning ceremony and records no raw secret. TPM
reset, missing TPM, missing/deleted NV index, owner mismatch, or an unavailable
TBS path is fail closed and requires re-pair/enrollment. A disk snapshot,
SQLite row, JSON record, MSI property, or rollback journal may never restore or
advance that generation.

## Required admission observations

The one open-session/dispatch path must retain and revalidate:

- the broker's named-pipe stream/handle until the request completes;
- process and impersonated-token handles, plus process creation/image
  observations, for their required lifetimes in the private core adapter;
- peer SID, integrity level, and session identity;
- broker/client image identity and SCM configuration identity;
- exact registry owner, protected DACL, ACE set, and every pinned ancestor;
- nonce binding, expiry, and replay state;
- the TPM-backed monotonic generation and all broker/core generation slots.

Immediately before transcript authorization, the broker re-queries pipe process
and session IDs and the private core module compares them to the retained
process/token observations. A PID, path, SID, registry value, or caller
assertion without the retained handle and exact owner chain is not an
observation that satisfies this ADR.

## Failure and compatibility rules

The integrated FFI crate, private core adapter modules, and broker link are
present, but the runtime remains fail closed until the installer-owned TPM
policy/non-exportable handle enrollment and caller are available. Missing or
contradictory observations return typed
unavailable/deployment-required/re-pair outcomes before opening storage,
registry state, journal, listener, bootstrap, or service-ready state.
Unsupported platforms remain typed unavailable/manual-required. No fake success,
no-op adapter, same-process DPAPI authority, mutex/file-lock authority, split
snapshot, or caller-selected key/capability is permitted.

## Rejected alternatives

- A second helper process or helper protocol: duplicates the front-door seam and
  creates another identity and replay surface.
- A second safe adapter crate: requires a public proof-construction seam or a
  forbidden dependency cycle; private core modules preserve the sealed API.
- An in-process DPAPI, file lock, JSON, or SQLite authority: same-user writers
  can replace or roll back the claimed authority.
- `sysinfo`/path-only identity or flattened ACL observations: they do not retain
  the process/token/registry handles or prove the exact image, owner, DACL, and
  ancestor chain.
- Caller-provided attestation or enrollment: it lets an untrusted caller mint
  protected authority.
- TPM-like disk counters: they can be restored after reset and do not provide
  monotonic owner authority.

## Integrated production roots and remaining workspace obligations

The FFI, private core Windows, and provisioner roots below are integrated source
at canonical `a6d7d9adf`; graph mapping records reviewed topology, not runtime
completion:

```text
crates/ocentra-protected-capability-custody-windows-ffi/Cargo.toml
crates/ocentra-protected-capability-custody-windows-ffi/src/lib.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/enrollment.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/peer.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/scm.rs
crates/protected-capability-custody-core/src/broker_admission/platform/windows/monotonic.rs
crates/ocentra-protected-capability-custody-provisioner/Cargo.toml
crates/ocentra-protected-capability-custody-provisioner/src/main.rs
crates/ocentra-protected-capability-custody-provisioner/src/provisioning/
```

The FFI manifest is an active workspace member with a real `lib` target, the
package-local lint policy above, and only raw safe RAII wrappers. Core has the
target-specific dependency on the FFI package and owns the private safe
adapter module tree. The WP01-owned provisioner is the only other permitted FFI
consumer and is BIN-only with a `main` target plus private
`src/provisioning/` modules; it has no library/public API. WP12 invokes and
packages that binary; the broker and client continue depending on core/protocol
only. No second Windows manifest, public adapter target, or broker protocol is
planned.

The source implementation order through raw owned-handle/TBS/TPM wrappers,
private core Windows enrollment/peer/SCM/monotonic modules, construction
through the existing core runtime seams, and the read-only provisioner
preflight is integrated. The remaining order is external
OEM/firmware/MDM `TPM_RH_PLATFORM` and NV lifecycle authority, authenticated
owner handoff, protected registry/SCM mutation, independent broker/client/token
observations, enrolled counter generation, the core monotonic provider, the
Parent Runtime WP12 installer-side MSI/WiX invocation/lifecycle contract, then
real transport callers. The broker remains unavailable through those
intermediate steps.

The installer-side contract is intentionally narrow: WP12 owns package
identity, elevated custom-action scheduling, artifact/build wiring, upgrade and
rollback ordering, and uninstall/deprovisioning outcomes. It does not own
protected authority, parse or transport raw `authValue`, choose a TPM index or
policy, mint an opaque proof, or let a caller bypass the private core adapter.
Upgrade and rollback must preserve the TPM generation and fail closed rather
than restore disk state; uninstall must use an explicit owner-approved
deprovisioning path and must not silently remove protected enrollment.

## Tests, proof, and downstream routing

The existing 11 WP01 test roots remain required and absent. Two additional
adapter expectations are recorded as absent planned core-private tests only,
for 13 expected tests total:

```text
crates/protected-capability-custody-core/src/broker_admission/platform/windows_adapter_test.rs
crates/protected-capability-custody-core/src/broker_admission/platform/tpm_nv_counter_test.rs
```

Parent Runtime WP12 separately owns the future package/lifecycle test roots
`scripts/release/windows/parent-protected-custody/tests/` and
`tests/repo-tooling/parent-protected-custody-package.test.mjs`. Those tests
must verify real MSI/WiX/custom-action and upgrade/rollback/uninstall boundary
behavior without treating package success as protected authority. They do not
replace the private core adapter/TPM tests or the later installer/runtime proof.

No test source or proof is created by this ADR packet. The tests must exercise
the real private core module seams and FFI RAII wrappers, not a disconnected
helper, fake authority, or caller assertion.

Account WP05A, Device Trust WP01, Device Trust WP03, and Browser WP06 remain
blocked downstream consumers. They may consume only opaque broker outcomes
after this owner supplies installer enrollment, tests, proof, and runtime
evidence; none may mint or replace protected authority.

The graph may authorize only the remaining implementation phase for installer
provisioning and caller integration. Normal WP01 state remains validation, and
READY, PR_READY, tests, proof, merge, and DONE remain unchanged until their own
evidence exists.
