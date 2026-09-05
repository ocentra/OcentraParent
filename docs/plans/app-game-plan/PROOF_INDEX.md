<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App Game Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# App Game Plan Proof Index

## Deterministic proof root

```text
output/app-game-plan-proof/<workpack-file-stem>/
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
plan: app-game-plan
workpack: <workpack id and name>
owner: agent-protocol | agent-core | app-game-core | agent-service | parent-runtime-core | apps/portal | schema-domain-generated-edge | android-agent | policy/enforcement-handoff | notification-handoff | platform-proof | docs-only
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
runtime-safe: no private launcher tokens, chat/content, raw screenshots, decrypted payloads, raw account identifiers, or child private activity payloads unless an explicit expectation allows the field.
local harness: enough file/line/command/artifact/evidence-ref/platform context for Codex/MCP/humans to debug without reading terminal walls.
```

## Required proof themes

```text
owned package/crate/route boundary
source/custody labels
unsupported/manual-required states
negative cases
portal proof when UI changes
service/protocol proof when runtime changes
source freshness proof before policy preview
authority proof before adapter action
platform capability proof before platform readiness
no historical checked-row overclaim
```

## No-claim language

Do not claim broad app/game product readiness from a single workpack, historical checklist row, generated read model, staged journal proof, portal row, policy dry-run, docs-only update, or mock/platform-preflight-only proof.
