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

> **Live-code audit (2026-07-17):** [Project Progress Matrix](../../PLAN_CODE_STATUS_MATRIX.md) records current implementation, blockers, dependencies, and next unblocker. Rows remain proof-gated; this audit does not check unsupported work.

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

- [x] Local-dev-observability and product-safe logging separated in docs/API.
- [x] Parent scopes defined.
- [x] Portal dev-log route implemented or moved to bridge path.
- [ ] Agent-service current logging path mapped to Rust crate migration.
- [x] `/api/dev/log-snapshot` role documented as snapshot, not primary store.
- [x] Cloudflare infra scope kept separate.
- [x] README/package docs updated.
- [x] Route tests or smoke checks added.
- [x] Focused commands pass.
- [x] Proof root written.
- [x] Workpack completion section filled.

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

Proof custody: regenerated locally at `output/logging-domain-parity-proof/04-rust-logging-core-crate/` against source commit `268b1f93d01d8410bf1fa5eccacf79dedad4f9aa`; the ignored root contains the five artifacts named by WP04 and records both the normal and `test-support` validation commands.

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

- [x] Parent TypeScript logger usage pattern implemented or documented at API boundary.
- [x] Parent Rust logger usage pattern implemented through logging-core.
- [x] Portal dev/runtime logging uses parent logger instead of ad hoc fetch/console path.
- [ ] Agent-service startup/health/dev diagnostics use logging-core.
- [ ] Validation/evidence scripts log run_id and command_id where useful.
- [x] At least one TypeScript runtime path produces source/context fields queryable from storage.
- [x] At least one Rust service path produces source/context fields queryable from storage or fixture output.
- [x] Tests verify registered source/context fields are preserved.
- [ ] Checks prevent new raw console logging in touched logging surfaces.
- [ ] Checks prevent ad hoc JSON log writers outside logging-domain/logging-core.
- [x] MCP or CLI query proof shows useful source/context values.
- [x] Proof root and workpack completion section filled.

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

- [x] `check-logging-domain-parity.mjs` added.
- [x] `check-local-evidence-wrapper.mjs` added.
- [x] `check-dev-log-routing.mjs` added.
- [x] `check-logging-exports.mjs` added.
- [x] Root scripts added.
- [x] Validation chain updated at safe point.
- [x] Logging evidence smoke script added.
- [x] Agent guidance references wrapper usage.
- [x] Negative/failure checks verified.
- [ ] Focused validation passes.
- [x] Proof root written.
- [x] Workpack completion section filled.
