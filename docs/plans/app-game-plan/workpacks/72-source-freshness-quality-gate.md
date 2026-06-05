# 72 Source Freshness Quality Gate

## Scope

Add an activity-domain source freshness quality gate for existing app/game
`sourceStatusRows`.

This workpack proves source rows can be classified as fresh, stale, missing,
manual-required, unavailable, or empty before a future policy evaluator consumes
them. Only recent source rows with evidence become policy-eligible. Every row
keeps adapter dispatch false.

This does not add portal rendering, source subscriptions, policy runtime
consumption, timers, child delivery, adapter execution, broad blocking, provider
delivery, or platform support claims.

## Implementation

- Export `ActivityAppGameSourceStatusRowSchema` so downstream quality gates can
  parse the existing read-model source rows without duplicating the contract.
- Add `app-game-source-freshness-quality-gate` activity-domain contracts and
  evaluator.
- Add focused tests for fresh, stale, missing, empty, manual-required,
  unavailable, and malformed source rows.
- Add proof artifacts under both app-game and app plan proof roots.

## Proof

- `cmd /c npm run build --workspace @ocentra-parent/activity-domain`
- `cmd /c npm exec --workspace @ocentra-parent/activity-domain -- vitest run tests/app-game-source-freshness-quality-gate.test.ts`
- `node scripts/test/app-game-source-freshness-quality-gate-proof.mjs`
- focused format/schema/source checks plus lane/hub guards before PR-ready
  handoff

Proof artifacts live in:

```text
output/app-game-plan-proof/72-source-freshness-quality-gate
output/app-plan-proof/72-source-freshness-quality-gate
```

## No-Claim Boundaries

- Source freshness quality summarizes already-stored service/read-model source
  rows only.
- Fresh source quality does not execute a policy evaluator, timer, adapter, or
  blocking action.
- Stale, missing, manual-required, unavailable, and empty rows are never policy
  eligible.
- No portal UI, child UX, provider delivery, scheduler runtime, platform
  adapter, or broad blocking support is claimed.

## Product Doc Decision

Feature docs and plan checklists are updated because a new reusable
activity-domain proof exists. `docs/product-capability-checklist.md` is not
updated in this slice because no feature status moves and the central checklist
is owned by another lane.
