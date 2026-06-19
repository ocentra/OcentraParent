# crates/network-core
- target_kind: Rust crate with network runtime and event-chain helpers
- owned_paths: `crates/network-core/Cargo.toml`; `crates/network-core/src/lib.rs`; `crates/network-core/src/network_runtime.rs`
- declared_responsibility: network runtime decision helpers that convert adapter, capture, parser, and observation state into child-domain events
- observed_responsibility: the crate is a thin include shim; `src/network_runtime.rs` mirrors the browser-core runtime-decision/event-chain pattern with network-specific gates and intent names
- should_own: network observation intent, capture-gate decisioning, and network runtime event-chain synthesis
- should_not_own: generic child-domain protocol definitions, network-domain schema contracts, or unrelated adapter implementation details
- allowed_dependencies: `ocentra-eventing`; `ocentra-parent-agent-protocol`; `serde`
- suspicious_dependencies: direct protocol event construction is expected, but the crate duplicates the same child-runtime pattern already present in `crates/browser-core`
- expected_dependents: network plan tests, runtime adapters, proof helpers, and any caller needing network runtime events
- shared_contract_schema_usage: uses `ChildRuntimeDomain::Network`, `ChildDomainObservedEventProfile`, `ChildDomainObservedSignal`, `ChildDomainAiAnalysisRequirement`, `ChildDomainPolicyEvaluationRequirement`, and the child-domain event helpers from the protocol crate
- duplicate_or_near_duplicate_shapes: `NetworkRuntimeInput`, `NetworkRuntimeDecision`, and the intent/state enums are near clones of the browser runtime model with only adapter/capture/parser specifics changed
- id_name_status_drift: low, but `NetworkParserState` and `NetworkCapturePermissionState` encode proof-gate status inside a runtime facade instead of a shared gate contract
- direct_import_vs_event_boundary: direct protocol imports are the boundary; no separate event bus or publish/subscribe layer is present
- event_bus_usage: none observed
- logging_and_proof_chain_expectations: any future runtime/path work should emit structured, redacted decision and proof milestones; no logger surface is present in the sampled files
- boundary_violations: `src/lib.rs` is an `include!` shim that hides the real API boundary, and the crate duplicates browser-core's event-chain logic instead of owning a distinct reusable child-runtime layer
- dry_common_core_candidates: shared child-runtime decision core for browser and network intents; shared helper for observed-event -> evidence -> AI/policy request chaining
- dead_frontage_or_shims: `src/lib.rs` is a thin include shim and should not be the conceptual owner of the API surface
- proposed_fix_packets: replace the include shim with a normal module root, move shared runtime/event-chain logic into a common helper, and keep only network-specific intent/state in this crate
- severity: medium-high
- confidence: high
- evidence_paths: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent\crates\network-core\Cargo.toml`; `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent\crates\network-core\src\lib.rs`; `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent\crates\network-core\src\network_runtime.rs`; `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent\crates\browser-core\src\runtime_decision.rs`

## Current Refresh Audit - 2026-06-19

- responsibility: network runtime decisioning that converts adapter, capture permission, parser, and observation intent state into child-domain observed/evidence/AI/policy events.
- deps: direct crate deps are `ocentra-evidence`, `ocentra-eventing`, `ocentra-parent-agent-protocol`, `ocentra-network-evidence`, and `serde`; no extra runtime dependency layer is present.
- violations: the old `include!` shim claim is stale, but the crate still mirrors the browser-core runtime-decision/event-chain shape instead of owning a clearly distinct shared child-runtime core.
- duplicated shapes: `NetworkObservationIntent`, `NetworkRuntimeInput`, `NetworkRuntimeDecision`, `NetworkRuntimeEventChain`, and the `network_runtime_*` projection helpers closely track `browser-core`'s runtime-decision flow and its event-chain wrappers.
- barrel/reexport/shim debt: `src/lib.rs` is now a normal module root, so there is no source re-export shim here; the remaining shim-like file is `tests/unit.rs`, which only forwards to the leaf test modules.
- schema drift: `NetworkCapturePermissionState` and `NetworkParserState` keep proof-gate state inside a runtime facade instead of a shared gate contract, so the crate still owns local policy/capture semantics that would be better normalized.
- event/log/proof misuse: no structured logger or proof-milestone surface is present in the sampled runtime files, so event construction happens directly without a visible redacted proof chain.
- test/proof structure issues: the test suite is split into two leaf files that both assert the same event-chain projection pattern, but there is no dedicated contract/proof helper boundary for the repeated browser/network runtime behavior.
- score: 6.5/10 structural debt, with the main risk in cross-crate duplication and gate-shape drift rather than in immediate functional breakage.
- fix recommendation: extract a shared child-runtime decision/event-chain core with `crates/browser-core`, keep network-only intent/state local, and leave protocol event construction to the shared child-runtime owner.
- decouple recommendation: split capture/parsing gate state from the runtime decision facade, then move the common observed-to-evidence-to-AI/policy chain projection behind a reusable helper with one owner.
- blockers: no blocker for the audit refresh itself; follow-up refactor work is blocked on choosing the shared owner for the duplicated child-runtime decision core and its gate schema.
- exact likely paths: `crates/network-core/src/network_runtime.rs`; `crates/network-core/src/lib.rs`; `crates/network-core/tests/unit.rs`; `crates/network-core/tests/unit/network_flow.rs`; `crates/network-core/tests/unit/runtime_flow.rs`; `crates/browser-core/src/runtime_decision.rs`; `crates/agent-protocol/src/child_domain_runtime.rs`; `crates/network-core/Cargo.toml`

## Current Refresh Audit - 2026-06-19

- responsibility: `crates/network-core` owns network runtime decisioning and event-chain projection from adapter, capture permission, parser, and observation intent state into child-domain observed/evidence/AI/policy events.
- dependencies: direct dependencies are `ocentra-evidence`, `ocentra-eventing`, `ocentra-parent-agent-protocol`, `ocentra-network-evidence`, and `serde`; the crate has no extra boundary layer or facade package to absorb shared runtime logic.
- violations: no active include/re-export shim remains in the current source boundary, but the crate still embeds the same child-runtime projection pattern as the browser core instead of isolating a reusable shared core.
- duplicated shapes: `NetworkObservationIntent`, `NetworkAdapterState`, `NetworkCapturePermissionState`, `NetworkParserState`, `NetworkRuntimeInput`, `NetworkRuntimeDecision`, and `NetworkRuntimeEventChain` are all close structural repeats of the browser child-runtime decision model with only network-specific labels and gates changed.
- barrel/re-export/shim debt: `src/lib.rs` is now a normal module root with `pub mod network_runtime;`, so barrel debt is low; the remaining maintainability risk is repeated runtime helper forwarding inside `network_runtime.rs` and any test-layer forwarding around the same shape.
- schema drift: `NetworkCapturePermissionState` and `NetworkParserState` still encode gate/proof status inside a runtime facade, which leaves the network crate carrying local policy semantics that should likely be normalized in a shared gate contract.
- event/log/proof misuse: runtime construction goes straight to child-domain event helpers with no visible structured logging or proof-milestone boundary, so redacted proof evidence is not separated from event synthesis.
- test/proof structure issues: the current shape implies repeated projection checks around `network_runtime_event_chain` and the thin helper wrappers, but there is no dedicated shared contract/proof helper boundary for browser/network parity.
- dry score: 72/100. The score is held down by cross-crate duplication, gate-shape drift, and direct helper forwarding; it is not lower because the crate boundary itself is now clean and the `lib.rs` shim debt is gone.
- fix recommendation: extract a shared child-runtime decision/event-chain core with `crates/browser-core`, keep network-only intent and gate state local, and leave protocol event construction to the shared owner.
- decouple recommendation: split capture/parser gate state out of the runtime decision facade first, then centralize observed-to-evidence-to-AI/policy projection behind one reusable helper.
- blockers: no blocker for this audit refresh; the follow-up refactor is blocked only by ownership choice for the shared runtime core and the exact schema home for capture/parser gates.
- exact likely paths: `crates/network-core/src/network_runtime.rs`; `crates/network-core/src/lib.rs`; `crates/network-core/tests/unit.rs`; `crates/network-core/tests/unit/network_flow.rs`; `crates/network-core/tests/unit/runtime_flow.rs`; `crates/browser-core/src/runtime_decision.rs`; `crates/agent-protocol/src/child_domain_runtime.rs`; `crates/network-core/Cargo.toml`
