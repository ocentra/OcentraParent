# Screen Evidence Analysis

## Parent Outcome

Parents can optionally use local screen summaries to understand visible activity
when browser/app/network evidence is insufficient, while keeping raw images out
of Ocentra custody by default.

## Ocentra Requirement

Screen evidence is opt-in, local-first, temporary, encrypted, summarized, and
deleted according to visible retention rules. Policy consumes summaries and
evidence refs, not retained screenshots or raw model text.

## Roadmap And Expectations

- Roadmap: V0.5.3 screen evidence, V0.7 local AI policy, V5 policy product.
- Expectations: [screen evidence](../expectations/screen-evidence.md),
  [AI](../expectations/ai.md), [data custody](../expectations/data-custody.md).
- Supporting docs:
  [screen settings inventory](../screen-control-settings-inventory.md).
- Modules: `packages/activity-domain`, `packages/parent-domain`,
  `crates/agent-core`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
screenshots/live screen, video safety, and local-first privacy.

Some competitors use screenshots or screen visibility. Ocentra's default
position is more privacy-preserving: local summaries first, raw image retention
only with explicit parent settings.

## Current Ocentra State

- Local screen-analysis queue direction and contracts exist.
- `@ocentra-parent/activity-domain` now proves parent opt-in settings, cadence
  and trigger gates, temporary encrypted queue custody, deletion/expiry/delete
  failure states, raw-image non-retention, confidence/unknown handling, and
  policy eligibility with focused contract tests plus
  `scripts/test/screen-evidence-settings-retention-proof.mjs`.
- Local AI and policy can consume evidence summaries in dry-run paths.
- The desktop local adapter path now uses the shared `xcap` capture API on
  Windows/macOS and an X11 command backend on Linux. Windows proof captures
  active-window, selected-window, and parent-opt-in primary-display scopes with
  encrypted temporary custody and raw delete-after-success proof. Linux WSLg
  proof captures a real X11 selected window with encrypted custody and raw
  deletion. macOS live capture proof still requires platform execution evidence
  before parity is claimed.
- The screen-capture adapter crate now has a Rust trigger scheduler proof for
  parent-enabled managed-browser trigger inputs, native app foreground trigger
  inputs, cadence-due decisions, debounce, and disabled-parent suppression. The
  Windows trigger matrix proof captures real browser and Notepad windows after
  scheduler enqueue decisions and deletes raw temp images.
- `scripts/test/screen-ai-local-vlm-proof.mjs` now runs a local Qwen2-VL proof
  matrix over 16 real window captures plus a disabled no-capture/no-AI row. The
  window capture, local analysis, policy, explanation, and deletion plumbing is
  real, but most visible content is controlled fixture content for deterministic
  harness proof. The matrix covers controlled browser video, education video,
  Vimeo-style video, social feed/chat, browser game, bypass tool, shopping,
  school/productivity, native app, controlled native game window, native
  owned-process time-limit, unknown low-confidence fallback, controlled violence
  text, and three timed cadence frames. Every captured row validates screen
  analysis, local AI safety result, deterministic policy dry-run, parent
  explanation artifact, and raw image deletion. Live external site/account proof
  remains required before product-complete claims for YouTube, Vimeo, Facebook,
  shopping, or similar real services.
- `scripts/test/screen-ai-prerequisite-merge-proof.mjs` records the merged
  PR258 checkpoint commit and verifies the current branch contains the screen
  capture and AI analysis proof artifacts before this continuation claims the
  prerequisite stack.
- Local model output now has a strict `ScreenLocalModelOutputSchema` guard, and
  `scripts/test/screen-ai-invalid-output-proof.mjs` proves invalid category,
  confidence, risk-signal, or missing-text output cannot become a screen analysis
  result or policy candidate.
- `selectStricterPolicyAction` in `@ocentra-parent/parent-domain/policy` and
  `scripts/test/screen-ai-stricter-rule-proof.mjs` prove local AI cannot weaken
  a stricter parent rule before policy handoff. This is a policy candidate gate;
  it does not claim final enforcement execution.
- `scripts/test/screen-ai-local-vlm-proof.mjs` includes an
  `unknown-native-process` scenario that opens a controlled native window,
  triggers `unknownProcessForegroundStart`, captures the selected window,
  analyzes it through the local VLM path, records low-confidence unknown state,
  and produces an ask-parent dry-run without retaining the raw image.
- `scripts/test/screen-ai-local-vlm-proof.mjs` also proves a controlled
  `native-game` path: it triggers `nativeGameForegroundStart`, captures a real
  selected native window, classifies it as `game` through the local VLM path,
  and produces an ask-parent dry-run. This remains a local-machine controlled
  window proof; service-owned foreground watcher wiring is still separate.
- `scripts/test/screen-ai-ocr-route-proof.mjs` proves the local OCR route can
  turn typed OCR text evidence into a schema-valid `ScreenAnalysisResult` and
  parent policy dry-run without a vision model or retained raw image. This is
  a route/contract proof; production OCR adapter execution remains separate.
- `scripts/test/screen-ai-local-text-route-proof.mjs` proves screen-derived
  typed activity evidence can enter `LocalAiEvaluationInput`, produce a
  schema-valid local-only `LocalAiSafetyResult`, and hand off to a dry-run
  policy decision with evidence, memory, graph, and rule references intact.
  This is a local text route contract proof, not live model inference or model
  quality proof.
- `scripts/test/screen-ai-deterministic-route-proof.mjs` proves
  structured screen-adjacent evidence can create a `deterministicRules`
  analysis route and dry-run policy decision without claiming captured pixels,
  OCR, VLM, or raw-image custody. The route requires local evidence, known
  category, sufficient confidence, and `unavailableNoImage` deletion state.
- `scripts/test/screen-ai-observe-policy-proof.mjs` proves observe-only parent
  settings can still permit local screen analysis while rejecting policy
  handoff. The proof validates `policyUseEnabled:false`,
  `policyEligible:false`, no policy decision creation, and rejection of
  `policyUseEnabled:true` while `analysisMode` is `observeOnly`.
- `scripts/test/screen-ai-protected-surface-proof.mjs` proves protected
  surfaces degrade honestly: capability status is `protectedSurface`, custody is
  unavailable, no raw image or model provider is claimed, no policy decision is
  created, and a policy-eligible protected result is rejected. Live OS
  permission prompt proof remains separate.
- `scripts/test/screen-ai-action-dispatch-proof.mjs` now links the
  screen-derived native owned-process time-limit policy decision into the real
  Windows Rust service time-limit adapter path. The proof preserves the screen
  policy decision ID and evidence refs through dispatch, restart recovery,
  parent cancel, expiry, and a real Windows process-termination result.
- `scripts/test/screen-ai-block-action-dispatch-proof.mjs` now links a
  screen-derived block decision into the real Windows Rust service
  owned-process block adapter path. The proof preserves the screen policy
  decision ID, local AI result ID, and evidence refs through dispatch and
  terminates a controlled owned process. It intentionally does not claim
  category, browser, network/domain, mobile, or broad block adapters.
- `scripts/test/screen-ai-portal-chain-proof.mjs` now proves the real Rust
  service read model reaches the parent portal Activity Screen tab and renders
  trigger, capture scope, AI provider/category/confidence, policy eligibility,
  raw image deletion, custody, queue, digest, and evidence refs. This is
  service-to-portal chain proof from a seeded local activity event, not live
  external-account trigger proof.
- `screenControlSettingsPortalProof` and
  `scripts/test/screen-settings-portal-proof.mjs` now prove the real parent
  portal Settings route renders the Screen control catalog as read-only
  settings/capability proof. The proof starts the real Rust agent plus Vite
  portal, opens `#/settings-rules`, verifies the 474-setting catalog, 11 tabs,
  68 proof-required controls, 9 unavailable sensitive modes, and fail-closed
  screen gates, then captures
  `output/screen-plan-proof/settings-ui/parent-settings-screen-catalog.png`.
  This is not writable opt-in or retention-control UX.
- `scripts/test/screen-ai-service-cadence-proof.mjs` now proves an explicit
  opt-in Rust service cadence loop on Windows: it opens a real foreground
  browser fixture, records three timed active-window captures through the
  service, writes encrypted queue metadata, holds the pending queue at three
  records after the cap, ingests three `ScreenAnalysisSummarized` events into
  the local ActivityStore, and reads three Activity Screen rows back through the
  real WebSocket command path. The provider is `serviceCaptureMetadata`, so this
  is capture/read-model cadence proof, not a VLM quality or policy-action claim.
- `scripts/test/screen-ai-service-foreground-proof.mjs` now proves an explicit
  opt-in Rust service foreground watcher on Windows: it starts from a real
  Chromium active window, launches/focuses a native Notepad window, requires the
  encrypted screen queue to grow after that foreground action, deletes the raw
  temporary image material after queue handoff, and reads the latest foreground
  Activity Screen row through the real WebSocket command path. The provider is
  `serviceCaptureMetadata`, so this is native foreground capture/read-model
  proof, not browser URL trigger ownership, game classification, VLM quality, or
  policy-action proof.
- `scripts/test/screen-ai-service-analysis-proof.mjs` now proves an explicit
  opt-in Rust service analysis loop on Windows: it opens a real foreground
  browser fixture, lets service cadence enqueue one encrypted active-window
  capture, invokes a local adapter command through the service analysis runtime,
  records a `localVision` Activity Screen read-model row with capture reason,
  scope, queue job, model runtime ref, category, confidence, evidence digest,
  policy eligibility, and deleted-image state, reads it back over the real
  WebSocket command path, and drains the processed encrypted queue record. The
  adapter command is a local proof adapter for runtime plumbing, not a production
  VLM quality claim.
- `scripts/test/screen-ai-service-native-game-analysis-proof.mjs` now proves
  the service-owned native game classification path on Windows: it focuses a
  controlled native game-like window, lets the opt-in foreground runtime enqueue
  the encrypted active-window capture, then runs the opt-in analysis runtime
  with a local adapter command that records a `localVision` Activity Screen row
  with `primaryCategory: game`, preserved queue job, capture reason, scope,
  digest, policy eligibility, and deleted-image state. This proves service
  capture-to-analysis plumbing for a native game-like surface; installed
  commercial-game identity detection remains app/game evidence scope.
- `scripts/test/screen-ai-deletion-retention-custody-proof.mjs` now writes a
  combined pipeline artifact proving screen queue TTL/deletion custody contracts:
  successful deletion and expired deletion require proof, delete-failed state
  remains visible in queue health, retry bounds are enforced, and unsupported raw
  screenshot retention is rejected. This is contract proof; production TTL
  sweeper execution and parent retention UI remain separate.
- `scripts/test/screen-ai-service-disabled-suppression-proof.mjs` now proves the
  service-owned disabled setting against the real Rust service on Windows: an
  enabled phase creates one encrypted cadence queue record, then a disabled
  phase runs cadence, foreground, and analysis runtimes with the parent setting
  off and proves no new screen rows, no new queue jobs, no local vision row, and
  no pending queue drain. Product UI controls for the setting remain separate.
- `scripts/test/screen-ai-service-retention-sweeper-proof.mjs` now proves an
  explicit opt-in Rust service retention sweeper on Windows: an enabled cadence
  phase creates one encrypted active-window queue record with temporary-image
  TTL metadata, then a sweeper-only phase runs with capture and analysis
  disabled, removes the expired queue record, and reads an `expiredDeleted`
  Activity Screen row for the original queue job over the real WebSocket command
  path. Product UI controls for retention duration and cloud retention policy
  remain separate.
- `scripts/test/screen-ai-live-operator-proof.mjs` now provides the executable
  live operator gate for this feature. It prints a required nine-scenario
  manifest template, refuses to claim live proof without operator-supplied real
  URLs/apps, opens/focuses those surfaces when a manifest is supplied, captures
  the selected browser window or active native window, runs the local VLM path, validates screen AI and
  policy dry-run contracts, deletes raw image material, and writes redacted
  proof artifacts. Browser live rows now also record page readiness evidence
  before capture: final hostname, redacted final URL, title hash/length, visible
  text hash/length, and blank-page rejection. The current harness readiness
  artifact is
  `output/screen-ai-pipeline-proof/live-operator/harness-readiness/proof-summary.json`;
  a full nine-row live operator run now proves ordinary YouTube `video`/`warn`,
  education YouTube `school`/`allow`, Vimeo `video`/`warn`, Facebook/social
  `chat`/`warn`, browser game `game`/`time-limit`, shopping
  `shopping`/`ask-parent`, school/productivity `school`/`allow`, native Notepad
  `productivity`/`allow`, and protected-surface degraded state with no raw
  image, AI, or policy claim at
  `output/screen-ai-pipeline-proof/live-operator/proof-summary.json`.
  Authenticated-account social proof remains separate from this public/live
  surface proof.
- Android child-agent scaffold now has emulator MediaProjection proof with
  explicit OS consent, foreground service, captured frame digest, and raw temp
  deletion. Physical Android parity and silent background capture are not
  claimed.
- The screen-AI browser trigger proof validates managed-browser URL,
  browser-video, social-feed, and cloud-game trigger rows through
  activity-domain browser AI and screen-analysis schemas, then proves local-AI
  context builder consumption through the executable proof path. The previously
  referenced `test-results/screen-ai-browser-trigger-proof/proof.json` artifact
  is not present in the current checkout, so browser-trigger artifact closure
  remains a D/browser-lane handoff item. The social row is manual-required and
  the cloud-game row is unavailable/scaffold-only, so this does not claim
  authenticated social, cloud-frame, mobile parity, UI, or enforcement support.
- Product settings and quality proof are incomplete.
- Raw screen control settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Writable opt-in UI, parent retention controls, authenticated-account social
proof beyond public/live surface proof, OCR/vision quality beyond controlled
fixtures and the full live operator matrix, production local vision adapter
quality beyond the service proof adapter, service-owned live trigger event
producers beyond the timed cadence loop and native active-window foreground
watcher, writable parent-facing settings UX for disabled capture/analysis,
physical Android/iOS proof, live macOS capture proof, Linux root/Wayland portal
proof, browser-trigger artifact closure, browser/network/mobile/broad block
action adapters from screen-derived decisions, and production parent explanation
UX remain. Browser-trigger proof now covers contract flow into screen evidence and
local-AI context only; live trigger producers, authenticated surfaces,
cloud-streamed frames, mobile parity, and UI remain separate proof gates.

## Checklist

- [x] Parent opt-in setting contract and service runtime gates.
- [x] Capability/status contract.
- [x] Encrypted temporary image queue.
- [x] Local OCR/vision summary proof route.
- [x] Image deletion and retention state proof.
- [x] Confidence and unknown handling.
- [x] Policy decision references summary evidence.
- [x] Portal read-model explanation and audit proof.
- [x] Parent Settings route renders read-only Screen settings/capability proof.

Product-facing writable opt-in UI, retention controls, production OCR/VLM
quality, authenticated-account social proof, broad adapters, and production
explanation UX remain in the Current Gap section above.

## Next AI Instructions

Never route raw screen images to Ocentra cloud by default. Treat summaries,
confidence, deletion state, and custody labels as required product fields.
