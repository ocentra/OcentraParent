# WP74 App/Game Source Freshness Policy Consumption

## Scope

Add a parent-domain policy-consumption proof that validates native app and
native game policy requests against existing service-backed `sourceStatusRows`
before any policy compile or adapter handoff can be considered ready.

This keeps native apps and native games as separate product meanings over the
shared app/game evidence spine. It consumes already-projected read-model rows;
it does not scan the OS, read raw private executable paths, dispatch adapters,
block apps or games, or mutate child-device state.

## Owned Proof

- Contract:
  `packages/parent-domain/src/app-game-source-freshness-policy-consumption.ts`
- Data/rules:
  `packages/parent-domain/src/app-game-source-freshness-policy-consumption-*.ts`
- Tests:
  `packages/parent-domain/tests/app-game-source-freshness-policy-consumption.test.ts`
- Harness:
  `scripts/test/app-game-source-freshness-policy-consumption-proof.mjs`
- Evidence:
  `output/app-game-plan-proof/74-source-freshness-policy-consumption` and
  `output/app-plan-proof/74-source-freshness-policy-consumption`

## Acceptance

- Native app policy requests require fresh inventory, runtime, and foreground
  rows with evidence refs.
- Native game policy requests require fresh inventory, runtime, foreground, and
  launcher rows with evidence refs.
- Stale, missing, permission-limited, unavailable, adapter-error,
  manual-required, and not-claimed source rows block policy compile.
- Readiness rows never include raw private source rows and never request direct
  adapter calls.
- Focused parent-domain tests and the proof harness pass.

## Non-Goals

- No portal or child-facing UI.
- No runtime policy evaluator execution beyond readiness proof.
- No adapter dispatch, broad blocking, or platform hard-control support.
- No package export while another lane owns `packages/parent-domain/package.json`.
