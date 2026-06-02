# 20 PR, CI, And Rollout Gate

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

- [ ] Run focused checks and full validation before PR-ready handoff.
- [ ] Push the branch when ready for review.
- [ ] Include package/runtime scope in PR body.
- [ ] Watch CI and fix D-owned failures.
- [ ] After merge, primary pulls main and tells active workers to rebase.

## Acceptance And Proof

Merge notes are detailed enough to support the next manual package/runtime test
gate.

## Parallel Ownership Notes

Workers do not merge to `main`; primary owns integration.
