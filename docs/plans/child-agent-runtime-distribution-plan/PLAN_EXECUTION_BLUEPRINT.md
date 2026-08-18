<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `Child Agent Runtime Distribution Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: child runtime/package readiness or PR readiness.

<!-- /agent-capsule -->

# Child Agent Runtime Distribution Execution Blueprint

## Execution rule

Child package/runtime work stays separate from parent-client distribution and setup journey proof.

Use this source-first loop:

```text
AGENTS.md
  -> PLAN_STATE.md
  -> NEXT_ACTIONS.md
  -> WORKPACK_INDEX.md
  -> graph inspect/why for one workpack
  -> complete that coherent production-source packet
  -> write its complete expected test-source packet
  -> focused validation
  -> proof regeneration
  -> PROOF_INDEX.md
```

Do not alternate one production line and one test run. Do not run broad validation while the selected source/test packet is structurally incomplete.

## Deterministic proof root

```text
output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/
```

## Focused command families after source and test source exist

```bash
ocentra-child-runtime focused tests
ocentra-child-runtime-android-bridge focused tests plus Android instrumentation
ocentra-parent-agent-maintenance focused updater/handoff tests
selected child-labelled platform package/lifecycle harness
selected child workpack architecture/Enforcer/graph gates
```

Exact expected coverage is in `TEST_PROOF_EXPECTATIONS.md`. If required source or test source does not exist, keep the row open; do not replace it with a broad unrelated command.

## Universal proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## No-claim boundaries

Do not claim package/platform/service/readiness for a child runtime unless the selected proof root proves that exact platform and artifact state.

Implementation-only graph edges order source packets. They do not promote normal READY/DONE or bypass tests, proof, checklist, CI, review, or merge.
