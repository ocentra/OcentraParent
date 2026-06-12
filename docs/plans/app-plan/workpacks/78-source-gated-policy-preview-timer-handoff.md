# WP78 Source-Gated Policy Preview Timer Handoff

Cross-recorded from the shared app/game plan.

## Scope

Native app source-gated policy preview rows now have a parent-domain
timer-handoff readiness proof. The proof consumes WP76 redacted preview rows,
marks preview-ready native app rows as future timer sequencing candidates, and
keeps source-manual or compiler-manual rows blocked before timer runtime.

## Evidence

- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-handoff.ts`
- `packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-handoff.test.ts`
- `scripts/test/app-game-source-gated-policy-preview-timer-handoff-proof.mjs`
- `test-results/app-game-source-gated-policy-preview-timer-handoff-proof/proof.json`
- `output/app-plan-proof/78-source-gated-policy-preview-timer-handoff/proof.json`

## No-Claim Boundaries

This does not update the package manifest, start service/runtime timers, render
portal UI, run the policy evaluator, dispatch adapters, deliver child UX, prove
broad app blocking, enforce platform controls, or expose raw private source
rows.

## Execution Detail

Minimum context:

- `docs/plans/policy-control-plane-plan/AGENTS.md`
- `docs/plans/app-game-plan/workpacks/78-source-gated-policy-preview-timer-handoff.md`
- `docs/plans/app-plan/workpacks/76-source-gated-policy-preview-read-model.md`

Owner boundary:

- This workpack records timer-handoff readiness only.
- Policy source truth and schedules belong to `policy-control-plane-plan`.
- Runtime timers and enforcement belong to service/enforcement owners.

Expected tests/proof names:

- `app-plan.wp78.timer-handoff-readiness`
- `app-plan.wp78.manual-required-blocked`
- `app-plan.wp78.no-runtime-timer-claim`
- `app-plan.wp78.no-raw-source-exposure`

Failure conditions:

- Handoff readiness is treated as scheduled timer execution, child delivery, or enforcement proof.
