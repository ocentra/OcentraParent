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
  command boundary, records `localVision` or explicit unavailable/invalid output
  Activity Screen rows, and drains processed queue records. The current proof
  adapter validates runtime plumbing and custody, not production model quality.
- `local-ai-parent-assistant-runtime-proof` now ties the provider scheduler
  proof to Parent Assistant answer/status/action contracts, including cited
  local answer, queued/degraded/unavailable lifecycle, child-safety priority,
  optional API boundary, and no direct enforcement.
- Parent Assistant provider routing proof now exposes local configured,
  degraded, unavailable, API authorized-unavailable, and API authorized-degraded
  states without making remote/API AI part of child safety decisions.
- Dry-run policy evaluator and evidence context builder exist in proof form.
- Screen-derived time-limit and block decisions now have Windows
  owned-process adapter handoff proof. Model quality, video/social live
  coverage, and browser/category/network/mobile/broad enforcement handoff remain
  incomplete.
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
- `ScreenFamilyAiHubRouteSchema` and
  `scripts/test/screen-family-ai-hub-routing-proof.mjs` now prove the
  screen-specific family AI hub route contract for hard visual analysis:
  child-device local analysis must be attempted before household hub selection,
  selected hub routing stays in local LAN custody with no retention,
  redacted/cropped input is required, and remote/API fallback is rejected for
  child-safety decisions. This is route/custody contract proof; it does not
  claim real hub runtime discovery, production model quality, policy authority,
  UI, or enforcement.

## Current Gap

Ocentra needs product-grade model configuration, local model artifacts,
production screen model quality, confidence handling,
authenticated-account social proof beyond public/live surface proof, production
parent explanation UI/runtime portal consumption, broader enforcement handoff,
production browser-trigger producers, real family AI hub runtime/discovery,
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
- [x] Social/video and screen summary handling proof path.
- [x] Parent explanation and audit proof path.
- [x] Parent explanation read-model proof path.
- [x] Tests with real stored evidence.
- [x] Screen hard-visual routing prefers child-local then household family hub
      before remote/API fallback.

Product-grade model configuration, model artifacts, production model quality,
authenticated-account social proof, and broad enforcement handoff remain in the
Current Gap section above.

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
