# 24 AI Classifier Digest Boundary

## Target State

AI consumes stored app/game evidence or structured digests and returns
classification candidates only.

## Current Proof

WP24 now adds a parent-domain policy-facing classifier boundary while preserving
the existing activity-domain app/game digest as the source spine. The proof
requires stored evidence refs, runtime and prompt refs, bounded confidence,
fallback/degraded state, and evidence-only policy handoff. Forbidden action,
duration, and raw scan fields are rejected before policy sees classifier output.

`packages/activity-domain` remains unchanged because codex-a currently owns that
lock. The proof harness still runs the existing activity-domain digest test.

## Scope Covered

- App/game AI digest refs.
- Unknown app/game classification result.
- Category/risk candidates with source/confidence.
- Model/runtime/prompt-template refs.
- No direct action authority.

## Tests And Proof

- [x] Missing evidence refs rejected.
- [x] Confidence outside `0..1` rejected.
- [x] Block/terminate/hide/suspend/shield fields rejected.
- [x] Duration field in AI output rejected.
- [x] Raw OS scan result in AI output rejected.
- [x] Provider unavailable and low-confidence fallback states are explicit.
- [x] Proof artifacts recorded in
      `output/app-game-plan-proof/24-ai-classifier-digest-boundary/`.

## Done Signal

AI can help classify unknown apps/games without scanning the OS or enforcing.

Use the standard checklist in [workpacks README](README.md).

## Completion

- Branch: `codex/app-game-read-model-service-events`.
- Contract:
  `packages/parent-domain/src/app-game-ai-classifier-boundary.ts`.
- Values:
  `packages/parent-domain/src/app-game-ai-classifier-boundary-values.ts`.
- Data:
  `packages/parent-domain/src/app-game-ai-classifier-boundary-data.ts`.
- Test:
  `packages/parent-domain/tests/app-game-ai-classifier-boundary.test.ts`.
- Harness:
  `node scripts/test/app-game-ai-classifier-boundary-proof.mjs`.
- Existing source-spine validation:
  `cmd /c npm run test --workspace @ocentra-parent/activity-domain -- app-game-evidence-claim`.
- Product checklist decision: no product checklist update because product status
  did not move and codex-a owns the checklist lock. Runtime classifier service
  events, portal rendering, provider/model quality, policy evaluator
  consumption, and adapter enforcement remain gaps.
