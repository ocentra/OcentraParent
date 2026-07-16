<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: implementation completion, model readiness, privacy readiness, or PR readiness.

<!-- /agent-capsule -->

# AI Plan Execution Blueprint

## Execution rule

This plan is large. Do not try to execute it top-to-bottom.

Use this loop:

```text
AGENTS.md
  -> PLAN_STATE.md
  -> NEXT_ACTIONS.md
  -> WORKPACK_INDEX.md
  -> exactly one selected workpack
  -> TEST_PROOF_EXPECTATIONS.md
  -> PROOF_INDEX.md
```

## Proof-root rule

For any selected workpack, derive the proof root from the workpack filename:

```text
output/ai-plan-proof/<workpack-file-stem>/
```

Example:

```text
workpacks/07-ai-job-queue-contract.md
output/ai-plan-proof/07-ai-job-queue-contract/
```

## Pre-edit note

Before editing source or docs, write:

```text
Assigned workpack:
Implementation slice:
Expected source/doc files:
Expected tests/proof files:
Proof root:
Adjacent handoffs that are read-only:
No-claim boundaries:
```

## High-risk AI boundaries

Do not claim readiness without proof for the selected slice:

```text
input custody and redaction
model/provider routing
local/remote provider distinction
prompt/template versioning
output parser and schema validation
timeout/error/degraded behavior
memory/reference provenance
AI result journal and replay
parent explanation source citations
abuse/privacy/performance boundaries
```

## Likely source ownership map

```text
packages/ai-domain/**
packages/text-domain/**
packages/evidence-domain/** when evidence refs are touched
packages/agent-protocol-domain/** when protocol contracts are touched
crates/agent-protocol/** only for cross-language contract parity
crates/agent-service/** only for selected runtime boundary proof
apps/portal/** only for selected parent-visible AI surface proof
scripts/test/** selected AI proof runners
```

## Focused command policy

Use relevant commands only:

```bash
npm run build --workspace @ocentra-parent/ai-domain
npm run test --workspace @ocentra-parent/ai-domain
npm run build --workspace @ocentra-parent/text-domain
npm run test --workspace @ocentra-parent/text-domain
cargo test -p ocentra-parent-agent-protocol ai
cargo test -p ocentra-parent-agent-service ai
npm run test --workspace @ocentra-parent/portal -- ai
npm run lint:architecture -- --files packages/ai-domain packages/text-domain packages/evidence-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/ai-plan
```

If a command or test path does not exist, record the missing location and keep the row open.

## Proof update rule

Every completed workpack needs:

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

plus any workpack-specific artifacts named by the selected workpack.

## DONE / PR_READY criteria

DONE for one workpack requires:

```text
source/docs/tests updated
focused commands run or blocker recorded
negative cases covered or explicitly open
proof artifacts written
CHECKLIST_INDEX.md rows updated when exact rows exist
selected workpack Fill-before-DONE section updated
PLAN_STATE.md open gaps updated if state changed
```

PR_READY for the whole plan requires the rollout/PR-gate workpack and all prerequisite proof roots named by that gate.
