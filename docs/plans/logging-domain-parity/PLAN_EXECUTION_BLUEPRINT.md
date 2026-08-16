<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs the exact execution order.
> Stop rule: Choose one workpack; do not implement multiple workpacks unless assigned.
> Proves: execution routing only.
> Does not prove: implementation completion or PR readiness.

<!-- /agent-capsule -->

# Logging Domain Parity Execution Blueprint

## Execution order

Default order:

```text
1. WP01 Current State and Reference Audit
2. WP02 TypeScript Logging Package Parity
3. WP03 Parent Logging Architecture and Routing
4. WP09 Log Control, Retention, and Bridge Lifecycle
5. WP04 Rust Logging Core Crate
6. WP05 Local Validation Evidence
7. WP07 MCP Query Interface
8. WP08 Logger Instrumentation and Adoption
9. WP10 Proof Trace Pipeline
10. WP06 Validation and Enforcement
```

## Parallelism

Allowed only with explicit assignment:

```text
WP02 and WP04 may run in parallel if fixture/export boundaries are coordinated.
WP03 may run with WP02 if portal bridge contracts are clear.
WP09 waits for bridge/path helpers from WP02.
WP05 waits for storage primitives from WP02/WP04/WP09.
WP07 waits until enough queryable data exists from WP02/WP05.
WP08 waits until relevant logger APIs exist from WP02/WP04, but can begin with narrow portal/service adoption.
WP10 waits until proof-mode controls, instrumentation, and query service exist.
WP06 waits until files it checks exist.
```

## Codex implementation startup prompt

Use this prompt after syncing the branch:

```text
You are working in OcentraParent on branch codex/tracking-plan-full-continuation-a.
Work only in docs/plans/logging-domain-parity assigned scope and the source files named by the selected workpack.
Read docs/plans/logging-domain-parity/README.md, AGENTS.md, PLAN_STATE.md, NEXT_ACTIONS.md, WORKPACK_INDEX.md.
Select exactly one assigned workpack.
Do not touch other plan folders.
Do not add proof-only changes before source behavior and tests unless the workpack says proof-routing-only.
Follow PLAN -> CODE -> TEST -> RUN/FIX -> PROOF -> DOC.
```

## Reference repo fetch command

When implementation needs the games reference:

```bash
rm -rf /tmp/ocentra-games-ref
git clone --depth=1 https://github.com/ocentra/ocentra-games.git /tmp/ocentra-games-ref
```

Reference paths:

```text
/tmp/ocentra-games-ref/packages/logging-domain
/tmp/ocentra-games-ref/vite/utils/testLogStorage.ts
/tmp/ocentra-games-ref/vite/utils/__tests__/mcp-validation-report.html
/tmp/ocentra-games-ref/.cursor/rules/ocentra-cloudflare-logging.mdc
/tmp/ocentra-games-ref/.cursor/rules/ocentra-cloudflare-logging.mdc
/tmp/ocentra-games-ref/infra/cloudflare/src/logging/log-config.ts
/tmp/ocentra-games-ref/infra/cloudflare/scripts/run-suite-helper.ts
/tmp/ocentra-games-ref/infra/cloudflare/test-runner/script/report/summary-reporter.ts
/tmp/ocentra-games-ref/AGENTS.md
```

Use reference code as a pattern. Do not blindly copy project-specific defaults.

## MCP design rule

Build or adapt a normal local MCP server for the agent interface.

Do not make Vite middleware the required MCP server unless the selected client actually requires it. Vite/local middleware may remain a log producer or dev bridge, but the MCP server should query the shared DuckDB/NDJSON query service directly.

## Proof trace design rule

Proof trace is a run-scoped mode, not a permanent global logging level.

Tests may enable selected sources/levels, run the UI/API/runtime path, flush/ingest, query the ordered trace, write proof artifacts, and then disable/clean the proof mode.

## Pre-edit note

Before editing source, write:

```text
Assigned workpack:
Implementation slice:
Source files expected to change:
Tests expected to change:
Proof root:
No-claim boundaries:
```

## During implementation

Use narrow source ownership:

```text
TypeScript package parity -> packages/logging-domain/**
Parent route fix -> apps/portal/** and crates/agent-service/** only as needed
Lifecycle controls -> logging-domain core/test-log/app-log/bridge scripts only
Rust logging core -> crates/logging-core/** and agent-service migration points
Local evidence wrappers -> scripts/dev/** and package/root scripts
MCP query interface -> packages/logging-domain/src/query/**, src/mcp/**, scripts/mcp-logging-server.ts
Logger instrumentation -> assigned portal/service/script surfaces only
Proof trace -> query/proof-trace helpers plus one selected vertical smoke path
Validation -> scripts/check-*.mjs and root scripts
```

## Proof update

After focused commands pass, write proof artifacts under the selected workpack root only.

Do not update sibling proof roots.

## Completion report

Use:

```text
Assigned workpack:
Real source behavior added:
Files changed:
Tests added/changed:
Focused commands run:
Proof artifacts:
Checklist/docs updated:
No-claim boundaries preserved:
Remaining gaps:
```

## Global no-touch rule

This plan must not modify other plan folders unless explicitly assigned by the user.

Specifically do not modify:

```text
docs/plans/tracking-plan/
docs/plans/eventing-plan/
docs/plans/* other than logging-domain-parity
```

while executing logging-domain parity work.
