# 57. Policy Evaluator Service Read Model

This native app-plan workpack is cross-recorded from shared app/game WP57.

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-policy-evaluator-service`
- Scope: service-backed native app/game policy evaluator read model.

## Goal

Expose native app policy evaluation readiness through the shared app/game
service read-model spine while preserving dry-run-only, disabled handoff, and
no-adapter-dispatch boundaries.

## Proof

- `scripts/test/app-game-policy-evaluator-service-proof.mjs`
- `output/app-plan-proof/57-policy-evaluator-service-read-model`
- `output/app-game-plan-proof/57-policy-evaluator-service-read-model`
- `test-results/app-game-policy-evaluator-service-proof/proof.json`

## DONE Checklist

- [x] Cross-recorded from shared app/game WP57.
- [x] Native app targets remain app/game evidence/readiness refs, not adapter
      authority.
- [x] Policy evaluation rows are dry-run read-model rows with
      `enforcementHandoffState=disabled`.
- [x] Block-launch, missing classifier context, and missing platform proof stay
      manual-required.
- [x] Portal rendering, parent rule persistence, notifications, child delivery,
      timers, broad app blocking, adapter execution, and platform support
      remain unclaimed.
