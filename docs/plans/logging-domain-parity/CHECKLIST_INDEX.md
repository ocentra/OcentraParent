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

- [ ] Reference games logging-domain files inspected.
- [ ] Parent current package files inspected.
- [ ] Live usage of parent logging-domain confirmed.
- [ ] Existing parent MCP presence or absence confirmed.
- [ ] Dead-code or split-route risks documented.
- [ ] Reference-to-target mapping confirmed.
- [ ] Existing parent exports listed before code changes.
- [ ] No unrelated plan folders touched.
- [ ] Proof root written.
- [ ] Workpack completion section filled.

## WP02 TypeScript Logging Package Parity

- [ ] `src/test-log` parity modules added/adapted.
- [ ] `src/transport` parity modules added/adapted.
- [ ] `src/app-log` parity modules added/adapted or explicit deferral recorded.
- [ ] `scripts/log-bridge.ts` added.
- [ ] DB ensure/rebuild/ingest/query/view scripts added.
- [ ] Package exports updated explicitly.
- [ ] Existing proof/contract exports preserved.
- [ ] Parent scopes added without generic Cloudflare default.
- [ ] TypeScript tests added/updated.
- [ ] Focused package build/test commands pass.
- [ ] Proof root written.
- [ ] Workpack completion section filled.

## WP03 Parent Logging Architecture and Routing

- [ ] Local-dev-observability and product-safe logging separated in docs/API.
- [ ] Parent scopes defined.
- [ ] Portal dev-log route implemented or moved to bridge path.
- [ ] Agent-service current logging path mapped to Rust crate migration.
- [ ] `/api/dev/log-snapshot` role documented as snapshot, not primary store.
- [ ] Cloudflare infra scope kept separate.
- [ ] README/package docs updated.
- [ ] Route tests or smoke checks added.
- [ ] Focused commands pass.
- [ ] Proof root written.
- [ ] Workpack completion section filled.

## WP09 Log Control, Retention, and Bridge Lifecycle

- [ ] Log decision provider implemented.
- [ ] Error/warn are always stored.
- [ ] Info/debug/log are controlled by environment/source/file/run selection.
- [ ] Console and storage decisions are separate.
- [ ] Fresh-run wipe can wipe selected scope/run/suite/file.
- [ ] Retention cleanup keeps configurable recent local sessions/files.
- [ ] Bridge health check exists.
- [ ] Bridge run-start endpoint records current run metadata.
- [ ] Stale run info is rejected or warned.
- [ ] Local bridge is default.
- [ ] Tunnel bridge mode is optional and condition-gated.
- [ ] Tests cover controls, wipe, retention, and bridge lifecycle.
- [ ] Proof root and workpack completion section filled.

## WP04 Rust Logging Core Crate

- [ ] `crates/logging-core` created.
- [ ] Workspace manifest updated.
- [ ] Rust log event types added.
- [ ] NDJSON writer added.
- [ ] Artifact writer added.
- [ ] Redaction helpers added.
- [ ] Agent run/diagnostic structs added.
- [ ] Agent-service delegates dev logging to logging-core.
- [ ] Rust tests added.
- [ ] TS/Rust fixture parity tests added.
- [ ] Focused cargo/npm commands pass.
- [ ] Proof root and workpack completion filled.

## WP05 Local Validation Evidence

- [ ] `agent:run` root script added.
- [ ] `agent:query` root script added.
- [ ] `codex:evidence` root script added.
- [ ] `scripts/dev/agent-run.mjs` implemented.
- [ ] `scripts/dev/agent-query.mjs` implemented.
- [ ] `scripts/dev/codex-evidence.mjs` implemented.
- [ ] Local artifact layout implemented.
- [ ] Agent run/diagnostics/artifacts NDJSON streams written.
- [ ] DuckDB tables/indexes added.
- [ ] Diagnostic parsers added for first supported toolchain set.
- [ ] Smoke test passes.
- [ ] Proof root and workpack completion filled.

## WP07 MCP Query Interface

- [ ] Existing parent MCP framework audited.
- [ ] Existing MCP reused/upgraded or absence recorded.
- [ ] Shared query service added for CLI and MCP.
- [ ] MCP server script added or existing server extended.
- [ ] `mcp:logging` package/root script added.
- [ ] Error query tool implemented.
- [ ] Recent logs query tool implemented.
- [ ] Source filter query tool implemented.
- [ ] Context filter query tool implemented.
- [ ] Flexible query tool implemented.
- [ ] Stats query tool implemented.
- [ ] Latest failures query tool implemented.
- [ ] Run diagnostics query tool implemented.
- [ ] Bounded local file slice query tool implemented.
- [ ] MCP and CLI share data access code.
- [ ] MCP smoke tests pass.
- [ ] Agent guidance documents MCP-first, CLI-fallback behavior.
- [ ] Proof root and workpack completion section filled.

## WP08 Logger Instrumentation and Adoption

- [ ] Parent TypeScript logger usage pattern implemented or documented at API boundary.
- [ ] Parent Rust logger usage pattern implemented through logging-core.
- [ ] Portal dev/runtime logging uses parent logger instead of ad hoc fetch/console path.
- [ ] Agent-service startup/health/dev diagnostics use logging-core.
- [ ] Validation/evidence scripts log run_id and command_id where useful.
- [ ] At least one TypeScript runtime path produces source/context fields queryable from storage.
- [ ] At least one Rust service path produces source/context fields queryable from storage or fixture output.
- [ ] Tests verify registered source/context fields are preserved.
- [ ] Checks prevent new raw console logging in touched logging surfaces.
- [ ] Checks prevent ad hoc JSON log writers outside logging-domain/logging-core.
- [ ] MCP or CLI query proof shows useful source/context values.
- [ ] Proof root and workpack completion section filled.

## WP10 Proof Trace Pipeline

- [ ] Proof trace mode controls added.
- [ ] Proof rows include proof_id and correlation_id.
- [ ] Proof rows include source/context/action/event fields.
- [ ] Query service can fetch a proof trace by proof_id.
- [ ] Query service can validate ordered expected steps.
- [ ] Query service reports missing/out-of-order steps.
- [ ] One Playwright or equivalent UI-to-result proof trace smoke exists.
- [ ] Proof trace can be flushed/ingested before assertion.
- [ ] Proof trace can be queried through CLI.
- [ ] Proof trace can be queried through MCP or has explicit MCP-followup blocker.
- [ ] Proof mode is disabled/cleaned after the test.
- [ ] Retention/wipe prevents stale proof traces from polluting normal evidence.
- [ ] Proof root and workpack completion section filled.

## WP06 Validation and Enforcement

- [ ] `check-logging-domain-parity.mjs` added.
- [ ] `check-local-evidence-wrapper.mjs` added.
- [ ] `check-dev-log-routing.mjs` added.
- [ ] `check-logging-exports.mjs` added.
- [ ] Root scripts added.
- [ ] Validation chain updated at safe point.
- [ ] Logging evidence smoke script added.
- [ ] Agent guidance references wrapper usage.
- [ ] Negative/failure checks verified.
- [ ] Focused validation passes.
- [ ] Proof root written.
- [ ] Workpack completion section filled.
