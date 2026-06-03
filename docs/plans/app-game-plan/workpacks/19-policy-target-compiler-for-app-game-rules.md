# 19 Policy Target Compiler For App/Game Rules

## Target State

App/game rules compile only with identity, category, evidence, schedule,
approval, authority, and capability proof.

## Scope

- App targets: specific app, package, bundle, AppUserModelId, desktop entry,
  executable hash, publisher, category, unknown, new, portable, risk, all
  non-system.
- Game targets: specific game, launcher game id, store id, category, unknown,
  new, launcher-game candidate, multiplayer, UGC, purchase-capable, mature,
  all games.
- Dry-run before enforcement.

## Tests And Proof

- Specific target requires identity ref.
- Unknown target compiles from unknown state.
- Block launch returns manual-required without proof.
- Wrong device/local user/stale evidence is rejected.
- Policy output carries evidence and capability refs.

## Done Signal

Parent rules compile into typed decisions without inventing evidence or adapter
authority.

Use the standard checklist in [workpacks README](README.md).

## Completion - 2026-06-03

- Owner/lane: `codex-c`
- Branch: `codex/app-game-read-model-service-events`
- Proof root:
  `output/app-game-plan-proof/19-policy-target-compiler-for-app-game-rules`
- Contract source:
  `packages/parent-domain/src/app-game-policy-target-compiler.ts` and
  `packages/parent-domain/src/app-game-policy-target-compiler-rules.ts`
- Test source:
  `packages/parent-domain/tests/app-game-policy-target-compiler.test.ts`

Completed proof:

- Specific app/game targets require identity proof.
- Unknown app/game targets compile only from unknown-state proof.
- Category/risk/multiplayer/UGC/purchase target families require category
  proof.
- Scheduled rules require schedule proof.
- Wrong-device, wrong-local-user, and stale evidence are rejected.
- Compiled output remains dry-run and carries evidence, rule, and capability
  refs.
- Unproved block-launch compiles to manual-required with disabled handoff.

Deferred:

- Rust/service parity, runtime evaluator execution, portal authoring/preview UI,
  timers, notifications, rollback, and adapter execution.
