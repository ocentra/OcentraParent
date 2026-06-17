# lan-plan Event Architecture Instruction

## Owns

- LAN discovery, pairing, household/device spine, signed peer state, add-device readiness;
- LAN proof scripts and LAN read models.

## Must not own

- account authority semantics;
- device-trust transition rules;
- eventing runtime abstraction;
- portal final rendering truth.

## Required chain

```text
LAN discovery/pairing input
-> lan-domain validates source/device/household state
-> service/runtime emits LAN event/read model
-> portal or remote/eventing consumes typed LAN state
```

## Logging/proof

Log discovery source, peer identity class, trust mapping, add-device readiness, rejection reason, and physical/manual-required boundary.

## Tests

Move contract/integration-like LAN tests out of unit where needed. Rust LAN closure tests should live in crate-level `tests/` categories, not only `src` modules.

## First architecture slice

Run B1 proof regeneration. Then B2 test truth repair. Eventing and remote should cite refreshed LAN proof, not stale rows.
