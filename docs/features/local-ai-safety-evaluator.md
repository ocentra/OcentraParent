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
  proofs, portal/read-model proof, retention/deletion custody, and
  protected-surface non-claims. The verifier writes
  `output/screen-ai-pipeline-proof/final-product-path/proof-summary.json` and
  does not rerun live capture/model inference or claim remote/API AI.
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

## Current Gap

Ocentra needs production-installed local model artifacts behind the typed
manifest boundary, production screen model/OCR quality beyond current local
proof, confidence handling, authenticated-account social proof beyond
public/live surface proof, production parent explanation portal rendering,
broader enforcement handoff, production browser-trigger producers, physical
household family AI hub runtime/discovery beyond the loopback proof,
cloud-streamed frame proof, mobile browser parity, and validation against
production external evidence variants.

## Checklist

- [x] Runtime/provider status.
- [x] One local AI runtime access lane per physical device, with child-safety
      priority and no duplicate same-device model load proof.
- [x] Evidence context builder proof path.
- [x] Parent-rule context proof path.
- [x] Local result contract with confidence/degraded state.
- [x] Deterministic policy integration.
- [x] Service WinRT OCR row consumed by typed parent policy dry-run.
- [x] Social/video and screen summary handling proof path.
- [x] Parent explanation and audit proof path.
- [x] Parent explanation read-model proof path.
- [x] Screen parent explanation service read-model proof path.
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
