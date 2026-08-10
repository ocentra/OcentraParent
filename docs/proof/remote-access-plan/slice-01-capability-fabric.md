# Remote Access WP01 Capability Fabric — Validation Manifest

Date: 2026-08-09
Plan: `remote-access-plan`
Workpack: `01-remote-capability-fabric`
Owner boundary: `crates/schema` Rust-owned view-only capability/grant/session contract

## Result

The narrow capability-fabric contract is implemented and locally validated.
The graph state is `validation`, not `done`: pairing/standing-access lifecycle,
relay/session runtime, device-trust integration, revoke/remove flow, custody,
and rollout proof remain open.

## Commands and results

| Command | Result |
| --- | --- |
| `cargo test -p ocentra-schema --test contract remote_capability_fabric -- --nocapture` | 6 passed, 0 failed (103 filtered) |
| `cargo fmt --all -- --check` | passed |
| `npm run lint:architecture -- --files crates/schema/src/remote_capability_fabric.rs crates/schema/tests/contract/remote_capability_fabric.rs docs/plans/remote-access-plan` | passed |
| `npm run hub:guard` | passed; no findings/conflicts |

## Negative coverage

The focused contract rejects deferred remote control, wrong household,
support/admin actor role without a parent grant, unpaired access, route
mismatch, missing device trust, revoked grants, and removed devices.
Serialization asserts the Rust-owned camel-case wire shape, route and parent
grant fields, schema v2, and the explicit v1 migration/default rejection path.

## No-claim boundary

This manifest does not claim relay availability, screen capture, pairing
workflow, standing-access persistence, revoke/remove runtime behavior,
device-trust handoff, custody/retention, portal disclosure, abuse controls,
remote input/control, CI, review, or merge to `main`.
