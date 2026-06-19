# crates/screen-core
- target_kind: Rust crate
- owned_paths: crates/screen-core/Cargo.toml; crates/screen-core/src/lib.rs; crates/screen-core/src/runtime_decision.rs
- declared_responsibility: Screen runtime decisioning plus child-domain observed/event handoff helpers around capture, AI, and policy signals.
- observed_responsibility: Converts capture-adapter status into runtime decisions and emits child-domain observation/evidence/policy request helpers.
- should_own: Screen runtime decision mapping, screen observation intent, and local event-record construction.
- should_not_own: Low-level capture mechanics, broad AI policy logic, or a public facade that hides the actual module split.
- allowed_dependencies: ocentra-evidence; ocentra-eventing; ocentra-parent-agent-protocol; ocentra-parent-screen-capture-adapter.
- suspicious_dependencies: ocentra-parent-screen-capture-adapter inside runtime_decision.rs because decisioning now depends on adapter semantics directly.
- expected_dependents: Screen runtime callers, child-domain event construction code, and any screen orchestration layer that needs runtime decisions.
- shared_contract_schema_usage: ChildDomainObservedEvent; ChildDomainObservedEventProfile; ChildDomainEvidenceRecordedEvent; ChildDomainAiAnalysisRequestedEvent; ChildDomainPolicyEvaluationRequestedEvent; ActivityCaptureCapabilityStatus.
- duplicate_or_near_duplicate_shapes: ScreenRuntimeDecisionId/ScreenAggregateId text-id macro pattern mirrors crates/screen-ai-core; runtime decision plus recorded-event shape mirrors other small core facades.
- id_name_status_drift: Low; names are coherent, but the crate uses a separate runtime vocabulary that overlaps with live-view and AI decisioning.
- direct_import_vs_event_boundary: Direct imports are acceptable for a pure decision helper; no event bus. The only boundary-sensitive edge is coupling runtime decisioning to capture status inside the crate.
- event_bus_usage: None; helpers build or transform protocol events locally.
- logging_and_proof_chain_expectations: No runtime logging here; proof-chain data is carried by the child-domain event helpers, so callers should keep any redaction/logging policy upstream.
- boundary_violations: Rust public re-export facade in lib.rs; duplicated helper macro with screen-ai-core; runtime_decision.rs reaches directly into capture-adapter semantics.
- dry_common_core_candidates: Shared branded-id helper with crates/screen-ai-core; shared child-domain observed-event/profile helper with crates/screen-live-view-core.
- dead_frontage_or_shims: lib.rs is a facade shim over runtime_decision.rs and should stay only if a stable crate surface is required.
- proposed_fix_packets: Extract one shared branded-id helper with screen-ai-core; replace the broad re-export with explicit exports; keep runtime_decision.rs focused on decision rules and event conversion.
- dry_score_0_to_100: 36
- score_breakdown:
  - owner_boundary_0_to_20: 14
  - dependency_direction_0_to_15: 6
  - shared_contract_schema_0_to_15: 4
  - duplicate_shape_control_0_to_15: 5
  - direct_import_vs_event_boundary_0_to_15: 3
  - decoupling_and_common_core_0_to_10: 2
  - proof_logging_alignment_0_to_10: 2
- first_fix_roi: High: a shared helper plus narrower exports would remove duplicated macro logic and clarify the crate boundary quickly.
- severity: medium
- confidence: high
- evidence_paths: crates/screen-core/Cargo.toml; crates/screen-core/src/lib.rs; crates/screen-core/src/runtime_decision.rs; crates/screen-capture-adapter/src/lib.rs

## Current Refresh Audit - 2026-06-19

- responsibility: `crates/screen-core` owns screen runtime decisioning, observation intent selection, and local event construction that bridges capture status into child-domain observed/evidence/policy request helpers.
- dependencies: It depends on `ocentra-parent-agent-protocol`, `ocentra-parent-screen-capture-adapter`, `ocentra-eventing`, and `ocentra-evidence`; the only domain-shaping dependency visible in the core logic is the direct use of capture-adapter schedule and attempt semantics in [`crates/screen-core/src/runtime_decision.rs`](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/screen-core/src/runtime_decision.rs).
- boundary_violations: I did not find an active public re-export or barrel in the inspected package files. The stale prior note about a lib.rs re-export facade does not match the current `lib.rs`, which only declares `pub mod runtime_decision;` and imports helper items locally. The remaining boundary pressure is semantic, not structural: runtime decisioning still reaches directly into capture-adapter status types.
- duplicated_shapes: `ScreenRuntimeDecisionId` and `ScreenAggregateId` are still local text-id wrappers defined via the `screen_text_id!` macro in [`crates/screen-core/src/runtime_decision.rs`](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/screen-core/src/runtime_decision.rs); that pattern overlaps with nearby screen/domain branded-id wrappers and is the main duplicated shape in this crate.
- barrel_reexport_shim_debt: No current barrel or re-export shim is present in [`crates/screen-core/src/lib.rs`](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/screen-core/src/lib.rs). The crate surface is small and direct, so shim debt is low; the only facade-like behavior is the crate root exposing the runtime decision module.
- schema_drift: Low. The event contracts and enum names in [`crates/screen-core/src/runtime_decision.rs`](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/screen-core/src/runtime_decision.rs) line up with the current test intent in [`crates/screen-core/tests/unit/runtime_decision.rs`](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/screen-core/tests/unit/runtime_decision.rs) and [`crates/screen-core/tests/unit/screen_flow.rs`](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/screen-core/tests/unit/screen_flow.rs). I did not see a mismatch between the current observation intents and the evidence/policy handoff tests.
- event_log_proof_misuse: None visible in the inspected files. The crate does not emit logs, does not appear to fake proof artifacts, and the tests assert the local event chain directly instead of asserting on unrelated logs or global proof state.
- test_proof_structure_issues: The tests are focused and useful, but they are still package-local unit tests only. `runtime_decision.rs` is covered by direct decision assertions, and `screen_flow.rs` covers the observed-event/evidence/request chain, but there is no higher-level contract or integration proof in this crate to pin down cross-crate behavior if the capture adapter semantics drift.
- dry_score_0_to_100: 52
- dry_score_reasons: The crate is small and mostly cohesive, which helps DRY, but it still carries a local branded-id macro, direct adapter-semantic coupling, and a thin module-root shim boundary. The score is better than the prior audit because the current files do not show an active re-export violation, but duplication and coupling are still real.
- fix_recommendation: Keep the crate root minimal, preserve the current direct module split, and extract the branded-id helper only if another screen package already needs the same parse/display semantics. If the capture-adapter dependency expands further, move the decision mapping behind a narrower semantic input instead of importing adapter details deeper into the crate.
- decouple_recommendation: Introduce a narrower screen-runtime input type that captures the semantics this crate actually needs, then adapt adapter-specific status values at the boundary before calling `evaluate_screen_runtime`.
- blockers: No blocking source issue was observed for the audit refresh itself. The main limitation is that I only reviewed the current crate files named in this audit, so any broader duplication claim would need a sibling-package comparison before it becomes a hard assertion.
- exact_likely_paths: [`crates/screen-core/src/lib.rs`](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/screen-core/src/lib.rs), [`crates/screen-core/src/runtime_decision.rs`](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/screen-core/src/runtime_decision.rs), [`crates/screen-core/tests/unit/runtime_decision.rs`](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/screen-core/tests/unit/runtime_decision.rs), [`crates/screen-core/tests/unit/screen_flow.rs`](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/screen-core/tests/unit/screen_flow.rs)
- no_barrels_no_reexports_hard_gate: No current violation found in the inspected package files. Exact likely paths for a violation would be [`crates/screen-core/src/lib.rs`](/C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/crates/screen-core/src/lib.rs) if a future `pub use`/barrel is introduced there, but there is no evidence of that pattern now.

## Current Refresh Audit - 2026-06-19

- responsibility: `crates/screen-core` owns screen runtime decisioning and child-domain event conversion. In this checkout, the public root still exposes the event-conversion API, but the `runtime_decision` source module itself is empty.
- dependencies: `crates/screen-core/Cargo.toml` wires `ocentra-evidence`, `ocentra-eventing`, `ocentra-parent-agent-protocol`, `ocentra-parent-screen-capture-adapter`, and `serde`. The unit tests also depend on capture-adapter scheduling/status types, which keeps the crate semantically tied to adapter behavior.
- violations: `crates/screen-core/src/lib.rs` uses `pub use runtime_decision::{...}` as a direct re-export barrel, which conflicts with the repo hard gate. More importantly, `crates/screen-core/src/runtime_decision.rs` is empty in this checkout, so the exported runtime API is structurally dangling.
- duplicated_shapes: The crate repeats the same one-line event-conversion shape across `screen_observed_event`, `screen_evidence_recorded_event`, `screen_ai_analysis_requested_event`, and `screen_policy_evaluation_requested_event`. The unit tests also repeat the same capture setup shape across the three local test files.
- barrel_reexport_shim_debt: High. The crate root is acting as a frontage layer over an absent runtime implementation, and the public API is wider than the source body that should back it.
- schema_drift: High risk. The tests still encode the intended schema and behavior, but the source module that should define the runtime decision helpers is empty. That is a public API versus implementation mismatch, not just a naming drift.
- event_log_proof_misuse: No runtime logging was present in the inspected files. The main issue is proof boundary absence: `crates/screen-core/proof/`, `crates/screen-core/proofs/ci/`, and `crates/screen-core/proofs/local/` are only `.gitkeep` placeholders, so there is no crate-local proof artifact structure supporting the published API.
- test_proof_structure_issues: Coverage is still unit-heavy and behavior-focused, which is good for the event helpers, but the harness is local-only and the proof directories are empty. `crates/screen-core/tests/unit.rs` is just a harness path in the tree, and there is no integration/proof layer anchoring the runtime decision surface.
- dry_score_0_to_100: 18
- dry_score_reasons: The score is low because the crate root is a re-export shim, the core runtime module is empty, and the proof/test scaffolding is disconnected from the public API. The tests validate useful behavior, but they are validating a surface that is not backed by a populated runtime module in this checkout.
- fix_recommendation: Restore a real `runtime_decision.rs` implementation first, then trim `lib.rs` to the smallest public surface that still reflects ownership. Keep the event-conversion helpers direct and explicit rather than widening the crate root.
- decouple_recommendation: Separate runtime decision input shaping from child-domain event conversion so capture-adapter semantics do not leak through the public crate root. That would let the runtime layer own only the decision rules while the boundary layer adapts adapter-specific status values.
- blockers: The source boundary is incomplete in this checkout because `crates/screen-core/src/runtime_decision.rs` is empty. That prevents a trustworthy structural audit of the runtime decision logic itself and makes the current public API look broken.
- exact_likely_paths: `crates/screen-core/Cargo.toml`, `crates/screen-core/src/lib.rs`, `crates/screen-core/src/runtime_decision.rs`, `crates/screen-core/tests/unit.rs`, `crates/screen-core/tests/unit/runtime_decision.rs`, `crates/screen-core/tests/unit/screen_flow.rs`, `crates/screen-core/tests/unit/observation_intent.rs`, `crates/screen-core/proof/.gitkeep`, `crates/screen-core/proofs/ci/.gitkeep`, `crates/screen-core/proofs/local/.gitkeep`
