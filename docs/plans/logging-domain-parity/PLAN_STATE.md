<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Plan State`
> Kind: current state and open gaps.
> Read when: immediately after plan AGENTS.md.
> Stop rule: Do not continue into implementation docs unless this file routes you there.
> Proves: only current plan state and open gap accounting.
> Does not prove: implementation completion, validation, or PR readiness.
> Proof rule: If state changes, update matching workpack, checklist, and proof path.

<!-- /agent-capsule -->

# Logging Domain Parity Plan State

## Scope

This plan upgrades OcentraParent logging from a mostly schema/proof-contract package to a practical local development, agent observability, MCP query, and proof-trace pipeline.

No external `docs/features/*` or `docs/expectations/*` files are routed by this plan today. The numbered plan docs in this folder are the routed source-of-truth inputs.

## Current source docs

The source docs are routed through workpacks:

```text
00-current-state-and-reference-audit.md
01-parent-logging-architecture.md
02-rust-logging-core-crate.md
03-local-validation-evidence.md
04-validation-and-enforcement.md
05-codex-continuation-plan.md
06-mcp-query-interface.md
07-logger-instrumentation-pattern.md
08-log-control-retention-bridge-lifecycle.md
09-proof-trace-pipeline.md
```

## Current status

```text
Plan route: added
Workpack route: added
WP01 audit closeout: audit docs present, but the named proof root is absent in this checkout
WP02 TypeScript package parity: source/tests present, but the named proof root is absent in this checkout
WP03 parent architecture/routing: source/tests present; workpack status is stale; focused portal/routing checks pass but the named proof root is absent
WP04 Rust logging core: source/tests present, but the named proof root is absent in this checkout
WP05 local validation evidence: source/tests/smokes present, but the named proof root is absent in this checkout
WP06 validation/enforcement: scripts and focused checks present, but enforcement does not verify proof inventory or honest workpack completion; named proof root is absent
WP07 MCP query interface: server, integration coverage, and canonical MCP proof roots are present; fresh-root latest-failures/run-diagnostics/artifact-slice plus CLI parity now prove the deterministic local evidence path, but checklist/workpack closeout is still open
WP08 logger instrumentation/adoption: partial portal/agent-service adoption is present, but repo-wide adoption is not proved and the named proof root is absent
WP09 log control/retention/bridge lifecycle: source/tests present, but the named proof root is absent in this checkout
WP10 proof trace pipeline: focused portal proof-trace tests pass, the standalone MCP proof-trace smoke is now self-seeding in a clean workspace, and the canonical proof root is present; checklist/workpack closeout is still open
Checklist state: CHECKLIST_INDEX.md remains unchecked across the plan
Proof inventory root: output/logging-domain-parity-proof/ now exists in this checkout, but only WP07 and WP10 roots are restored so far
Test-results roots: test-results/logging-domain-parity-mcp/ and test-results/logging-domain-parity-proof-trace/ now exist; the other named test-results/logging-domain-parity-* roots are still absent
PR-ready: false
```

## What is already understood

- `ocentra-games/packages/logging-domain` is the reference implementation.
- Games also had log-query tools exposed to agents and SQL-vs-tool validation evidence.
- Games required source files to register with the shared logger pattern, not merely define a logging package.
- Games used log decision controls, fresh-run wipe, bridge run-start, retention, and optional tunnel routing.
- Parent logging-domain currently has live schema/contract usage but does not have games-level local logging pipeline parity.
- WP01 completed the local MCP audit and confirmed no reusable parent MCP framework was found, so WP07 can implement the parent logging MCP layer if that remains true.
- Parent now has substantial source/test coverage for TypeScript package parity, Rust logging core, log-control/retention/bridge lifecycle, local validation evidence, MCP query interface support, logger instrumentation/adoption support, proof trace pipeline support, and validation/enforcement support, but the plan docs overclaim proof-backed closeout for those slices in this checkout.
- Local development observability is separate from production/product safe logging.
- Codex/local agents should consume compact deterministic evidence through CLI and MCP, not full raw terminal logs.
- The same log pipeline should also collect proof traces for Playwright/service/runtime paths.

## Open gaps

```text
- Recreate or remove the remaining claimed proof roots under output/logging-domain-parity-proof/*
- Recreate or remove the remaining claimed test-results/logging-domain-parity-* roots
- Reconcile WP03/WP06/WP07/WP10 workpack docs with current source and smoke behavior
- Decide whether "done" in this plan means source present, proof present, or both; the current docs mix those states
- Close WP03 portal/dev-log routing proof and WP06 enforcement proof against the now-restored WP07/WP10 inventory
```

## No-claim boundaries

Until implemented and validated, do not claim:

```text
logging-domain parity complete
portal dev-log routing fixed
agent-service logging migrated
proof trace coverage for product flows
production telemetry readiness
product runtime logging readiness
```

## Workpack summary

Workpacks are indexed in `WORKPACK_INDEX.md`.

Current default execution order:

```text
1. WP03 parent architecture/routing truthful closeout
2. WP06 enforcement hardening for proof inventory and honest closeout checks
3. remaining proof-inventory restoration or claim reduction for WP01/WP02/WP04/WP05/WP08/WP09
```

## Health rules

- Do not touch other plan folders while working this plan.
- Do not change source before selecting one workpack.
- Do not claim DONE without focused commands and proof artifacts.
- Do not add more proof-only read models as a substitute for bridge/NDJSON/DuckDB/query/MCP/instrumentation/proof-trace implementation.
