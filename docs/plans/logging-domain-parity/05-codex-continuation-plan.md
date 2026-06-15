# Codex Continuation Plan: Logging Domain Parity

## Summary
Use the existing TypeScript logging-domain as the contract surface, but finish the parity path by adding a reusable Rust logging core and a local developer-observability pipeline. The current gap is not the TS domain itself; it is the missing shared Rust logging primitive and the end-to-end local log flow needed for agent/service debugging.

## Key Changes
- Keep `packages/logging-domain` as the product/runtime-safe source of truth for logging types, transports, and app/test-log helpers.
- Add `crates/logging-core` as the Rust-side logging primitive instead of leaving `agent-service` with a one-off writer.
- Make local observability explicit and workspace-owned: NDJSON first, DuckDB/query second, bridge and command diagnostics as part of the same pipeline.
- Ensure portal and Rust callers either route into the new flow or are intentionally replaced, never silently dropped.
- Preserve the distinction between production custody/privacy proof contracts and local developer/agent observability.

## Validation
- TS: focused tests for transports, retention, and log serialization.
- Rust: `cargo check` and unit tests for the new crate and any direct consumers.
- Local smoke: prove logs are written, ingested, and queryable end to end.
- Negative coverage: missing bridge, missing endpoint, and invalid payload handling.

## Assumptions
- The local-observability work must not weaken the production-safe logging contracts.
- Scope names and local paths remain generic and must not hardcode a single deployment target.
- This file is a continuation note for GPT and human reviewers; it does not replace the numbered plan docs already in this folder.
