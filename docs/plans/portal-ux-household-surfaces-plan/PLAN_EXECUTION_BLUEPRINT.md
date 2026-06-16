<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: portal product readiness or PR readiness.

<!-- /agent-capsule -->

# Portal UX Household Surfaces Execution Blueprint

## Execution rule

Portal UX work must render typed domain/read-model state. It must not invent product truth, execute device work in the browser, or show fake readiness.

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Deterministic proof root

```text
output/portal-ux-household-surfaces-plan-proof/<workpack-file-stem>/
```

## Focused commands

```bash
npm run build --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal
npm run test:e2e --workspace @ocentra-parent/portal
npm run lint:architecture -- --files packages/portal-domain apps/portal docs/plans/portal-ux-household-surfaces-plan
```

If a command/test path does not exist, record the blocker and keep rows open.

## Universal proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## No-claim boundaries

Do not claim account, setup, policy, payment, data custody, device trust, or runtime readiness from portal UI alone.
