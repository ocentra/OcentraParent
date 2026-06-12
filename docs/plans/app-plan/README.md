<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `README.md`
> Kind: short plan entry point.
> Read when: Only when PLAN_INDEX, FEATURE_ROUTE_INDEX, or a hub assignment selects this plan.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update PLAN_STATE.md, assigned workpack, and proof/checklist routes.

<!-- /agent-capsule -->

# Native Apps Plan

This is the short, token-efficient entry point for native app identity, installed inventory, process/runtime, foreground app evidence, app-only policy targets, app catalog/settings, and legacy app-plan reconciliation.

## Default Agent Path

1. Read [AGENTS.md](AGENTS.md).
2. Read [PLAN_STATE.md](PLAN_STATE.md).
3. Read [NEXT_ACTIONS.md](NEXT_ACTIONS.md) when starting or resuming.
4. Read [WORKPACK_INDEX.md](WORKPACK_INDEX.md) and select one workpack.
5. Read [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) after the workpack is known.
6. Use [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) and [PROOF_INDEX.md](PROOF_INDEX.md) only for named rows/artifacts.

## Default No-Read Boundary

Do not read full checklists, all workpacks, checkpoints, sibling plans, or source trees unless the selected workpack names the exact handoff.
