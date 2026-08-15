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

Current code-first authority: [CODE_AUDIT.md](CODE_AUDIT.md), audited
2026-08-15. All 95 workpacks are mapped; 77 have no bounded Phase 1
source/test-writing gap and 18 remain incomplete. Historical checkboxes and
legacy package/script paths do not override that audit.

## Default Agent Path

1. Read [AGENTS.md](AGENTS.md).
2. Read [PLAN_STATE.md](PLAN_STATE.md).
3. Read [CODE_AUDIT.md](CODE_AUDIT.md) for actual source/test state.
4. Read [NEXT_ACTIONS.md](NEXT_ACTIONS.md) when starting or resuming.
5. Read [WORKPACK_INDEX.md](WORKPACK_INDEX.md) and select one workpack.
6. Read [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) after the workpack is known.
7. Use [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) and [PROOF_INDEX.md](PROOF_INDEX.md) only for named rows/artifacts.

## Default No-Read Boundary

Do not read full checklists, all workpacks, checkpoints, sibling plans, or source trees unless the selected workpack names the exact handoff.
