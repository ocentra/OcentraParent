<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `Native Apps Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Native Apps Plan Proof Index

## Deterministic proof root

For any selected workpack:

```text
output/app-plan-proof/<workpack-file-stem>/
```

## Required universal proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## Command log format

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```

If blocked:

```text
blocker:
required environment:
why this does not prove completion:
next command:
```

## Structured proof metadata

For new proof artifacts and new command-log entries, include structured metadata when available:

```text
plan: app-plan
workpack: <workpack id and name>
owner: schema-domain | app-core | agent-protocol | agent-service | portal-domain | apps/portal | app-game-handoff | policy/enforcement-handoff | notification-handoff | platform-proof | docs-only
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
correlation_id: <runtime/proof/evidence/session/action correlation id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <raw stdout/stderr artifact pointer, test-results path, proof file, screenshot/report path, platform output path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
source_custody_note: <local evidence ref | opaque source ref | parent-visible status only | n/a>
platform_note: <os/version/permission/capability/manual-required note or n/a>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store raw stdout/stderr, Playwright artifacts, platform logs, screenshots, service traces, or long failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Runtime and local harness split

Runtime/product-safe proof must show source, custody, redaction, capability, and authority boundaries. Local harness proof may include richer diagnostics, but it still stores raw logs by pointer and keeps plan docs compact.

```text
runtime-safe: no private account tokens, chat/content, raw screenshots, decrypted payloads, raw account identifiers, or child private activity payloads unless an explicit expectation allows the field.
local harness: enough file/line/command/artifact/evidence-ref/platform context for Codex/MCP/humans to debug without reading terminal walls.
```

## No-claim language

Do not claim platform/app/runtime readiness unless the selected proof root proves it and the rollout gate consumes it.

Do not claim broad native-app readiness from route normalization, package preview/scaffold, app-game-plan proof without a named app-only handoff, portal row, policy dry-run, docs-only update, mock evidence, or platform-preflight-only proof.
