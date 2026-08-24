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
SQLite replica state. It is not yet externally constructible or reachable.

## Existing production source

The reviewed live roots are the core manifest and its source files/directories:

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

The source audit found no public admission/factory that can safely cross into a
real owner process. SQLite remains a checked replica; it cannot become the
authority merely because it is durable.

## Required source boundary

The implementation phase must add a real isolated Windows broker process and a
client boundary. The expected source topology is recorded in the code map as
planned roots under `protected-capability-custody-broker` and
`protected-capability-custody-client`; those crates do not exist in this
checkout. The source packet must establish authenticated OS IPC, broker-owned
ACL/path/key/watermark/write-lease decisions, opaque admission/factory state,
startup/restart reconciliation, and fail-closed unavailable-platform results.

The broker must be a separate OS process. Same-process DPAPI, an in-process
broker, mutex/file-lock custody, caller attestation, caller-selected key or
capability, fake success, and no-op adapters are forbidden. The client may
receive only typed opaque results bound to the authenticated owner, target,
action, generation, and broker state.

## Expected test source

The complete test wave is intentionally deferred until the source packet is
stable. It must add the seven roots named by
`TEST_PROOF_EXPECTATIONS.md`, including binding/schema/transition units,
path-and-replica security, restart reconciliation, concurrent broker races,
and the Windows broker custody integration.

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

Keep WP01 open until the broker/client source exists, the seven expected test
roots are written and run, the Windows process/IPC and owner-bound custody
negative cases are retained, Enforcer/architecture checks pass, and proof and
checklist state are current. Implementation-only authorization does not change
normal READY, PR_READY, CI, merge, or DONE state.
