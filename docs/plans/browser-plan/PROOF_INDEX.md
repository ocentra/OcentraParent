<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Browser Plan Proof Index

## Deterministic proof root

```text
output/browser-plan-proof/<workpack-file-stem>/
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
plan: browser-plan
workpack: <workpack id and name>
owner: schema-domain | browser-domain | browser-core | agent-protocol | agent-service | portal-domain | apps/portal | policy/enforcement-handoff | ai-handoff | platform-proof | docs-only
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
correlation_id: <runtime/proof/evidence/tab/action correlation id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <raw stdout/stderr artifact pointer, test-results path, proof file, screenshot/report path, platform output path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
source_custody_note: <managed-profile | unmanaged-detection | local evidence ref | opaque source ref | parent-visible status only | n/a>
browser_boundary_note: <exact-url-claimed | active-tab-claimed | target-list-only | unmanaged-no-exact-url | no-content | n/a>
platform_note: <os/browser/version/permission/capability/manual-required note or n/a>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store raw stdout/stderr, Playwright artifacts, browser bridge logs, CDP target dumps, screenshots, service traces, or long failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Runtime and local harness split

Runtime/product-safe proof must show source, custody, redaction, capability, active-tab proof state, and authority boundaries. Local harness proof may include richer diagnostics, but it still stores raw logs by pointer and keeps plan docs compact.

```text
runtime-safe: no cookies, tokens, local storage, page bodies, form values, chat content, screenshots, decrypted payloads, child private activity payloads, or unmanaged exact URLs unless an explicit expectation allows the field.
local harness: enough file/line/command/artifact/evidence-ref/browser-boundary/platform context for Codex/MCP/humans to debug without reading terminal walls.
```

## Required proof themes

```text
managed/unmanaged distinction
source and custody labels
unsupported browser states
degraded/manual-required states
portal display proof when UI changes
service/protocol proof when runtime changes
no raw private content by default
active tab proof before exact active URL claim
policy/action/audit proof before intervention readiness
no stale generated-summary overclaim
```

## No-claim language

Do not claim broad browser product readiness from a single workpack, historical checked summary, reference/settings inventory, CDP target list alone, unmanaged process detection, managed intervention harness, portal UI, policy authoring, docs-only update, mock evidence, or platform-preflight-only proof.
