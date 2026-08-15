# 19 Policy Target Compiler For App/Game Rules

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `19 Policy Target Compiler For App/Game Rules`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

## Current status - Phase 1 open

The 2026-08-15 code audit found that the former TypeScript implementation
owners were removed and the current Rust file only reproduces the generated
TypeScript contract text. The workpack is therefore ready for implementation,
not complete or in validation.

Phase 1 still requires a Rust-owned compiler algorithm and focused checked-in
tests for the target families and negative states in this workpack. Phase 2
focused execution/Enforcer and Phase 3 proof remain later gates.

## Historical bounded contract slice - 2026-06-03

- Owner/lane: `codex-c`
- Branch: `codex/app-game-read-model-service-events`
- Proof root:
  `output/app-game-plan-proof/19-policy-target-compiler-for-app-game-rules`
- Contract source:
  `packages/parent-domain/src/app-game-policy-target-compiler.ts` and
  `packages/parent-domain/src/app-game-policy-target-compiler-rules.ts`
- Test source:
  `packages/parent-domain/tests/app-game-policy-target-compiler.test.ts`

The former TypeScript slice recorded:

- Specific app/game targets require identity proof.
- Unknown app/game targets compile only from unknown-state proof.
- Category/risk/multiplayer/UGC/purchase target families require category
  proof.
- Scheduled rules require schedule proof.
- Wrong-device, wrong-local-user, and stale evidence are rejected.
- Compiled output remains dry-run and carries evidence, rule, and capability
  refs.
- Unproved block-launch compiles to manual-required with disabled handoff.

That slice explicitly deferred:

- Rust/service parity, runtime evaluator execution, portal authoring/preview UI,
  timers, notifications, rollback, and adapter execution.

The removed `packages/parent-domain` files and ignored output root are
historical evidence only; they do not satisfy the current Rust-first Phase 1
implementation or test requirement.
