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

- Local AI runtime/provider status and provider scheduler proof exist.
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
- Browser-plan AI-24 now derives browser AI provider fallback decisions from
  validated local-provider, family-hub, and parent-approved remote route proofs.
  The proof covers local, family hub, remote, metadata-only, and no-AI fallback
  outcomes while rejecting hidden fallback, AI/policy authority, remote default
  blocking, remote outage disabling local safety, route conflicts, and
  request-id mismatches. It does not run a model, call a provider, evaluate
  policy, deliver UI, or enforce.
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

## Current Gap

Ocentra needs product-grade model configuration, local model artifacts,
production screen model quality, confidence handling, degraded states,
authenticated-account social proof beyond public/live surface proof, parent
explanations, broader enforcement handoff, production browser-trigger
producers, cloud-streamed frame proof, mobile browser parity, and validation
against production external evidence variants.

## Checklist

- [x] Runtime/provider status.
- [x] Evidence context builder proof path.
- [x] Parent-rule context proof path.
- [x] Local result contract with confidence/degraded state.
- [x] Deterministic policy integration.
- [x] Social/video and screen summary handling proof path.
- [x] Parent explanation and audit proof path.
- [x] Tests with real stored evidence.

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
