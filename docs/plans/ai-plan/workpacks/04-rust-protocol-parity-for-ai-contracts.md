# 04 - Rust Protocol Parity For AI Contracts

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `04 - Rust Protocol Parity For AI Contracts`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
