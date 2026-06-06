# AI Plan Implementation Checklist

## Current Branch Proof Snapshot

These entries summarize proof already produced on the current stacked branch.
They are not product-complete AI claims until the service/runtime/read-model path
consumes the same results.

| Proof                                             | Status              | Artifact                                                                                                                               | Non-claim                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------------------------------------------- | ------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Controlled captured screens analyzed by local VLM | P3 harness proved   | `output/ai-plan-proof/real-analysis/proof-summary.json`                                                                                | Uses real window capture and local VLM execution against controlled local fixtures for video/social/game/shopping/bypass/risk text. This is plumbing/harness proof only; live operator external URL/account proof remains before a product-complete claim.                                                                                                                                                                                                                                                   |
| Local AI safety result schema validation          | P3 contract proved  | `output/ai-plan-proof/real-analysis/youtube-ordinary-video/06-ai-result.json`                                                          | The proof validates generated safety results per scenario; service runtime/read-model consumption remains implementation work.                                                                                                                                                                                                                                                                                                                                                                               |
| Parent policy dry-run decision                    | P3 contract proved  | `output/ai-plan-proof/real-analysis/browser-game/07-policy-decision.json`                                                              | The proof covers allow, warn, ask-parent, time-limit, and block dry-run decisions; browser/network/mobile/broad block enforcement remains separate proof.                                                                                                                                                                                                                                                                                                                                                    |
| Screen-derived action handoff                     | P3 Windows proved   | `output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json`                                                                   | Native owned-process time-limit reaches the real Windows Rust service adapter path; this does not claim browser, network, mobile, or broad block enforcement.                                                                                                                                                                                                                                                                                                                                                |
| Raw image deletion after analysis                 | P3 proved           | `output/ai-plan-proof/real-analysis/proof-summary.json`                                                                                | Every captured row deletes the raw temp image after analysis; retention/live-view modes are separate explicit opt-in implementation work and are not claimed here.                                                                                                                                                                                                                                                                                                                                           |
| Disabled screen analysis suppression              | P3 proved           | `output/ai-plan-proof/real-analysis/disabled-no-capture-no-ai/01-source-evidence.json`                                                 | Proves the proof harness creates no capture/AI/policy result when the parent setting is disabled; product UI and service-owned disable suppression remain separate runtime wiring.                                                                                                                                                                                                                                                                                                                           |
| Parent explanation snapshots                      | P3 artifact proved  | `output/ai-plan-proof/real-analysis/youtube-ordinary-video/10-ui-snapshot.png` and peer rows                                           | These are proof artifact snapshots rendered from scenario outputs, not the production portal runtime.                                                                                                                                                                                                                                                                                                                                                                                                        |
| Local AI provider per-device runtime lane         | P3 runtime proved   | `output/ai-plan-proof/local-ai-runtime-provider-proof/proof.json`; `output/ai-plan-proof/local-ai-provider-scheduler-proof/proof.json` | Proves one provider/runtime lane per physical device, parent/child same-device sharing, child-safety priority, queued/degraded/unavailable states, and no duplicate same-device model load. It does not prove LAN cross-device routing, model quality, Portal UI, or API/remote provider availability.                                                                                                                                                                                                       |
| Local AI runtime status read model                | P3 contract proved  | `output/ai-plan-proof/local-ai-runtime-status-read-model-proof/proof-summary.json`                                                     | Projects existing provider proof rows into parent-facing runtime status rows with provider/runtime/model refs, child-safety priority visibility, ready/queued/degraded/unavailable counts, and setup/unavailable visibility. It does not render the production portal, execute a model, prove model quality, use remote/API AI, grant policy authority, or dispatch enforcement.                                                                                                                             |
| Stored-evidence context builder                   | P3 replay proved    | `output/ai-plan-proof/local-ai-stored-evidence-context/proof-summary.json`                                                             | Consumes existing stored browser, app/game, network-flow, and screen-summary proof artifacts through `buildLocalAiEvidenceContext`, proving a ready mixed context with child-device custody, runtime refs, parent-rule refs, and audit refs; hosted non-activity custody rejection; and partial missing-evidence degradation. It does not create fresh capture, execute a model, prove model quality, render portal UI, dispatch enforcement, or use remote/API AI.                                          |
| Windows WinRT OCR worker over captured pixels     | P3 real OCR proved  | `output/ai-plan-proof/screen-winrt-ocr-worker/proof-summary.json`                                                                      | Proves real selected-window browser/native captures can run through Windows WinRT OCR, become schema-valid `ScreenAnalysisResult` evidence, feed allow dry-run policy decisions, and delete raw temp images. It does not claim production OCR quality, service runtime integration, or cross-platform OCR parity.                                                                                                                                                                                            |
| Guided VLM worker contract                        | P3 contract proved  | `output/ai-plan-proof/screen-vlm-worker-contract-proof/proof-summary.json`                                                             | Proves source-cited encrypted queue jobs can enter a guided local VLM worker contract, schema-bound model output can convert into `ScreenAnalysisResult` evidence, policy eligibility requires deleted-image/query-store custody, and raw retention/remote AI are rejected. It does not claim live model execution, production VLM quality, portal UI, or enforcement.                                                                                                                                       |
| VLM execution readiness/status handoff            | P3 contract proved  | `output/ai-plan-proof/screen-vlm-execution-readiness-proof/proof-summary.json`                                                         | Proves encrypted temp-queue VLM jobs can become accepted readiness handoffs, queued/completed/manual-required status rows preserve local model/runtime/template refs, and completed rows require deleted-image/query-store custody. It explicitly does not claim live model execution, production VLM quality, portal runtime rendering, policy authority, or enforcement.                                                                                                                                   |
| VLM journal/read-model projection                 | P3 contract proved  | `output/ai-plan-proof/screen-vlm-journal-read-model-proof/proof-summary.json`                                                          | Proves completed VLM readiness status rows can become encrypted journal lines and Activity Screen read-model rows while preserving local model/runtime/template refs, policy refs, parent-rule refs, explanation refs, deletion refs, journal evidence refs, and no raw image retention. It does not claim live VLM execution, production model quality, portal runtime rendering, policy authority, or enforcement.                                                                                         |
| Service WinRT OCR over live browser capture       | P3 service proved   | `output/screen-ai-pipeline-proof/service-winrt-ocr/proof-summary.json`                                                                 | Starts the real Rust service, captures a live public Wikipedia Chrome window through timed cadence into the encrypted queue, invokes a Windows WinRT OCR adapter through the service analysis runtime, records a `localOcr` Activity Screen row with runtime/model/template metadata and a `school` result, drains the queue, and deletes adapter temp image material. It does not claim production OCR quality, authenticated-account coverage, enforcement, or cross-platform OCR parity.                  |
| Service WinRT OCR policy consumption              | P3 service proved   | `output/screen-ai-pipeline-proof/service-winrt-ocr-policy/proof-summary.json`                                                          | Reruns the real service WinRT OCR proof and consumes that exact `localOcr` Activity Screen row through typed parent-domain policy contracts, proving allow dry-run policy refs, evidence refs, parent rule refs, disabled enforcement handoff, and deleted-image/no-raw-retention custody. It does not claim final enforcement, broad adapters, production OCR quality, or authenticated-account coverage.                                                                                                   |
| Screen AI stricter parent-rule override           | P3 policy proved    | `output/ai-plan-proof/screen-ai-stricter-parent-rule-proof/proof-summary.json`                                                         | Consumes the real service WinRT OCR policy decision and proves a stricter parent-authored screen category rule overrides the local AI allow output while preserving evidence refs, the local AI result ref, dry-run policy shape, and disabled enforcement handoff. It does not rerun live capture, retain raw screenshots, use remote/API AI, claim local AI authority, or claim enforcement.                                                                                                               |
| Screen AI model artifact manifest                 | P3 contract proved  | `output/ai-plan-proof/screen-ai-model-artifact-manifest-proof/proof-summary.json`                                                      | Proves the screen AI local model artifact manifest/config boundary uses existing opaque artifact and manifest refs, verified cache status, local-only runtime status, and provider capability contracts. It does not download a production model, run inference, prove model quality, use remote/API AI, or embed raw evidence.                                                                                                                                                                              |
| Screen AI invalid output degradation              | P3 contract proved  | `output/ai-plan-proof/screen-ai-invalid-output-degrade-proof/proof-summary.json`                                                       | Proves malformed, unparseable, or timed-out screen AI local model output degrades into typed non-enforcing local AI safety results: invalid output is rejected before it becomes a result, unparseable output falls back to `unknown` with `invalid-output` degradation, and timeout falls back to `ask-parent` with overloaded runtime metadata while preserving evidence and parent rule refs. It does not execute a model, prove model quality, rerun capture, render portal UI, or dispatch enforcement. |
| Screen AI model output parser                     | P3 contract proved  | `output/ai-plan-proof/screen-ai-model-output-parser-proof/proof-summary.json`                                                          | Proves existing local AI evaluation input and safety result parser contracts accept schema-valid screen-derived video evidence with local-only runtime metadata and reject malformed action, confidence, unknown/degraded state, evidence/rule list, remote runtime, and missing observation-evidence shapes. It does not execute a model, prove model quality, rerun capture, render portal UI, or dispatch enforcement.                                                                                    |
| Screen-derived adapter readiness                  | P3 readiness proved | `output/screen-ai-pipeline-proof/adapter-readiness/proof-summary.json`                                                                 | Proves screen-derived owned-process time-limit/block decisions retain real Windows adapter proof while broad installed-app, host network/domain, exact active-tab, Android/iOS mobile, and Linux host targets remain manual-required, not-claimed, or unavailable. It does not implement broad/browser/network/mobile adapters or rerun live capture.                                                                                                                                                        |
| Screen summary into local AI context builder      | P3 replay proved    | `output/ai-plan-proof/screen-summary-ai-context/proof-summary.json`                                                                    | Replays the real WinRT OCR worker proof rows through `buildLocalAiEvidenceContext`, proving deleted local screen summaries are selected as `screen-summary` evidence with custody, runtime, parent-rule, and audit refs. It does not create new captures, claim model quality, or claim portal/final enforcement.                                                                                                                                                                                            |
| Screen summary parent explanation/audit context   | P3 replay proved    | `output/ai-plan-proof/screen-summary-parent-explanation/proof-summary.json`                                                            | Replays the real WinRT OCR worker proof rows through local AI context plus parent explanation/audit contracts, proving screen-summary refs, audit refs, parent-rule refs, dry-run policy refs, runtime refs, and deleted-image custody stay cited without raw retention, remote/API AI, portal UI, policy authority, or enforcement claims.                                                                                                                                                                  |
| Screen summary parent explanation read model      | P3 replay proved    | `output/ai-plan-proof/screen-summary-parent-explanation-read-model/proof-summary.json`                                                 | Converts the screen-summary parent explanation proof rows into parent-visible read-model rows that preserve screen-summary refs, audit refs, policy refs, parent rules, runtime refs, custody, and deleted-image state without raw image display, production portal runtime UI, policy authority, or enforcement claims.                                                                                                                                                                                     |
| Screen parent explanation service read model      | P3 service proved   | `output/ai-plan-proof/screen-summary-parent-explanation-service-read-model/proof-summary.json`                                         | Starts the real Rust service against a seeded ActivityStore and requests the Activity Screen read model over WebSocket, proving policy refs, parent rules, local runtime refs, parent explanation refs, deletion reasons, deleted-image state, and child-device custody survive service/query projection. It does not claim production portal rendering, new capture/model inference, remote/API AI, policy authority, or enforcement.                                                                       |
| Screen family AI hub runtime discovery            | P3 loopback proved  | `output/screen-ai-pipeline-proof/family-ai-hub-runtime-discovery/proof-summary.json`                                                   | Starts a real loopback family-hub endpoint, discovers it through existing LAN evidence schemas, links it to the selected screen family-hub route, and submits a redacted-crop job payload without raw screenshot transfer, raw retention, remote/API provider use, or Ocentra-hosted processing. Physical household LAN, production model quality, portal UI, policy authority, and enforcement remain separate gates.                                                                                       |
| Local text inference dry-run                      | P3 dry-run proved   | `output/ai-plan-proof/local-ai-text-inference-dry-run/proof-summary.json`                                                              | Proves schema-valid local AI input and matching local runtime metadata can produce ready, unavailable, and missing-evidence `LocalAiSafetyResult` rows while preserving typed evidence refs, parent-rule refs, runtime refs, prompt version refs, no raw prompt retention, and explicit no model-execution, no remote/API AI, no policy-authority, no enforcement, and no production-model-quality claims.                                                                                                   |
| Local recent memory and short-window activity     | P3 contract proved  | `output/ai-plan-proof/local-ai-recent-memory-window-proof/proof-summary.json`                                                          | Proves a parent-domain read model over the existing local AI context builder that selects only fresh `recent-activity` evidence inside the requested window, returns source-grounded recent-memory refs, omits stale/out-of-window and ungrounded rows, and rejects raw retention, remote/API AI, policy authority, and enforcement overclaims. It does not create fresh capture, execute a model, prove production model quality, render portal UI, or dispatch enforcement.                                |

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
- [x] Runtime status contract complete.
- [ ] Provider capability contract complete.
- [ ] Job queue contract complete.
- [ ] Provider route contract complete.
- [ ] Context builder contracts complete.
- [ ] Prompt/template version contract complete.
- [x] Memory reference contract complete for source-cited recent-activity
      memory in the short-window read model; broader semantic/long-term memory
      quality remains product-scope follow-up.
- [ ] Graph reference contract complete.
- [x] AI journal/read-model contract complete for screen VLM status
      projection; broader AI journal/read-model surfaces remain product-scope
      follow-ups.
- [ ] Remote assistant contract separated from child safety.

## V0.7 Runtime And Context

- [x] Stored-evidence context builder implemented with replay proof over stored
      browser, app/game, network-flow, and screen-summary artifacts.
- [ ] Parent-rule context builder implemented.
- [ ] Deterministic classifier lane implemented.
- [ ] Local text LLM adapter boundary implemented.
- [x] Local text inference dry-run implemented without model-execution,
      remote/API, policy-authority, enforcement, production-model-quality, or
      raw prompt-retention claims.
- [ ] Output parser implemented.
- [x] Invalid output and timeout degrade safely.
- [x] Provider queue and routing implemented.
- [x] Runtime status parent-facing read-model proof implemented; production
      service and portal rendering remain follow-up runtime work.
- [ ] AI result journal and SQLite ingest implemented.
- [x] Parent explanation read-model proof contract implemented; production
      portal consumption remains a UI/runtime gap.
- [x] Screen parent explanation refs preserved through service-backed Activity
      Screen query/read-model projection.

## Memory And Graph

- [x] Recent memory contract implemented for source-grounded recent-activity
      read-model rows over the local AI context builder.
- [x] Short-window recent activity implemented for the parent-domain read-model
      proof path, with stale/out-of-window evidence omitted.
- [x] Semantic memory source-citation guard implemented for screen AI context.
- [ ] Graph reference contract implemented.
- [ ] Minimal graph edges implemented.
- [x] Graph source-citation guard proof exists for screen AI context.

## Screen OCR/VLM

- [x] OCR worker contract implemented.
- [x] OCR worker execution proof exists.
- [x] Service WinRT OCR runtime proof exists.
- [x] Service WinRT OCR policy-consumption proof exists.
- [x] Guided VLM worker contract implemented.
- [x] VLM worker execution proof exists.
- [x] VLM execution readiness/status handoff proof exists.
- [x] Raw image deletion proof exists.
- [x] Screen summary feeds AI context builder.
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
- [x] Service WinRT OCR Activity Screen row feeds typed parent policy dry-run.
- [x] AI cannot override stricter parent rules for the screen service WinRT OCR
      policy proof path.
- [x] Policy decisions are journaled.
- [ ] Enforcement consumes policy decision only.
- [x] Windows owned-process time-limit adapter dispatch, restart recovery, parent cancel, expiry, and process termination tested from a screen-derived policy decision.
- [x] Browser, network, mobile, and broad block adapter unavailable/rollback
      readiness states tested as screen-derived non-claim rows, while only
      Windows owned-process time-limit/block rows claim real adapter execution.

## UI/UX

- [x] AI runtime status surface read-model proof; production portal screenshot
      remains follow-up UI work.
- [ ] AI jobs/activity surface.
- [ ] AI decision explanation surface.
- [ ] Memory/graph evidence surface.
- [ ] Remote boundary surface.
- [ ] Screen OCR/VLM degraded states visible.
- [ ] UI screenshots captured for changed states.

## Validation

- [x] TypeScript contract tests.
- [x] Rust parity tests.
- [ ] Stored-evidence integration tests.
- [x] Provider route/status tests.
- [x] Model output parser tests exist in
      `packages/parent-domain/tests/screen-ai-model-output-parser-proof.test.ts`
      and `scripts/test/screen-ai-model-output-parser-proof.mjs`, with proof
      artifact
      `output/ai-plan-proof/screen-ai-model-output-parser-proof/proof-summary.json`.
- [x] Policy integration tests for stricter parent-rule override on screen AI
      policy decisions.
- [x] Memory/graph source guard tests.
- [ ] Remote boundary tests.
- [x] Playwright UI proof.
- [x] `git diff --check`.
- [x] lane/hub guards.
- [x] focused proof scripts.
- [x] real capture proof artifacts under `output/screen-plan-proof/real-capture` when screen-derived AI is in scope.
- [x] real AI analysis proof artifacts under `output/ai-plan-proof/real-analysis`.
- [x] real WinRT OCR worker proof artifacts under `output/ai-plan-proof/screen-winrt-ocr-worker`.
- [x] local AI runtime status read-model proof artifacts under
      `output/ai-plan-proof/local-ai-runtime-status-read-model-proof`.
- [x] local AI runtime status read-model proof run:
      `node --check scripts/test/local-ai-runtime-status-read-model-proof.mjs`
      and `node scripts/test/local-ai-runtime-status-read-model-proof.mjs`.
- [x] local AI stored-evidence context proof artifacts under
      `output/ai-plan-proof/local-ai-stored-evidence-context`.
- [x] local AI stored-evidence context proof run:
      `node --check scripts/test/local-ai-stored-evidence-context-proof.mjs`
      and `node scripts/test/local-ai-stored-evidence-context-proof.mjs`.
- [x] local AI recent-memory and short-window activity proof artifacts under
      `output/ai-plan-proof/local-ai-recent-memory-window-proof`.
- [x] local AI recent-memory and short-window activity proof run:
      `node --check scripts/test/local-ai-recent-memory-window-proof.mjs` and
      `node scripts/test/local-ai-recent-memory-window-proof.mjs`.
- [x] guided VLM worker contract proof artifacts under
      `output/ai-plan-proof/screen-vlm-worker-contract-proof`.
- [x] guided VLM worker contract proof run:
      `node --check scripts/test/screen-ai-vlm-worker-contract-proof.mjs` and
      `node scripts/test/screen-ai-vlm-worker-contract-proof.mjs`.
- [x] VLM execution readiness/status handoff proof artifacts under
      `output/ai-plan-proof/screen-vlm-execution-readiness-proof`.
- [x] VLM execution readiness/status handoff proof run:
      `node --check scripts/test/screen-ai-vlm-execution-readiness-proof.mjs`
      and `node scripts/test/screen-ai-vlm-execution-readiness-proof.mjs`.
- [x] VLM journal/read-model proof artifacts under
      `output/ai-plan-proof/screen-vlm-journal-read-model-proof`.
- [x] VLM journal/read-model proof run:
      `node --check scripts/test/screen-ai-vlm-journal-read-model-proof.mjs`
      and `node scripts/test/screen-ai-vlm-journal-read-model-proof.mjs`.
- [x] service WinRT OCR runtime proof artifacts under
      `output/screen-ai-pipeline-proof/service-winrt-ocr`.
- [x] service WinRT OCR policy proof artifacts under
      `output/screen-ai-pipeline-proof/service-winrt-ocr-policy`.
- [x] screen AI model artifact manifest proof artifacts under
      `output/ai-plan-proof/screen-ai-model-artifact-manifest-proof`.
- [x] screen AI invalid output degradation proof artifacts under
      `output/ai-plan-proof/screen-ai-invalid-output-degrade-proof`.
- [x] screen AI invalid output degradation proof run:
      `node --check scripts/test/screen-ai-invalid-output-degrade-proof.mjs`
      and `node scripts/test/screen-ai-invalid-output-degrade-proof.mjs`.
- [x] screen AI model output parser proof artifacts under
      `output/ai-plan-proof/screen-ai-model-output-parser-proof`.
- [x] screen AI model output parser proof run:
      `node --check scripts/test/screen-ai-model-output-parser-proof.mjs` and
      `node scripts/test/screen-ai-model-output-parser-proof.mjs`.
- [x] screen AI stricter parent-rule proof artifacts under
      `output/ai-plan-proof/screen-ai-stricter-parent-rule-proof`.
- [x] screen AI stricter parent-rule proof run:
      `node --check scripts/test/screen-ai-stricter-parent-rule-proof.mjs` and
      `node scripts/test/screen-ai-stricter-parent-rule-proof.mjs`.
- [x] screen AI adapter readiness proof artifacts under
      `output/screen-ai-pipeline-proof/adapter-readiness`.
- [x] screen AI adapter readiness proof run:
      `node --check scripts/test/screen-ai-adapter-readiness-proof.mjs` and
      `node scripts/test/screen-ai-adapter-readiness-proof.mjs`.
- [x] screen summary context-builder replay proof artifacts under `output/ai-plan-proof/screen-summary-ai-context`.
- [x] screen summary parent explanation/audit replay proof artifacts under `output/ai-plan-proof/screen-summary-parent-explanation`.
- [x] screen summary parent explanation read-model replay proof artifacts under
      `output/ai-plan-proof/screen-summary-parent-explanation-read-model`.
- [x] screen summary parent explanation service read-model proof artifacts under
      `output/ai-plan-proof/screen-summary-parent-explanation-service-read-model`.
- [x] screen AI memory/graph source guard proof artifacts under
      `output/ai-plan-proof/screen-ai-memory-graph-source-guard-proof`.
- [ ] `npm run validate` or explicit approved omission.
