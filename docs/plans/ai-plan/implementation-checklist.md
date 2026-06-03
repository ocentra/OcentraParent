# AI Plan Implementation Checklist

## Current Branch Proof Snapshot

These entries summarize proof already produced on the current stacked branch.
They are not product-complete AI claims until the service/runtime/read-model path
consumes the same results.

| Proof | Status | Artifact | Non-claim |
| --- | --- | --- | --- |
| Real captured screen analyzed by local VLM | P3 proved | `output/ai-plan-proof/real-analysis/manual-browser-education-vlm/02-screen-analysis-result.json` | Controlled education fixture only; more real social/video/game/app cases remain. |
| Local AI safety result schema validation | P3 contract proved | `output/ai-plan-proof/real-analysis/manual-browser-education-vlm/04-local-ai-safety-result.json` | Remaining implementation task: service runtime must consume this result; script proof is not product-complete. |
| Parent policy dry-run decision | P3 contract proved | `output/ai-plan-proof/real-analysis/manual-browser-education-vlm/06-policy-dry-run-decision.json` | Remaining implementation task: wire the Rust dry-run evaluator/action path after crate-lock coordination; this is not a runtime-action claim. |
| Raw image deletion after analysis | P3 proved | `output/ai-plan-proof/real-analysis/manual-browser-education-vlm/03-deletion-after-analysis.json` | Retention/live-view modes are separate explicit opt-in implementation work and are not claimed here. |

## Contract And Source Truth

- [ ] Source index reconciled against current repo.
- [ ] TabAgent source audit refreshed before reuse.
- [ ] AI expectation doc remains source-of-truth input.
- [ ] Local AI feature doc updated when status changes.
- [ ] Parent assistant feature doc updated when remote/assistant status changes.
- [ ] Product capability checklist updated when proof/gaps change.

## V0.6 Contracts

- [ ] Local AI input contract complete.
- [ ] Local AI result contract complete.
- [ ] Runtime status contract complete.
- [ ] Provider capability contract complete.
- [ ] Job queue contract complete.
- [ ] Provider route contract complete.
- [ ] Context builder contracts complete.
- [ ] Prompt/template version contract complete.
- [ ] Memory reference contract complete.
- [ ] Graph reference contract complete.
- [ ] AI journal/read-model contract complete.
- [ ] Remote assistant contract separated from child safety.

## V0.7 Runtime And Context

- [ ] Stored-evidence context builder implemented.
- [ ] Parent-rule context builder implemented.
- [ ] Deterministic classifier lane implemented.
- [ ] Local text LLM adapter boundary implemented.
- [ ] Local text inference dry-run implemented.
- [ ] Output parser implemented.
- [ ] Invalid output and timeout degrade safely.
- [ ] Provider queue and routing implemented.
- [ ] Runtime status visible in service and portal.
- [ ] AI result journal and SQLite ingest implemented.
- [ ] Parent explanation read model implemented.

## Memory And Graph

- [ ] Recent memory contract implemented.
- [ ] Short-window recent activity implemented.
- [ ] Semantic memory source-citation guard implemented.
- [ ] Graph reference contract implemented.
- [ ] Minimal graph edges implemented.
- [ ] Graph rebuild/source proof exists.

## Screen OCR/VLM

- [ ] OCR worker contract implemented.
- [ ] OCR worker execution proof exists.
- [ ] Guided VLM worker contract implemented.
- [ ] VLM worker execution proof exists.
- [x] Raw image deletion proof exists.
- [ ] Screen summary feeds AI context builder.
- [x] Real browser-use capture artifact feeds AI analysis.
- [ ] Real app-use capture artifact feeds AI analysis.
- [ ] Timed cadence capture sequence feeds repeated AI analysis without queue flood.

## Real Analysis Proof

- [ ] YouTube ordinary video or controlled equivalent is analyzed.
- [ ] YouTube or Vimeo education video or controlled equivalent is analyzed.
- [ ] Vimeo video or controlled equivalent is analyzed.
- [ ] Facebook/social surface or controlled equivalent is analyzed.
- [ ] Browser game/cloud-game surface or controlled equivalent is analyzed.
- [ ] Native app foreground capture is analyzed.
- [ ] Native game or controlled game-window capture is analyzed.
- [ ] Bypass-tool fixture/app is analyzed.
- [ ] Shopping fixture/page is analyzed.
- [x] School/productivity fixture/page/app is analyzed.
- [ ] Unknown activity degrades to unknown/manual-required without invented certainty.
- [ ] Timed cadence captures can be analyzed repeatedly at bounded intervals.
- [ ] Disabled capture produces no AI analysis from screen.
- [ ] Final product-complete pipeline proof is deferred to `docs/plans/screen-ai-pipeline-plan` after screen and AI prerequisites are merged or explicitly stacked.

## Policy And Enforcement

- [ ] Deterministic policy consumes valid AI results only.
- [ ] AI cannot override stricter parent rules.
- [ ] Policy decisions are journaled.
- [ ] Enforcement consumes policy decision only.
- [ ] Adapter unavailable/rollback states tested.

## UI/UX

- [ ] AI runtime status surface.
- [ ] AI jobs/activity surface.
- [ ] AI decision explanation surface.
- [ ] Memory/graph evidence surface.
- [ ] Remote boundary surface.
- [ ] Screen OCR/VLM degraded states visible.
- [ ] UI screenshots captured for changed states.

## Validation

- [ ] TypeScript contract tests.
- [ ] Rust parity tests.
- [ ] Stored-evidence integration tests.
- [ ] Provider route/status tests.
- [ ] Model output parser tests.
- [ ] Policy integration tests.
- [ ] Memory/graph source guard tests.
- [ ] Remote boundary tests.
- [ ] Playwright UI proof.
- [ ] `git diff --check`.
- [ ] lane/hub guards.
- [ ] focused proof scripts.
- [ ] real capture proof artifacts under `output/screen-plan-proof/real-capture` when screen-derived AI is in scope.
- [ ] real AI analysis proof artifacts under `output/ai-plan-proof/real-analysis`.
- [ ] `npm run validate` or explicit approved omission.
