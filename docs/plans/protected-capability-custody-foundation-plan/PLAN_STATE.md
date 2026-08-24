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
implementation-only authorization. The existing
`crates/protected-capability-custody-core` source is substantive and
fail-closed, but it is not an operating custody system:

- admission/open/platform authority is sealed inside the crate;
- the SQLite state is a checked replica, not the authority for protected
  capability custody;
- no isolated broker process, authenticated OS IPC, broker-owned ACL/path/key/
  watermark/write-lease authority, opaque external factory, or production
  caller exists;
- the expected unit, security, recovery, concurrency, and Windows integration
  tests are not present;
- no current proof, pre-commit, CI, PR, or merge claim exists for this route.

The graph records the route as `validation` and separately authorizes only the
implementation phase. It must not derive ordinary READY or DONE from this
source map.

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

## Consumer routing

The reviewed graph records Account WP05A and Device Trust WP01/WP03 as
downstream consumers of this neutral boundary. These are source-order/unlock
relationships only. They do not transfer ownership, create a caller, or close
their existing authority and platform gaps. No edge points from this neutral
plan back to Account or Device Trust, so the route does not create a dependency
cycle.

## Exit conditions

The workpack can leave implementation-only authorization only after a real
broker/client source packet exists, focused source and boundary validation are
green, all expected tests are written and executed, the broker owns the
protected operation, restart/recovery and concurrent reservations are covered,
and retained proof/checklist/merge evidence is current. Until then, keep the
state open and report the exact missing adapter or caller.
