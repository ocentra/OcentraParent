<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Checklist Index`
> Kind: exact checklist router.
> Read when: a selected workpack references checklist rows.
> Stop rule: Do not scan unrelated checklist rows.
> Proves: checklist routing only.
> Does not prove: implementation completion.
> Proof rule: A checkbox can be checked only after proof artifacts and focused command results exist.

<!-- /agent-capsule -->

# Logging Domain Parity Checklist Index

This checklist tracks execution only. Proof artifacts are collected under:

```text
output/logging-domain-parity-proof/<workpack-id>/
test-results/logging-domain-parity-<proof-mode>/
```

## Fill rules

- Leave a checkbox unchecked until proof exists.
- Every checked row must cite one or more proof artifacts.
- Every proof item must list commands run, pass/fail/blocker, and no-claim boundaries.
- Do not mark parity complete from docs-only changes.
- Do not mark product runtime logging complete from local-dev evidence.

## WP01 Current State and Reference Audit

- [x] Reference games logging-domain files inspected.
- [x] Parent current package files inspected.
- [x] Live usage of parent logging-domain confirmed.
- [x] Existing parent MCP presence or absence confirmed.
- [x] Dead-code or split-route risks documented.
- [x] Reference-to-target mapping confirmed.
- [x] Existing parent exports listed before code changes.
- [x] No unrelated plan folders touched.
- [x] Proof root written.
- [x] Workpack completion section filled.

## WP02 TypeScript Logging Package Parity

- [x] `src/test-log` parity modules added/adapted.
- [x] `src/transport` parity modules added/adapted.
- [x] `src/app-log` parity modules added/adapted or explicit deferral recorded.
- [x] `scripts/log-bridge.ts` added.
- [x] DB ensure/rebuild/ingest/query/view scripts added.
- [x] Package exports updated explicitly.
- [x] Existing proof/contract exports preserved.
- [x] Parent scopes added without generic Cloudflare default.
- [x] TypeScript tests added/updated.
- [x] Focused package build/test commands pass.
- [x] Proof root written.
- [x] Workpack completion section filled.

## WP03 Parent Logging Architecture and Routing

- [x] Local-dev-observability and product-safe logging separated in docs/API.
- [x] Parent scopes defined.
- [x] Portal dev-log route implemented or moved to bridge path.
- [x] Agent-service current logging path mapped to Rust crate migration.
- [x] `/api/dev/log-snapshot` role documented as snapshot, not primary store.
- [x] Cloudflare infra scope kept separate.
- [x] README/package docs updated.
- [x] Route tests or smoke checks added.
- [x] Focused commands pass.
- [x] Proof root written.
- [x] Workpack completion section filled.

## WP09 Log Control, Retention, and Bridge Lifecycle

- [x] Log decision provider implemented.
- [x] Error/warn are always stored.
- [x] Info/debug/log are controlled by environment/source/file/run selection.
- [x] Console and storage decisions are separate.
- [x] Fresh-run wipe can wipe selected scope/run/suite/file.
- [x] Retention cleanup keeps configurable recent local sessions/files.
- [x] Bridge health check exists.
- [x] Bridge run-start endpoint records current run metadata.
- [x] Stale run info is rejected or warned.
- [x] Local bridge is default.
- [x] Tunnel bridge mode is optional and condition-gated.
- [x] Tests cover controls, wipe, retention, and bridge lifecycle.
- [x] Proof root and workpack completion section filled.

## WP04 Rust Logging Core Crate

- [x] `crates/logging-core` created.
- [x] Workspace manifest updated.
- [x] Rust log event types added.
- [x] NDJSON writer added.
- [x] Artifact writer added.
- [x] Redaction helpers added.
- [x] Agent run/diagnostic structs added.
- [x] Agent-service delegates dev logging to logging-core.
- [x] Rust tests added.
- [x] TS/Rust fixture parity tests added.
- [x] Focused cargo/npm commands pass.
- [x] Proof root and workpack completion filled.

## WP05 Local Validation Evidence

- [x] `agent:run` root script added.
- [x] `agent:query` root script added.
- [x] `codex:evidence` root script added.
- [x] `scripts/dev/agent-run.mjs` implemented.
- [x] `scripts/dev/agent-query.mjs` implemented.
- [x] `scripts/dev/codex-evidence.mjs` implemented.
- [x] Local artifact layout implemented.
- [x] Agent run/diagnostics/artifacts NDJSON streams written.
- [x] DuckDB tables/indexes added.
- [x] Diagnostic parsers added for first supported toolchain set.
- [x] Smoke test passes.
- [x] Proof root and workpack completion filled.

## WP07 MCP Query Interface

- [x] Existing parent MCP framework audited.
- [x] Existing MCP reused/upgraded or absence recorded.
- [x] Shared query service added for CLI and MCP.
- [x] MCP server script added or existing server extended.
- [x] `mcp:logging` package/root script added.
- [x] Error query tool implemented.
- [x] Recent logs query tool implemented.
- [x] Source filter query tool implemented.
- [x] Context filter query tool implemented.
- [x] Flexible query tool implemented.
- [x] Stats query tool implemented.
- [x] Latest failures query tool implemented.
- [x] Run diagnostics query tool implemented.
- [x] Bounded local file slice query tool implemented.
- [x] MCP and CLI share data access code.
- [x] MCP smoke tests pass.
- [x] Agent guidance documents MCP-first, CLI-fallback behavior.
- [x] Proof root and workpack completion section filled.

## WP08 Logger Instrumentation and Adoption

- [x] Parent TypeScript logger usage pattern implemented or documented at API boundary.
- [x] Parent Rust logger usage pattern implemented through logging-core.
- [x] Portal dev/runtime logging uses parent logger instead of ad hoc fetch/console path.
- [x] Agent-service startup/health/dev diagnostics use logging-core.
- [x] Validation/evidence scripts log run_id and command_id where useful.
- [x] At least one TypeScript runtime path produces source/context fields queryable from storage.
- [x] At least one Rust service path produces source/context fields queryable from storage or fixture output.
- [x] Tests verify registered source/context fields are preserved.
- [x] Checks prevent new raw console logging in touched logging surfaces.
- [x] Checks prevent ad hoc JSON log writers outside logging-domain/logging-core.
- [x] MCP or CLI query proof shows useful source/context values.
- [x] Proof root and workpack completion section filled.

## WP10 Proof Trace Pipeline

- [x] Proof trace mode controls added.
- [x] Proof rows include proof_id and correlation_id.
- [x] Proof rows include source/context/action/event fields.
- [x] Query service can fetch a proof trace by proof_id.
- [x] Query service can validate ordered expected steps.
- [x] Query service reports missing/out-of-order steps.
- [x] One Playwright or equivalent UI-to-result proof trace smoke exists.
- [x] Proof trace can be flushed/ingested before assertion.
- [x] Proof trace can be queried through CLI.
- [x] Proof trace can be queried through MCP or has explicit MCP-followup blocker.
- [x] Proof mode is disabled/cleaned after the test.
- [x] Retention/wipe prevents stale proof traces from polluting normal evidence.
- [x] Proof root and workpack completion section filled.

## WP06 Validation and Enforcement

- [x] `check-logging-domain-parity.mjs` added.
- [x] `check-local-evidence-wrapper.mjs` added.
- [x] `check-dev-log-routing.mjs` added.
- [x] `check-logging-exports.mjs` added.
- [x] Root scripts added.
- [x] Validation chain updated at safe point.
- [x] Logging evidence smoke script added.
- [x] Agent guidance references wrapper usage.
- [x] Negative/failure checks verified.
- [x] Focused validation passes.
- [x] Proof root written.
- [x] Workpack completion section filled.
