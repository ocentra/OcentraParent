# 20 PR, CI, And Rollout Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `20 PR, CI, And Rollout Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

D has branch work and validation, but it is not integrated until primary reviews,
opens/watches PR, and merges after green CI.

## Where We Want To Be

The D branch handoff includes detailed scope, touched paths, validation,
package/proof artifacts, docs/checklist state, known gaps, and CI/PR status.

## Requirement Checklist

- [ ] Run focused checks before PR-ready handoff.
- [ ] Push the branch when ready for review.
- [ ] Include package/runtime scope in PR-ready report.
- [ ] Watch CI and fix D-owned failures.
- [ ] After merge, primary pulls main and tells active workers to rebase.

## Acceptance And Proof

Merge notes are detailed enough to support the next manual package/runtime test
gate.

Current proof: PR-ready reporting must include the release-support contract,
node proof script, feature/expectation docs, exact validation, PR #218 untouched
state, checklist lock blocker, and known non-claims.

## Parallel Ownership Notes

Workers do not merge to `main`; primary owns integration.
