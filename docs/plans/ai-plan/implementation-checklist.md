# AI Plan Implementation Checklist

## Current Branch Proof Snapshot

These entries summarize proof already produced on the current stacked branch.
They are not product-complete AI claims until the service/runtime/read-model path
consumes the same results.

| Proof                                             | Status                  | Artifact                                                                                                                               | Non-claim                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------------------------------------------- | ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Controlled captured screens analyzed by local VLM | P3 harness proved       | `output/ai-plan-proof/real-analysis/proof-summary.json`                                                                                | Uses real window capture and local VLM execution against controlled local fixtures for video/social/game/shopping/bypass/risk text. This is plumbing/harness proof only; live operator external URL/account proof remains before a product-complete claim.                                                                                                                                                                                                                                                   |
| Local AI safety result schema validation          | P3 contract proved      | `output/ai-plan-proof/real-analysis/youtube-ordinary-video/06-ai-result.json`                                                          | The proof validates generated safety results per scenario; service runtime/read-model consumption remains implementation work.                                                                                                                                                                                                                                                                                                                                                                               |
| Parent policy dry-run decision                    | P3 contract proved      | `output/ai-plan-proof/real-analysis/browser-game/07-policy-decision.json`                                                              | The proof covers allow, warn, ask-parent, time-limit, and block dry-run decisions; browser/network/mobile/broad block enforcement remains separate proof.                                                                                                                                                                                                                                                                                                                                                    |
| Screen-derived action handoff                     | P3 Windows proved       | `output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json`                                                                   | Native owned-process time-limit reaches the real Windows Rust service adapter path; this does not claim browser, network, mobile, or broad block enforcement.                                                                                                                                                                                                                                                                                                                                                |
| Raw image deletion after analysis                 | P3 proved               | `output/ai-plan-proof/real-analysis/proof-summary.json`                                                                                | Every captured row deletes the raw temp image after analysis; retention/live-view modes are separate explicit opt-in implementation work and are not claimed here.                                                                                                                                                                                                                                                                                                                                           |
| Disabled screen analysis suppression              | P3 proved               | `output/ai-plan-proof/real-analysis/disabled-no-capture-no-ai/01-source-evidence.json`                                                 | Proves the proof harness creates no capture/AI/policy result when the parent setting is disabled; product UI and service-owned disable suppression remain separate runtime wiring.                                                                                                                                                                                                                                                                                                                           |
| Parent explanation snapshots                      | P3 artifact proved      | `output/ai-plan-proof/real-analysis/youtube-ordinary-video/10-ui-snapshot.png` and peer rows                                           | These are proof artifact snapshots rendered from scenario outputs, not the production portal runtime.                                                                                                                                                                                                                                                                                                                                                                                                        |
| Local AI provider per-device runtime lane         | P3 runtime proved       | `output/ai-plan-proof/local-ai-runtime-provider-proof/proof.json`; `output/ai-plan-proof/local-ai-provider-scheduler-proof/proof.json` | Proves one provider/runtime lane per physical device, parent/child same-device sharing, child-safety priority, queued/degraded/unavailable states, and no duplicate same-device model load. It does not prove LAN cross-device routing, model quality, Portal UI, or API/remote provider availability.                                                                                                                                                                                                       |
| Event-driven AI consumer boundary                 | P2 screen runtime proof | `output/screen-ai-pipeline-proof/event-driven-runtime/proof-summary.json`                                                              | Proves the screen successful path can consume typed evidence events and publish typed AI result events through the reusable Rust eventing runtime, with policy/action/read-model/deletion phases carrying prior event refs. General AI job contracts, degraded-result events, live service consumers, and household mesh event bridge remain planned.                                                                                                                                                        |
| Household AI provider mesh contracts              | planned                 | `output/ai-plan-proof/household-ai-provider-mesh-contract/proof-summary.json`                                                          | Must prove provider advertisement, capability, AI work item, claim, lease, result validation, custody, child-agent authority, and event-driven state transitions. Does not claim physical household LAN product readiness.                                                                                                                                                                                                                                                                                   |
| Household mesh event bridge                       | planned                 | `output/ai-plan-proof/household-mesh-event-bridge-proof/proof-summary.json`                                                            | Must prove selected local events become typed LAN messages, incoming LAN messages are validated before local republish, and remote peers cannot publish directly into another runtime's bus.                                                                                                                                                                                                                                                                                                                 |
| Household LAN AI provider claim lease             | planned                 | `output/ai-plan-proof/household-ai-provider-claim-lease-proof/proof-summary.json`                                                      | Must prove one lease per job, duplicate claim rejection, lease expiry requeue, max-attempt dead-letter, and idempotent duplicate message behavior.                                                                                                                                                                                                                                                                                                                                                           |
| Child-agent AI policy authority proof             | planned                 | `output/ai-plan-proof/child-agent-ai-policy-authority-proof/proof-summary.json`                                                        | Must prove provider returns worker result only, child agent validates/stores result, policy consumes only accepted result, and provider cannot publish policy/enforcement events.                                                                                                                                                                                                                                                                                                                            |
| Mobile dormant AI provider proof                  | planned                 | `output/ai-plan-proof/mobile-dormant-ai-provider-proof/proof-summary.json`                                                             | Must prove mobile provider does not claim heavy jobs while desktop/laptop provider exists, degrades under low battery/thermal state, and is eligible only under explicit fallback policy.                                                                                                                                                                                                                                                                                                                    |
| Local AI runtime status read model                | P3 contract proved      | `output/ai-plan-proof/local-ai-runtime-status-read-model-proof/proof-summary.json`                                                     | Projects existing provider proof rows into parent-facing runtime status rows with provider/runtime/model refs, child-safety priority visibility, ready/queued/degraded/unavailable counts, and setup/unavailable visibility. It does not render the production portal, execute a model, prove model quality, use remote/API AI, grant policy authority, or dispatch enforcement.                                                                                                                             |
| Stored-evidence context builder                   | P3 replay proved        | `output/ai-plan-proof/local-ai-stored-evidence-context/proof-summary.json`                                                             | Consumes existing stored browser, app/game, network-flow, and screen-summary proof artifacts through `buildLocalAiEvidenceContext`, proving a ready mixed context with child-device custody, runtime refs, parent-rule refs, and audit refs; hosted non-activity custody rejection; and partial missing-evidence degradation. It does not create fresh capture, execute a model, prove model quality, render portal UI, dispatch enforcement, or use remote/API AI.                                          |
| Windows WinRT OCR worker over captured pixels     | P3 real OCR proved      | `output/ai-plan-proof/screen-winrt-ocr-worker/proof-summary.json`                                                                      | Proves real selected-window browser/native captures can run through Windows WinRT OCR, become schema-valid `ScreenAnalysisResult` evidence, feed allow dry-run policy decisions, and delete raw temp images. It does not claim production OCR quality, service runtime integration, or cross-platform OCR parity.                                                                                                                                                                                            |
| Guided VLM worker contract                        | P3 contract proved      | `output/ai-plan-proof/screen-vlm-worker-contract-proof/proof-summary.json`                                                             | Proves source-cited encrypted queue jobs can enter a guided local VLM worker contract, schema-bound model output can convert into `ScreenAnalysisResult` evidence, policy eligibility requires deleted-image/query-store custody, and raw retention/remote AI are rejected. It does not claim live model execution, production VLM quality, portal UI, or enforcement.                                                                                                                                       |
| VLM execution readiness/status handoff            | P3 contract proved      | `output/ai-plan-proof/screen-vlm-execution-readiness-proof/proof-summary.json`                                                         | Proves encrypted temp-queue VLM jobs can become accepted readiness handoffs, queued/completed/manual-required status rows preserve local model/runtime/template refs, and completed rows require deleted-image/query-store custody. It explicitly does not claim live model execution, production VLM quality, portal runtime rendering, policy authority, or enforcement.                                                                                                                                   |
| VLM journal/read-model projection                 | P3 contract proved      | `output/ai-plan-proof/screen-vlm-journal-read-model-proof/proof-summary.json`                                                          | Proves completed VLM readiness status rows can become encrypted journal lines and Activity Screen read-model rows while preserving local model/runtime/template refs, policy refs, parent-rule refs, explanation refs, deletion refs, journal evidence refs, and no raw image retention. It does not claim live VLM execution, production model quality, portal runtime rendering, policy authority, or enforcement.                                                                                         |
| Service WinRT OCR over live browser capture       | P3 service proved       | `output/screen-ai-pipeline-proof/service-winrt-ocr/proof-summary.json`                                                                 | Starts the real Rust service, captures a live public Wikipedia Chrome window through timed cadence into the encrypted queue, invokes a Windows WinRT OCR adapter through the service analysis runtime, records a `localOcr` Activity Screen row with runtime/model/template metadata and a `school` result, drains the queue, and deletes adapter temp image material. It does not claim production OCR quality, authenticated-account coverage, enforcement, or cross-platform OCR parity.                  |
| Service WinRT OCR policy consumption              | P3 service proved       | `output/screen-ai-pipeline-proof/service-winrt-ocr-policy/proof-summary.json`                                                          | Reruns the real service WinRT OCR proof and consumes that exact `localOcr` Activity Screen row through typed parent-domain policy contracts, proving allow dry-run policy refs, evidence refs, parent rule refs, disabled enforcement handoff, and deleted-image/no-raw-retention custody. It does not claim final enforcement, broad adapters, production OCR quality, or authenticated-account coverage.                                                                                                   |
| Screen AI stricter parent-rule override           | P3 policy proved        | `output/ai-plan-proof/screen-ai-stricter-parent-rule-proof/proof-summary.json`                                                         | Consumes the real service WinRT OCR policy decision and proves a stricter parent-authored screen category rule overrides the local AI allow output while preserving evidence refs, the local AI result ref, dry-run policy shape, and disabled enforcement handoff. It does not rerun live capture, retain raw screenshots, use remote/API AI, claim local AI authority, or claim enforcement.                                                                                                               |
| Screen AI model artifact manifest                 | P3 contract proved      | `output/ai-plan-proof/screen-ai-model-artifact-manifest-proof/proof-summary.json`                                                      | Proves the screen AI local model artifact manifest/config boundary uses existing opaque artifact and manifest refs, verified cache status, local-only runtime status, and provider capability contracts. It does not download a production model, run inference, prove model quality, use remote/API AI, or embed raw evidence.                                                                                                                                                                              |
| Screen AI invalid output degradation              | P3 contract proved      | `output/ai-plan-proof/screen-ai-invalid-output-degrade-proof/proof-summary.json`                                                       | Proves malformed, unparseable, or timed-out screen AI local model output degrades into typed non-enforcing local AI safety results: invalid output is rejected before it becomes a result, unparseable output falls back to `unknown` with `invalid-output` degradation, and timeout falls back to `ask-parent` with overloaded runtime metadata while preserving evidence and parent rule refs. It does not execute a model, prove model quality, rerun capture, render portal UI, or dispatch enforcement. |
| Screen AI model output parser                     | P3 contract proved      | `output/ai-plan-proof/screen-ai-model-output-parser-proof/proof-summary.json`                                                          | Proves existing local AI evaluation input and safety result parser contracts accept schema-valid screen-derived video evidence with local-only runtime metadata and reject malformed action, confidence, unknown/degraded state, evidence/rule list, remote runtime, and missing observation-evidence shapes. It does not execute a model, prove model quality, rerun capture, render portal UI, or dispatch enforcement.                                                                                    |
| Screen-derived adapter readiness                  | P3 readiness proved     | `output/screen-ai-pipeline-proof/adapter-readiness/proof-summary.json`                                                                 | Proves screen-derived owned-process time-limit/block decisions retain real Windows adapter proof while broad installed-app, host network/domain, exact active-tab, Android/iOS mobile, and Linux host targets remain manual-required, not-claimed, or unavailable. It does not implement broad/browser/network/mobile adapters or rerun live capture.                                                                                                                                                        |
| Screen summary into local AI context builder      | P3 replay proved        | `output/ai-plan-proof/screen-summary-ai-context/proof-summary.json`                                                                    | Replays the real WinRT OCR worker proof rows through `buildLocalAiEvidenceContext`, proving deleted local screen summaries are selected as `screen-summary` evidence with custody, runtime, parent-rule, and audit refs. It does not create new captures, claim model quality, or claim portal/final enforcement.                                                                                                                                                                                            |
| Screen summary parent explanation/audit context   | P3 replay proved        | `output/ai-plan-proof/screen-summary-parent-explanation/proof-summary.json`                                                            | Replays the real WinRT OCR worker proof rows through local AI context plus parent explanation/audit contracts, proving screen-summary refs, audit refs, parent-rule refs, dry-run policy refs, runtime refs, and deleted-image custody stay cited without raw retention, remote/API AI, portal UI, policy authority, or enforcement claims.                                                                                                                                                                  |
| Screen summary parent explanation read model      | P3 replay proved        | `output/ai-plan-proof/screen-summary-parent-explanation-read-model/proof-summary.json`                                                 | Converts the screen-summary parent explanation proof rows into parent-visible read-model rows that preserve screen-summary refs, audit refs, policy refs, parent rules, runtime refs, custody, and deleted-image state without raw image display, production portal runtime UI, policy authority, or enforcement claims.                                                                                                                                                                                     |
| Screen parent explanation service read model      | P3 service proved       | `output/ai-plan-proof/screen-summary-parent-explanation-service-read-model/proof-summary.json`                                         | Starts the real Rust service against a seeded ActivityStore and requests the Activity Screen read model over WebSocket, proving policy refs, parent rules, local runtime refs, parent explanation refs, deletion reasons, deleted-image state, and child-device custody survive service/query projection. It does not claim production portal rendering, new capture/model inference, remote/API AI, policy authority, or enforcement.                                                                       |
| Screen family AI hub runtime discovery            | P3 loopback proved      | `output/screen-ai-pipeline-proof/family-ai-hub-runtime-discovery/proof-summary.json`                                                   | Starts a real loopback family-hub endpoint, discovers it through existing LAN evidence schemas, links it to the selected screen family-hub route, and submits a redacted-crop job payload without raw screenshot transfer, raw retention, remote/API provider use, or Ocentra-hosted processing. Physical household LAN, production model quality, portal UI, policy authority, and enforcement remain separate gates.                                                                                       |
| Local AI deterministic classifier                 | P3 contract proved      | `output/ai-plan-proof/local-ai-deterministic-classifier-proof/proof-summary.json`                                                      | Proves typed local AI inputs can produce deterministic dry-run `LocalAiSafetyResult` rows for classify, allow, warn, ask-parent, time-limit, block, missing-evidence, and runtime-unavailable cases while preserving evidence, parent-rule, runtime, prompt, and trace refs. It does not execute a model, prove production model quality, use remote/API AI, retain raw evidence, grant policy authority, render portal UI, or dispatch enforcement.                                                         |
| Local AI classifier report read model             | P3 contract proved      | `output/ai-plan-proof/local-ai-classifier-read-model-manual-report-proof/proof-summary.json`                                           | Projects deterministic classifier dry-run rows into parent-domain ready, manual-required, and unavailable report rows while preserving evidence, parent-rule, runtime, provider, model, prompt, proof, and trace refs. It does not execute a model, prove production model quality, use remote/API AI, retain raw evidence or raw model output, grant policy authority, render portal UI, or dispatch enforcement.                                                                                           |
| Local text LLM adapter boundary                   | P3 contract proved      | `output/ai-plan-proof/local-ai-text-llm-adapter-boundary-proof/proof-summary.json`                                                     | Proves schema-valid local text adapter handoff rows for ready, unavailable, and manual-required states while preserving local runtime, provider, model, prompt, evidence, parent-rule, parser, and trace refs. It rejects raw prompt retention, raw model-output retention, model execution, remote/API AI, policy authority, enforcement, and production model-quality overclaims.                                                                                                                          |
| Local text inference dry-run                      | P3 dry-run proved       | `output/ai-plan-proof/local-ai-text-inference-dry-run/proof-summary.json`                                                              | Proves schema-valid local AI input and matching local runtime metadata can produce ready, unavailable, and missing-evidence `LocalAiSafetyResult` rows while preserving typed evidence refs, parent-rule refs, runtime refs, prompt version refs, no raw prompt retention, and explicit no model-execution, no remote/API AI, no policy-authority, no enforcement, and no production-model-quality claims.                                                                                                   |
| Local AI result journal/SQLite ingest             | P3 contract proved      | `output/ai-plan-proof/local-ai-result-journal-sqlite-proof/proof-summary.json`                                                         | Proves ready, unavailable, and manual-required `LocalAiSafetyResult` rows can become journal entries, SQLite-ingest rows, and a parent-facing read-model snapshot while preserving result, request, evidence, parent-rule, runtime, provider, model, prompt, and proof refs. It does not claim production storage runtime, model execution, model quality, raw prompt/model-output retention, remote/API AI, policy authority, portal UI, or enforcement.                                                    |
| Local AI prompt/template version                  | P3 contract proved      | `output/ai-plan-proof/local-ai-prompt-template-version-proof/proof-summary.json`                                                       | Proves the prompt/template version is schema-bound and reconciled across context-builder request, local AI evaluation input, safety result, provider/model metadata, input-binding rows, and output schema refs while rejecting raw prompt retention, raw model-output retention, remote/API AI, policy authority, enforcement, portal UI, model execution, and production model-quality claims.                                                                                                             |
| Local recent memory and short-window activity     | P3 contract proved      | `output/ai-plan-proof/local-ai-recent-memory-window-proof/proof-summary.json`                                                          | Proves a parent-domain read model over the existing local AI context builder that selects only fresh `recent-activity` evidence inside the requested window, returns source-grounded recent-memory refs, omits stale/out-of-window and ungrounded rows, and rejects raw retention, remote/API AI, policy authority, and enforcement overclaims. It does not create fresh capture, execute a model, prove production model quality, render portal UI, or dispatch enforcement.                                |
| Local AI graph reference/minimal edges            | P3 contract proved      | `output/ai-plan-proof/local-ai-graph-reference-contract-proof/proof-summary.json`                                                      | Proves local AI graph refs are schema-validated, source-cited, and read through minimal activity-memory graph edges only when selected evidence, policy version, parent action refs, freshness, endpoints, and time range match. It does not claim a production graph storage/index runtime, model execution, model quality, UI, policy authority, enforcement, remote/API AI, or raw evidence retention.                                                                                                    |
| Local AI contract completeness                    | P3 contract proved      | `output/ai-plan-proof/local-ai-contract-completeness-proof/proof-summary.json`                                                         | Proves the baseline local AI input, safety-result, provider-capability, job-queue, and provider-route contracts line up over one local-only runtime route while preserving evidence refs, parent-rule refs, memory/graph refs, prompt/runtime refs, queue position, duplicate-runtime blocking, and provider capability metadata. It does not claim model execution, model quality, portal UI, policy authority, enforcement, remote/API AI, raw prompt retention, or raw evidence retention.                |
| Remote assistant child-safety boundary            | P3 contract proved      | `output/ai-plan-proof/local-ai-remote-assistant-boundary-proof/proof-summary.json`                                                     | Proves parent-authorized remote assistant requests must cite approved stored evidence and parent-owned report bundles, stay outside the child-safety decision path, preserve local AI and deterministic policy authority, degrade to local-only fallback, and reject raw retention, policy-authority, enforcement, and remote-override overclaims. It does not execute a remote provider, render portal UI, prove model quality, or change local safety decisions.                                           |
| Parent-rule context builder                       | P3 contract proved      | `output/ai-plan-proof/local-ai-parent-rule-context-builder-proof/proof-summary.json`                                                   | Proves grounded parent-rule context refs are selected only when their target evidence refs are already selected by `buildLocalAiEvidenceContext`, while ungrounded parent-rule refs are omitted and degraded with `parent-rule-missing`. It rejects raw evidence retention, remote/API AI, model execution, model quality, policy authority, enforcement, and portal UI overclaims. It does not create fresh capture, execute a model, prove model quality, render portal UI, or dispatch enforcement.       |

## Contract And Source Truth

- [ ] Source index reconciled against current repo.
- [ ] TabAgent source audit refreshed before reuse.
- [ ] AI expectation doc remains source-of-truth input.
- [ ] Local AI feature doc updated when status changes.
- [ ] Parent assistant feature doc updated when remote/assistant status changes.
- [ ] Product capability checklist updated when proof/gaps change.

## V0.6 Contracts

- [x] Local AI input contract complete for evidence-cited request, current
      observation, parent-rule, memory, graph, model-request, and prompt refs.
- [x] Local AI result contract complete for evidence-cited result, parent-rule,
      memory, graph, local runtime, prompt, confidence, unknown, and degraded
      state refs.
- [x] Runtime status contract complete.
- [x] Provider capability contract complete for local-only provider task,
      resource class, privacy mode, and fallback-order metadata.
- [x] Job queue contract complete for physical-device scheduler state, queued
      child-safety job position, duplicate-runtime blocking, and unavailable
      route rejection.
- [x] Provider route contract complete for selected local runtime refs and
      provider/runtime/status alignment.
- [x] Eventing consumer contract complete for the screen successful-path
      evidence-event -> AI-result event flow on `crates/ocentra-eventing`;
      general AI job contracts and degraded-result events remain follow-up work.
- [ ] Household AI provider mesh contracts complete for provider advertisement,
      heartbeat, capability, work item, claim, lease, result, validation,
      custody, and child-agent authority.
- [ ] Mesh transport envelope contract complete for selected event export/import
      through the Household Mesh Bridge.
- [x] Context builder contracts complete, including stored-evidence replay,
      screen-summary replay, recent-memory/graph selection, and parent-rule
      context builder proof rows.
- [x] Prompt/template version contract complete for context-builder,
      evaluation-input, safety-result, provider/model, input-binding, output
      schema, and non-retention refs.
- [x] Memory reference contract complete for source-cited recent-activity
      memory in the short-window read model; broader semantic/long-term memory
      quality remains product-scope follow-up.
- [x] Graph reference contract complete for source-cited local AI graph refs and
      minimal activity-memory graph edge proof; production graph storage/index
      runtime remains a follow-up.
- [x] AI journal/read-model contract complete for screen VLM status
      projection; broader AI journal/read-model surfaces remain product-scope
      follow-ups.
- [x] Remote assistant contract separated from child safety for
      parent-authorized report/explanation requests; production remote provider
      execution and portal assistant UI remain follow-up scope.

## V0.7 Runtime And Context

- [x] Stored-evidence context builder implemented with replay proof over stored
      browser, app/game, network-flow, and screen-summary artifacts.
- [x] Parent-rule context builder implemented with grounded selection and
      ungrounded-rule degradation proof.
- [x] Deterministic classifier lane implemented for local-only dry-run
      classify/allow/warn/ask-parent/time-limit/block rows without
      model-execution, remote/API, raw-evidence retention, policy-authority,
      enforcement, portal UI, or production-model-quality claims.
- [x] Classifier read-model/manual-required report bridge implemented for
      ready, manual-required, and unavailable parent-facing rows without
      model-execution, remote/API, raw retention, policy-authority,
      enforcement, portal UI, or production-model-quality claims.
- [x] Local text LLM adapter boundary implemented without model-execution,
      remote/API, policy-authority, enforcement, production-model-quality, raw
      prompt, or raw model-output retention claims.
- [x] Local text inference dry-run implemented without model-execution,
      remote/API, policy-authority, enforcement, production-model-quality, or
      raw prompt-retention claims.
- [ ] Output parser implemented.
- [x] Invalid output and timeout degrade safely.
- [x] Local provider queue and same-device routing implemented.
- [ ] Household provider route selection implemented and proved with desktop/
      laptop preference, mobile dormant/fallback, stale/offline/revoked
      rejection, and custody mismatch rejection.
- [ ] Provider claim/lease/idempotency lifecycle implemented and proved.
- [ ] Provider result validation implemented and proved before policy
      consumption.
- [x] Runtime status parent-facing read-model proof implemented; production
      service and portal rendering remain follow-up runtime work.
- [x] AI result journal and SQLite ingest proof implemented without production
      storage runtime, raw prompt/model-output retention, remote/API AI,
      policy authority, portal UI, or enforcement claims.
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
- [x] Graph reference contract implemented with source evidence, policy/action
      citation, confidence, derived-index version, and explicit non-claims.
- [x] Minimal graph edges implemented for local activity-memory graph reads with
      selected evidence, policy/action refs, endpoint, freshness, and time-window
      filtering.
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
- [ ] Household AI provider mesh surface: provider class, trust, capability,
      queue depth, heartbeat, resource state, worker-only status, claim/lease,
      validation, and child-agent authority.
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
- [x] Remote boundary tests cover parent-authorized remote assistant requests,
      local-only fallback, child-safety path rejection, raw-retention rejection,
      and policy/enforcement/override overclaim rejection in
      `packages/parent-domain/tests/local-ai-remote-assistant-boundary-proof.test.ts`
      and `scripts/test/local-ai-remote-assistant-boundary-proof.mjs`.
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
- [x] local AI result journal/SQLite ingest proof artifacts under
      `output/ai-plan-proof/local-ai-result-journal-sqlite-proof`.
- [x] local AI result journal/SQLite ingest proof run:
      `node --check scripts/test/local-ai-result-journal-sqlite-proof.mjs` and
      `node scripts/test/local-ai-result-journal-sqlite-proof.mjs`.
- [x] local AI stored-evidence context proof artifacts under
      `output/ai-plan-proof/local-ai-stored-evidence-context`.
- [x] local AI stored-evidence context proof run:
      `node --check scripts/test/local-ai-stored-evidence-context-proof.mjs`
      and `node scripts/test/local-ai-stored-evidence-context-proof.mjs`.
- [x] local AI prompt/template version proof artifacts under
      `output/ai-plan-proof/local-ai-prompt-template-version-proof`.
- [x] local AI prompt/template version proof run:
      `node --check scripts/test/local-ai-prompt-template-version-proof.mjs`
      and `node scripts/test/local-ai-prompt-template-version-proof.mjs`.
- [x] local AI recent-memory and short-window activity proof artifacts under
      `output/ai-plan-proof/local-ai-recent-memory-window-proof`.
- [x] local AI recent-memory and short-window activity proof run:
      `node --check scripts/test/local-ai-recent-memory-window-proof.mjs` and
      `node scripts/test/local-ai-recent-memory-window-proof.mjs`.
- [x] local AI contract completeness proof artifacts under
      `output/ai-plan-proof/local-ai-contract-completeness-proof`.
- [x] local AI contract completeness proof run:
      `node --check scripts/test/local-ai-contract-completeness-proof.mjs` and
      `node scripts/test/local-ai-contract-completeness-proof.mjs`.
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
- [x] local AI remote assistant boundary proof artifacts under
      `output/ai-plan-proof/local-ai-remote-assistant-boundary-proof`.
- [x] local AI remote assistant boundary proof run:
      `node --check scripts/test/local-ai-remote-assistant-boundary-proof.mjs`
      and `node scripts/test/local-ai-remote-assistant-boundary-proof.mjs`.
- [x] local AI deterministic classifier proof artifacts under
      `output/ai-plan-proof/local-ai-deterministic-classifier-proof`.
- [x] local AI deterministic classifier proof run:
      `node --check scripts/test/local-ai-deterministic-classifier-proof.mjs`
      and `node scripts/test/local-ai-deterministic-classifier-proof.mjs`.
- [x] local AI classifier read-model/manual-required report proof artifacts under
      `output/ai-plan-proof/local-ai-classifier-read-model-manual-report-proof`.
- [x] local AI classifier read-model/manual-required report proof run:
      `node --check scripts/test/local-ai-classifier-read-model-manual-report-proof.mjs`
      and
      `node scripts/test/local-ai-classifier-read-model-manual-report-proof.mjs`.
- [x] local text LLM adapter boundary proof artifacts under
      `output/ai-plan-proof/local-ai-text-llm-adapter-boundary-proof`.
- [x] local text LLM adapter boundary proof run:
      `node --check scripts/test/local-ai-text-llm-adapter-boundary-proof.mjs`
      and `node scripts/test/local-ai-text-llm-adapter-boundary-proof.mjs`.
- [ ] Household AI provider mesh contract proof run.
- [ ] Household mesh event bridge proof run.
- [ ] Household AI provider claim/lease proof run.
- [ ] Household AI provider result validation proof run.
- [ ] Child-agent AI policy authority proof run.
- [ ] Mobile dormant AI provider proof run.
- [ ] No-raw-screen-transfer mesh proof run.
- [ ] AI mesh event topology proof run.
- [ ] `npm run validate` or explicit approved omission.
