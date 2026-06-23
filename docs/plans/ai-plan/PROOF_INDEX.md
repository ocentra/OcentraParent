<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# AI Plan Proof Index

## Deterministic proof root

For any selected workpack:

```text
output/ai-plan-proof/<workpack-file-stem>/
```

Example:

```text
workpacks/07-ai-job-queue-contract.md
output/ai-plan-proof/07-ai-job-queue-contract/
```

## Required universal proof files

Every proof root needs:

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

Add workpack-specific proof files when the selected workpack names them.

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
plan: ai-plan
workpack: <workpack id and name>
owner: schema-domain | ai-domain | child-ai-core | screen-ai-core | agent-protocol | agent-service | portal-domain | apps/portal | docs-only | handoff
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
correlation_id: <runtime/proof/job/evidence correlation id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <raw stdout/stderr artifact pointer, test-results path, proof file, screenshot/report path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
redaction_note: <prompt/provider/source data redacted or n/a>
custody_note: <local-only | parent-owned-source | remote-parent-authorized | n/a>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store raw stdout/stderr, Playwright artifacts, screenshots, long model output, provider traces, or failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Runtime and local harness split

Runtime/product-safe proof must show redaction, custody, source-reference, and authority boundaries. Local harness proof may include richer diagnostics, but it still stores raw logs by pointer and keeps plan docs compact.

```text
runtime-safe: no raw prompt payloads, model secrets, provider raw output, screenshots, browser content, or child activity data unless the selected expectation explicitly allows the field.
local harness: enough file/line/command/artifact/evidence-ref context for Codex/MCP/humans to debug without reading terminal walls.
```

## No-claim language

Do not claim:

```text
AI provider ready
local model ready
remote assistant ready
memory ready
classification ready
explanation ready
parent assistant action ready
PR_READY
```

unless the selected proof root proves the claim and the rollout/PR-gate workpack aggregates it when broad readiness is claimed.

## High-risk proof requirements

Any workpack touching AI inference, model routing, assistant output, memory, or parent-visible action must prove:

```text
input custody/redaction
output schema validation
source/reference provenance
timeout/degraded/manual-required state
negative case
no private payload leakage
no product-ready claim from mocked provider alone
```
