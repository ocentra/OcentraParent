# crates/app-game-core
- target_kind: Rust core crate for app-game runtime decision translation and child-domain event wrapping.
- owned_paths: `crates/app-game-core/src/lib.rs`, `crates/app-game-core/src/runtime_decision.rs`, `crates/app-game-core/tests/unit.rs`, `crates/app-game-core/tests/unit/*.rs`
- declared_responsibility: convert app-game observation intent into child-domain observed/evidence/policy event contracts and record app-game runtime decisions.
- observed_responsibility: evaluates app-game capability, foreground, and classification state into decision state, wraps child-domain event helpers, and verifies the mapping with unit tests.
- should_own: app-game-specific observation intent mapping, runtime decision enums, app-game event wrappers, and contract-focused tests for the app-game child-domain path.
- should_not_own: shared generic runtime engines, unrelated domain logic, UI/runtime orchestration, logging policy, or cross-domain contract families.
- allowed_dependencies: `ocentra-eventing`, `ocentra-parent-agent-protocol`, `serde`, local `runtime_decision` module, and only the evidence helpers actually needed by the crate.
- suspicious_dependencies: `ocentra-evidence` looks stale or over-broad in `Cargo.toml`; the sampled source files do not use it directly.
- expected_dependents: app-game runtime callers, app-game proof/tests, and any parent-side code that needs app-game observation/event translation.
- shared_contract_schema_usage: `ChildDomainObservedEvent`, `ChildDomainObservedEventProfile`, `ChildDomainEvidenceRecordedEvent`, `ChildDomainAiAnalysisRequestedEvent`, `ChildDomainPolicyEvaluationRequestedEvent`, `ChildRuntimeDomain::AppGame`.
- duplicate_or_near_duplicate_shapes: near mirror of `crates/app-core` with renamed enums, ID wrappers, decision logic, and recorded-event construction; both crates share the same text-id macro pattern and event-contract pattern.
- id_name_status_drift: `AppGameObservationIntent` and `RecordForegroundSession` are domain-specific names for logic that is structurally the same as `app-core`, so the naming can drift from the implementation if the domains diverge.
- direct_import_vs_event_boundary: direct construction of child-domain events is the only boundary here; there is no event bus layer. The `lib.rs` public re-export leaks `runtime_decision` internals across the crate boundary.
- event_bus_usage: none observed.
- logging_and_proof_chain_expectations: no logging or proof-chain instrumentation is present; callers that exercise this crate should own correlated proof milestones if the runtime path is validated elsewhere.
- boundary_violations: `pub use` re-export from `lib.rs` violates the repo no-reexports rule and turns the crate root into a frontage surface.
- dry_common_core_candidates: extract the same shared runtime-decision template as `app-core`; the two crates look like domain-specialized copies of one core translator.
- dead_frontage_or_shims: `lib.rs` is mostly frontage around `runtime_decision`; `ocentra-evidence` currently looks like a dead dependency, and the public re-export surface is thinner than the underlying implementation.
- proposed_fix_packets: remove the crate-root re-export barrel, prune the unused evidence dependency if it stays unused, then factor the shared runtime-decision mechanics into a reusable core or macro-driven template with domain-specific constants.
- severity: high
- confidence: high
- evidence_paths: `crates/app-game-core/Cargo.toml`, `crates/app-game-core/src/lib.rs`, `crates/app-game-core/src/runtime_decision.rs`, `crates/app-game-core/tests/unit.rs`

## Current Refresh Audit - 2026-06-19

- responsibility: translate app-game observation intent into child-domain event contracts and runtime decisions for the app-game path only; this crate should stay a narrow mapper, not a shared runtime layer.
- dependencies: `ocentra-eventing`, `ocentra-parent-agent-protocol`, `serde`, the local `runtime_decision` module, and only the evidence helpers actually used by the crate.
- boundary_violations: the crate-root `pub use` in `lib.rs` is still a no-reexports violation and keeps frontage logic at the root; `ocentra-evidence` remains suspiciously broad for the observed source usage.
- duplicated_shapes: this is still a near mirror of `crates/app-core` with renamed enums, ID wrappers, decision logic, and recorded-event construction; the shape duplication appears intentional but still expensive.
- barrel_reexport_shim_debt: `lib.rs` remains a thin frontage/shim around `runtime_decision`, so the crate root exposes internal shape instead of forcing direct module ownership.
- schema_drift: `AppGameObservationIntent` and `RecordForegroundSession` are domain-specific names for logic that appears structurally aligned with `app-core`, so names can drift from behavior if the domains diverge further.
- event_log_proof_misuse: no logging or proof-chain instrumentation is present here; the risk is callers treating this crate as proof-complete when the runtime path has no correlated milestone emission of its own.
- test_proof_structure_issues: the unit test surface validates the mapping shape, but it remains tightly coupled to the duplicate translator pattern instead of proving a shared core contract or a cleaner boundary split.
- dry_score: 31/100. The score is low because the crate duplicates a sibling translator, keeps a crate-root re-export, and carries suspicious dependency breadth; the only mitigation is that the domain boundary is still narrow and test-backed.
- fix_recommendation: remove the crate-root re-export, prune unused dependency surface if `ocentra-evidence` still is not directly needed, and collapse the repeated translator mechanics into a shared internal template or reusable core.
- decouple_recommendation: separate the app-game-specific naming and event wrapping from the shared mapping mechanics so the sibling core can own the structural translator while this crate owns only app-game constants and domain labels.
- blockers: I did not re-run crate tests or architecture checks in this refresh-only pass, so the audit reflects structural inspection rather than runtime proof.
- exact_likely_paths: `crates/app-game-core/Cargo.toml`, `crates/app-game-core/src/lib.rs`, `crates/app-game-core/src/runtime_decision.rs`, `crates/app-game-core/tests/unit.rs`

## Current Refresh Audit - 2026-06-19

- responsibility: own the app-game runtime decision translator, the typed runtime decision record event, and the child-domain observed-event mapping for the app-game path only.
- dependencies: `ocentra-eventing`, `ocentra-parent-agent-protocol`, `serde`, the local `runtime_decision` module, and the declared but currently unused `ocentra-evidence` dependency.
- violations: no current `pub use`/barrel is visible in `src/lib.rs`, so the earlier no-reexports note appears stale; the remaining boundary issue is the broad declared dependency surface relative to actual source use.
- duplicated_shapes: `src/runtime_decision.rs` still mirrors the sibling app-core translator pattern with renamed enums, ID wrappers, decision evaluation, and recorded-event construction; the three unit test files also repeat the same event-path assertions in a sibling-style layout.
- barrel_reexport_shim_debt: the crate root is now thin rather than re-export-heavy, so the shim debt is mostly gone at the source level, but the crate still functions as a narrow frontage over `runtime_decision`.
- schema_drift: the app-game naming layer (`AppGameObservationIntent`, `AppGameRuntimeDecisionId`, `AppGameAggregateId`) can drift from the structural pattern shared with sibling runtime translators, and the unused evidence dependency suggests the declared contract may be broader than the implemented one.
- event_log_proof_misuse: no logging or proof-chain instrumentation is present in the crate, so callers can easily mistake the unit tests for full chain proof even though no correlated milestone emission exists here.
- test_proof_structure_issues: coverage is unit-only and split across `tests/unit/*.rs`; it proves mapping shape and event contracts, but not a shared core boundary or any architecture gate, and it does not exercise the declared evidence dependency.
- dry_score: 39/100. The crate is narrower than before because the root re-export is no longer visible, but the translator logic still duplicates a sibling shape and the dependency surface is wider than the used code.
- fix_recommendation: remove the unused `ocentra-evidence` dependency if it is not needed, then move the repeated translator mechanics into a shared internal template or common core so this crate only owns app-game constants and naming.
- decouple_recommendation: separate the app-game-specific labels and typed wrappers from the repeated event/decision mechanics, and let a sibling shared translator own the structural pattern if another domain needs the same flow.
- blockers: this refresh did not rerun crate tests or architecture validation, so the audit remains a structural snapshot rather than a proof-backed verification.
- exact_likely_paths: `crates/app-game-core/Cargo.toml`, `crates/app-game-core/src/lib.rs`, `crates/app-game-core/src/runtime_decision.rs`, `crates/app-game-core/tests/unit.rs`, `crates/app-game-core/tests/unit/runtime_decision.rs`, `crates/app-game-core/tests/unit/observation_intent.rs`, `crates/app-game-core/tests/unit/app_game_flow.rs`
