# App Plan WP01: Rust Runtime-Decision Contract Evidence

Date: 2026-07-28

## Scope

This proof covers the `app-plan` workpack `01-contract-boundary-and-effect-schemas` at the Rust-owned `crates/app-core` boundary. It intentionally does not introduce TypeScript product-contract authority.

## Proven behavior

- Runtime decision aggregate and decision IDs require canonical prefixes and opaque lowercase/digit/hyphen suffixes.
- The contract test exercises all 18 capability, foreground, and classification input tuples against a checked-in Rust fixture.
- The event contract is version 2 and the emitted event envelope exactly matches its checked-in Rust fixture.
- Missing capability stays manual-required and invariant coverage confirms it cannot publish AI or policy handoffs.

## Validation

| Command | Result | Evidence |
| --- | --- | --- |
| `cargo test -p ocentra-app-core --test contract_runtime_decision --message-format short` | pass, 4 tests | decision matrix, event envelope, typed event, malformed ID rejection |
| `cargo test -p ocentra-app-core --test invariant_runtime_decision --message-format short` | pass, 2 tests | handoff/manual-required invariants |
| `cargo clippy -p ocentra-app-core --all-targets -- -D warnings` | pass | app-core source and visible tests |
| `npm run lint:architecture -- --files crates/app-core/Cargo.toml crates/app-core/src/runtime_decision.rs crates/app-core/src/runtime_ids.rs crates/app-core/tests/contract crates/app-core/tests/invariant` | pass | architecture, ownership, source-shape, and test policy |
| `git diff --check` | pass | whitespace/integrity |

## Negative and no-claim boundaries

- Prefixless, empty, and display-name runtime IDs are rejected.
- Background inventory and inventory-only inputs cannot become foreground enforcement behavior.
- This is contract/runtime-decision proof only. It does not prove live Windows/macOS/mobile collection, persistence/read-model handling, portal state, policy execution, enforcement execution, authorization, or platform permission readiness.

## Integration repair

The source PR contained unresolved Git conflict markers in `crates/app-core/tests/contract/runtime_decision.rs`; the repair is included and covered by the contract test and lint gate. The old ignored `output/` proof was not used as durable evidence; this tracked record replaces its claims.
