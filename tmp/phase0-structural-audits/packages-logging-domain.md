# packages/logging-domain
- target_kind: TypeScript logging contract package with local observability and support-workflow surfaces
- owned_paths: packages/logging-domain/package.json, packages/logging-domain/README.md, packages/logging-domain/src/**, packages/logging-domain/scripts/**, packages/logging-domain/tests/**
- declared_responsibility: operational logging, audit history, redaction, provider custody, support workflow contracts, and local dev observability
- observed_responsibility: this package owns shared log schemas, transport and bridge helpers, NDJSON/test-log storage, app-log helpers, and many support workflow contracts in one boundary
- should_own: shared logging schemas, redaction-safe operational fields, and compact proof/read-model shapes that are genuinely domain-level
- should_not_own: heavy bridge/server/runtime plumbing, script orchestration, or broad support workflow execution if those can be split into narrower owners
- allowed_dependencies: `@ocentra-parent/event-domain`, `@ocentra-parent/schema-domain`, and `duckdb`, plus workspace-local tooling needed for observability helpers
- suspicious_dependencies: the package imports `AgentDeviceIdSchema` and `AgentPlatformSchema` from event-domain, which ties logging ownership to another domain just to reuse identity fields
- expected_dependents: portal dev observability, logging scripts, local proof/test harnesses, and Rust compatibility writers
- shared_contract_schema_usage: `withParser` and branded schemas are used correctly, but the package also mixes those contracts with transport, app-log, and bridge helpers instead of keeping a pure contract layer
- duplicate_or_near_duplicate_shapes: `AgentLogEntrySchema` and `DevLogEntrySchema` are almost the same record, and the TS log entry/snapshot shapes mirror the Rust logging-core event and snapshot structs
- id_name_status_drift: the package carries both `Agent*` and `Dev*` log names while Rust uses `ParentLogEvent`, so the same conceptual log record is named differently across the boundary
- direct_import_vs_event_boundary: `core/logger` and the transport/script exports make this package a mixed contract/runtime surface rather than a clean domain-only package
- event_bus_usage: none
- logging_and_proof_chain_expectations: this is the main logging proof owner, so any runtime or test-path change here should continue to emit structured evidence instead of growing ad hoc helpers
- boundary_violations: contracts, local observability, bridge transport, app-log storage, and a large support-workflow catalog all live together, which makes the package closer to a mini platform than a narrow logging domain
- dry_common_core_candidates: log levels, log sources, log field values, NDJSON writer shapes, bridge payloads, app-log storage helpers, and the shared read-model/guard lifecycle patterns
- dead_frontage_or_shims: the README describes three logging modes and the package exports both contracts and runtime helpers, so the package has a strong frontage layer that could hide the true ownership split
- proposed_fix_packets: split pure logging contracts from local observability and support workflow tooling, and define a logging-specific identity contract instead of borrowing agent event identities from event-domain
- severity: high
- confidence: high
- evidence_paths: packages/logging-domain/package.json, packages/logging-domain/README.md, packages/logging-domain/src/contracts.ts, packages/logging-domain/src/core/logger.ts, packages/logging-domain/src/core/logConfig.ts, packages/logging-domain/src/core/logDecisionProvider.ts

## Current Refresh Audit - 2026-06-19

- responsibility: operational logging, audit history, redaction, provider custody, support workflow contracts, local observability, and bridge/query helpers; the package still mixes pure contract leaves with runtime and tooling ownership.
- deps: `@ocentra-parent/schema-domain/effect`, `@ocentra-parent/event-domain/primitives`, Node `fs`/`http`/`path`/`url`/`crypto`, and local test-log/DuckDB helpers; `src/contracts.ts` still borrows agent identity schemas from `event-domain`.
- violations: contract schemas, runtime logger/bridge/server helpers, app-log persistence, and test-log storage all live in one boundary across `src/core/*`, `src/transport/*`, `src/app-log/*`, and `src/test-log/*`; the package also owns a large support-workflow catalog instead of staying logging-narrow.
- duplicated shapes: `AgentLogEntrySchema` and `DevLogEntrySchema`; `AppLogEntrySchema` mirrors the same log row fields with a second casing; bridge payload, stored test-log line, and app-log entry all repeat the same log-core shape with renamed keys; the support read models repeat `schemaVersion`, `status`, `manualRequired`, `auditRefs`, and `redactionRefs` patterns.
- barrel/reexport/shim debt: no TS/JS re-export barrels were found in `src`; the debt is a wide explicit export map in `package.json` plus a conceptual shim layer between the contract files and the runtime helpers in `src/core/logger.ts`, `src/transport/bridgeTransport.ts`, `src/transport/bridgeServer.ts`, and `src/app-log/createAppLogStorage.ts`.
- schema drift: `README.md` still describes structured logging/redaction, but the live code now also owns local dev observability, app-log persistence, bridge transport/server, and a support-proof catalog; the same conceptual log identity is split between `Agent*`, `Dev*`, and stored bridge/log row names.
- event/log/proof misuse: no event bus ownership was observed; `src/core/logger.ts` mixes policy checks, stack parsing, env resolution, and bridge dispatch, while the support workflow files are proof/policy catalogs rather than a narrow logging schema leaf.
- test/proof structure issues: coverage is concentrated in `tests/unit` with one integration smoke; there is no clearly separated executable proof layer for bridge/server/app-log/test-log behavior, so helper/runtime drift can happen without a focused proof harness.
- score: `34/100` with high confidence.
- fix recommendation: split pure logging/redaction/support-proof contracts from local observability and persistence/runtime helpers; keep `src/contracts.ts` and the support-proof files as the contract leaf, and move bridge/server/app-log/test-log behavior behind a smaller observability implementation boundary.
- decouple recommendation: first decouple `src/core/logger.ts` from transport dispatch, then peel `src/transport/bridgeServer.ts`, `src/app-log/createAppLogStorage.ts`, and `src/test-log/*` into a sibling implementation boundary that depends on the contract leaf but does not own it.
- blockers: none for this refresh; the source and markdown evidence were sufficient to refresh the audit without touching code.
- exact likely paths: `packages/logging-domain/package.json`, `packages/logging-domain/README.md`, `packages/logging-domain/src/contracts.ts`, `packages/logging-domain/src/core/logger.ts`, `packages/logging-domain/src/core/logConfig.ts`, `packages/logging-domain/src/core/logDecisionProvider.ts`, `packages/logging-domain/src/core/logRuntimeConstants.ts`, `packages/logging-domain/src/transport/bridgeTransport.ts`, `packages/logging-domain/src/transport/bridgeServer.ts`, `packages/logging-domain/src/transport/bridgeLogPayload.ts`, `packages/logging-domain/src/app-log/createAppLogStorage.ts`, `packages/logging-domain/src/app-log/types.ts`, `packages/logging-domain/src/test-log/types.ts`, `packages/logging-domain/src/test-log/bridgeConvert.ts`, `packages/logging-domain/src/test-log/ndjsonWriter.ts`, `packages/logging-domain/src/test-log/ndjsonPaths.ts`, `packages/logging-domain/src/support-proof-contract.ts`, `packages/logging-domain/src/package-info.ts`, `packages/logging-domain/tests/unit/package-exports.test.ts`.
