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
can consume without obtaining or manufacturing protected authority. The current
`ocentra-protected-capability-custody-core` is a real fail-closed substrate for
binding, custody transitions, path validation, platform records, and checked
SQLite replica state. The neutral protocol, broker, and client packages plus the
core-owned admission facade are now active production source, but the broker is
deliberately unavailable before state creation because the required protected
Windows authority adapters and installer-owned enrollment do not yet exist.

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
```

The 2026-08-24 source packet was independently reviewed for consolidation. It
adds the isolated-process/package boundary, one shared bounded wire protocol,
opaque request/result types, startup preflight, dynamic enrolled-client pipe
ACL construction, split immutable-enrollment/runtime registry custody, and a
narrow core-owned broker entry. Internal authority constructors remain private.
The service fails with `DeploymentRequired` before opening storage, registry,
journal, listener, bootstrap, or service-ready state when protected admission is
unavailable. SQLite remains a checked replica; it cannot become the authority
merely because it is durable.

## Required source boundary

The source topology now contains a separate Windows broker binary, a client
boundary, and the neutral `protected-capability-custody-protocol` wire owner as
active Cargo workspace members. This is source presence, not operating custody.
The remaining production boundary must provide a safe pinned Windows
`OpenProcess` observation, impersonated token SID/integrity/session observation,
exact registry owner/DACL/parent-chain verification, a non-restorable monotonic
provider, immutable broker/SCM identity, installer/SCM provisioning, and a real
enrolled caller. Until those owners exist, startup must remain unavailable and
must not create or mutate custody state.

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

## Expected test source

The complete test wave is intentionally deferred until the source packet is
stable. Internal core behavior must use core-owned unit-test modules under
`src/` so private storage/path/authority state is tested without making those
constructors public. Public protocol tests belong to the protocol package;
broker process/race/Windows custody tests belong to the broker package; and
client admission/IPC-authentication tests belong to the client package. No
test may import private source with path tricks, use a mock/in-process broker,
or create a dependency cycle merely to reach private state.

## Consumers and unlocks

| Consumer | Graph relationship | Boundary |
| --- | --- | --- |
| Account WP05A Runtime Effect Fencing Coordinator | implementation dependency | Consumes opaque owner reservation outcomes; does not own broker authority or SQLite custody. |
| Device Trust WP01 Device Trust Source of Truth | implementation dependency | Consumes neutral protected custody for owner-participant persistence; retains device identity/currentness and signer authority. |
| Device Trust WP03 Parent Step-Up Auth | implementation dependency | Consumes neutral protected custody for ceremony/participant handoff; retains parent/device authority and ceremony semantics. |

These are downstream source-order relationships only. They do not make any
consumer runtime-ready and do not remove their existing Account, Device Trust,
Cloudflare, platform, caller, test, or proof blockers.

## Acceptance gates

Keep WP01 open until the protected Windows adapters, installer/SCM enrollment,
and real production caller exist; the complete core, protocol, broker, and
client expected test roots are written and run; the Windows process/IPC and
owner-bound custody negative cases are retained; Enforcer/architecture checks
pass; and proof/checklist state is current. Independently reviewed source
consolidation does not change normal READY, PR_READY, CI, merge, or DONE state.
