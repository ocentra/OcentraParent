# 04 - Rust Protocol Parity For AI Contracts

## Target State

Every AI contract crossing into Rust has mirrored structs, constants, enum
values, serialization tests, and TypeScript parity proof.

## Where We Are

Rust service has local AI runtime, provider scheduler, chat generation, parent
assistant, policy preview, and memory graph pieces. AI result/context parity must
be explicit before runtime consumers grow.

## Checklist

- [ ] Identify Rust-crossing AI shapes.
- [ ] Add protocol constants and structs in the right crate.
- [ ] Add serialization tests for field names and enum values.
- [ ] Add TS/Rust parity fixture if shape is shared.
- [ ] Reject stringly route ids or inline event names.

## Proof

- `cargo test -p ocentra-parent-agent-protocol`
- focused service/core tests for touched Rust AI shapes.
