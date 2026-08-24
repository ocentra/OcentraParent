# Protected Capability Custody Foundation Plan State

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan State
> Kind: current state and open gaps.
> Proves: only the current protected-custody route and its stated boundaries.
> Does not prove: implementation, tests, proof, runtime reachability, or release.

<!-- /agent-capsule -->

## Current status

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

`broker_admission.rs` is a narrow core-owned facade seam: it may expose only
typed broker-entry/request/result operations after authenticated process/IPC
validation. It must not expose `CustodyAdmission`, `CurrentBindingPort`,
`PlatformCustodyOwner`, `PlatformDatabaseGuard`, or any constructor that lets a
caller implement or mint those sealed owner traits. The neutral protocol crate
owns the wire contract consumed by both broker and client; the broker binary
does not own a client-visible copy of that contract.

## Consumer routing

The reviewed graph records Account WP05A and Device Trust WP01/WP03 as
downstream consumers of this neutral boundary. These are source-order/unlock
relationships only. They do not transfer ownership, create a caller, or close
their existing authority and platform gaps. No edge points from this neutral
plan back to Account or Device Trust, so the route does not create a dependency
cycle.

## Exit conditions

The workpack can leave validation only after real protected Windows adapters,
installer/SCM enrollment, and a production caller exist; focused source and
boundary validation are green; all expected tests are written and executed;
the broker owns the protected operation; restart/recovery and concurrent
reservations are covered; and retained proof/checklist/merge evidence is
current. Until then, keep the state open and report the exact missing adapter
or caller.
