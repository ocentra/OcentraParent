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

- [x] Time budget uses stored session summaries.
- [x] Foreground-only and running-time modes stay distinct.
- [x] Bonus time extends decision with audit refs.
- [x] Dry-run exposes expected action without executing.
- [x] Restart recovery preserves active timer state where implemented.

## Done Signal

Time/budget decisions can be explained from evidence, schedule, parent action,
and capability state.

Use the standard checklist in [workpacks README](README.md).

## Current Status - Phase 1/2 Complete; Phase 3 Open

Commit `b82cfc608` adds the current Rust-owned composition from stored
`AppGameSessionSummary` rows into the WP51 runtime evaluator. It carries an
explicit daily/weekly period, keeps running and foreground duration modes
distinct, rounds partial stored milliseconds up instead of undercounting,
binds active schedule evidence to the compiler input, preserves pending or
approved bonus audit refs, and carries active or restart-recovered timer refs.
Duplicate/incoherent summaries, manual-estimate bypasses, mismatched schedule
evidence, and timers attached to non-time-limit actions fail closed. Adapter
state remains `NotDispatched`.

Current source/test owners:

- `crates/app-game-core/src/app_game_time_budget.rs`
- `crates/app-game-core/src/app_game_time_budget_types.rs`
- `crates/app-game-core/src/app_game_time_budget_sessions.rs`
- `crates/app-game-core/src/app_game_time_budget_schedule.rs`
- `crates/app-game-core/src/app_game_time_budget_policy.rs`
- `crates/app-game-core/tests/contract/app_game_time_budget.rs`

Verified on 2026-08-15:

- Focused WP20 contract tests: 6 passed.
- `cargo test -p ocentra-app-game-core --test contract`: 74 passed.
- `cargo clippy -p ocentra-app-game-core --all-targets -- -D warnings`: passed.
- Focused Enforcer `architecture-policy`, `source-shape`, `required-tests`,
  `no-test-doubles`, `no-naked-domain-strings`, `validation-bypass`, and
  `reexports`: passed for the eight touched source/test files.
- The repository pre-commit hook reran crate tests, focused architecture,
  generated-artifact, contract/source-shape, and Rust-format gates: passed.

Phase 3 remains open. This slice does not claim service persistence,
portal/child UX, notification delivery, timer scheduling, rollback, platform
adapter execution, retained proof, or whole-plan readiness.

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
