# WP01 Contract Boundary And Effect Schemas

- checkedAt: `2026-06-17T02:13:51Z`
- branch: `codex/tracking-plan-full-continuation-a`
- commit: `1f192e52b931d3b2b8080f3e9479d37a94172958`
- result: `pass`

## Commands

- `npm run test --workspace @ocentra-parent/enforcement-domain -- enforcement`
- `cargo test -p ocentra-parent-agent-protocol enforcement`
- `npm run lint:architecture -- --files packages/enforcement-domain/src/enforcement.ts packages/enforcement-domain/tests/unit/enforcement.test.ts packages/enforcement-domain/tests/unit/enforcement-permission-dependency.test.ts packages/enforcement-domain/tests/unit/enforcement-audit-boundary.test.ts packages/enforcement-domain/tests/unit/enforcement-timer.test.ts crates/agent-protocol/src/enforcement.rs crates/agent-protocol/src/enforcement_tests.rs crates/agent-protocol/src/enforcement_unavailable_tests.rs docs/plans/v0-8-enforcement-control-plan/workpacks/01-contract-boundary-and-effect-schemas.md`

## Owning surfaces

- `packages/enforcement-domain/src/enforcement.ts`
- `crates/agent-protocol/src/enforcement.rs`
- `packages/enforcement-domain/tests/unit/enforcement.test.ts`
- `packages/enforcement-domain/tests/unit/enforcement-permission-dependency.test.ts`
- `packages/enforcement-domain/tests/unit/enforcement-audit-boundary.test.ts`
- `packages/enforcement-domain/tests/unit/enforcement-timer.test.ts`
- `crates/agent-protocol/src/enforcement_tests.rs`
- `crates/agent-protocol/src/enforcement_unavailable_tests.rs`

## Covered proof

- enforcement intent, action, result, audit, timer, and active timer state contracts stay owned by enforcement-domain before service and portal consumers
- Effect Schema branded ids and typed enums guard TypeScript-side external inputs
- Rust protocol mirrors the same public shapes and stable serialized literals
- invalid status, unavailable-status, audit-boundary, timer-reason, and active-state identity mismatch cases reject
- app source raw enforcement command names were not needed for this contract owner slice; shared protocol/domain values remain the public boundary

## Tiny audit-fix

- `crates/agent-protocol/src/enforcement_tests.rs` replaced a weak `assert!(parsed.is_err())` with a concrete failure assertion naming the invalid `blocked-by-label` status. This was strictly required to make the architecture gate honestly validate the Rust parity proof.

## Remaining gaps

- this slice proves contract ownership and parity, not adapter execution readiness
- broader capability matrix, approval/runtime handoff, UI consumption, and rollout workpacks remain open
