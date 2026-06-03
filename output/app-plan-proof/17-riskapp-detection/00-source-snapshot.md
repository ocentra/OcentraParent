# WP17 Risk App Detection Source Snapshot

Captured before reporting DONE on 2026-06-03.

Relevant current sources:

- `WP17 app-plan workpack doc`
- `docs/plans/app-plan/source-index.md`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/ai.md`
- `packages/activity-domain/src/app-game-category-risk.ts`
- `activity-domain app/game category risk primitives`
- `packages/activity-domain/tests/app-game-category-risk.test.ts`
- `packages/parent-domain/src/app-game-policy-target-compiler.ts`
- `packages/parent-domain/src/app-game-policy-target-compiler-rules.ts`

Implementation added in this slice:

- `packages/parent-domain/src/app-riskdetection.ts`
- `packages/parent-domain/src/app-riskdetection-rules.ts`
- `packages/parent-domain/src/app-riskdetection-data.ts`
- `packages/parent-domain/tests/app-riskdetection.test.ts`
- `scripts/test/app-riskdetection-proof.mjs`

Boundary note:

`packages/activity-domain` already owns raw app/game category-risk evidence
contracts and is currently locked by `codex-a`. WP17 therefore reuses that
taxonomy as source evidence and adds the missing native-app product candidate
contract in `parent-domain` without editing A-owned files.
