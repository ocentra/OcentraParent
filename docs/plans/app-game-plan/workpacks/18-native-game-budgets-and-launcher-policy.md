# 18 Native Game Budgets And Launcher Policy

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `18 Native Game Budgets And Launcher Policy`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Native games have game-specific budgets and rules that can count known games and
explicit launcher-game candidates without counting launcher-only rows as play.

## Scope

- Game budget targets.
- Candidate inclusion policy.
- Launcher-only exclusion/default posture.
- Game category/rating/multiplayer/UGC/purchase inputs.
- Budget dry-run and parent preview.

## Tests And Proof

- [x] Known game session counts toward game budget.
- [x] Launcher-only session does not count by default.
- [x] Launcher-game candidate counts only when parent policy allows candidate state.
- [x] Rating/UGC/multiplayer/purchase signals do not enforce directly.

## Done Signal

Game budgets are useful and honest without treating every launcher row as play.

Use the standard checklist in [workpacks README](README.md).

## Current Status - Phase 1/2 Complete; Phase 3 Open

Commit `0ee4525d8` adds the current Rust-owned native-game budget composition in
`ocentra-app-game-core`. It validates coherent game session kinds, counts known
games and parent-approved launcher-game candidates, excludes launcher-only and
unapproved candidate rows, rejects duplicate session refs and caller-supplied
generic evaluator sessions, and passes the composed sessions into the existing
WP51 runtime evaluator. Rating, UGC, multiplayer, and purchase signals remain
advisory and the resulting adapter state remains `NotDispatched`.

Current source/test owners:

- `crates/app-game-core/src/app_game_native_game_budget.rs`
- `crates/app-game-core/src/app_game_native_game_budget_types.rs`
- `crates/app-game-core/src/app_game_native_game_budget_accounting.rs`
- `crates/app-game-core/tests/contract/app_game_native_game_budget.rs`

Verified on 2026-08-15:

- `cargo test -p ocentra-app-game-core --test contract`: 68 passed.
- `cargo clippy -p ocentra-app-game-core --all-targets -- -D warnings`: passed.
- Focused Enforcer `architecture-policy`, `source-shape`, `required-tests`,
  `no-test-doubles`, `no-naked-domain-strings`, `validation-bypass`, and
  `reexports`: passed for the six touched source/test files.
- The repository pre-commit hook reran crate tests, focused architecture,
  generated-artifact, contract/source-shape, and Rust-format gates: passed.

Phase 3 remains open. This slice does not claim budget persistence, service
composition, portal authoring/preview, notifications, bonus-time integration,
platform adapter execution, retained proof, or whole-plan readiness.

## Historical Contract Slice - 2026-06-03

- Owner/lane: `codex-c`
- Branch: `codex/app-game-read-model-service-events`
- Proof root:
  `output/app-game-plan-proof/18-native-game-budgets-and-launcher-policy`
- Contract source:
  `packages/parent-domain/src/native-game-budget-policy.ts` and
  `packages/parent-domain/src/native-game-budget-policy-rules.ts`
- Test source:
  `packages/parent-domain/tests/native-game-budget-policy.test.ts`

Completed proof:

- Known game session rows count toward the budget.
- Launcher-only rows remain excluded by default.
- Launcher-game candidates count only when the parent policy explicitly allows
  parent-approved candidate state.
- Rating, UGC, multiplayer, and purchase signals cannot request direct
  enforcement.
- Native game budget decisions remain dry-run only and do not create adapter
  handoff.

Deferred:

- Rust/service parity, budget persistence, portal authoring/preview UI,
  notifications, bonus-time integration, and platform adapter execution.
