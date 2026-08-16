<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `Workpacks Router for App + Game Plan`
> Kind: short plan entry point.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

# Workpacks Router for App + Game Plan

This file is now a short router. The original workpacks README is preserved at
[README_FULL_ORIGINAL.md](README_FULL_ORIGINAL.md).

Use the plan-level [WORKPACK_INDEX.md](../WORKPACK_INDEX.md) to find
an assigned workpack. Do not read every file in this directory.

Default worker path:

1. Read `../PLAN_STATE.md`.
2. Read `../WORKPACK_INDEX.md`.
3. Open only the workpack named by the hub assignment.
4. Update the assigned workpack and relevant checklist rows before reporting
   `DONE` or `PR_READY`.
