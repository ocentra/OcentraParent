# 18 Native Game Budgets And Launcher Policy

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

- Known game session counts toward game budget.
- Launcher-only session does not count by default.
- Launcher-game candidate counts only when parent policy allows candidate state.
- Rating/UGC/multiplayer/purchase signals do not enforce directly.

## Done Signal

Game budgets are useful and honest without treating every launcher row as play.

Use the standard checklist in [workpacks README](README.md).

## Completion - 2026-06-03

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
