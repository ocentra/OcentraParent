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
