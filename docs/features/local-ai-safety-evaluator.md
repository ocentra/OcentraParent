# Local AI Safety Evaluator

## Parent Outcome

Parents can get context-aware decisions and explanations from local evidence:
allow, warn, time-limit, ask-parent, block, or unknown, with evidence refs and
confidence/degraded status.

## Ocentra Requirement

Child-safety AI runs in the child-device safety path. It consumes typed evidence
and parent rules, produces typed results, and feeds deterministic policy. It
does not directly enforce.

## Roadmap And Expectations

- Roadmap: V0.6 local AI contracts, V0.7 local AI policy evaluator, V5 policy
  product.
- Expectations: [AI](../expectations/ai.md), [policy](../expectations/policy.md),
  [data custody](../expectations/data-custody.md).
- Browser URL/video AI planning:
  [browser URL and video AI intelligence plan](../plans/browser-plan/v0-5-browser-url-video-ai-intelligence-plan.md).
- Browser social/platform AI planning:
  [social platform account feed and gating plan](../plans/browser-plan/v0-5-social-platform-account-feed-gating-plan.md).
- Browser-game AI planning:
  [browser games/cloud gaming gating plan](../plans/browser-plan/v0-5-browser-games-cloud-gaming-gating-plan.md).
- Modules: `packages/parent-domain`, `crates/agent-service`,
  `crates/agent-core`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially AI
assistant setup/control, video safety, social app controls, and local-first
privacy.

Competitors often rely on static categories, platform ratings, or cloud ML.
Ocentra's differentiator is local, evidence-backed, parent-rule-aware AI. That
claim needs proof, not slogans.

## Current Ocentra State

- Local AI runtime/provider status and provider scheduler proof exist. The
  provider proof now includes one runtime access lane per physical device:
  TypeScript and Rust proof entries carry `runtimeAccessLaneCount=1`, the Rust
  service scheduler keeps independent lanes per physical device, same-device
  parent/child jobs still share one lane, and tracked proof artifacts live under
  `output/ai-plan-proof/local-ai-runtime-provider-proof` and
  `output/ai-plan-proof/local-ai-provider-scheduler-proof`.
- `LocalAiRuntimeStatusSurfaceReadModelSchema` and
  `scripts/test/local-ai-runtime-status-read-model-proof.mjs` now project the
  existing provider proof rows into parent-facing runtime status rows. The proof
  preserves provider/runtime/model refs, child-safety priority visibility,
  ready/queued/degraded/unavailable counts, and explicit setup/unavailable
  states while keeping production portal rendering, remote/API AI, policy
  authority, model execution/quality, and enforcement unclaimed.
- The screen service analysis runtime now consumes an encrypted screen queue job,
  runs through the local provider scheduler and service-owned local adapter
  command boundary, records `localVision`, `localOcr`, or explicit
  unavailable/invalid output Activity Screen rows, and drains processed queue
  records. The service proof adapter validates runtime plumbing and custody; the
  Windows WinRT OCR proof validates local OCR over live public Wikipedia pixels,
  but neither proof claims production model/OCR quality.
- `ScreenVlmWorkerJobSchema`, `ScreenVlmWorkerResultSchema`, and
  `scripts/test/screen-ai-vlm-worker-contract-proof.mjs` now provide a
  first-class guided VLM worker contract matching the OCR worker boundary:
  source-cited encrypted temp queue input, bounded local image pixels,
  schema-bound local model output, conversion to `ScreenAnalysisResult`,
  deleted-image/query-store custody before policy eligibility, and no raw
  retention or remote/API AI. This is contract proof, not live model-quality or
  production inference proof.
- `ScreenVlmExecutionReadinessProofSchema` and
  `scripts/test/screen-ai-vlm-execution-readiness-proof.mjs` now prove the
  next VLM handoff/status layer: accepted encrypted temp-queue handoffs,
  queued/completed/manual-required status rows, preserved local
  model/runtime/template refs, deleted query-store custody for completed rows,
  and explicit non-claims for live model execution, production VLM quality,
  portal runtime rendering, policy authority, and enforcement.
- `local-ai-parent-assistant-runtime-proof` now ties the provider scheduler
  proof to Parent Assistant answer/status/action contracts, including cited
  local answer, queued/degraded/unavailable lifecycle, child-safety priority,
  optional API boundary, and no direct enforcement.
- Parent Assistant provider routing proof now exposes local configured,
  degraded, unavailable, API authorized-unavailable, and API authorized-degraded
  states without making remote/API AI part of child safety decisions.
- Dry-run policy evaluator and evidence context builder exist in proof form.
- `scripts/test/local-ai-stored-evidence-context-proof.mjs` now proves the
  general stored-evidence context-builder path over existing stored browser,
  app/game, network-flow, and screen-summary proof artifacts. The proof builds a
  ready mixed context with child-device custody, local runtime refs, parent-rule
  refs, and audit evidence refs; rejects Ocentra-hosted non-activity custody
  before child-activity evidence can enter local model input; and degrades to a
  partial context with explicit missing evidence kinds. It consumes existing
  proof artifacts and does not create fresh capture, execute a model, prove
  model quality, render portal UI, dispatch enforcement, or use remote/API AI.
- `LocalAiParentRuleContextBuilderProofSchema` and
  `scripts/test/local-ai-parent-rule-context-builder-proof.mjs` now prove the
  parent-rule context builder path over the existing stored-evidence context
  builder. Grounded parent-rule context refs are selected only when their target
  evidence refs are already selected for local model input, while ungrounded
  parent-rule refs are omitted and degraded with `parent-rule-missing`. The proof
  rejects raw evidence retention, remote/API AI, model-execution,
  model-quality, policy-authority, enforcement, and portal UI overclaims.
- `LocalAiTextInferenceDryRunResultSchema` and
  `scripts/test/local-ai-text-inference-dry-run-proof.mjs` now prove the local
  text inference dry-run boundary over schema-valid local AI input and matching
  local runtime metadata. The proof emits ready, unavailable, and
  missing-evidence `LocalAiSafetyResult` rows while preserving typed evidence
  refs, parent-rule refs, runtime refs, prompt version refs, no raw prompt
  retention, and explicit no model-execution, no remote/API AI, no
  policy-authority, no enforcement, and no production-model-quality claims.
- `LocalAiTextLlmAdapterBoundaryProofSchema` and
  `scripts/test/local-ai-text-llm-adapter-boundary-proof.mjs` now prove the
  local text LLM adapter handoff boundary before parser/result creation. The
  proof preserves local runtime, provider, model, prompt, evidence, parent-rule,
  parser, and trace refs; emits ready, unavailable, and manual-required adapter
  rows; and rejects raw prompt retention, raw model-output retention, remote/API
  AI, model-execution, policy-authority, enforcement, and production-quality
  overclaims.
- `LocalAiTextOutputParserProofSchema` and
  `scripts/test/local-ai-text-output-parser-proof.mjs` now prove the generic
  local text output parser boundary after the adapter handoff. Schema-valid
  local output becomes policy-eligible `LocalAiSafetyResult` material only when
  runtime, provider, model, prompt, evidence, and parent-rule refs match the
  adapter proof; malformed, remote, mismatched, or manual-required output is
  rejected before policy eligibility without retaining raw model output.
- `LocalAiTextParserReadModelSnapshotSchema` and
  `scripts/test/local-ai-text-parser-read-model-proof.mjs` now prove the
  parent-visible read-model projection over generic local text parser rows.
  Parsed local output becomes ready read-model rows with parser, runtime,
  provider, model, prompt, evidence, parent-rule, and source-proof refs, while
  rejected or manual parser rows remain visible as manual-required without raw
  output retention, model execution, remote/API AI, policy authority,
  enforcement, portal UI, or production-quality claims.
- `LocalAiPromptTemplateVersionProofSchema` and
  `scripts/test/local-ai-prompt-template-version-proof.mjs` now prove the
  prompt/template version contract boundary. The proof reconciles one
  schema-bound prompt version across context-builder request, local AI
  evaluation input, safety result, provider/model metadata, input-binding rows,
  and output schema refs while rejecting raw prompt retention, raw model-output
  retention, remote/API AI, policy authority, enforcement, portal UI, model
  execution, and production model-quality claims.
- `LocalAiResultReadModelSnapshotSchema` and
  `scripts/test/local-ai-result-journal-sqlite-proof.mjs` now prove local AI
  safety results can flow into journal entries, SQLite-ingest rows, and a
  parent-facing read-model snapshot while preserving result, request, evidence,
  parent-rule, runtime, provider, model, prompt, and proof refs. The proof keeps
  ready, unavailable, and manual-required rows visible; rejects raw prompt/model
  output retention plus remote/API, policy-authority, and enforcement claims;
  and does not claim production storage runtime, model execution, model quality,
  portal UI, or enforcement.
- `LocalAiRecentMemoryWindowReadModelSchema` and
  `scripts/test/local-ai-recent-memory-window-proof.mjs` now prove the recent
  memory plus short-window activity read-model boundary over the existing local
  AI context builder. The proof selects only fresh `recent-activity` evidence
  inside the requested window, keeps returned memory refs grounded to selected
  source evidence, emits omitted stale/out-of-window and ungrounded-memory
  counts, and rejects raw retention, remote/API AI, policy authority, and
  enforcement overclaims. It does not create fresh capture, execute a model,
  prove production model quality, render portal UI, or dispatch enforcement.
- `LocalAiGraphReferenceContractProofSchema` and
  `scripts/test/local-ai-graph-reference-contract-proof.mjs` now prove the
  general local AI graph reference/minimal edge boundary. Graph refs must cite
  selected source evidence, optional policy version, and parent action refs
  before use; minimal activity-memory graph edges are returned only when
  endpoint, freshness, selected evidence, policy/action refs, and time range
  match. The proof keeps production graph storage/index runtime, model
  execution, model quality, portal UI, policy authority, enforcement,
  remote/API AI, and raw evidence retention unclaimed.
- `LocalAiContractCompletenessProofSchema` and
  `scripts/test/local-ai-contract-completeness-proof.mjs` now prove the
  baseline local AI input, safety-result, provider-capability, job-queue, and
  provider-route contracts line up over a single local-only runtime route. The
  proof preserves evidence refs, parent-rule refs, memory refs, graph refs,
  prompt/runtime refs, queue position, duplicate-runtime blocking, and provider
  capability metadata while rejecting model-execution, model-quality, portal
  UI, policy-authority, enforcement, remote/API AI, raw-prompt, and
  raw-evidence claims.
- `LocalAiRemoteAssistantBoundaryProofSchema` and
  `scripts/test/local-ai-remote-assistant-boundary-proof.mjs` now prove the
  remote assistant child-safety boundary. Parent-authorized remote report
  requests must cite approved stored evidence and parent-owned report bundles,
  remain outside the child-safety decision path, preserve local AI and
  deterministic policy authority, degrade to a local-only fallback, and reject
  raw retention, policy-authority, enforcement, and remote-override overclaims.
  This does not execute a remote provider, render portal UI, prove model
  quality, or change local safety decisions.
- The service WinRT OCR policy proof now reruns the real Windows service
  OCR path over live public Wikipedia pixels and consumes that exact
  `localOcr` Activity Screen row through `PolicyDecisionSchema`, producing an
  allow dry-run with evidence refs, parent rule refs, disabled enforcement
  handoff, and deleted-image/no-raw-retention custody. It does not claim final
  enforcement, production OCR quality, or authenticated-account coverage.
- Screen-derived time-limit and block decisions now have Windows
  owned-process adapter handoff proof. Model quality, video/social live
  coverage, and browser/category/network/mobile/broad enforcement handoff remain
  incomplete.
- `scripts/test/screen-ai-adapter-readiness-proof.mjs` now proves
  screen-derived adapter readiness stays contract-first: Windows owned-process
  actions retain real execution proof, while broad installed-app, host
  network/domain, exact active-tab, Android/iOS mobile, and Linux host adapter
  rows remain manual-required, not-claimed, or unavailable with deleted-image
  and no-raw-retention custody. It does not implement broad adapters or rerun
  live capture.
- The live operator harness now has a full nine-row public/live surface proof:
  ordinary YouTube validates `video`/`warn`, education YouTube validates
  `school`/`allow`, Vimeo validates `video`/`warn`, Facebook/social validates
  `chat`/`warn`, browser game validates `game`/`time-limit`, shopping validates
  `shopping`/`ask-parent`, school/productivity validates `school`/`allow`,
  native Notepad validates `productivity`/`allow`, and protected-surface
  degraded state makes no AI or policy claim. Authenticated-account social
  proof remains separate.
- The screen-AI browser trigger proof now composes managed-browser URL,
  browser-video, social-feed, and cloud-game trigger rows into the existing
  `LocalAiEvidenceContextBuildResult` path. The proof produces ready contexts
  for managed URL/video, a partial manual-required social context, and a partial
  unavailable cloud-game context, all without remote AI, direct policy
  authority, enforcement, authenticated-account, cloud-frame, or mobile parity
  claims.
- The screen summary context-builder replay proof now feeds the real WinRT OCR
  worker proof rows into `buildLocalAiEvidenceContext`. It proves deleted
  child-device query-store screen summaries become selected `screen-summary`
  evidence with local runtime refs, parent-rule refs, audit refs, and
  `screen-image-deleted` custody/deletion state. This is context-builder replay
  proof; it does not create new captures, claim production model quality,
  portal UI, or final enforcement.
- `ScreenSummaryParentExplanationSchema` and
  `scripts/test/screen-summary-parent-explanation-proof.mjs` now replay the
  same WinRT OCR screen-summary rows through a parent explanation/audit bundle.
  The proof cites screen-summary refs, audit evidence refs, parent rule refs,
  dry-run policy refs, local runtime refs, local-only custody, and
  `screen-image-deleted` deletion state while keeping raw image retention,
  remote/API AI, policy authority, portal runtime UI, and enforcement unclaimed.
- `ScreenSummaryParentExplanationReadModelSnapshotSchema` and
  `scripts/test/screen-summary-parent-explanation-read-model-proof.mjs` now
  convert those parent explanation rows into parent-visible read-model rows. The
  proof preserves screen-summary refs, audit evidence refs, parent rule refs,
  dry-run policy refs, local runtime refs, custody labels, and deleted-image
  reasons while still not claiming raw image display, remote/API AI, production
  portal runtime rendering, policy authority, or enforcement.
- `scripts/test/screen-summary-parent-explanation-service-read-model-proof.mjs`
  now starts the real Rust service against a seeded local ActivityStore and
  requests the Activity Screen read model over WebSocket. The service-backed row
  preserves the screen policy decision ref, policy action/reason refs, parent
  rule refs, local runtime refs, parent explanation refs, deletion reasons,
  deleted-image state, and child-device custody. This closes service/query
  read-model custody for screen-summary parent explanations, but still does not
  claim production portal rendering, new capture/model inference, remote/API AI,
  or enforcement.
- `scripts/test/screen-ai-final-product-path-proof.mjs` now verifies the
  retained final screen-AI path artifacts: real live/operator trigger rows,
  local VLM analysis rows, dry-run policy decisions, Windows action handoff
  proofs, portal/read-model proof, service-backed Activity Screen read-model
  proof, retention/deletion custody, and protected-surface non-claims. The
  verifier writes
  `output/screen-ai-pipeline-proof/final-product-path/proof-summary.json` and
  does not rerun live capture/model inference or claim remote/API AI.
- `ScreenAiStricterParentRuleProofSchema` and
  `scripts/test/screen-ai-stricter-parent-rule-proof.mjs` now consume the real
  service WinRT OCR policy decision and prove a stricter parent-authored screen
  category rule wins over local AI output. The proof preserves the local AI
  result ref, evidence refs, dry-run policy shape, and disabled enforcement
  handoff while keeping local AI authority, remote/API AI, raw image retention,
  and enforcement unclaimed.
- `ScreenFamilyAiHubRouteSchema` and
  `scripts/test/screen-family-ai-hub-routing-proof.mjs` now prove the
  screen-specific family AI hub route contract for hard visual analysis:
  child-device local analysis must be attempted before household hub selection,
  selected hub routing stays in local LAN custody with no retention,
  redacted/cropped input is required, and remote/API fallback is rejected for
  child-safety decisions. This is route/custody contract proof; it does not
  claim real hub runtime discovery, production model quality, policy authority,
  UI, or enforcement.
- `ScreenAiModelArtifactManifestSchema` and
  `scripts/test/screen-ai-model-artifact-manifest-proof.mjs` now prove the
  screen AI local model artifact manifest/config boundary. The proof reuses
  existing local model artifact/cache/runtime/provider contracts, requires
  opaque artifact and manifest refs, verified cache integrity, local-only runtime
  status, and the screen safety capability, while rejecting
  remote/API/Ocentra-hosted processing, model-quality, execution, and
  raw-evidence claims.
- `ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema` and
  `scripts/test/screen-family-ai-hub-runtime-discovery-proof.mjs` now prove a
  real loopback runtime/discovery exchange for that route. The script starts a
  local family-hub endpoint, discovers it, validates child-agent hello,
  heartbeat, and route evidence with existing LAN discovery schemas, submits a
  redacted-crop job payload, and rejects raw screenshot transfer, raw retention,
  remote/API provider use, and Ocentra-hosted processing. This is local
  runtime/discovery plumbing proof; physical household LAN, production model
  quality, policy authority, UI, and enforcement remain separate gates.
- `scripts/test/screen-ai-invalid-output-degrade-proof.mjs` now proves
  malformed, unparseable, or timed-out screen AI local model output degrades into
  typed non-enforcing local AI safety results. Invalid model output is rejected
  before it can become a result; unparseable output falls back to `unknown` with
  `invalid-output` degradation, and timeout falls back to `ask-parent` with
  overloaded runtime metadata. Both retain evidence and parent rule refs and do
  not claim model execution, model quality, new capture, portal UI, or
  enforcement dispatch.
- `scripts/test/screen-ai-model-output-parser-proof.mjs` now proves the screen
  AI model input/output parser boundary uses existing local AI evaluation and
  safety result contracts. The proof accepts schema-valid screen-derived video
  evidence with local-only runtime metadata and rejects malformed action,
  confidence, unknown/degraded state, evidence/rule list, remote runtime, and
  missing current-observation evidence shapes. This does not execute a model,
  prove model quality, rerun capture, render portal UI, or dispatch
  enforcement.
- `ScreenAiMemoryGraphSourceGuardProofSchema` and
  `scripts/test/screen-ai-memory-graph-source-guard-proof.mjs` now prove the
  screen AI memory/graph source-citation guard through the real local-AI
  context builder. Recent-memory and graph references must cite selected stored
  screen evidence before model input, ungrounded derived refs are rejected, and
  the proof keeps remote/API AI, policy authority, raw evidence embedding, and
  enforcement unclaimed.
- `scripts/test/screen-ai-model-runtime-backpressure-proof.mjs` now proves the
  screen model runtime flood-control contract: one heavy local screen AI job can
  run per physical child device, queued heavy jobs stay bounded, lower-priority
  cadence/background work degrades as overloaded when the queue is full, and no
  overload row can become policy eligible, retain raw images, or fall back to a
  remote/API provider. This is backpressure contract proof, not live model
  execution, production model quality, portal UI, or enforcement.
- `runLocalAiDeterministicClassifier` and
  `scripts/test/local-ai-deterministic-classifier-proof.mjs` now prove a
  deterministic local classifier lane over typed `LocalAiEvaluationInput`
  records. The proof emits schema-valid `LocalAiSafetyResult` rows for video,
  productivity/window, network-review, missing-evidence, and
  runtime-unavailable paths while preserving evidence refs, parent rule refs,
  runtime/model refs, prompt refs, and trace refs. It does not execute a model,
  prove production model quality, use remote/API AI, retain raw evidence, grant
  policy authority, render portal UI, or dispatch enforcement.
- `LocalAiClassifierReportSnapshotSchema` and
  `scripts/test/local-ai-classifier-read-model-manual-report-proof.mjs` now
  prove a parent-domain read-model/report bridge over those deterministic
  classifier rows. The proof projects ready, manual-required, and unavailable
  parent-facing report rows while preserving evidence refs, parent rule refs,
  runtime/provider/model refs, prompt refs, proof refs, and classifier trace
  refs. It does not execute a model, prove production model quality, use
  remote/API AI, retain raw evidence or raw model output, grant policy
  authority, render portal UI, or dispatch enforcement.

## Current Gap

Ocentra needs production-installed local model artifacts behind the typed
manifest boundary, production screen model/OCR quality beyond current local
proof, confidence handling, authenticated-account social proof beyond
public/live surface proof, production parent explanation portal rendering,
broader enforcement handoff, production browser-trigger producers, physical
household family AI hub runtime/discovery beyond the loopback proof,
production remote assistant provider execution and portal assistant UI,
cloud-streamed frame proof, mobile browser parity, and validation against
production external evidence variants.

## Checklist

- [x] Runtime/provider status.
- [x] Runtime status parent-facing read-model proof path.
- [x] One local AI runtime access lane per physical device, with child-safety
      priority and no duplicate same-device model load proof.
- [x] Evidence context builder proof path.
- [x] Parent-rule context proof path.
- [x] Parent-rule context builder proof path.
- [x] Local text inference dry-run proof path without model-execution,
      remote/API, policy-authority, enforcement, production-quality, or raw
      prompt-retention claims.
- [x] Local text LLM adapter boundary proof path without model-execution,
      remote/API, policy-authority, enforcement, production-quality, raw prompt,
      or raw model-output retention claims.
- [x] Local text output parser proof path without model-execution, remote/API,
      policy-authority, enforcement, production-quality, or raw model-output
      retention claims.
- [x] Local text parser read-model proof path for ready and manual-required
      parent-visible rows without model-execution, remote/API, raw output
      retention, policy-authority, enforcement, portal UI, or production-quality
      claims.
- [x] Local prompt/template version proof path without model-execution,
      model-quality, remote/API, policy-authority, enforcement, portal UI, raw
      prompt, or raw model-output retention claims.
- [x] Local AI result journal/SQLite ingest proof path without production
      storage runtime, raw prompt/model-output retention, policy authority,
      remote/API AI, or enforcement claims.
- [x] Local recent-memory and short-window activity read-model proof path
      without fresh-capture, model-execution, remote/API, policy-authority,
      enforcement, portal UI, or production-quality claims.
- [x] Local AI input/result/provider capability/job queue/provider route
      contract-completeness proof path without model-execution, model-quality,
      remote/API, policy-authority, enforcement, portal UI, raw-prompt, or
      raw-evidence claims.
- [x] Remote assistant contract boundary is separated from child safety for
      parent-authorized report/explanation requests, with local-policy
      authority preserved and remote-provider execution/UI still unclaimed.
- [x] Local deterministic classifier proof path for dry-run classify, allow,
      warn, ask-parent, time-limit, and block rows without model-execution,
      remote/API, raw-evidence retention, policy-authority, enforcement, portal
      UI, or production-quality claims.
- [x] Local classifier read-model/manual-required report bridge proof path for
      ready, manual-required, and unavailable parent-facing rows without
      model-execution, remote/API, raw retention, policy-authority,
      enforcement, portal UI, or production-quality claims.
- [x] Local result contract with confidence/degraded state.
- [x] Deterministic policy integration.
- [x] Service WinRT OCR row consumed by typed parent policy dry-run.
- [x] Social/video and screen summary handling proof path.
- [x] Parent explanation and audit proof path.
- [x] Parent explanation read-model proof path.
- [x] Screen parent explanation service read-model proof path.
- [x] Screen AI stricter parent-rule proof over the real service WinRT OCR
      policy decision.
- [x] Screen service WinRT OCR local adapter proof path.
- [x] Screen guided VLM worker contract proof path.
- [x] Screen VLM execution readiness/status handoff proof path.
- [x] Screen-derived adapter readiness keeps unsupported/broad adapters
      manual-required, not-claimed, or unavailable without claim upgrades.
- [x] Final screen-AI product path artifact gate.
- [x] Tests with real stored evidence.
- [x] Screen hard-visual routing prefers child-local then household family hub
      before remote/API fallback.
- [x] Screen AI model artifact manifest/config contract boundary.
- [x] Screen family AI hub runtime/discovery loopback proof validates real
      endpoint discovery, route evidence, redacted job exchange, and no
      raw/remote/Ocentra-hosted processing.
- [x] Screen AI invalid output and timeout degrade to typed non-enforcing
      local AI safety results.
- [x] Screen AI model output parser proof rejects malformed model output and
      non-local runtime shapes.
- [x] Screen AI recent-memory and graph refs require selected stored screen
      evidence citations before model input.
- [x] Local AI graph reference/minimal edge proof validates source-cited graph
      refs and activity-memory graph edge reads without UI, model-quality,
      policy-authority, enforcement, remote/API AI, or raw-retention claims.
- [x] Screen AI model runtime flood-control/backpressure proof rejects duplicate
      active heavy runtimes, queue overflow, policy-eligible overload rows,
      remote provider fallback, and raw image retention.

Production-installed model artifacts, production model/OCR quality,
authenticated-account social proof, physical household family AI hub
runtime/discovery, and broad enforcement handoff remain in the Current Gap
section above.

## Next AI Instructions

Keep AI as input to policy, not authority. If local model execution is not
available, expose unavailable/degraded state and deterministic fallback.
Browser URL/video AI work must consume typed browser evidence and return
schema-valid classification evidence; it must not read browser state directly or
enforce without parent policy. Social platform AI work must classify typed
signup/feed/account/messaging evidence only and must not accuse, approve, block,
or enforce without parent policy and audit refs.
Browser-game AI work must classify typed game URL, runtime-signal, metadata,
cloud, UGC, educational, purchase, and memory evidence only; it must not inspect
game chat, cloud-streamed frames, or native game state without separate proof.
