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

- Known game session counts toward game budget.
- Launcher-only session does not count by default.
- Launcher-game candidate counts only when parent policy allows candidate state.
- Rating/UGC/multiplayer/purchase signals do not enforce directly.

## Done Signal

Game budgets are useful and honest without treating every launcher row as play.

Use the standard checklist in [workpacks README](README.md).

## Current Status - Phase 1 Active

The 2026-08-15 code audit found the historical TypeScript contract owner below
was removed. Current Rust code has a generic compiler/evaluator and launcher
evidence rows, but no native-game budget composition that classifies known-game,
launcher-only, and parent-approved launcher-game-candidate sessions before
runtime evaluation.

This workpack is active for a bounded `ocentra-app-game-core` composition and
focused tests. Rating, UGC, multiplayer, and purchase signals must remain
advisory; the result must stay dry-run and must not add persistence, service,
portal, notification, or adapter-execution claims.

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
