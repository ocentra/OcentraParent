# 05 App And Game Session Handoff

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
