# WP106 Child UX Runtime Audit Handoff

## Scope

Create a parent-domain proof that consumes existing app/game child-facing UX
cards and records whether each native app/native game child surface row is ready
for future runtime audit persistence.

This workpack keeps the shared app/game evidence spine intact:

- native apps and native games share the same child UX handoff contract;
- target domain remains explicit on every row;
- browser-game child UX stays in the browser plan;
- ready rows require child reason refs, child status refs, and future runtime
  audit proof refs;
- missing child reason/status rows stay blocked before runtime audit;
- manual-required/unavailable rows remain no-adapter and do not claim runtime
  delivery.

## Non-Claims

This slice does not implement child runtime delivery, child request UI rendering,
child status persistence, runtime audit persistence, adapter dispatch, platform
enforcement, or private diagnostic exposure.

## Proof

- Contract:
  `packages/parent-domain/src/app-game-child-ux-runtime-audit-handoff.ts`
- Rules:
  `packages/parent-domain/src/app-game-child-ux-runtime-audit-handoff-rules.ts`
- Tests:
  `packages/parent-domain/tests/app-game-child-ux-runtime-audit-handoff.test.ts`
- Harness:
  `scripts/test/app-game-child-ux-runtime-audit-handoff-proof.mjs`
- Evidence:
  `output/app-game-plan-proof/106-child-ux-runtime-audit-handoff`
  and `output/app-plan-proof/106-child-ux-runtime-audit-handoff`
