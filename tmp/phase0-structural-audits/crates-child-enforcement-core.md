# crates/child-enforcement-core

## Current Refresh Audit - 2026-06-19

- Responsibility: this crate owns the child-side enforcement decision core, including enforcement action inputs, typed requested/recorded event contracts, and the execute-or-not decision helper.
- Staleness: the prior file content was effectively just the heading, so this is the first real current audit record in the file.
- Dependencies: `ocentra-eventing`, `ocentra-policy-control-core`, and `serde`; no runtime/UI dependency is pulled in by the crate itself.
- Violations: `src/lib.rs` still exposes the only module through `pub use enforcement_action::*;`, which is a forbidden Rust public re-export in this repository.
- Duplicated shapes: the local mode/state enums mirror downstream enforcement vocabulary, and the three text-id wrappers repeat the same parse/as_str/display/try_from pattern behind one macro.
- Barrel/re-export/shim debt: `src/lib.rs` is dead frontage with no ownership logic beyond `mod enforcement_action; pub use enforcement_action::*;`, so the module root is less explicit than it should be.
- Schema drift: the event type strings, schema version, idempotency separator, and `child-enforcement-decision:` prefix are hard-coded in `src/enforcement_action.rs`, so any contract rename or version bump needs coordinated edits in that single module and every consumer.
- Event/log/proof misuse: the crate is correctly log-free for now because it is a pure contract/decision core, but it also has no crate-owned proof artifact yet; the current snapshot only shows unit tests, not a dedicated proof surface.
- Test/proof structure issues: `tests/unit.rs` is only a trampoline into `tests/unit/action_decision.rs`; the test coverage proves the happy-path decision matrix and typed event round-trip, but it does not yet separate contract regression, schema regression, or proof-oriented verification.
- DRY score: 82/100. The score is not lower because the core logic is compact and the decision helper is centralized. It is not higher because the public re-export, repeated text-id shape, and hard-coded contract literals still create avoidable coupling.
- Fix recommendation: remove the root `pub use`, keep callers on explicit `enforcement_action` imports, and leave enforcement decision logic inside the named module until there is a second real implementation that justifies an adapter split.
- Decouple recommendation: if more enforcement variants appear, extract shared text-id and event-contract helpers into a small internal support module rather than broadening the public surface.
- Blockers: no code changes were part of this refresh, so this audit is informational only; the only live blocker is the existing forbidden re-export debt that remains unaddressed.
- Exact likely paths: `crates/child-enforcement-core/src/lib.rs`, `crates/child-enforcement-core/src/enforcement_action.rs`, `crates/child-enforcement-core/tests/unit.rs`, `crates/child-enforcement-core/tests/unit/action_decision.rs`, `crates/child-enforcement-core/Cargo.toml`, `crates/child-enforcement-core/proof/.gitkeep`, `crates/child-enforcement-core/proofs/local/.gitkeep`, `crates/child-enforcement-core/proofs/ci/.gitkeep`.
- target_kind: Rust enforcement core
- owned_paths: crates/child-enforcement-core/Cargo.toml; crates/child-enforcement-core/src/lib.rs; crates/child-enforcement-core/src/enforcement_action.rs
- declared_responsibility: Policy-action execution boundaries, enforcement adapter orchestration, rollback/recovery state, and enforcement audit hooks.
- observed_responsibility: Defines enforcement action state/enums, event contracts, and the execute-or-not decision logic for child enforcement.
- should_own: Enforcement decision logic and the enforcement action requested/decision-recorded domain events.
- should_not_own: Unrelated policy control, runtime orchestration, UI, notification shaping, or a public barrel façade.
- allowed_dependencies: ocentra-eventing; ocentra-policy-control-core; serde.
- suspicious_dependencies: The crate is intentionally small, but `lib.rs` exposes its only module through a barrel-like public re-export, which weakens the ownership boundary.
- expected_dependents: `crates/child-runtime`, enforcement proof tests, and any downstream consumers of enforcement decisions/events.
- shared_contract_schema_usage: Uses eventing contracts and policy-control-core authority state directly without inventing duplicate external schemas.
- duplicate_or_near_duplicate_shapes: The enforcement action mode/state enums parallel the child-runtime gate vocabulary; the root lib is a dead-simple frontage layer over one module.
- id_name_status_drift: `EnforcementActionMode`, adapter, rollback, and idempotency states are local enum names that can drift if the upstream policy semantics change.
- direct_import_vs_event_boundary: Pure event-contract module, but the root `pub use enforcement_action::*;` makes the owning boundary too broad and hides the single implementation module.
- event_bus_usage: None; this is a pure decision-plus-event-contract crate.
- logging_and_proof_chain_expectations: As core execution logic, it should be logger-ready in downstream chains, but the crate itself does not currently carry logging surfaces.
- boundary_violations: Forbidden Rust public re-export in `lib.rs`.
- dry_common_core_candidates: If the crate grows, factor shared action-state and idempotency helpers into a dedicated module instead of extending the root frontage.
- dead_frontage_or_shims: `lib.rs` is effectively dead frontage aside from `mod enforcement_action; pub use enforcement_action::*;`.
- proposed_fix_packets: Replace the barrel with explicit module-path imports, keep enforcement decision logic in the named module, and add a small adapter layer only if a second implementation module appears.
- severity: high
- confidence: high
- evidence_paths: crates/child-enforcement-core/Cargo.toml; crates/child-enforcement-core/src/lib.rs; crates/child-enforcement-core/src/enforcement_action.rs; crates/child-enforcement-core/tests/unit.rs; crates/child-enforcement-core/tests/unit/action_decision.rs

## Current Refresh Audit - 2026-06-19

- Responsibility: this crate is the child-side enforcement contract core. It owns enforcement action inputs/decisions, the typed requested/recorded event contracts, and the decision helper that turns an enforcement request into a recorded result.
- Deps: `ocentra-eventing`, `ocentra-policy-control-core`, and `serde`; there is no broader runtime or UI dependency in the crate itself.
- Violations: `src/lib.rs` is still a pure public re-export frontage over `src/enforcement_action.rs`, which is the forbidden Rust re-export pattern for this repo.
- Duplicated shapes: the local mode/state enums mirror downstream enforcement vocabulary and can drift if child-runtime or policy-control naming changes; the text-id wrappers also repeat the same parse/display pattern three times.
- Barrel/reexport/shim debt: `src/lib.rs` has no real ownership logic beyond `mod enforcement_action; pub use enforcement_action::*;`, so it acts as dead frontage instead of a clear module root.
- Schema drift: event type strings, schema versioning, and the `child-enforcement-decision:` prefix are all hard-coded in one module, so any upstream contract rename will require coordinated edits here and in consumers.
- Event/log/proof misuse: the crate does not currently emit logs, which is fine for a pure contract core, but there is no crate-owned proof artifact beyond unit tests; the `proof/` and `proofs/` trees are placeholders only.
- Test/proof structure issues: `tests/unit.rs` is only a trampoline into `tests/unit/action_decision.rs`; the test surface covers the happy-path decision matrix and the typed event round-trip, but there is no separate contract, schema, or regression proof file yet.
- Score: 80/100, high severity.
- Fix recommendation: remove the root `pub use`, make callers import `enforcement_action` explicitly, and keep all enforcement decision logic in that named module until a second implementation module forces an adapter split.
- Decouple recommendation: if more enforcement variants appear, split shared text-id and event-contract helpers into a small internal support module, not into a new public crate surface.
- Blockers: no code changes were allowed for this refresh, so this audit remains informational only; there is no validation blocker for the read-only assessment itself.
- Exact likely paths: `crates/child-enforcement-core/src/lib.rs`, `crates/child-enforcement-core/src/enforcement_action.rs`, `crates/child-enforcement-core/Cargo.toml`, `crates/child-enforcement-core/tests/unit.rs`, `crates/child-enforcement-core/tests/unit/action_decision.rs`, `crates/child-enforcement-core/proof/.gitkeep`, `crates/child-enforcement-core/proofs/local/.gitkeep`, `crates/child-enforcement-core/proofs/ci/.gitkeep`.
