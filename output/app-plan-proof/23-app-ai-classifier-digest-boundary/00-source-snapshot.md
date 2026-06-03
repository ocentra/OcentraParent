Source snapshot for app-plan WP23 app AI classifier digest boundary.

- Branch: `codex/app-game-read-model-service-events`
- Starting commit: `b1aa213 Add app risk detection proof`
- Owning shared proof: app-game WP24 under
  `output/app-game-plan-proof/24-ai-classifier-digest-boundary`.
- Existing source inspected:
  - `packages/activity-domain/src/app-game.ts`
  - `packages/activity-domain/tests/app-game-evidence-claim.test.ts`
  - `packages/parent-domain/src/app-riskdetection.ts`
  - `docs/plans/app-plan/workpacks/23-app-ai-classifier-digest-boundary.md`
  - `docs/plans/app-game-plan/workpacks/24-ai-classifier-digest-boundary.md`
- Existing behavior: app/game digest refs already exist in activity-domain; WP17
  native app risk detection already references local AI digest candidates.
- Gap before this work: native app classifier handoff did not yet have focused
  policy-boundary proof for missing evidence refs, forbidden action fields,
  duration fields, raw scan fields, runtime/prompt refs, and fallback state.
