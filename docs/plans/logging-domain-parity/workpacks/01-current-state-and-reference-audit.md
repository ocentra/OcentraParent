<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `WP01 Current State and Reference Audit`
> Kind: assigned workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: Do not open sibling workpacks unless this workpack routes there.
> Proves: current-state/reference mapping only.
> Does not prove: implementation completion, package parity, or PR readiness.
> Proof rule: Before DONE, write proof artifacts under this workpack proof root.

<!-- /agent-capsule -->

# WP01 Current State and Reference Audit

## Purpose

Confirm the actual current parent state and reference games implementation before code edits.

This prevents Codex from implementing from memory or interpreting `logging-domain` as more proof/read-model contracts.

## Source inputs

Read only these by default:

```text
docs/plans/logging-domain-parity/00-current-state-and-reference-audit.md
docs/plans/logging-domain-parity/01-parent-logging-architecture.md
docs/plans/logging-domain-parity/06-mcp-query-interface.md
OcentraParent/package.json
OcentraParent/packages/logging-domain/package.json
OcentraParent/packages/logging-domain/src/contracts.ts
OcentraParent/apps/portal/src/dev-logger.ts
OcentraParent/crates/agent-service/src/dev_log.rs
OcentraParent/crates/agent-service/src/app.rs
ocentra-games/packages/logging-domain/package.json
ocentra-games/packages/logging-domain/src/test-log/**
ocentra-games/packages/logging-domain/src/transport/**
ocentra-games/packages/logging-domain/src/app-log/**
ocentra-games/packages/logging-domain/scripts/**
ocentra-games/AGENTS.md MCP tool list
ocentra-games/.cursor/rules/ocentra-games-rules.mdc MCP guidance
```

## Target state

A proof-backed audit exists showing:

```text
- games reference files mapped
- parent current files mapped
- live parent usage identified
- existing parent MCP presence or absence confirmed
- missing parity capabilities listed
- implementation workpack routing confirmed
```

## Existing MCP audit

Before implementation, check whether parent already has an MCP system from earlier roadmap work.

Remote pre-check found no obvious MCP implementation, but Codex must confirm locally with:

```bash
git grep -ni "mcp\|model context protocol\|modelcontextprotocol" -- .
find . -iname '*mcp*' -o -iname '*modelcontext*'
```

Record result in:

```text
output/logging-domain-parity-proof/01-current-state-and-reference-audit/04-existing-mcp-audit.json
```

If an existing MCP system is found, WP07 must upgrade/adapt it instead of creating a second system.

## Required proof root

```text
output/logging-domain-parity-proof/01-current-state-and-reference-audit/
```

Required artifacts:

```text
00-reference-file-map.json
01-parent-current-state.json
02-live-usage-map.json
03-gap-summary.md
04-existing-mcp-audit.json
16-validation-commands.log
```

## Checklist rows

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

## Implementation notes

This is audit work. Do not modify source code unless a path is clearly broken and the user explicitly asks for repair in the same turn.

The audit must distinguish:

```text
live dependency / schema usage
real logging pipeline
proof-only contract
local development observability
product/runtime safe logging
existing MCP framework vs no MCP framework
```

## Focused commands

```bash
node -e "console.log('logging parity audit complete after artifact generation')"
```

## Manual-required gaps

This workpack does not close any implementation gap. It only confirms the map.

## Fill before DONE or PR-ready

```text
Workpack id and branch:
Touched files:
Validation commands and results:
Proof artifacts:
Product/runtime claims:
Known gaps/manual-required states:
```

Do not report implementation completion from this workpack.

## Current audit note

This workpack is still useful as the routing and current-state inventory slice,
but the appended completion record was stale. The named proof root
`output/logging-domain-parity-proof/01-current-state-and-reference-audit/` is
absent in this checkout, so the earlier artifact list is not currently
provable from workspace state.

During the June 16, 2026 audit, the plan/workpack docs, owning package/crate
surfaces, and focused validation paths were re-read, but that re-check does not
restore the missing proof pack. Keep WP01 audit-only until proof inventory and
status claims are reconciled elsewhere in the plan.
