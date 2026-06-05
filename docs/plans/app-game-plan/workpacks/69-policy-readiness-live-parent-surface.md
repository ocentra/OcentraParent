# 69. Policy Readiness Live Parent Surface

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-policy-readiness-live-surface`
- Scope: parent-visible app/game policy readiness surface wiring.

## Goal

Project the existing WP52 app/game policy readiness service event into the
parent App/Game Sessions dashboard intent so parents can see whether policy
inputs are ready, missing, or manual-required before any evaluator or adapter
claim is made.

## In Scope

- Request `agent.activity.app-game.policy-readiness.read-model.get` from the
  portal overview command list.
- Parse `agent.activity.app-game.policy-readiness.read-model.reported` through
  the existing agent-protocol parser in portal live activity state.
- Feed the validated parser result into the shared App/Game Sessions dashboard
  intent.
- Render policy input readiness as dashboard metrics and evidence drawer rows
  through the existing app/game surface data.
- Add focused portal tests and proof outputs.

## Out Of Scope

- New backend service contracts or Rust protocol work beyond the existing WP52
  command/event.
- Runtime policy evaluator execution.
- Parent rule authoring, timer execution, rollback, or preference mutation.
- Adapter dispatch, broad installed-app blocking, notifications, child delivery,
  or platform support claims.

## Proof

- `scripts/test/app-game-policy-readiness-live-surface-proof.mjs`
- `output/app-game-plan-proof/69-policy-readiness-live-parent-surface`
- `output/app-plan-proof/69-policy-readiness-live-parent-surface`
- `test-results/app-game-policy-readiness-live-surface-proof/proof.json`

## DONE Checklist

- [x] Hub lock covers the exact implementation, docs, proof, and validation
      paths.
- [x] The work reuses the existing WP52 policy readiness event and does not add
      a duplicate readiness contract.
- [x] Portal live activity state parses policy readiness through the existing
      protocol parser.
- [x] The App/Game Sessions dashboard intent exposes ready, missing, and
      manual-required policy input rows without treating them as evaluator
      decisions.
- [x] Proof pack records no evaluator execution, no adapter dispatch, no broad
      blocking, no notification delivery, no child delivery, and no platform
      support claim.
