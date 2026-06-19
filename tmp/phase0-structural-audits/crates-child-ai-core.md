# crates/child-ai-core
- target_kind: Rust crate
- owned_paths: crates/child-ai-core/Cargo.toml; crates/child-ai-core/src/lib.rs; crates/child-ai-core/src/tracking_boundary.rs; crates/child-ai-core/src/child_domain_analysis.rs
- declared_responsibility: Child-side AI analysis completion and tracking-domain classification guardrails.
- observed_responsibility: Validates tracking analysis request constraints, then forwards to tracking-core; also completes child-domain AI analysis events.
- should_own: Child-domain AI analysis handoff, validation of tracking request boundaries, and the minimal child-facing AI contract.
- should_not_own: A general-purpose tracking facade or a crate-level re-export shim that hides the real ownership split.
- allowed_dependencies: ocentra-eventing; ocentra-evidence; ocentra-parent-agent-protocol; ocentra-tracking-core.
- suspicious_dependencies: ocentra-tracking-core is fine as a backend, but the crate looks more like a validation wrapper than a real owner of tracking behavior.
- expected_dependents: Child AI request handlers, tracking workflow callers, and any child-domain analysis handoff logic.
- shared_contract_schema_usage: TrackingAiAnalysisRequestedEvent; TrackingNearbyPlaceClassifiedEvent; ChildDomainAiAnalysisRequestedEvent; ChildDomainAiAnalysisCompletedEvent; PrivatePayloadState.
- duplicate_or_near_duplicate_shapes: The two module entrypoints are tiny pass-throughs; lib.rs mirrors the same facade pattern as the Rust screen crates.
- id_name_status_drift: Low; the child/tracking names are coherent, though the crate mixes child-AI and tracking terminology.
- direct_import_vs_event_boundary: Direct imports are fine; no event bus. The wrapper around tracking-core is a direct validation-to-delegation boundary, which is acceptable but thin.
- event_bus_usage: None; helper functions complete or classify protocol events directly.
- logging_and_proof_chain_expectations: No runtime logging here; the validation wrapper should stay strict and not become a hidden policy sink.
- boundary_violations: Rust public re-export facade in lib.rs; thin wrapper over tracking-core plus child-domain analysis that looks more like a forwarding shim than a stable core.
- dry_common_core_candidates: Tracking validation helper could move into tracking-core if this crate stays only as a child-AI boundary.
- dead_frontage_or_shims: lib.rs is a two-symbol facade over analysis and tracking modules.
- proposed_fix_packets: Remove the facade re-export; decide whether tracking validation belongs here or in tracking-core; keep only child-AI boundary logic in the crate if it remains separate.
- dry_score_0_to_100: 35
- score_breakdown:
  - owner_boundary_0_to_20: 14
  - dependency_direction_0_to_15: 7
  - shared_contract_schema_0_to_15: 3
  - duplicate_shape_control_0_to_15: 4
  - direct_import_vs_event_boundary_0_to_15: 3
  - decoupling_and_common_core_0_to_10: 2
  - proof_logging_alignment_0_to_10: 2
- first_fix_roi: Medium: the crate is small, so either inlining or extracting the wrapper should be cheap.
- severity: medium
- confidence: high
- evidence_paths: crates/child-ai-core/Cargo.toml; crates/child-ai-core/src/lib.rs; crates/child-ai-core/src/tracking_boundary.rs; crates/child-ai-core/src/child_domain_analysis.rs

## Current Refresh Audit - 2026-06-19

- Responsibility: the crate is the child-side AI boundary for analysis completion and for guarding the tracking nearby-place classification handoff.
- Deps: it depends on `ocentra-eventing`, `ocentra-evidence`, `ocentra-parent-agent-protocol`, and `ocentra-tracking-core`; the tracking core dependency is the backend it delegates into, not a side dependency.
- Violations: there is no literal Rust `pub use` barrel in the current owned files, but `src/lib.rs` is still a very thin public entrypoint and `src/tracking_boundary.rs` is a validation wrapper over `tracking-core` rather than a stable owner of behavior.
- Duplicated shapes: the crate repeats the common "validate request, then delegate" pattern seen in other small Rust boundary crates, and `tests/unit.rs` is only a module dispatcher with no higher-level contract shape of its own.
- Barrel/reexport/shim debt: the old facade smell is now more about shape than syntax; `lib.rs` exposes only the two modules and a const, while `tracking_boundary.rs` still acts like a shim that could live in `tracking-core` if this crate is meant to stay child-AI-only.
- Schema drift: the crate mixes child-AI and tracking vocabulary in one boundary, so the ownership line is still fuzzy even though the protocol names themselves are coherent.
- Event/log/proof misuse: there is no runtime logging here, which is fine, but the boundary does not emit any proof milestone or dedicated audit artifact beyond unit assertions, so the handoff is only asserted at the API level.
- Test/proof structure issues: the tests cover one happy-path classification and a few negative filters, but they do not prove a deeper ownership split, no-checkpoint contract, or any integration against a real tracking owner.
- Score: 35/100. The score stays low because the crate is still thin, duplicated, and directionally closer to a validation shim than a true domain owner.
- Fix recommendation: either keep the crate narrowly as the child-AI completion boundary and move the tracking validation into `ocentra-tracking-core`, or make this crate own a real child-domain contract instead of forwarding tracking behavior.
- Decouple recommendation: split the tracking-specific classification guardrails out of `src/tracking_boundary.rs` first, then keep `src/child_domain_analysis.rs` as the only child-domain completion adapter if the crate remains separate.
- Blockers: no hard runtime blocker is visible in the crate itself; the real blocker is architectural ownership choice, because the current code is small enough that either consolidation or a rename/split would be cheap.
- Exact likely paths: `crates/child-ai-core/src/lib.rs`, `crates/child-ai-core/src/tracking_boundary.rs`, `crates/child-ai-core/src/child_domain_analysis.rs`, `crates/child-ai-core/tests/unit.rs`, `crates/child-ai-core/tests/unit/tracking_boundary.rs`, `crates/child-ai-core/tests/unit/child_domain_policy_handoff.rs`, and if the tracking logic moves, the likely home is `crates/tracking-core/src/*`.

## Current Refresh Audit - 2026-06-19

- Staleness: the older audit is directionally correct but stale in emphasis; it underplayed how much of this crate is still just a thin ownership shell around protocol helpers and validation delegation.
- Responsibility: `crates/child-ai-core` currently owns child-side AI completion plus a narrow guardrail for tracking nearby-place classification, but it does not own a broad child AI domain core.
- Dependencies: the crate depends on `ocentra-eventing`, `ocentra-evidence`, `ocentra-parent-agent-protocol`, and `ocentra-tracking-core`; the first three are contract/support libraries, while `tracking-core` is the backend that makes the boundary feel split rather than owned here.
- Violations: there is no literal Rust re-export barrel or unsafe code in the inspected files, but the architectural violation is a soft one: `tracking_boundary.rs` is a forwarding validation layer, and `child_domain_analysis.rs` is a direct protocol helper passthrough.
- Duplicated shapes: the crate repeats the common "check a request, then hand off" shape seen in other thin boundary crates; `tests/unit.rs` also behaves as a module dispatcher instead of a contract-level test owner.
- Barrel/re-export/shim debt: no syntax barrel exists now, so the debt is mostly shim debt. `lib.rs` is only a two-module entrypoint plus a const, and `tracking_boundary.rs` looks like code that could live in `ocentra-tracking-core` if the child crate is meant to stay child-AI-only.
- Schema drift: there is no obvious field-level schema mismatch in the owned files, but the crate still mixes child-domain analysis vocabulary with tracking vocabulary, which makes the boundary read as two adjacent contracts instead of one clearly named owner.
- Event/log/proof misuse: runtime logging is not expected here and none is present, which is fine. The gap is proof density: the behavior is asserted only through unit tests, with no dedicated proof artifact, trace marker, or end-to-end contract evidence for the ownership split.
- Test/proof structure issues: the tracking tests prove one happy path and several rejection filters, while the child-domain policy handoff tests prove event handoff semantics, but neither set proves an actual decoupled ownership boundary or a stronger contract envelope around the shim.
- DRY score 0-100 with reasons: 34/100. The score stays low because the crate is tiny, duplicated in shape, and mostly a validation/forwarding shell rather than a reused domain owner. It is slightly better than a pure barrel because the tests do cover concrete policy and classification behavior, but the ownership story is still thin.
- Fix recommendation: decide whether `child-ai-core` is the real child-domain AI owner or just a narrow adapter. If it is an adapter, move the tracking guardrails into `crates/tracking-core` and leave this crate with only the child-domain completion path. If it is the owner, add real child-domain contract behavior here instead of forwarding tracking policy.
- Decouple recommendation: split the tracking-specific classification guardrails first, then leave `src/child_domain_analysis.rs` as the only child-facing completion adapter if the crate must remain separate.
- Blockers: no runtime blocker is visible in the inspected files. The blocker is architectural choice, because the current code size is small enough that either consolidation or split-out is cheap, but leaving it as-is preserves the thin-shim shape.
- Exact likely paths: `crates/child-ai-core/src/lib.rs`, `crates/child-ai-core/src/tracking_boundary.rs`, `crates/child-ai-core/src/child_domain_analysis.rs`, `crates/child-ai-core/tests/unit.rs`, `crates/child-ai-core/tests/unit/tracking_boundary.rs`, `crates/child-ai-core/tests/unit/child_domain_policy_handoff.rs`, and if the tracking logic is moved, `crates/tracking-core/src/*`.
