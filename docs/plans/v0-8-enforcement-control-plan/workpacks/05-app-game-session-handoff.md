# 05 App And Game Session Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `05 App And Game Session Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

App/game session evidence and read models exist in progress form. Enforcement
must consume stored session evidence rather than portal timers or AI guesses.

## Where We Want To Be

Time budgets and app/game actions are grounded in real session summaries,
foreground/running time, target identity, and confidence state.

## Requirement Checklist

- [ ] Require app/game session summary refs for app/game time-limit decisions.
- [ ] Distinguish launcher, game, app, unknown, foreground, and background.
- [ ] Preserve unknown or ask-parent state when identity is weak.
- [ ] Record running/foreground duration used by policy.
- [ ] Show parent-facing rule/action/result details.

## Acceptance And Proof

Tests prove duration and target identity come from stored evidence/read models,
not UI state.

## Parallel Ownership Notes

This workpack may touch app/game feature docs if capability status changes.
