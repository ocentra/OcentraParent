# ADR-PCC-002: One Windows Front-Door Custody Module

## Status

Accepted for implementation-only routing on 2026-08-25. This ADR records an
implementation shape; it does not claim that the adapters, enrollment, tests,
proof, runtime availability, READY, or DONE exist.

## Context

The canonical baseline is `20da6048f24fa7cc61aa1309780dc7d672ce6ee4`. The
existing protected-custody core, neutral wire package, broker process, and
client are real Rust source. The broker still fails closed before custody state
creation because safe pinned Windows identity observation, protected registry
custody, immutable enrollment, and a non-restorable monotonic authority are
absent. The current broker/client platform paths are stubs by design; they must
remain unavailable until a real replacement is linked.

The design must preserve the neutral plan's ownership: the broker owns the
protected operation, the core owns its sealed admission and custody decisions,
and callers receive only opaque typed outcomes. A caller must not provide an
identity, attestation, key, capability, lease, generation, or success flag.

## Decision

Build one Rust Windows front-door process: the existing
`ocentra-protected-capability-custody-broker`, with the existing custody core
linked in-process. Add one safe Windows adapter module at that process seam;
do not add a second helper process or a second helper protocol.

The adapter is split into two crates with one ownership seam:

1. `ocentra-protected-capability-custody-windows-ffi` is a tiny raw-wrapper
   module. It contains only Win32/TBS/TPM calls and owned-handle wrappers. Its
   package-scoped `unsafe_code` allowance is isolated to this crate;
   `unsafe_op_in_unsafe_fn` is denied and all other workspace lint denies remain
   enabled. It contains
   no custody decisions, enrollment policy, persistence authority, or caller
   interface.
2. `ocentra-protected-capability-custody-windows` is the safe adapter module.
   It links the FFI wrappers and presents a small opaque interface to the
   existing broker/core admission seam. It owns observation, enrollment
   decoding, fail-closed mapping, and lifetime retention, but it cannot mint
   core authority or expose raw handles to callers.

The external seam is one dispatch/open-session path. The broker obtains the
peer identity from the authenticated Windows pipe and retained OS handles,
then dispatches the caller's operation through the existing broker entry. No
caller-supplied identity or attestation is accepted, and no alternate helper
transport is introduced.

## Module depth and seam

The FFI module is intentionally shallow and narrow: it hides unsafe ABI and
handle lifetime mechanics behind owned values. The safe Windows adapter is the
deep module: its small interface concentrates process/token/registry/TPM
verification, enrollment interpretation, monotonic checks, and error mapping
for every broker caller. This gives callers leverage and keeps verification
locality at one seam. The broker remains the owner of protected decisions; the
adapter is an Adapter at that seam, not a second authority.

## Enrollment and authority

Enrollment is installer-only and immutable at runtime. A pre-provisioned,
elevated owner/MDM/OEM ceremony must establish and pin the broker image and SCM
identity, the enrolled client SID/image, the protected registry root and exact
ancestor chain, and the TPM2 NV counter/index. Runtime code may verify those
records but may not self-enroll, widen them, or treat a user-writable registry
or disk record as authority.

The monotonic generation authority is a TPM2 NV counter accessed through TBS.
TPM reset, missing TPM, missing/deleted NV index, owner mismatch, or an
unavailable TBS path is fail closed and requires re-pair/enrollment. A disk
snapshot, SQLite row, JSON record, or rollback journal may never restore or
advance that generation.

## Required admission observations

The one open-session/dispatch path must retain and revalidate:

- named-pipe, process, and impersonated-token handles for their required
  lifetimes;
- peer SID, integrity level, and session identity;
- broker/client image identity and SCM configuration identity;
- exact registry owner, protected DACL, ACE set, and every pinned ancestor;
- nonce binding, expiry, and replay state;
- the TPM-backed monotonic generation and all broker/core generation slots.

Revalidation occurs at the last point before session material or custody state
is emitted. A PID, path, SID, registry value, or caller assertion without the
retained handle and exact owner chain is not an observation that satisfies this
ADR.

## Failure and compatibility rules

The current fail-closed stubs remain in place until the replacement adapter,
enrollment ceremony, and broker link are present. Missing or contradictory
observations return typed unavailable/deployment-required/re-pair outcomes
before opening storage, registry state, journal, listener, bootstrap, or
service-ready state. Unsupported platforms remain typed unavailable/manual-
required. No fake success, no-op adapter, in-process substitute, same-process
DPAPI authority, mutex/file-lock authority, split snapshot, or caller-selected
key/capability is permitted.

## Rejected alternatives

- A second helper process or helper protocol: duplicates the front-door seam and
  creates another identity and replay surface.
- An in-process DPAPI, file lock, JSON, or SQLite authority: same-user writers
  can replace or roll back the claimed authority.
- `sysinfo`/path-only identity or flattened ACL observations: they do not retain
  the process/token/registry handles or prove the exact image, owner, DACL, and
  ancestor chain.
- Caller-provided attestation or enrollment: it lets an untrusted caller mint
  protected authority.
- TPM-like disk counters: they can be restored after reset and do not provide
  monotonic owner authority.

## Planned production roots and workspace obligations

These roots are planned and absent at the canonical baseline; listing them in
the graph is ownership routing, not source presence:

```text
crates/ocentra-protected-capability-custody-windows-ffi/Cargo.toml
crates/ocentra-protected-capability-custody-windows-ffi/src/lib.rs
crates/ocentra-protected-capability-custody-windows/Cargo.toml
crates/ocentra-protected-capability-custody-windows/src/lib.rs
```

Both manifests must become active workspace members with real `lib` targets.
The FFI package must carry the package-scoped unsafe/lint policy described
above. The safe package may depend on the FFI package and the existing neutral
protocol/core contracts; the broker may depend on the safe package. No caller
may depend directly on the FFI package.

The implementation order is: raw owned-handle/TBS/TPM wrappers; safe adapter
observation and enrollment verification; broker in-process link at the single
dispatch/open-session seam; installer/SCM/MDM/OEM provisioning; then the
production caller. Existing broker/client stubs remain blocked through every
intermediate step.

## Tests, proof, and downstream routing

The existing 11 WP01 test roots remain required and absent. Two additional
adapter expectations are recorded as absent planned tests only: one Windows
adapter custody test root for process/token/registry/SCM checks and one TPM2
NV/TBS monotonic-counter test root. No test source or proof is created by this
ADR packet.

Account WP05A, Device Trust WP01, Device Trust WP03, and Browser WP06 remain
blocked downstream consumers. They may consume only the future opaque broker
outcomes after this owner supplies the real adapter, enrollment, tests, proof,
and runtime evidence; none may mint or replace protected authority.

The graph may authorize only the implementation phase for the missing planned
production roots. Normal WP01 state remains validation, and READY, PR_READY,
tests, proof, merge, and DONE remain unchanged until their own evidence exists.
