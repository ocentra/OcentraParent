# AI Plan Implementation Checklist

## Current Branch Proof Snapshot

These entries summarize proof already produced on the current stacked branch.
They are not product-complete AI claims until the service/runtime/read-model path
consumes the same results.

| Proof                                             | Status             | Artifact                                                                                     | Non-claim                                                                                                                                                                                                                                                  |
| ------------------------------------------------- | ------------------ | -------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Controlled captured screens analyzed by local VLM | P3 harness proved  | `output/ai-plan-proof/real-analysis/proof-summary.json`                                      | Uses real window capture and local VLM execution against controlled local fixtures for video/social/game/shopping/bypass/risk text. This is plumbing/harness proof only; live operator external URL/account proof remains before a product-complete claim. |
| Local AI safety result schema validation          | P3 contract proved | `output/ai-plan-proof/real-analysis/youtube-ordinary-video/06-ai-result.json`                | The proof validates generated safety results per scenario; service runtime/read-model consumption remains implementation work.                                                                                                                             |
| Parent policy dry-run decision                    | P3 contract proved | `output/ai-plan-proof/real-analysis/browser-game/07-policy-decision.json`                    | The proof covers allow, warn, ask-parent, time-limit, and block dry-run decisions; browser/network/mobile/broad block enforcement remains separate proof.                                                                                                  |
| Screen-derived action handoff                     | P3 Windows proved  | `output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json`                         | Native owned-process time-limit reaches the real Windows Rust service adapter path; this does not claim browser, network, mobile, or broad block enforcement.                                                                                              |
| Raw image deletion after analysis                 | P3 proved          | `output/ai-plan-proof/real-analysis/proof-summary.json`                                      | Every captured row deletes the raw temp image after analysis; retention/live-view modes are separate explicit opt-in implementation work and are not claimed here.                                                                                         |
| Disabled screen analysis suppression              | P3 proved          | `output/ai-plan-proof/real-analysis/disabled-no-capture-no-ai/01-source-evidence.json`       | Proves the proof harness creates no capture/AI/policy result when the parent setting is disabled; product UI and service-owned disable suppression remain separate runtime wiring.                                                                         |
| Parent explanation snapshots                      | P3 artifact proved | `output/ai-plan-proof/real-analysis/youtube-ordinary-video/10-ui-snapshot.png` and peer rows | These are proof artifact snapshots rendered from scenario outputs, not the production portal runtime.                                                                                                                                                      |

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
- [x] VLM worker execution proof exists.
- [x] Raw image deletion proof exists.
- [ ] Screen summary feeds AI context builder.
- [x] Real browser-use capture artifact feeds AI analysis.
- [x] Real app-use capture artifact feeds AI analysis.
- [x] Timed cadence capture sequence feeds repeated AI analysis without queue flood.

## Real Analysis Proof

- [x] YouTube ordinary video or controlled equivalent is analyzed.
- [x] YouTube or Vimeo education video or controlled equivalent is analyzed.
- [x] Vimeo video or controlled equivalent is analyzed.
- [x] Facebook/social surface or controlled equivalent is analyzed.
- [x] Browser game/cloud-game surface or controlled equivalent is analyzed.
- [x] Native app foreground capture is analyzed.
- [x] Native game or controlled game-window capture is analyzed.
- [x] Native owned-process time-limit capture is analyzed and linked to adapter dispatch proof.
- [x] Bypass-tool fixture/app is analyzed.
- [x] Shopping fixture/page is analyzed.
- [x] School/productivity fixture/page/app is analyzed.
- [x] Unknown activity degrades to unknown/manual-required without invented certainty.
- [x] Timed cadence captures can be analyzed repeatedly at bounded intervals.
- [x] Disabled capture produces no AI analysis from screen.
- [ ] Live external URL/account captures prove real YouTube, Vimeo, social, shopping, and browser-game surfaces; controlled equivalents are not product-complete proof.
- [ ] Final product-complete pipeline proof is deferred to `docs/plans/screen-ai-pipeline-plan` after screen and AI prerequisites are merged or explicitly stacked.

## Policy And Enforcement

- [x] Deterministic policy consumes valid AI results only.
- [ ] AI cannot override stricter parent rules.
- [x] Policy decisions are journaled.
- [ ] Enforcement consumes policy decision only.
- [x] Windows owned-process time-limit adapter dispatch, restart recovery, parent cancel, expiry, and process termination tested from a screen-derived policy decision.
- [ ] Browser, network, mobile, and broad block adapter unavailable/rollback states tested as part of the screen+AI delivery, with real adapters where the target platform is available.

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
- [x] Playwright UI proof.
- [x] `git diff --check`.
- [x] lane/hub guards.
- [x] focused proof scripts.
- [x] real capture proof artifacts under `output/screen-plan-proof/real-capture` when screen-derived AI is in scope.
- [x] real AI analysis proof artifacts under `output/ai-plan-proof/real-analysis`.
- [ ] `npm run validate` or explicit approved omission.
