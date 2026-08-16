<!-- agent-capsule -->

> Agent Capsule
> Doc: `Plan Execution Standard`
> Kind: global plan/workpack quality contract.
> Read when: a plan route, workpack, or PR_DONE flow needs to decide whether a plan is executable enough for an agent to proceed.
> Stop rule: apply only to the selected plan/workpack; do not scan sibling plans.
> Proves: execution standard only.
> Does not prove: any plan implementation or PR readiness.
> Proof rule: if a plan/workpack is changed to satisfy this standard, update the selected plan's route/proof/checklist docs.

<!-- /agent-capsule -->

# Plan Execution Standard

This standard exists so small/cheap agents can execute plans without broad reasoning.

## Minimum executable plan route

Every executable plan must route through:

```text
AGENTS.md
PLAN_STATE.md
NEXT_ACTIONS.md
WORKPACK_INDEX.md
selected workpack only
TEST_PROOF_EXPECTATIONS.md
PROOF_INDEX.md
CHECKLIST_INDEX.md when present
PLAN_EXECUTION_BLUEPRINT.md when order/proof/DONE rules are unclear
```

## Minimum executable workpack

A workpack is executable only if it has:

```text
goal
owned scope
out-of-scope boundaries
required inputs
expected source/doc/test paths
proof root or deterministic proof-root rule
acceptance criteria
focused validation commands or command blocker rules
negative cases
manual-required/degraded gaps
Fill-before-DONE report block
```

If any of these are missing, the agent must run a **workpack-normalization pass** before implementation.

## Deterministic proof root fallback

If no exact proof root is named, derive:

```text
output/<plan-name>-proof/<workpack-file-stem>/
```

Every proof root must include:

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

Blocked commands require:

```text
blocker:
required environment:
why this does not prove completion:
next command:
```

## Pre-edit note

Before editing source or docs, write:

```text
Assigned plan:
Assigned workpack:
Implementation slice:
Expected source/doc files:
Expected tests/proof files:
Proof root:
Adjacent handoffs that are read-only:
No-claim boundaries:
```

## DONE rule

A workpack is DONE only when:

```text
source/docs/tests changed or explicit no-code reason recorded
focused commands ran or blockers were recorded
negative cases covered or explicitly open
proof artifacts written under the selected proof root
CHECKLIST_INDEX rows updated only for proven rows
selected workpack Fill-before-DONE section updated
PLAN_STATE updated only if current status changed
```

## PR_READY rule

A plan is PR_READY only when its route-gate/rollout workpack consumes all required proof roots or records blockers and the PR report lists remaining open workpacks.

## Hard rejection

Reject the claim if any of these are true:

```text
full plan was scanned instead of one workpack
sibling plans were edited without selected handoff
proof root missing
validation command missing
only happy-path proof exists for a high-risk claim
manual-required state hidden
mock/scaffold/docs-only proof used as product readiness
CHECKLIST_INDEX and PROOF_INDEX disagree
PLAN_STATE says open but report says done
```
