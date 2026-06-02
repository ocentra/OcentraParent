# 22 Windows Owned-Process Terminate Time-Limit Proof

## Target State

Scoped Windows owned-process app/game time-limit and terminate proof remains
distinct from broad app/game blocking.

## Scope

- Reuse existing scoped owned-process proof.
- Recheck process identity before action.
- Record timer, result, rollback, audit, and child-status refs.
- Keep package-wide block launch manual-required.

## Tests And Proof

- Owned test process reaches limit.
- Dry-run does not terminate.
- Supported action terminates only proved owned/current target.
- Already exited, unavailable, rollback, and stale states are journaled.
- Broad block remains manual-required.

## Done Signal

The existing Windows proof is preserved and extended only within its scoped
claim boundary.

Use the standard checklist in [workpacks README](README.md).
