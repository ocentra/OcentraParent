# 23 App AI Classifier Digest Boundary

Sources: [full scope plan](../v0-5-native-apps-full-scope-plan.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), and
`docs/expectations/ai.md`.

## Where We Are

The repo already states that AI can consume stored evidence references and
classify unknown or ambiguous app candidates. Activity-domain already contains
the shared app/game AI digest source spine. AI does not scan the OS, count time,
inspect content, or enforce.

## Where We Want To Be

App AI classification consumes only stored evidence/digests and returns
confidence-bounded candidates with evidence refs, uncertainty, model/runtime,
prompt version, and fallback. Parent policy remains the action authority.

## Scope

- [x] App evidence digest input.
- [x] Unknown/ambiguous app candidate digest.
- [x] Risk/category classifier result.
- [x] Prompt/version/model/runtime refs.
- [x] Invalid-output rejection.
- [x] Provider/fallback/degraded states.
- [x] Policy handoff without direct action.

## Touched Paths

- `packages/parent-domain/src/app-game-ai-classifier-boundary.ts`
- `packages/parent-domain/src/app-game-ai-classifier-boundary-values.ts`
- `packages/parent-domain/src/app-game-ai-classifier-boundary-data.ts`
- `packages/parent-domain/tests/app-game-ai-classifier-boundary.test.ts`
- `scripts/test/app-game-ai-classifier-boundary-proof.mjs`
- `output/app-plan-proof/23-app-ai-classifier-digest-boundary/`
- `output/app-game-plan-proof/24-ai-classifier-digest-boundary/`

## Tests And Proof

- [x] AI result missing evidence refs is rejected.
- [x] Confidence outside `0..1` is rejected.
- [x] Block/terminate/suspend/shield action in AI output is rejected.
- [x] Duration field in AI output is rejected.
- [x] Raw process scan result in AI output is rejected.
- [x] Policy consumes AI candidate as evidence only.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [x] Confirm source docs read: [folder README](../README.md),
      [source index](../source-index.md),
      [current snapshot](../current-app-snapshot.md),
      [full scope plan](../v0-5-native-apps-full-scope-plan.md),
      [platform deep dive](../v0-5-native-apps-platform-deep-dive.md),
      [test blueprint](../v0-5-native-apps-test-blueprint.md),
      [UI/UX guide](../ui-ux-requirements-guide.md),
      [main checklist](../implementation-checklist.md), and this workpack.
- [x] Confirm this is native/installed-app scope, not browser pages, browser
      games, or game-specific product semantics unless the source docs
      explicitly route that handoff.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing app/game source layout inspected; no parallel app-control truth
      created.
- [x] Before-state source snapshot recorded in
      `output/app-plan-proof/23-app-ai-classifier-digest-boundary/00-source-snapshot.md`.
- [x] Contracts updated first where this workpack changes behavior.
- [x] Rust/service/portal parity marked N/A because this is TypeScript
      contract proof only.
- [x] Raw evidence artifacts captured where applicable in
      `03-runtime-evidence.json`; live journal/SQLite/read-model behavior is
      N/A for this slice.
- [x] Tests/proof listed in this workpack and
      [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented
      or explicitly marked N/A/manual-required with reason.
- [x] Required fixtures are present in the parent-domain proof matrix; UI and
      platform fixtures are N/A because no UI/platform claim moved.
- [x] Validation command outputs saved in the proof pack and summarized in
      [main checklist](../implementation-checklist.md).
- [x] UI snapshots marked N/A in `ui-not-applicable.md`.
- [x] Security/no-claim negative proof captured.
- [x] Manual platform proof marked N/A because no platform claim moved.
- [x] Platform limitations remain not-claimed/manual-required where runtime
      provider, policy, portal, or adapter proof is missing.
- [x] Evidence/proof artifact paths recorded in
      [main checklist](../implementation-checklist.md).
- [x] Feature/expectation/product-checklist/README update decision recorded in
      [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before
      `DONE`.

## Manual-Required Gaps

AI classification is evidence only. It must not upgrade identity, duration,
content, or enforcement claims.

## Completion

- Branch: `codex/app-game-read-model-service-events`.
- Shared proof: app-game WP24 under
  `output/app-game-plan-proof/24-ai-classifier-digest-boundary/`.
- Native app proof:
  `output/app-plan-proof/23-app-ai-classifier-digest-boundary/`.
- Contract:
  `packages/parent-domain/src/app-game-ai-classifier-boundary.ts`.
- Test:
  `packages/parent-domain/tests/app-game-ai-classifier-boundary.test.ts`.
- Harness:
  `node scripts/test/app-game-ai-classifier-boundary-proof.mjs`.
- Product checklist decision: no product checklist update because product status
  did not move and codex-a owns the checklist lock. Live model quality,
  provider execution, service events, portal rendering, policy evaluator
  consumption, and adapter enforcement remain gaps.
