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
[test blueprint](../v0-8-enforcement-control-test-blueprint.md),
[folder README](../README.md),
[enforcement-integrity-tamper feature](../../features/enforcement-integrity-tamper.md), and
[enforcement expectation](../../expectations/enforcement.md).

## Purpose

Define the final rollout gate so PRs, CI, and merge reports carry exact scope,
validation, proof roots, non-claims, and remaining open workpacks instead of
fake-green summary claims.

## Central schema boundary

```text
v0-8-enforcement-control-plan owns rollout truth for this plan's route docs, workpacks, proof roots, and no-claim summary.
primary/coordinator lanes own PR creation, review, CI watching, merge, and branch integration.
feature docs and product checklist own product-status truth when a selected workpack changes it.
```

## Source Inputs

- `../v0-8-enforcement-control-20-step-plan.md`
- `../v0-8-enforcement-control-test-blueprint.md`
- `../PLAN_STATE.md`
- `../PROOF_INDEX.md`
- `../../features/enforcement-integrity-tamper.md`
- `../../expectations/enforcement.md`

## Target State

Every V0.8 handoff includes exact scope, touched paths, validation, proof files,
product-doc updates, known non-claims, PR state, and CI state.

## Required proof fields

```text
touched_paths_state
validation_state
proof_root_state
doc_update_state
no_claim_summary_state
ci_state
review_state
merge_state
remaining_workpack_state
no_fake_green_claim
no_claim
```

## Tests And Proof

Proof root: `output/v0-8-enforcement-control-plan-proof/20-rollout-docs-ci-pr-gate/`

Focused validation should record:

- focused commands for the selected workpack(s) being handed off
- broader validation only when PR-ready scope requires it
- proof-root inventory and doc-route sync checks
- CI/PR blockers when local proof exists but merge readiness does not

## AI Worker Checklist

- [ ] Update feature docs and checklist rows when status/proof changes.
- [ ] Update touched module READMEs when ownership or gaps change.
- [ ] Run focused checks plus full validation when ready for PR.
- [ ] Create PR only after branch diff and validation are acceptable.
- [ ] Merge only after green CI and reviewed diff.

## Where We Are

A has PR-ready work. Primary still needs review, PR creation, CI watching, merge,
pull, and worker rebase instructions.

## Negative Cases

- route-doc cleanup alone must not claim product readiness
- one focused proof root must not hide remaining open workpacks
- local green without CI/review must not be reported as merge-ready
- stale feature docs, README ownership notes, or plan-state rows must block ready claims
- merge status must not be inferred from PR creation alone

## Manual-Required Gaps

- Primary/coordinator still owns integration, CI watching, merge, and pull
  coordination.
- Open workpacks remain blockers unless explicitly carried as manual-required or
  not-ready gaps.
- Cross-branch conflict resolution remains a worker/primary coordination task,
  not proof of readiness by itself.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch recorded.
- [ ] Validation commands and results recorded in `16-validation-commands.log`.
- [ ] Proof artifacts under `output/v0-8-enforcement-control-plan-proof/20-rollout-docs-ci-pr-gate/`.
- [ ] PR/CI state, remaining open workpacks, and no-claim summary listed explicitly.
