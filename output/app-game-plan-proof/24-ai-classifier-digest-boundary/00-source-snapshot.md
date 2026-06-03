Source snapshot for app-game WP24 AI classifier digest boundary.

- Branch: `codex/app-game-read-model-service-events`
- Starting commit: `b1aa213 Add app risk detection proof`
- Existing source inspected:
  - `packages/activity-domain/src/app-game.ts`
  - `packages/activity-domain/tests/app-game-evidence-claim.test.ts`
  - `packages/activity-domain/src/app-game-category-risk.ts`
  - `docs/plans/app-game-plan/workpacks/24-ai-classifier-digest-boundary.md`
  - `docs/plans/app-plan/workpacks/23-app-ai-classifier-digest-boundary.md`
- Existing behavior: activity-domain already exposes app/game AI digest refs and
  a basic AI classification digest contract.
- Gap before this work: policy-facing classifier output did not yet have focused
  proof for missing evidence refs, forbidden action fields, duration fields, raw
  scan fields, runtime/prompt refs, and fallback state.
- Lock note: `packages/activity-domain` is currently locked by codex-a, so this
  slice adds parent-domain policy boundary proof and runs existing
  activity-domain digest tests without editing that package.
