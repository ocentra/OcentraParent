# 22 Windows Owned-Process Terminate Time-Limit Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `22 Windows Owned-Process Terminate Time-Limit Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

## Completion Notes - 2026-06-03

- Extended `scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs` instead
  of changing locked `crates/agent-core` or `crates/agent-service` files.
- Added real-service dry-run no-action proof for a live owned child process.
- Added stale timer action mismatch proof that rejects before adapter execution,
  leaves the owned process running, then proves timer recovery and cancel.
- Preserved the existing owned/current expiry path through the scoped process
  adapter and the existing unavailable/already-exited coverage.
- Recorded proof under
  `output/app-game-plan-proof/22-windows-owned-process-terminate-time-limit-proof/`.
- Product checklist unchanged: this proves scoped owned-process time-limit
  behavior only, not broad app/game blocking or package-wide launch control.

Use the standard checklist in [workpacks README](README.md).
