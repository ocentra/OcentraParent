# 20 Time Budget, Schedule, And Bonus-Time Integration

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `20 Time Budget, Schedule, And Bonus-Time Integration`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

App and game schedules/budgets consume session summaries and preserve approval,
bonus-time, schedule, and audit refs.

## Scope

- App daily/weekly limits.
- Game budgets.
- Schedule windows.
- Bonus time.
- Ask-parent extension.
- Timer state and recovery refs.

## Tests And Proof

- Time budget uses stored session summaries.
- Foreground-only and running-time modes stay distinct.
- Bonus time extends decision with audit refs.
- Dry-run exposes expected action without executing.
- Restart recovery preserves active timer state where implemented.

## Done Signal

Time/budget decisions can be explained from evidence, schedule, parent action,
and capability state.

Use the standard checklist in [workpacks README](README.md).

## Current Status - Phase 1 Active

The 2026-08-15 code audit found that the historical TypeScript owners and test
paths below are no longer tracked. WP51 now provides a Rust-owned generic
compiler/evaluator boundary, but there is no current composition from stored
`AppGameSessionSummary` rows into daily/weekly running-versus-foreground budget
evaluation, schedule evidence, bonus approval/audit refs, and active/recovered
timer state.

This workpack is active for that bounded `ocentra-app-game-core` composition
and focused contract tests. The slice must remain dry-run and may not claim
service persistence, portal/child UX, notifications, timer scheduling,
rollback, or adapter execution.

## Historical Contract Completion - 2026-06-03

- Owner: `codex-c`
- Branch: `codex/app-game-read-model-service-events`
- Proof root:
  `output/app-game-plan-proof/20-time-budget-schedule-bonus-time-integration/`
- Native app cross-record:
  `output/app-plan-proof/19-time-budget-schedule-bonus-time-integration/`

Completed:

- Added `packages/parent-domain/src/app-game-time-budget-policy-rules.ts` and
  `packages/parent-domain/src/app-game-time-budget-policy.ts`.
- Added focused tests in
  `packages/parent-domain/tests/app-game-time-budget-policy.test.ts` and
  `packages/parent-domain/tests/app-game-time-budget-policy-recovery.test.ts`.
- Proved stored app/game session refs, running versus foreground duration
  modes, schedule evidence refs, bonus-time approval/audit refs,
  ask-parent/manual-required dry-run states, effective budget math, and
  restart-recovered timer refs at the TypeScript parent-domain contract layer.

Deferred:

- Rust protocol parity, service persistence, runtime evaluator execution,
  portal budget authoring/preview UI, notification delivery, child request UX,
  adapter execution, and platform timer/rollback execution remain later
  workpacks.
