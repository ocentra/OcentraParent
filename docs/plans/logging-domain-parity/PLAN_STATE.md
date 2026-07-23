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

## Current ownership interpretation

```text
packages/logging-domain:
  TypeScript local logging helpers, bridge transport/server, NDJSON writers, DuckDB/query helpers, log-control/wipe/retention helpers, MCP query helpers, and proof-trace helper surfaces.

crates/logging-core:
  Rust NDJSON/artifact/dev-log/diagnostic/redaction/source/context helper crate.

scripts/dev:
  Local agent wrapper, query, evidence, MCP, and proof-trace entrypoints used by Codex/local development flows.

apps/portal:
  Dev-log producer/consumer path only when selected; portal owns UI/projection and cannot become the logging system owner.

crates/agent-service:
  Rust service producer/consumer path only when selected; agent-service owns service runtime behavior and consumes logging-core helpers.

cloudflare-control-plane-plan:
  Backend/Cloudflare infra logging owner when Cloudflare runtime logging is selected.

Product/support telemetry owners:
  Product-safe logging, support diagnostics, retention policy, and customer-facing telemetry policy remain outside this local parity plan unless a selected handoff names them.
```

## Current coupling risks

```text
- Local developer/agent evidence is not production telemetry readiness.
- MCP smoke proof is not full MCP interface readiness.
- Proof-trace smoke proof is not all product-flow proof coverage.
- Portal dev logger proof is not repo-wide portal instrumentation.
- Agent-service startup/dev-log proof is not full Rust logging adoption.
- Logging-domain package parity is not product runtime logging readiness.
- Proof-inventory query proof detects missing/stale roots; it does not restore or close missing roots by itself.
- Proof roots alone do not close checklist/workpack rows without focused commands and no-claim boundaries.
```

## Current proof interpretation

```text
output/logging-domain-parity-proof/<workpack>/ is the active proof route.
test-results/logging-domain-parity-* roots are supporting result roots and must exist before cited.
WP03, WP06, WP07, WP08, WP09, and WP10 have canonical proof roots in this checkout.
WP07 and WP10 proof roots exist, but checklist/workpack closeout remains open.
WP08 is a bounded partial-proof for portal dev logger, logging-domain source/context storage/query, and agent-service startup/dev-log only.
WP06 remains partial-proof while the root routing validation failure is outside this delegated logging-owned slice.
```

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
WP03 parent architecture/routing: the portal dev-log consumer slice now has a canonical proof root, parent scopes are defined in the logging package, and focused portal/logging checks pass; the broader workpack remains open only for the Rust-side agent-service mapping row outside this delegated slice
WP04 Rust logging core: source/tests present, but the named proof root is absent in this checkout
WP05 local validation evidence: source/tests/smokes present, but the named proof root is absent in this checkout
WP06 validation/enforcement: root checker scripts, wrapper scripts, and local evidence smoke are present; logging-owned proof-inventory query surfaces now detect missing/stale proof roots and stale closeout claims through agent-query/MCP plus focused tests, and the canonical WP06 proof root is present; full focused validation remains open because one root routing check fails against an owning surface outside this delegated slice
WP07 MCP query interface: server, integration coverage, and canonical MCP proof roots are present; fresh-root latest-failures/run-diagnostics/artifact-slice plus CLI parity now prove the deterministic local evidence path, but checklist/workpack closeout is still open
WP08 logger instrumentation/adoption: a canonical partial-proof root now exists for the portal dev logger path, logging-domain storage/query path, and agent-service startup/dev-log path; repo-wide adoption is still not proved
WP09 log control/retention/bridge lifecycle: focused log-decision, wipe, retention, and bridge lifecycle commands now have a canonical proof root; detailed checklist closeout remains open
WP10 proof trace pipeline: focused portal proof-trace tests pass, the standalone MCP proof-trace smoke is now self-seeding in a clean workspace, and the canonical proof root is present; checklist/workpack closeout is still open
Checklist state: WP03 now reflects its written proof root, WP06 now has 11/12 rows checked against focused proof, WP08 now has 8/12 rows checked against its canonical partial-proof root, and the remaining workpacks stay open as documented in CHECKLIST_INDEX.md
Proof inventory root: output/logging-domain-parity-proof/ now contains canonical WP03, WP06, WP07, WP08, WP09, and WP10 roots in this checkout
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
- Reconcile the remaining WP07/WP10 checklist closeout and keep WP08 scoped to its canonical partial-proof boundary instead of inflating it to repo-wide adoption
- Decide whether done in this plan means source present, proof present, or both; the current docs mix those states
- Close the remaining WP03 Rust-side route claim and hand off the root dev-log-routing failure to the owning portal/agent-service slice before claiming full WP06 focused-validation closure
```

## No-claim boundaries

Until implemented and validated, do not claim:

```text
logging-domain parity complete
full WP03 parent architecture/routing closure
agent-service logging migrated
proof trace coverage for product flows
production telemetry readiness
product runtime logging readiness
full MCP logging interface completion
repo-wide instrumentation adoption
```

## Workpack summary

Workpacks are indexed in `WORKPACK_INDEX.md`.

Current default execution order:

```text
1. remaining proof-inventory restoration or claim reduction for WP01/WP02/WP04/WP05/WP09 now that WP08 has a canonical partial-proof root
2. resolve or reduce the remaining WP03 Rust-side agent-service mapping claim under its owning slice
3. hand off the root lint:dev-log-routing failure to the owning portal/agent-service slice before claiming full WP06 focused-validation closure
```

## Health rules

- Do not touch other plan folders while working this plan.
- Do not change source before selecting one workpack.
- Do not claim DONE without focused commands and proof artifacts.
- Do not add more proof-only read models as a substitute for bridge/NDJSON/DuckDB/query/MCP/instrumentation/proof-trace implementation.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.
