# 20 Rollout Docs And CI/PR Gate

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

A has PR-ready work. Primary still needs review, PR creation, CI watching, merge,
pull, and worker rebase instructions.

## Where We Want To Be

Every V0.8 handoff includes exact scope, touched paths, validation, proof files,
product-doc updates, known non-claims, PR state, and CI state.

## Requirement Checklist

- [ ] Update feature docs and checklist rows when status/proof changes.
- [ ] Update touched module READMEs when ownership or gaps change.
- [ ] Run focused checks plus full validation when ready for PR.
- [ ] Create PR only after branch diff and validation are acceptable.
- [ ] Merge only after green CI and reviewed diff.

## Acceptance And Proof

The PR body and merge report are detailed enough for a future coordinator to see
what changed and what remains manual-required.

## Parallel Ownership Notes

Primary owns integration. A owns branch fixes and conflict resolution on the
worker branch.
