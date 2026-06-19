# crates/agent-core
- target_kind: Rust runtime core crate.
- owned_paths: crates/agent-core/Cargo.toml; crates/agent-core/README.md; crates/agent-core/src/lib.rs; crates/agent-core/src/browser_event_runtime.rs; crates/agent-core/src/network_event_runtime.rs; crates/agent-core/src/activity_store_app_game.rs; crates/agent-core/src/household_mesh_event_bridge.rs
- declared_responsibility: Local runtime core for child-device behavior that should not live in the HTTP service shell.
- observed_responsibility: Broad in-process runtime core for evidence, journaling, query-store support, browser/network/screen/event runtime helpers, and app-game projection helpers.
- should_own: Local runtime helpers, journal/query-store support, deterministic parsing/projection logic, and platform-specific capture/enforcement helpers behind explicit boundaries.
- should_not_own: WebSocket schema names, service transport plumbing, or another layer's wire-shape ownership.
- allowed_dependencies: ocentra-parent-agent-protocol, ocentra-eventing, ocentra-network-evidence, ocentra-tracking-core, serde, rusqlite, sysinfo, platform-specific capture libs.
- suspicious_dependencies: `src/lib.rs` is a large `pub use` barrel; `browser_event_runtime.rs`, `network_event_runtime.rs`, and `screen_event_runtime.rs` each duplicate parse/report scaffolding; `activity_store_app_game.rs` is a dense projection bucket.
- expected_dependents: crates/agent-service, protocol parity tests, and local runtime proof/test harnesses.
- shared_contract_schema_usage: Very high; this crate consumes protocol types/constants heavily and turns them into runtime evidence, read models, and reports.
- duplicate_or_near_duplicate_shapes: Browser/network/screen runtime modules all define similar input/payload/report/result scaffolds; network remote-delivery status helpers mirror protocol status states; app-game and activity-store modules repeat row/projection shapes across sources and read models.
- id_name_status_drift: Medium; names are usually explicit, but the root facade and repeated runtime family modules blur where the real ownership boundary lives.
- direct_import_vs_event_boundary: Direct protocol imports are correct here, but the crate should not become a second owner of wire shapes or transport-facing contract definitions.
- event_bus_usage: Local `ocentra-eventing` and journal/runtime helpers are present; this is an internal runtime bus, not a service transport bus.
- logging_and_proof_chain_expectations: Runtime/projection helpers in this crate participate in command/event/read-model chains and should stay logger-ready with redaction and correlated proof milestones.
- boundary_violations: `src/lib.rs` uses forbidden Rust re-exports; the crate root is a frontage shim with too much surface area.
- dry_common_core_candidates: Common parse/result scaffolding for browser, network, and screen runtime modules; shared status/state enums; shared app-game projection helpers.
- dead_frontage_or_shims: `src/lib.rs` is the dominant shim; many `*_tests` modules mainly pin parity around the same runtime families.
- proposed_fix_packets: Narrow family modules, extract shared runtime/projection helpers, and remove the root barrel so the crate reads as a core engine rather than a catch-all facade.
- severity: high.
- confidence: high.
- evidence_paths: crates/agent-core/README.md; crates/agent-core/src/lib.rs; crates/agent-core/src/browser_event_runtime.rs; crates/agent-core/src/network_event_runtime.rs; crates/agent-core/src/network_event_runtime_state.rs; crates/agent-core/src/activity_store_app_game.rs; crates/agent-core/src/household_mesh_event_bridge.rs

## Current Refresh Audit - 2026-06-19

- Current responsibility: local runtime core for child-device evidence, journaling, query-store support, network/browser/screen runtime proof, app-game projection, enforcement gating, and household-mesh bridge validation.
- Dependencies: heavy direct use of `ocentra-parent-agent-protocol`, `ocentra-eventing`, `serde`, `rusqlite`, and service-facing constants; `network_event_runtime.rs` and `activity_store_app_game.rs` remain the densest dependency hubs.
- Boundary violations: `src/lib.rs` is still a wide re-export facade, and `network_event_runtime.rs` adds another internal re-export surface instead of staying narrowly module-owned.
- Duplicated shapes: browser/network/screen runtime families still mirror each other with near-identical payload/report/state scaffolds; app-game rows/projections and network remote-delivery state/report pairs repeat the same shape patterns.
- Barrel/reexport/shim debt: the crate root remains the dominant shim, with large `pub use` blocks in `src/lib.rs` and family-level re-exports in `network_event_runtime.rs` and `activity_store_app_game.rs`.
- Schema/contract drift: the crate still consumes protocol contract shapes directly, but ownership is blurred by root-level export surfaces and staged runtime helpers that sit close to wire-shaped names.
- Event bus/log/proof misuse: `network_event_runtime.rs` uses `EventBus` as a local proof bus and stores journal envelopes for assertions, but proof milestones are still implicit and coupled to runtime mechanics rather than separated from them; logging/redaction concerns remain mostly structural rather than enforced in this crate.
- Test/proof structure issues: proof coverage is broad but repetitive, with many `*_tests` modules asserting the same runtime-family invariants; the tests validate bus output and envelope counts more than contract boundaries or decoupled ownership.
- Current DRY score: `4/10` because the crate has a clear runtime mission, but the root facade, repeated runtime-family scaffolds, and dense app-game/network buckets keep duplication high.
- Fix-pass recommendation: first trim the root facade and the family-level re-export surfaces, then extract shared runtime/report helpers from the browser, network, and app-game buckets before changing semantics.
- Decouple-pass recommendation: split the network remote-delivery proof family and the app-game projection family into smaller ownership units, then separate proof helpers from runtime plumbing so the crate reads as a core engine instead of a catch-all facade.
- Sequencing/blockers: the root barrel and network/app-game density are the main blockers for clean decoupling; safest order is `src/lib.rs` facade first, then `network_event_runtime.rs`, then `activity_store_app_game.rs`, then the associated test modules.
- Exact likely file paths: `crates/agent-core/src/lib.rs`, `crates/agent-core/src/network_event_runtime.rs`, `crates/agent-core/src/network_event_runtime_state.rs`, `crates/agent-core/src/browser_event_runtime.rs`, `crates/agent-core/src/activity_store_app_game.rs`, `crates/agent-core/src/household_mesh_event_bridge.rs`, `crates/agent-core/src/network_event_runtime_tests.rs`, `crates/agent-core/src/activity_store_app_game_tests.rs`.

## Current Refresh Audit - 2026-06-19

- Responsibility: `crates/agent-core` is the local runtime core for child-device behavior, evidence capture, journaling, query-store support, and the runtime/proof helpers that sit below the HTTP service shell.
- Dependencies: the crate still leans heavily on `ocentra-parent-agent-protocol`, `ocentra-eventing`, `serde`, `rusqlite`, and protocol constants; `src/network_event_runtime.rs` is the strongest dependency hub, with `src/activity_store_app_game.rs` as the next densest boundary.
- Violations: `src/lib.rs` remains a broad re-export facade, and `src/network_event_runtime.rs` adds another internal `pub use` surface that keeps ownership blurred instead of narrowly module-owned.
- Duplicated shapes: the browser, network, and screen runtime families still mirror each other in payload/report/state layout; the app-game projection path repeats row/session/read-model patterns; network remote-delivery types and reports repeat the same record/state/report scaffolding across many submodules.
- Barrel/re-export/shim debt: `src/lib.rs` is still the dominant shim, `src/network_event_runtime.rs` re-exports many submodule proofs/types, and `src/activity_store_app_game.rs` re-exports live-source errors/results instead of keeping a tighter internal boundary.
- Schema drift: I did not see an obvious field-level mismatch in the inspected files, but the crate still blurs schema ownership by consuming protocol names/constants directly while exposing runtime helpers through large facade layers, which increases drift risk over time.
- Event/log/proof misuse: `src/network_event_runtime.rs` treats `EventBus` as an in-process proof bus and stores journal/dead-letter state for assertions; that is acceptable for local proofing, but the proof concerns are still coupled to runtime mechanics rather than isolated as a separate proof layer.
- Test/proof structure issues: proof coverage is fragmented across many `*_tests` modules and tends to assert repeated family invariants or journal output counts; the test structure validates runtime plumbing more than boundary clarity or ownership separation.
- DRY score: `34/100`. The crate has real modular subdirectories and repeated family patterns that are at least organized, but the root barrel, family-level re-exports, and repeated runtime/projection scaffolds still dominate the shape of the boundary.
- Fix recommendation: remove the root facade first, then collapse the family-level re-export surfaces in `src/network_event_runtime.rs` and `src/activity_store_app_game.rs`, and only then extract shared runtime/report helpers where the browser, network, screen, and app-game families truly converge.
- Decouple recommendation: split the network remote-delivery proof family into smaller ownership units and separate app-game source/projection code from the public report helpers so the crate reads like a core engine instead of a catch-all facade.
- Blockers: the crate root barrel is the main structural blocker, followed by the large network remote-delivery surface and the app-game projection bucket; without trimming those first, the rest of the boundary stays hard to reason about.
- Exact likely paths: `crates/agent-core/src/lib.rs`, `crates/agent-core/src/network_event_runtime.rs`, `crates/agent-core/src/network_event_runtime_state.rs`, `crates/agent-core/src/activity_store_app_game.rs`, `crates/agent-core/src/activity_store_app_game_observation.rs`, `crates/agent-core/src/activity_store_app_game_rows.rs`, `crates/agent-core/src/network_event_runtime_tests.rs`, `crates/agent-core/src/activity_store_app_game_tests.rs`
