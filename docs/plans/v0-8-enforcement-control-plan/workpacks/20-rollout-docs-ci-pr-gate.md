# 20 Rollout Docs And CI/PR Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `20 Rollout Docs And CI/PR Gate`
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
