# data-custody-storage-plan Event Architecture Instruction

## Owns

- custody classification and storage/export/delete substrate contracts;
- sync/export endpoint contract shape;
- storage-custody Rust substrate decisions;
- recovery/export persistence boundary, not trust-state behavior.

## Must not own

- tracking-specific retention algorithms;
- device-trust recovery semantics;
- portal hosted UI truth;
- parent-domain wrapper ownership.

## Required chain

```text
consumer requests custody/export/delete action
-> custody owner validates substrate policy and persistence contract
-> storage/event journal records custody result
-> consumer plan observes read model or typed response
-> downstream plan proves its own behavior separately
```

## Logging/proof

Log custody class, retention/export/delete decision, storage target, redaction state, and downstream handoff. Proof must show the consumer received a typed custody result, not just that a proof helper file exists.

## Tests

- data/production/endpoint packages: unit and contract for substrate contracts.
- storage-custody-core: crate tests for Rust substrate decisions and invariants.
- tracking/device-trust/portal tests are consumer proof, not substrate closure.

## First architecture slice

Run substrate truth repair: restore proof roots, rewire proof scripts to direct owners, fix storage-custody-core test imports, then rerun owner-scoped commands.
