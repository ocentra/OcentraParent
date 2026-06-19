# crates/logging-core
- target_kind: Rust logging core and local NDJSON persistence crate
- owned_paths: crates/logging-core/Cargo.toml, crates/logging-core/src/**, crates/logging-core/tests/**
- declared_responsibility: low-level log event modeling, redaction, diagnostics, dev-log persistence, and compatibility writing
- observed_responsibility: one crate owns event schema, snapshot schema, NDJSON writing, legacy dev-log compatibility, diagnostics, source labeling, and redaction in the same boundary
- should_own: primitive log event/value types, file-path and segment sanitization, redaction helpers, and the dev-log persistence primitives that are truly shared
- should_not_own: product-specific logging policy, TS bridge/server behavior, support-workflow contracts, or higher-level runtime decisions
- allowed_dependencies: serde, serde_json, sha2, std, and tokio only where async file paths are needed
- suspicious_dependencies: the crate mixes compatibility shims, diagnostics, artifacts, agent-run metadata, and persistence in one owner instead of carving a narrower core
- expected_dependents: logging-domain adapters, local proof/test code, and Rust runtime callers that need NDJSON log persistence
- shared_contract_schema_usage: the Rust `ParentLogEvent` and `LogSnapshot` shapes mirror the TS logging-domain snapshot and dev-entry schemas rather than consuming a shared schema package
- duplicate_or_near_duplicate_shapes: `LogFieldValue`, `LogLevel`, `LogSource`, `ParentLogEvent`, and `LogSnapshot` closely track TS logging-domain log entry and snapshot shapes
- id_name_status_drift: Rust uses `ParentLogEvent` and `LogSnapshot` while the TS side uses `AgentLogEntry`, `DevLogEntry`, and `AgentLogSnapshot`, so the same conceptual log record is named differently across boundaries
- direct_import_vs_event_boundary: this crate is not evented, but `DevLogger` plus `LegacyDir` and `resolve_compat_dev_log_path` form a compatibility frontage that can hide the true current storage model
- event_bus_usage: none
- logging_and_proof_chain_expectations: any change here should keep the NDJSON payload, redaction, and legacy compatibility path testable because downstream logging proofs depend on the emitted file shape
- boundary_violations: diagnostics, artifacts, agent-run metadata, and dev-log compatibility all live beside the core log record model, which makes the crate broader than a minimal logging core
- dry_common_core_candidates: `LogLevel`, `LogSource`, `LogFieldValue`, `LogFields`, `ParentLogEvent`, `LogSnapshot`, and the redaction/path helpers
- dead_frontage_or_shims: `DevLogTarget::LegacyDir` and `resolve_compat_dev_log_path` are explicit legacy shims that should stay isolated from the core model
- proposed_fix_packets: separate legacy compatibility writing from the core event schema, and split out any future diagnostics or agent-run helpers before the crate becomes a second logging-domain package
- severity: medium
- confidence: medium-high
- evidence_paths: crates/logging-core/Cargo.toml, crates/logging-core/src/lib.rs, crates/logging-core/src/event.rs, crates/logging-core/src/dev_log.rs, crates/logging-core/src/snapshot.rs, crates/logging-core/src/redaction.rs, crates/logging-core/src/diagnostic.rs, crates/logging-core/src/field.rs, crates/logging-core/src/source.rs, crates/logging-core/src/ndjson_writer.rs

## Current Refresh Audit - 2026-06-19
- responsibility: low-level Rust logging model plus NDJSON persistence and redaction helpers; the crate still owns dev-log compatibility and diagnostics in the same boundary.
- deps: `chrono`, `serde`, `serde_json`, and `sha2`; no extra runtime crate dependencies are present in `Cargo.toml`.
- violations: the crate root is still a wide public frontage over event, dev-log, diagnostic, field, level, path, redaction, snapshot, and source modules, so ownership is broader than a narrow logging core.
- duplicated shapes: `ParentLogEvent` and `LogSnapshot` mirror the TS logging-domain entry/snapshot concepts; `DiagnosticKind` / `DiagnosticSeverity` / `AgentDiagnostic` duplicate downstream diagnostic envelopes that should stay adjacent to the logger but not define it.
- barrel/reexport/shim debt: no `pub use` barrels are present, but `src/lib.rs` is a broad module index, and `DevLogTarget::LegacyCompat` plus `CompatDevLogWriter` act as an explicit compatibility shim layer over the current storage model.
- schema drift: the log record and snapshot shapes are still stable, but `AgentDiagnostic` mixes runtime execution metadata with log-core ownership, which makes the crate look like a small platform instead of a narrow persistence library.
- event/log/proof misuse: `DevLogger` in `src/dev_log.rs` combines ID creation, env selection, compatibility routing, and record emission; that is acceptable for the current boundary but it also embeds policy and fallback behavior that downstream proofs must treat carefully.
- test/proof structure issues: the crate has unit and integration coverage, but the proof surface is implicit in the runtime modules rather than separated into a dedicated harness for NDJSON shape, redaction, and legacy compatibility.
- score: 59/100.
- fix recommendation: keep the log record and snapshot model canonical, but separate legacy compatibility writing and diagnostics as soon as the crate grows another public consumer.
- decouple recommendation: move any future agent-run or diagnostic envelope helpers into sibling modules or another crate so `logging-core` remains focused on record shape, redaction, and persistence.
- blockers: no hard blocker for the refresh; the only mismatch is that the crate still bundles compatibility, diagnostics, and persistence together.
- exact likely paths: `crates/logging-core/Cargo.toml`, `crates/logging-core/src/lib.rs`, `crates/logging-core/src/event.rs`, `crates/logging-core/src/dev_log.rs`, `crates/logging-core/src/snapshot.rs`, `crates/logging-core/src/diagnostic.rs`, `crates/logging-core/src/field.rs`, `crates/logging-core/src/source.rs`, `crates/logging-core/src/redaction.rs`, `crates/logging-core/src/compat_dev_log.rs`, `crates/logging-core/src/ndjson_writer.rs`.

## Current Refresh Audit - 2026-06-19
- responsibility: compact Rust logging core for log schema, field typing, source naming, redaction, NDJSON persistence, artifact emission, and legacy dev-log compatibility
- deps: narrow crate deps are `chrono`, `serde`, `serde_json`, and `sha2`; the code surface itself is std-heavy and does not pull in broader workspace APIs
- violations: the crate still bundles core event modeling with diagnostics, artifacts, and agent-run metadata; that is broader than a minimal logging core and creates mixed ownership
- duplicated_shapes: `ParentLogEvent`, `LogSnapshot`, `LogFieldValue`, `LogLevel`, and `LogSource` mirror TS-side logging-domain shapes; `AgentRunEvent`, `AgentDiagnostic`, and `ArtifactRef` repeat adjacent event-record patterns with the same camelCase/serde style
- barrel/reexport/shim debt: no TS/Rust re-export barrels were found in `crates/logging-core`; the main debt is the compatibility shim frontage in `compat_dev_log.rs` and the dual-path writer selection in `dev_log.rs`
- schema_drift: `ParentLogEvent`/`LogSnapshot` use Rust-specific names while the TS fixture uses `schemaVersion` and `DevLogEntry`; the crate-level naming and the fixture shape are close but not unified
- event_log_proof_misuse: `dev_log.rs` writes both scoped NDJSON and legacy flat files from the same logger entrypoint, which can obscure the active storage model; `write_agent_info` is still a convenience wrapper over persistence rather than a proof boundary
- test_proof_structure_issues: tests validate serialization, redaction, append ordering, and legacy output, but they remain one broad integration-style file instead of separate focused proof slices for schema, path safety, and compat routing
- score: 6/10
- fix_recommendation: split the legacy dev-log compat path out of the core event model boundary, then isolate diagnostics/artifact/agent-run emitters into their own owners or modules with explicit proof coverage
- decouple_recommendation: keep `field`, `level`, `source`, `event`, `snapshot`, `path`, and `redaction` as the stable core; move `compat_dev_log`, `dev_log`, `diagnostic`, `artifact`, and `agent_run` behind narrower call sites
- blockers: no direct blocker on audit refresh; the main limitation is that only the inspected crate surface and the existing fixture were used, so cross-crate TS parity is inferred from the observed naming and fixture shape
- exact_likely_paths: crates/logging-core/src/lib.rs, crates/logging-core/src/dev_log.rs, crates/logging-core/src/compat_dev_log.rs, crates/logging-core/src/event.rs, crates/logging-core/src/snapshot.rs, crates/logging-core/src/field.rs, crates/logging-core/src/level.rs, crates/logging-core/src/source.rs, crates/logging-core/src/path.rs, crates/logging-core/src/redaction.rs, crates/logging-core/src/diagnostic.rs, crates/logging-core/src/artifact.rs, crates/logging-core/src/agent_run.rs, crates/logging-core/tests/logging_core.rs, crates/logging-core/tests/fixtures/dev-log-entry.json

## Current Refresh Audit - 2026-06-19
- responsibility: narrow Rust logging core for log schema, field typing, source naming, redaction, NDJSON persistence, artifact emission, and legacy dev-log compatibility
- dependencies: `chrono`, `serde`, `serde_json`, `sha2`, and `std`; the crate stays self-contained and does not pull in broader workspace owners
- violations: the crate still mixes core event modeling with diagnostics, artifact writing, and agent-run metadata, so the ownership boundary is wider than a minimal logging core
- duplicated shapes: `ParentLogEvent`, `LogSnapshot`, `LogFieldValue`, `LogLevel`, and `LogSource` mirror the TS logging-domain shapes; `AgentDiagnostic`, `AgentRunEvent`, and `ArtifactRef` reuse the same envelope-and-metadata pattern with different names
- barrel/re-export/shim debt: there are no `pub use` barrels, but `src/lib.rs` is a broad module index and `compat_dev_log.rs` plus the legacy branch in `dev_log.rs` form an explicit compatibility shim layer
- schema drift: Rust keeps `ParentLogEvent` and `LogSnapshot` naming while the TS fixture and downstream logging-domain concepts use different entry/snapshot names, so the same log concept is represented by parallel shapes
- event/log/proof misuse: `write_agent_info` and `DevLogger` combine ID generation, env selection, routing, and emission, which makes the runtime storage choice harder to separate from proof intent
- test/proof structure issues: `tests/logging_core.rs` covers serialization, redaction, append ordering, artifact writing, fixture parity, and compat writing in one file, so proof slices are broader than the core schema boundary
- dry_score: 61/100 because the primitive log types are coherent and shared, but the crate still owns compatibility, diagnostics, artifacts, and agent-run helpers in one place
- fix recommendation: keep the core event, field, source, path, and redaction models canonical, then split legacy compatibility and adjacent emitters into narrower owners before more callers depend on them
- decouple recommendation: retain `field`, `level`, `source`, `event`, `snapshot`, `path`, and `redaction` as the stable core, and move `compat_dev_log`, `dev_log`, `diagnostic`, `artifact`, and `agent_run` behind separate call sites or sibling crates
- blockers: no hard blocker for the refresh; the only constraint is that this audit is based on the inspected crate surface and the local fixture rather than a full cross-crate parity sweep
- exact_likely_paths: crates/logging-core/src/lib.rs, crates/logging-core/src/dev_log.rs, crates/logging-core/src/compat_dev_log.rs, crates/logging-core/src/event.rs, crates/logging-core/src/snapshot.rs, crates/logging-core/src/field.rs, crates/logging-core/src/level.rs, crates/logging-core/src/source.rs, crates/logging-core/src/path.rs, crates/logging-core/src/redaction.rs, crates/logging-core/src/diagnostic.rs, crates/logging-core/src/artifact.rs, crates/logging-core/src/agent_run.rs, crates/logging-core/tests/logging_core.rs, crates/logging-core/tests/fixtures/dev-log-entry.json
