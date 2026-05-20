# Contract Feature Expectations

Contract features define meaning shared across runtimes.

## Expected Deliverables

- Effect Schema contract in the owning domain package.
- Branded primitives for domain-bearing text.
- Parser helpers from the repo's schema-domain helpers.
- Exact valid and invalid TypeScript tests.
- Rust protocol struct when Rust sends or receives the shape.
- Rust serialization/parity test with exact field names and values.
- Protocol constants for commands, events, fields, ids, and stable strings.

## Acceptance

- Invalid payloads fail at the boundary.
- Valid payloads parse into branded/domain types.
- Rust and TypeScript agree on schema version, field names, and enum values.
- The contract does not leak implementation-specific storage details unless that is the contract's purpose.
- Runtime code consumes the contract instead of inventing local equivalents.

## Non-Goals

- Do not add runtime behavior just because a contract exists.
- Do not add broad future fields without a concrete expected use.
- Do not create parallel contracts for the same concept in multiple packages.

## Done Signal

The contract can be used from TypeScript and Rust without string guessing, and tests would fail if a field name, enum value, schema version, or branded primitive changed incorrectly.
