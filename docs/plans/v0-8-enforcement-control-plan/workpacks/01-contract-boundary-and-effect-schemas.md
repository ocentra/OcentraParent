# 01 Contract Boundary And Effect Schemas

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `01 Contract Boundary And Effect Schemas`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

V0.8 already has several typed enforcement and product-control shapes. The full
program still needs one reviewed boundary for every state the portal, service,
policy evaluator, proof scripts, and Rust protocol share.

## Where We Want To Be

Every enforcement value crossing TypeScript, Rust, service, or portal boundaries
is schema-backed, branded, versioned, and tested before runtime code consumes it.

## Requirement Checklist

- [x] Keep contract ownership in domain packages before service or portal code.
- [x] Use Effect Schema brands and decode helpers for ids, states, and enums.
- [x] Mirror Rust-crossing shapes in `crates/agent-protocol`.
- [x] Add invalid payload tests for every external input.
- [x] Keep command names, fields, reason codes, and text tokens out of app code.

## Acceptance And Proof

Contract tests and Rust parity tests pass before a runtime path accepts or emits
the value. Current proof runs:
`npm run test --workspace @ocentra-parent/enforcement-domain -- enforcement`,
`cargo test -p ocentra-parent-agent-protocol enforcement`, and
`npm run lint:architecture -- --files packages/enforcement-domain/src/enforcement.ts packages/enforcement-domain/tests/unit/enforcement.test.ts packages/enforcement-domain/tests/unit/enforcement-permission-dependency.test.ts packages/enforcement-domain/tests/unit/enforcement-audit-boundary.test.ts packages/enforcement-domain/tests/unit/enforcement-timer.test.ts crates/agent-protocol/src/enforcement.rs crates/agent-protocol/src/enforcement_tests.rs crates/agent-protocol/src/enforcement_unavailable_tests.rs docs/plans/v0-8-enforcement-control-plan/workpacks/01-contract-boundary-and-effect-schemas.md`.
Owning source surfaces live in
`packages/enforcement-domain/src/enforcement.ts` and
`crates/agent-protocol/src/enforcement.rs`.
Focused invalid-input coverage lives in
`packages/enforcement-domain/tests/unit/enforcement.test.ts`,
`packages/enforcement-domain/tests/unit/enforcement-permission-dependency.test.ts`,
`packages/enforcement-domain/tests/unit/enforcement-audit-boundary.test.ts`,
`packages/enforcement-domain/tests/unit/enforcement-timer.test.ts`,
`crates/agent-protocol/src/enforcement_tests.rs`, and
`crates/agent-protocol/src/enforcement_unavailable_tests.rs`.
Current proof artifacts live under
`output/v0-8-enforcement-control-plan-proof/01-contract-boundary-and-effect-schemas/`
and
`docs/proof/v0-8-enforcement-control-plan/slice-04-contract-boundary-and-effect-schemas.md`.
A tiny audit-fix was required to close this slice honestly:
`crates/agent-protocol/src/enforcement_tests.rs` now asserts the concrete
deserialize failure instead of a weak `is_err()` check so the architecture gate
can verify the Rust parity proof.

## Parallel Ownership Notes

This workpack blocks most other implementation. Do not create duplicate action,
result, capability, or audit shapes in app/runtime folders.
