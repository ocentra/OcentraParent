<!-- agent-capsule -->

> Agent Capsule
> Doc: Screen Evidence Analysis
> Kind: feature documentation; read only when selected by FEATURE_ROUTE_INDEX, PLAN_INDEX, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Screen Evidence Analysis

## Parent Outcome

Parents can optionally use local screen summaries to understand visible activity
when browser/app/network evidence is insufficient, while keeping raw images out
of Ocentra custody by default.

## Ocentra Requirement

Screen evidence is opt-in, local-first, temporary, encrypted, summarized, and
deleted according to visible retention rules. Policy consumes summaries and
evidence refs, not retained screenshots or raw model text.

When screen-derived AI work routes to a trusted household provider, the child
agent still owns the AI work ledger, result validation, policy authority,
audit, and deletion/custody proof. Raw screenshot transfer to LAN providers is
forbidden by default; allowed mesh payloads must be metadata, screen summaries,
OCR text, redacted crops, or encrypted local artifact refs with explicit custody
rules.

## Roadmap And Expectations

- Roadmap: V0.5.3 screen evidence, V0.7 local AI policy, V5 policy product.
- Expectations: [screen evidence](../expectations/screen-evidence.md),
  [AI](../expectations/ai.md), [data custody](../expectations/data-custody.md).
- Supporting docs:
  [screen settings inventory](../plans/screen-plan/workpacks/screen-control-settings-inventory.md).
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
- Reusable Rust eventing now exists in `crates/ocentra-eventing`; screen
  capture/queue/deletion/summary work must publish typed events for AI, policy,
  action, audit, read-model, and deletion consumers instead of wiring direct
  screen-to-AI calls.
- `scripts/test/screen-ai-event-driven-runtime-proof.mjs` now proves the first
  screen eventing runtime chain over typed capture, encrypted queue, AI result,
  summary, policy, action dry-run, deletion, and portal-read-model events
  without raw-image escape. This is an in-process runtime proof; live service
  producers/subscribers and household mesh execution remain separate gates.
- `scripts/test/screen-service-event-bridge-proof.mjs` now proves the service
  bridge from an Activity Screen read-model row into the existing typed screen
  event runtime chain. The bridge rejects raw-image-retained rows and missing
  policy refs before publication, then reuses the core screen event path for
  capture, encrypted queue, AI request/result, summary, policy, dry-run action,
  deletion, and portal-read-model events. This is service bridge proof, not a
  new live capture run or always-on production subscription claim.
- `scripts/test/screen-service-event-subscription-proof.mjs` now proves a
  service-owned `screen.service.row.ready` subscriber using `ocentra-eventing`.
  The service startup now retains `ScreenAiServiceEventRuntime::start()` before
  serving requests, and the runtime-start test proves that helper registers the
  real subscriber and dispatches through the event bus. The subscriber consumes
  typed Activity Screen rows, invokes the existing service bridge, records
  accepted/rejected row dispatch state, publishes the ordered downstream screen
  runtime chain for safe rows, and rejects raw-image retained rows before
  downstream screen events are recorded. This is subscriber startup/runtime
  proof; externally proving every live trigger producer against the subscriber
  remains a separate production gate.
- `scripts/test/screen-service-analysis-row-ready-proof.mjs` now proves the
  service analysis runtime starts the service row-ready event runtime, converts
  a recorded Activity Screen analysis result through the shared row mapper, and
  publishes `screen.service.row.ready`. Current service analysis rows still lack
  policy refs, so the existing bridge records `MissingPolicyDecision` and blocks
  downstream policy/action/deletion/portal publication until the policy producer
  supplies those refs. This closes the analysis-producer-to-row-ready handoff
  without claiming final policy/action publication.
- `scripts/test/screen-service-policy-ref-producer-proof.mjs` now proves the
  Rust service event-record producer writes bridge-required dry-run policy refs
  for policy-eligible service analysis rows before `screen.service.row.ready`
  publication: policy decision, action, reason, parent rule, explanation, and
  deletion proof refs. Non-policy-eligible rows still do not fabricate policy
  refs, and the existing subscriber proof remains the downstream runtime-chain
  proof for safe rows. This is not broad parent-rule compiler coverage, final
  enforcement execution, or a new live external capture run.
- `scripts/test/screen-service-capture-event-producer-proof.mjs` now proves the
  service capture producers publish typed `screen.capture.observed` and
  `screen.queue.encrypted` events after the encrypted queue write. The core
  screen runtime exposes a capture/queue-only publisher that carries no AI,
  policy, action, deletion, or portal refs, the service bridge maps Activity
  Screen metadata rows into that payload while rejecting raw-retained rows, and
  the timed cadence plus native foreground runtime loops call the shared
  producer after real queue handoff. This closes the live cadence/foreground
  capture event producer hop without claiming a new external live capture run,
  retention sweeper deletion event publication, final enforcement, or model
  quality.
- `scripts/test/screen-service-deletion-event-producer-proof.mjs` now proves the
  service retention sweeper publishes typed `screen.deletion.committed` events
  after expired encrypted queue records are removed. The core screen runtime
  exposes a deletion-only publisher that requires a deletion proof ref while
  keeping AI, policy, and action refs empty for TTL cleanup; the service bridge
  maps Activity Screen retention rows into that payload while rejecting raw
  retention and missing deletion proof rows; and the retention sweeper runtime
  calls the shared producer after queue removal. This closes the TTL deletion
  event producer hop without claiming parent retention UI persistence, final
  enforcement, a new live capture run, or model quality.
- `scripts/test/screen-delete-failed-read-model-proof.mjs` now proves Rust
  protocol/read-model parity for `deleteFailed` screen custody rows. The
  ActivityStore screen summary reports failed queue-health status and
  `deleteFailedCount` for delete-failed rows instead of collapsing every screen
  row to deleted status. This is contract/read-model surfacing, not an OS
  filesystem deletion-failure simulation, and it does not weaken
  delete-after-success/delete-after-expiry defaults.
- `scripts/test/screen-ai-degraded-portal-proof.mjs` now proves the real Rust
  service plus parent portal Screen Analysis route can render degraded Activity
  Screen OCR/VLM read-model rows from the local activity store. The proof fixes
  the portal's default Activity Screen command payload, preserves
  model/runtime/template refs, renders `localOcr` `modelUnavailable` and
  `localVision` `degraded` rows, and screenshots the portal state under
  `output/ai-plan-proof/activity-screen-ai-degraded-portal-proof`. This is
  degraded read-model visibility proof; it does not execute OCR/VLM inference,
  capture fresh pixels, grant policy authority, or dispatch enforcement.
- `scripts/test/screen-child-disclosure-proof.mjs` now proves the screen child
  disclosure contract. It defines child-visible disabled, paused, active
  capture, protected-surface, and deleted-summary states with calm tokenized
  copy; requires active capture to be ready, scoped, and represented as a child
  capture banner; requires deleted-summary state to cite deleted local custody;
  and rejects hidden capture, raw screenshot display, remote viewer, and
  policy-authority claims. It also renders desktop/mobile child-visible
  disclosure screenshots from the same contract while keeping child-agent
  deployment/delivery unclaimed.
- `scripts/test/screen-ai-enforcement-handoff-guard-proof.mjs` now proves the
  screen enforcement handoff guard contract. It builds a schema-valid handoff
  payload only from a dry-run policy decision that has not already been handed
  off, an enabled parent policy rule, summary/local-AI/audit refs already on the
  policy decision, a confidence state, and a guard audit event. Raw pixels, raw
  model text, retained screenshots, and local-AI authority claims are rejected.
  This is contract proof only; downstream adapter execution and
  broad/browser/network/mobile enforcement remain separate gates.
- `scripts/test/screen-ai-household-mesh-proof.mjs` now proves the
  screen-derived household mesh contract/runtime boundary: redacted
  summary/custody payloads, no raw screenshot transfer, provider claim/lease,
  worker-only provider result, child-agent result validation before policy, and
  duplicate/expired/wrong-provider/wrong-claim/evidence/custody/raw-transfer/
  authority rejection. Physical LAN execution remains separate.
- Household mesh screen AI execution is planned through the
  [Household AI Provider Mesh Plan](../plans/ai-plan/household-ai-provider-mesh-plan.md)
  and [Screen AI Pipeline Plan](../plans/screen-ai-pipeline-plan/README.md).
  Existing legacy family-hub route/runtime-discovery proofs are precursors
  only; they do not upgrade into household mesh completion unless provider
  claim/lease, result validation, no-raw-screen-transfer, and
  child-agent-only policy authority all pass.
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
  proof captures a real X11 selected `xmessage` window with encrypted custody,
  raw deletion, a retained operator-safe visual artifact, and local Qwen2-VL
  analysis for the Linux external-gate manifest. macOS live capture proof still
  requires platform execution evidence before parity is claimed.
- `scripts/test/screen-local-platform-proof-batch.mjs` now accounts for the
  locally provable platform surface as one batch: Windows active/scope capture,
  Android emulator MediaProjection consent/capture/deletion, Linux WSLg/X11
  selected-window capture/deletion, and Linux WSLg external-gate local VLM
  analysis. The batch records the current host inventory for Android
  SDK/ADB/AVDs/devices and WSL display state, so a fresh emulator run is
  distinguishable from a retained artifact. It explicitly keeps Android
  physical-device parity, native Linux Wayland/PipeWire parity, macOS
  ScreenCaptureKit, and iOS ReplayKit as external-required gates before
  product-complete platform capture readiness. It now also wires
  `--run-android-physical` to the physical Android external-gate runner so a
  real unlocked phone can upgrade the gate without changing the local batch
  contract.
- `ScreenMacosCaptureCapabilityProofSchema` and
  `scripts/test/screen-macos-capture-capability-proof.mjs` add the macOS
  ScreenCaptureKit readiness gate: the proof records current Apple
  ScreenCaptureKit, scoped content filter, Screen Recording privacy, and
  PPPC/MDM manual-required boundaries while keeping display/window capture,
  permission, deletion, silent background capture, raw remote upload, and
  raw-retention-by-default product claims blocked until live macOS proof exists.
- `ScreenLinuxCaptureCapabilityProofSchema` and
  `scripts/test/screen-linux-capture-capability-proof.mjs` add the Linux
  readiness gate: the proof consumes the retained WSLg/X11 selected-window
  artifact, records current XDG Desktop Portal ScreenCast/PipeWire and
  ImageMagick X server capture source boundaries, and keeps native X11
  root-display, native Wayland GNOME/KDE/wlroots, unknown Wayland, unsupported
  compositor, raw remote upload, and raw-retention-by-default states blocked
  until native session proof plus deletion proof exists.
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
- `selectStricterPolicyAction` in `@ocentra-parent/schema-domain/policy` and
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
- E-D added a network row30 trigger planner in `ocentra-network-evidence` that
  can request screen-summary confirmation only when the network cascade
  recommends it and parent settings, encrypted local queue, deletion, local
  runtime, debounce, and protected-surface guards allow it. This is trigger-plan
  proof only; it does not execute capture, OCR/VLM, policy, portal, adapter, or
  enforcement behavior.
- `ScreenDetectorPromptPackSchema`, `ScreenDetectorPromptOutputSchema`, and
  `scripts/test/screen-detector-prompt-pack-proof.mjs` now prove
  detector-specific prompt pack contracts for social/video, chat, game, school,
  bypass, adult, violence, shopping/payment, and signup/identity screen cases.
  The contracts reject open-ended screen descriptions, raw prompt text, private
  messages, names, credentials, full OCR text, raw screenshot refs, policy
  authority, and enforcement claims. This is prompt/schema proof, not production
  model quality or live inference.
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
- `ScreenLocalAiResourceProofSchema` and
  `scripts/test/screen-local-ai-resource-scheduler-proof.mjs` prove the
  screen-specific local AI resource guard for OCR/VLM/deterministic analysis:
  screen jobs carry type, priority, heavy/light/no-model resource weight,
  timeout/skipped/degraded states, max image pixels, OCR snippet caps,
  local-only custody, no remote AI, and no raw screenshot retention. The proof
  also invokes the existing local provider singleton scheduler proof, so this
  closes the screen scheduler/resource gate without claiming final live
  capture-to-policy pipeline completion.
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
- `scripts/test/screen-parent-portal-summary-ui-proof.mjs` now proves the
  dedicated parent portal Screen Analysis route. The proof launches the real
  portal/agent path, clicks the Activity Screen read-model command, and captures
  desktop/mobile route screenshots. The route uses a
  `@ocentra-parent/portal-domain/screen-summary-panel` intent to render
  service-backed Activity Screen read-model rows with capability, queue job,
  summary/category, confidence, model/runtime, deletion/custody, policy,
  audit/evidence refs, and not-claimed enforcement details.
- `screenControlSettingsPortalProof` and
  `scripts/test/screen-settings-portal-proof.mjs` now prove the real parent
  portal Settings route renders the Screen control catalog as read-only
  settings/capability proof. The proof starts the real Rust agent plus Vite
  portal, opens `#/settings-rules`, verifies the 474-setting catalog, 11 tabs,
  68 proof-required controls, 9 unavailable sensitive modes, and fail-closed
  screen gates, then captures
  `output/screen-plan-proof/settings-ui/parent-settings-screen-catalog.png`.
  This is not writable opt-in or retention-control UX.
- `screenEvidenceSettingsWritableUiProof` and
  `scripts/test/screen-settings-writable-controls-proof.mjs` now prove the real
  parent portal Settings route can build typed local screen-summary setting
  drafts from the real `ScreenAnalysisParentSettingSchema`: disabled,
  observe-only five-minute summaries, and strict one-minute policy dry-run with
  explicit triggers. The proof runs the real Rust agent plus Vite portal on the
  B-lane ports, clicks the Settings controls in Playwright, captures
  `output/screen-plan-proof/settings-writable-controls/parent-settings-writable-controls.png`,
  and writes
  `output/screen-plan-proof/settings-writable-controls/proof-summary.json`.
  This is not service persistence, child-agent runtime application, raw
  retention enablement, live view, or remote screenshot upload.
- `ScreenAnalysisParentSetting` Rust protocol structs and
  `scripts/test/screen-settings-service-persistence-proof.mjs` now prove the
  service-side parent screen setting persistence boundary. The service runtime
  returns a disabled default without silently enabling capture, trigger capture,
  policy use, or raw image retention; persists a parent strict dry-run setting
  to a local child-device JSON store across reload; and rejects raw image
  retention, observe-only policy use, stale base versions, and unsafe
  inconsistent settings before persistence. This is backend persistence proof,
  not parent portal command wiring, product-complete retention-control UI, raw
  retention enablement, live view, or raw remote upload.
- `scripts/test/screen-settings-service-command-proof.mjs` now proves typed
  service command wiring for persisted parent screen settings. The TypeScript
  protocol adapter reuses the owning
  `@ocentra-parent/activity-domain/screen-evidence-settings`
  `ScreenAnalysisParentSettingSchema`, Rust protocol mirrors
  `agent.screen-settings.get` and `agent.screen-settings.replace`, and the
  Rust service WebSocket handler routes get/replace commands into the local
  JSON-backed `ScreenSettingsRuntime`. The proof shows replace persists a
  strict dry-run setting, get reports it after runtime restart, and raw-image
  retention is rejected before persistence. This is backend command-path proof,
  not parent portal form submission, product-complete retention-control UI,
  raw retention enablement, live view, or raw remote upload.
- `scripts/test/screen-settings-portal-service-command-proof.mjs` now proves
  the real parent Settings route submits the parent-approved local short-TTL raw
  retention intent through the Rust service WebSocket command path. The proof
  starts the actual agent and Vite portal, clicks the approved retention
  setting, sends `agent.screen-settings.replace`, renders the accepted service
  response and audit ref, sends `agent.screen-settings.get`, verifies the
  persisted JSON store has `retainRawImage:true` with
  `temporaryImageTtlSeconds:120`, and captures
  `output/screen-plan-proof/settings-service-command/parent-settings-service-command.png`.
  This is portal-to-service settings persistence proof for the local approved
  short-TTL mode; raw retention still remains disabled by default, raw remote
  upload remains forbidden, and live view, relay/cache, privacy/legal approval,
  broad platform parity, and production OCR/VLM quality remain separate gates.
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
- `ScreenEvidenceRemoteBoundarySettingSchema` and
  `scripts/test/screen-evidence-settings-retention-proof.mjs` now prove the
  explicit raw-retention/live-view/remote boundary for local screen summaries:
  raw screenshot retention and live view are disabled in this mode, raw
  screenshot remote upload is schema-forced false, and the only accepted remote
  path is a parent-approved redacted summary with an audit ref and
  parent-owned-export custody. The proof writes
  `output/screen-plan-proof/remote-retention-boundary/proof-summary.json`.
  This is not live-view transport, retention UI, or privacy/legal approval.
- `ScreenFamilyAiHubRouteSchema` and
  `scripts/test/screen-family-ai-hub-routing-proof.mjs` now prove the legacy
  screen-specific household-provider route contract for hard visual cases: the
  child device local analysis attempt must be visible first, a selected
  household route stays inside `live-lan-child-agent` custody, redacted/cropped
  input is required, no retention is allowed, and raw screenshot transfer,
  remote/API fallback, and Ocentra-hosted processing are rejected. This is
  precursor contract proof for the household mesh; provider claim/lease,
  child-agent result validation, production model quality, UI, and enforcement
  remain separate gates.
- `ScreenFamilyAiHubRuntimeDiscoveryReadModelSchema` and
  `scripts/test/screen-family-ai-hub-runtime-discovery-proof.mjs` now prove the
  first loopback runtime/discovery layer for the legacy household-provider
  route. The proof starts a real loopback provider HTTP endpoint, discovers it,
  records child-agent hello, heartbeat, and route evidence through existing LAN
  discovery schemas, selects the existing screen household-provider route after
  a child-local degraded attempt, submits a redacted-crop job payload, and
  writes
  `output/screen-ai-pipeline-proof/family-ai-hub-runtime-discovery/proof-summary.json`.
  The exchange log proves no raw full screenshot bytes are sent, no raw image is
  retained, and remote/API or Ocentra-hosted processing is not used. Physical
  household mesh discovery, provider claim/lease/result validation, production
  VLM quality, portal UI, policy authority,
  cloud relay, and enforcement remain separate gates.
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
  surface proof. `scripts/test/screen-plan-external-gates-proof.mjs` now
  enumerates `authenticated-account-social-capture` as a digest-backed external
  evidence gate that requires operator consent, redacted account identifiers,
  local capture, local analysis, policy dry-run, raw image deletion proof, and
  no raw private content before any authenticated-account claim can close. The
  same external gate intake now rejects pixel evidence unless it carries
  structured local capture, local analysis, and deletion refs; live-view relay
  and privacy/legal gates require their own no-retention/encryption/viewer-audit
  or approval-scope refs instead of generic live-surface booleans. The generated
  `output/screen-plan-proof/external-gates/manual-evidence-status.md` file lists
  each missing gate and its required proof-ref fields for operator collection.
  The Linux WSLg external-gate entry is now satisfied by
  `scripts/test/screen-linux-wslg-external-gate-proof.mjs`, which captures a real
  native Linux `xmessage` surface, analyzes the retained operator-safe artifact
  with local Qwen2-VL, and cites the screen-capture adapter deletion proof; the
  remaining macOS, physical Android, iOS, live-view, hosted relay,
  privacy/legal, and authenticated-account gates remain missing.
- Android child-agent scaffold now has emulator MediaProjection proof with
  explicit OS consent, foreground service, captured frame digest, and raw temp
  deletion. Physical Android parity and silent background capture are not
  claimed.
- `scripts/test/screen-android-mediaprojection-capability-proof.mjs` now
  records the Android MediaProjection source-doc/readiness gate at
  `output/screen-plan-proof/android/proof-summary.json`. It consumes the
  existing emulator proof, records per-session consent and Android 14
  foreground-service/app-window-sharing requirements, requires
  stop-callback-on-user-stop behavior for MediaProjection modes, rejects silent
  background capture, and blocks physical-device product readiness until
  physical-device capture and deletion proof exist. The capability proof now
  accepts an optional physical-device proof ref only after
  `scripts/test/screen-android-physical-external-gate-proof.mjs` captures a
  real non-emulator Android target, verifies raw temp deletion, analyzes an
  operator-safe retained live app surface with local Qwen2-VL, and satisfies the
  external-gate manifest. `scripts/test/screen-android-physical-target-readiness-proof.mjs`
  records the current Samsung S9 target at
  `output/screen-plan-proof/android-physical-target-readiness/proof-summary.json`:
  Wi-Fi ADB sees physical model `SM-G965W` on Android 10, but the target is
  locked behind keyguard/PIN, so the physical gate remains blocked instead of
  being claimed from emulator evidence.
- `scripts/test/screen-ios-replaykit-capability-proof.mjs` now records the iOS
  ReplayKit source-doc/no-overclaim gate at
  `output/screen-plan-proof/ios/proof-summary.json`. It treats iOS capture as
  explicit in-app ReplayKit session or broadcast-upload-extension work only,
  rejects arbitrary silent background capture of other apps, and blocks product
  readiness until physical-device ReplayKit execution and deletion proof exist.
- The screen-AI browser trigger proof validates managed-browser URL,
  browser-video, social-feed, and cloud-game trigger rows through
  activity-domain browser AI and screen-analysis schemas, then proves local-AI
  context builder consumption through the executable proof path. The previously
  referenced `test-results/screen-ai-browser-trigger-proof/proof.json` artifact
  is not present in the current checkout, so browser-trigger artifact closure
  remains a D/browser-lane handoff item. The social row is manual-required and
  the cloud-game row is unavailable/scaffold-only, so this does not claim
  authenticated social, cloud-frame, mobile parity, UI, or enforcement support.
- `ScreenIntelligenceRouteRequestSchema`,
  `ScreenManagedBrowserStructuredExtractionSchema`, and
  `scripts/test/screen-router-structured-extraction-proof.mjs` now prove the
  screen intelligence router contract: existing typed evidence is checked before
  capture, managed-browser structured URL/title/metadata/bounded visible-text
  extraction runs before screenshots, enough structured evidence produces
  `noScreenNeeded`, native app/game/launcher/unknown-process paths route only to
  parent-allowed active-window or selected-window scopes, and protected or
  credential-risk surfaces return unavailable. This is contract proof only; it
  does not claim live managed-browser producer integration, real DOM or
  accessibility capture, portal UI, policy execution, enforcement, or final
  pipeline closure.
- `ScreenRawScreenshotRetentionOptInSettingSchema`,
  `ScreenLiveViewOptInSettingSchema`, and
  `scripts/test/screen-optional-retention-live-preflight-proof.mjs` now prove
  the separate optional raw-retention/live-view preflight contract: raw
  retention requires explicit parent approval, audit ref, custody label, TTL,
  delete proof, and no raw remote upload; live view requires explicit parent
  approval, viewer audit, platform proof ref, LAN or relay transport label, no
  frame retention, no session recording, and no remote input. The proof writes
  `output/screen-plan-proof/27-28-optional-retention-live-preflight/proof-summary.json`.
  This is not runtime retention enablement, live transport, relay/cache,
  platform permission prompt proof, or privacy/legal approval.
- `scripts/test/screen-live-view-session-transport-proof.mjs` now proves the
  first runtime transport slice for optional live view. It captures a real
  active-window frame through the Rust screen-capture adapter, keeps the raw
  temp frame only until transport, sends the frame over a local view-only
  loopback session with digest/HMAC/sequence validation, deletes the raw temp
  frame after transport, and records no raw-frame cache, session recording, or
  remote input at
  `output/screen-plan-proof/live-view-session-transport/proof-summary.json`.
  This is a local harness proof only; production service session workers,
  platform live-view permission-prompt screenshots, relay/cache execution,
  parent UI persistence carry-forward, physical-device parity, and privacy/legal
  approval remain separate gates.
- `scripts/test/screen-live-view-service-session-proof.mjs` now consumes the
  real loopback live-frame transport/deletion artifact and the parent Settings
  command proof, then proves the service session readiness boundary. The
  `ScreenLiveViewServiceSessionGateSchema` accepts disabled and
  loopback-transport-only rows while keeping product live view false, carries
  `parentUiPersistenceState: proved`, and rejects product-readiness overclaims
  without live-view permission evidence, service runtime, no frame cache, no
  session recording, and no remote input. The proof writes
  `output/screen-plan-proof/live-view-service-session/proof-summary.json`, with
  parent UI persistence evidence in
  `output/screen-plan-proof/live-view-parent-ui-persistence/proof-summary.json`.
  This is not production service runtime, platform prompt screenshot,
  relay/cache, privacy/legal approval, or product-complete live view.
- `scripts/test/screen-live-view-runtime-proof.mjs` now proves the Rust
  `agent-service` live-view runtime decision boundary. The service-side state
  machine consumes the retained loopback live-frame transport/deletion artifact,
  carries parent UI persistence proof, rejects capture-only permission, rejects
  missing transport/deletion proof, rejects frame caching, session recording,
  and remote input, and can represent a service-runtime-ready but not
  product-ready state. The proof writes
  `output/screen-plan-proof/live-view-runtime/proof-summary.json`. This is not
  production live-view worker startup, platform prompt screenshots, relay/cache
  execution, physical-device parity, privacy/legal approval, or
  product-complete live view.
- `ScreenLiveViewProductionReadinessEvidenceSchema` and
  `scripts/test/screen-live-view-platform-permission-proof.mjs` now prove live
  view cannot become product-ready from opaque proof refs or ordinary capture
  permission. A ready bundle must carry a matching live-view prompt artifact
  ref/digest, viewer audit, live transport proof, physical-device parity proof,
  privacy/legal approval, production worker start proof, and relay/cache proof
  when relay-backed. The current proof keeps Android MediaProjection as
  capture-only evidence and records `liveViewProductReady:false`; it does not
  provide real platform prompt screenshots, live transport, physical-device
  parity, production worker start, relay/cache execution, or privacy/legal
  approval.
- `scripts/test/screen-live-view-worker-startup-proof.mjs` now proves the Rust
  `agent-service` live-view worker startup gate behind that runtime decision
  boundary. The gate separates `startupPermitted` from actual worker execution,
  refuses startup permission unless the runtime is product-ready and real
  live-view prompt artifacts, relay/cache execution when needed, physical-device
  parity, and privacy/legal approval are all present, and now proves a separate
  service-owned worker execution record can set `workerStarted:true` only after
  that gate and no raw-frame cache, session recording, or remote input are
  present. The proof consumes the existing real loopback frame
  transport/deletion, runtime, and parent UI persistence artifacts and writes
  `output/screen-plan-proof/live-view-worker-startup/proof-summary.json`.
  Controlled worker execution is still not treated as a live platform prompt
  screenshot, relay/cache execution, physical-device live-view parity,
  privacy/legal approval, live platform worker session, or product-complete
  live view.
- The Rust `agent-service` startup now has an explicit env-gated live-view
  worker runtime hook. It defaults to disabled, reads live-view mode,
  transport, permission, deletion, parent UI, platform prompt, relay/cache,
  physical parity, privacy/legal, raw-frame cache, recording, and remote-input
  gates from protocol-owned constants, and starts the worker only when the
  existing runtime and startup safety decisions both allow it. Focused service
  tests prove default-disabled behavior, capture-only permission blocking,
  deletion-proof blocking, successful all-gates startup, and unsafe remote-input
  refusal. This is service-owned startup behavior; it still does not prove
  platform prompt screenshots, hosted relay infrastructure, physical parity,
  privacy/legal approval, or product-complete live view.
- `scripts/test/screen-live-view-relay-cache-proof.mjs` now proves the
  relay-backed live-view cache execution item with a real captured frame. The
  proof writes an end-to-end encrypted relay envelope to an ephemeral local
  relay cache, verifies the digest after parent-side decryption, deletes the raw
  temp frame and relay cache, and rejects raw-frame cache, session recording,
  remote input, hosted relay infrastructure, platform prompt, physical parity,
  privacy/legal, and product-live-view claims.
- `ScreenManagedBrowserCdpScreenshotRequestSchema`,
  `ScreenManagedBrowserCdpScreenshotArtifactSchema`, and
  `scripts/test/screen-managed-browser-cdp-capture-proof.mjs` now prove the
  managed-browser CDP screenshot capture path: a real Chromium page target loads
  a public live page, `Page.captureScreenshot` captures page, viewport, and crop
  modes, each capture is tied to target/URL/title evidence refs, the image bytes
  pass through encrypted temporary queue custody, and raw/encrypted temp material
  is deleted. This is capture-path proof only; production managed-browser
  URL-trigger ownership, OCR/VLM quality, policy action, enforcement, live view,
  and raw screenshot retention remain separate gates.
- `ScreenOcrWorkerJobSchema`, `ScreenOcrWorkerResultSchema`, and
  `scripts/test/screen-ai-winrt-ocr-worker-proof.mjs` now prove the Windows
  WinRT OCR worker path over real selected-window pixels from a public live
  Wikipedia browser page and a native Notepad window. The proof runs the Rust
  screen-capture adapter, keeps the raw image only as an analysis temp file,
  runs Windows `Windows.Media.Ocr`, converts the OCR result into
  `ScreenAnalysisResult` evidence, creates allow dry-run policy decisions, and
  deletes the raw temp images. This is Windows OCR worker execution proof only;
  production OCR quality tuning, cross-platform OCR parity,
  authenticated-account surfaces, enforcement, live view, and raw retention
  remain separate gates.
- `ScreenOcrRedactionPolicySchema`,
  `ScreenOcrRedactionResultSchema`, and
  `scripts/test/screen-ocr-redaction-proof.mjs` now prove the OCR sensitive text
  redaction contract. Parent-controlled OCR text settings enforce a bounded
  snippet cap, disabled OCR text state, credential-like suppression, PII-like
  redaction, no raw text retention, no raw image retention, and no remote AI.
  `scripts/test/screen-ocr-redaction-portal-intent-proof.mjs` now proves
  Activity Screen read-model rows carry redacted OCR snippets and redaction
  notes into the Screen Analysis portal intent while omitting raw email, phone,
  credential, raw image retention, and remote AI. The service WinRT OCR proof now
  proves bounded OCR snippets and the structured `redactionNotes` array persist
  from adapter output into the service-backed Activity Screen read model. A real
  service-emitted redaction proof now captures a local Chrome text surface as
  pixels, runs WinRT OCR through the service queue, applies service-side local
  OCR redaction before event persistence, drains the queue, deletes adapter temp
  image material, and screenshots the real `#/screen-analysis` portal route with
  `[redacted-email]`, `[redacted-phone]`, `piiLikeTextRedacted`, and
  `credentialLikeTextRedacted` while omitting raw sensitive text. The service
  proof now also writes and consumes
  `output/screen-ai-pipeline-proof/service-winrt-ocr-redaction/parent-redaction-policy.json`
  as the persisted parent-selected OCR text retention/redaction policy.
- `scripts/test/screen-local-ocr-vision-runtime-model-proof.mjs` now aggregates
  the existing WinRT OCR worker, service OCR, guided VLM worker, VLM execution
  readiness, VLM journal/read-model, and local resource scheduler artifacts into
  the WP17 local OCR/vision runtime model proof. It proves local-only OCR/VLM
  contracts, worker input/output boundaries, structured output, no remote AI,
  no raw retention, model/runtime/template metadata, and
  degraded/manual-required states without claiming production model quality or
  cross-platform parity.
- `scripts/test/screen-ocr-tesseract-baseline-proof.mjs` now records the
  Tesseract OCR baseline runtime extraction check. It verifies the upstream
  project/docs/license direction, installs/invokes local Tesseract 5.5.0
  through the standard Windows install path, and extracts expected visible
  terms from a retained real public Vimeo screenshot artifact. It also records
  extraction duration, CPU time, peak working set, and three derived
  failure-mode scenarios from the same real screenshot: sparse page
  segmentation, downscaled small text, and cropped player UI. Production OCR
  selection and PaddleOCR comparison remain open.
- `scripts/test/screen-ocr-paddleocr-evaluation-proof.mjs` now records the
  PaddleOCR/PP-OCR candidate readiness and Windows runtime/quality gate. It verifies
  current PyPI versions, installed `paddleocr` 3.6.0 and `paddlepaddle` 3.3.1,
  Tesseract baseline availability, local official PP-OCRv5 model-cache custody,
  and an explicit local inference attempt against the retained real public Vimeo
  screenshot. The current PP-OCRv5 runtime now executes locally with
  `enable_mkldnn=false`, but extracts zero text from that real proof image, so
  it is not selected for production. The proof also runs deleted preprocessing
  variants over the same real image, covering original, 2x upscale, grayscale
  contrast 2x upscale, and grayscale sharpen 2x upscale inputs; current
  PP-OCRv5 still extracts zero text from every variant. The same proof can run
  an explicitly prepared isolated Python 3.10 fallback with
  `paddleocr` 2.7.0.3, `paddlepaddle` 2.6.2, and `numpy<2`; that fallback
  completed local inference against the same real Vimeo screenshot, extracted 15
  text strings, matched the `vimeo`, `video`, and `player` baseline terms, and
  recorded init/predict timing, CPU time, and peak RSS. This proves a local
  PaddleOCR-family fallback can analyze the captured evidence, but it does not
  prove current PP-OCRv5 quality or select production OCR.
- `scripts/test/screen-ocr-windows-candidate-selection-proof.mjs` now records
  the current Windows service OCR route decision. It aggregates the real WinRT
  OCR service proof, WinRT redaction service proof, Tesseract baseline, and
  PaddleOCR/PP-OCR quality evidence to select `windows-winrt-ocr` as the current
  Windows service OCR route for this lane. Tesseract remains a measured fallback
  baseline, current PP-OCRv5 remains not selected because it extracts zero text
  from the real proof image and deleted preprocessing variants, and pinned
  PaddleOCR 2.x remains a measured fallback candidate only.
  This is not a cross-platform OCR or final production-quality claim.
- `scripts/test/screen-vlm-guided-classifier-readiness-proof.mjs` records the
  small guided VLM classifier readiness boundary for screen-plan WP36 by
  reusing the typed execution-readiness proof. It proves local-only custody,
  guided worker template/version pinning, bounded image pixels, deleted
  query-store custody before completed status, open-ended prompt rejection, and
  manual-required behavior when runtime is unavailable. The proof now records
  local provider command probes for `ollama`, LM Studio `lms`, legacy
  `lmstudio`, `llama-server`, and the local `llama-mtmd-cli` runtime path. It
  detects the cached llama.cpp/Qwen2-VL binary, model, and mmproj files plus the
  retained `screen-ai-local-vlm-proof` matrix artifact, which covers 16
  controlled browser/native scenarios, 17 real window captures, schema
  validation, policy dry-runs, and raw image deletion. It also cross-checks the
  retained nine-scenario live operator matrix at
  `output/screen-ai-pipeline-proof/live-operator/proof-summary.json`, covering
  public/live URL plus native-app captures through local VLM classification,
  schema validation, policy dry-run, and raw image deletion. It also
  cross-checks
  `output/screen-plan-proof/36-vlm-resource-crop-readiness/proof-summary.json`,
  which verifies retained controlled and live-operator VLM capture inputs stay
  below the worker max-image-pixel budget and that the managed-browser CDP crop
  capture path exists with deletion proof. It also cross-checks
  `output/screen-plan-proof/36-vlm-runtime-resource-measurement/proof-summary.json`,
  which runs the local llama.cpp/Qwen2-VL runtime over retained parent proof
  screenshots and records per-sample wall time, sampled CPU seconds, and peak
  working set while producing normalized screen-analysis JSON. It also records
  `output/screen-plan-proof/36-vlm-live-crop-quality/proof-summary.json`, which
  captures real public Vimeo, Wikipedia, 2048 browser game, and eBay
  managed-browser crops through CDP, analyzes each crop with local Qwen2-VL,
  matches expected `video`, `school`, `game`, and `shopping` categories plus
  visible terms, and deletes each raw crop. It also records
  `output/screen-plan-proof/36-vlm-model-selection/proof-summary.json`, which
  selects the cached local llama.cpp/Qwen2-VL route for the current Windows
  proof path after runtime, resource, quality, local-only, and deletion
  evidence. It also records
  `output/screen-plan-proof/36-vlm-rollout-fallback-gate/proof-summary.json`,
  which allows that Windows route only inside the measured local image/resource
  envelope after public live video/school/game/shopping/social-feed crop quality
  passes, and proves runtime-missing, oversized-input, over-budget, and
  authenticated-social-unproved states fall back to OCR/manual-required instead
  of remote AI or raw screenshot retention. It still does not claim
  authenticated-account social proof, broader rollout thresholds across more
  hardware profiles, or cross-platform model/runtime parity.
- `scripts/test/screen-ai-service-winrt-ocr-proof.mjs` now proves the
  service-owned Windows WinRT OCR analysis path over real live Chrome/Wikipedia
  pixels: the Rust service cadence loop captures an active-window frame into
  the encrypted queue, a local Windows `Windows.Media.Ocr` adapter reads the
  queued image, the service records a `localOcr` Activity Screen row with
  runtime/model/template metadata, bounded `ocrTextSnippets`, the
  `redactionNotes` array shape, and a `school`/policy-eligible result, the
  WebSocket read model exposes the row, the queue drains, and adapter temp image
  material is deleted. It does not claim sensitive-text redaction on this public
  Wikipedia source, production OCR quality,
  authenticated-account/social coverage, cross-platform OCR parity,
  enforcement, live view, or raw retention.
- `scripts/test/screen-ai-service-winrt-ocr-policy-proof.mjs` now reruns that
  real service WinRT OCR proof and consumes the resulting `localOcr` Activity
  Screen row through typed parent-domain policy contracts. It proves the
  `school`/policy-eligible OCR row produces an allow dry-run decision with
  activity, journal, and query-store evidence refs, a parent rule ref, policy
  refs on the Activity Screen read-model row, deleted-image custody, and
  `rawImageRetained:false`. It does not claim final enforcement, broad adapter
  dispatch, production OCR quality, live view, or raw retention.
- `ScreenAiAdapterReadinessReadModelSchema` and
  `scripts/test/screen-ai-adapter-readiness-proof.mjs` now prove the
  screen-derived adapter-readiness handoff stays honest after policy: real
  Windows owned-process time-limit and owned-process block decisions keep their
  retained adapter execution proof, while broad installed-app, host
  network/domain, managed exact active-tab, Android/iOS mobile, and Linux host
  targets remain manual-required, not-claimed, or unavailable. The proof keeps
  source evidence refs, deleted-image custody, and `rawImageRetained:false`.
  The same parent-domain contract now exposes `ScreenAiAdapterCompletionArtifact`
  parsing plus `screenAiFinalAdapterCompletionGate`, so the final pipeline row
  cannot close unless every broad/browser/network/mobile blocker provides a
  screen-derived apply result, rollback-or-expiry result, audit ref, and
  deleted-image custody artifact. `scripts/test/screen-ai-final-adapter-dependency-audit.mjs`
  imports the built contract and currently records the gate as unsatisfied with
  all five required upstream artifacts missing. This does not rerun live capture
  or implement broad/browser/network/mobile adapters.
- `ScreenSummaryParentExplanationSchema` and
  `scripts/test/screen-summary-parent-explanation-proof.mjs` now prove the
  parent explanation/audit context can consume those deleted local
  screen-summary refs. The proof cites screen-summary refs, parent rule refs,
  dry-run policy refs, local runtime refs, audit evidence refs, and
  `screen-image-deleted` custody while keeping raw image retention, remote/API
  AI, portal runtime UI, policy authority, and enforcement unclaimed.
- `ScreenSummaryParentExplanationReadModelSnapshotSchema` and
  `scripts/test/screen-summary-parent-explanation-read-model-proof.mjs` now turn
  those local-only parent explanation rows into a parent-visible read model that
  preserves screen-summary refs, audit refs, parent rules, dry-run policy refs,
  runtime refs, custody, and deletion state without showing raw screenshots or
  claiming production portal runtime rendering.
- `scripts/test/screen-summary-parent-explanation-service-read-model-proof.mjs`
  now proves those screen-summary parent explanation refs survive the
  child-device query-store/service read-model boundary. The script starts the
  real Rust service against a seeded ActivityStore, requests the Activity Screen
  read model over WebSocket, and verifies policy refs, parent rule refs, local
  runtime refs, parent explanation refs, deletion reasons, deleted-image state,
  and child-device custody. It does not claim production portal rendering, new
  capture/model inference, remote/API AI, or enforcement.
- The Screen Analysis portal route now renders those service-backed parent
  explanation refs as explicit `Parent explanation refs` details instead of
  hiding them under local-AI result wording. The
  `SCREEN_PARENT_PORTAL_SUMMARY_UI_PROOF` Playwright proof asserts the real
  portal/agent command path, parent explanation ref visibility, product-boundary
  copy, raw screenshot absence, desktop screenshot, and mobile screenshot.
- `scripts/test/screen-ai-final-product-path-proof.mjs` now validates the
  retained final path evidence across real live/operator trigger artifacts,
  capture metadata, local VLM analysis, policy dry-runs, Windows action handoff
  proofs, portal/read-model proof, retention/deletion custody, and protected
  surface non-claims at
  `output/screen-ai-pipeline-proof/final-product-path/proof-summary.json`. This
  gate now requires
  `output/screen-ai-pipeline-proof/live-operator-evidence-bundle/proof-summary.json`,
  which copies only redacted source/AI/policy/deletion artifacts, redacted model
  logs, and parent-explanation screenshots for remote review while excluding raw
  screenshots and encrypted queue payloads. This is a stacked artifact gate; it
  does not rerun the live operator session or claim managed-browser trigger
  ownership, authenticated-account social proof, or broad browser/network/mobile
  adapters.
- `scripts/test/screen-plan-closure-audit-proof.mjs` now records the remaining
  partial screen-plan gates at
  `output/screen-plan-proof/screen-plan-closure-audit/proof-summary.json`. The
  audit verifies current macOS, Linux, Android, iOS, live-view, OCR, and VLM
  readiness artifacts are present while separately proving platform/product
  readiness remains false where real-device, platform prompt, relay/cache,
  PP-OCRv5 quality/resource, cross-platform OCR parity, authenticated-account
  social, or broader VLM rollout gates are still missing. This prevents the
  stacked product-path artifact from being treated as full screen-plan
  completion.
- Product-complete retention controls and quality proof are incomplete.
- Raw screen control settings are preserved as design inputs, not
  product-complete implementation proof.
- `scripts/test/screen-optional-visibility-capability-status-proof.mjs` now
  proves optional raw-retention/live-view parent opt-in readiness rows. Disabled
  modes stay unavailable, approved raw retention remains manual-required without
  runtime/deletion proof, and approved raw retention can become ready only when
  existing service settings runtime proof, deletion-event proof, child
  disclosure readiness, and child device readiness are all present while current
  raw-frame retention and raw remote upload stay false. Approved live view
  remains blocked when the only platform evidence is capture consent. This is a
  child/device capability readiness proof, not active raw screenshot storage or
  live transport.
- `scripts/test/screen-raw-retention-runtime-proof.mjs` now proves the shared
  `ScreenAnalysisParentSetting` contract and Rust service settings runtime can
  accept optional raw screenshot retention only as a parent-approved local
  short-TTL mode. The proof keeps delete-after-success and delete-after-expiry
  mandatory, rejects long TTL, disabled-analysis, and delete-after-expiry=false
  raw-retention settings, and records the service/runtime validation artifact at
  `output/screen-plan-proof/raw-retention-runtime/proof-summary.json`. Raw
  retention remains disabled by default, raw remote upload remains forbidden,
  and this does not complete parent retention-control UI, live view,
  relay/cache, privacy/legal approval, or broad platform parity.
- `ScreenOptionalVisibilityRuntimeSettingsStateSchema` and
  `scripts/test/screen-optional-visibility-runtime-settings-proof.mjs` now prove
  the separate writable runtime settings contract for optional raw-retention and
  live-view opt-ins. The proof accepts parent-approved LAN live view as
  view-only state while keeping `productLiveViewReady:false`, accepts raw
  retention separately, rejects stale writes, rejects mixed raw-retention plus
  live-view custody, and keeps raw screenshot remote upload false. The proof
  writes
  `output/screen-plan-proof/optional-visibility-runtime-settings/proof-summary.json`.
  It does not start live-view workers, prove platform prompt screenshots,
  physical-device parity, hosted relay infrastructure, privacy/legal approval,
  remote input, or product-complete live view.

## Current Gap

Product-complete retention-control rollout beyond the proved parent-approved
local short-TTL Settings command path and optional visibility runtime settings,
actual production live-view worker start, hosted relay infrastructure,
platform permission prompt proof, physical-device live-view parity, child-agent
disclosure runtime deployment,
privacy/legal approval, authenticated-account
social proof beyond public/live surface proof, production OCR/VLM quality beyond
the WinRT OCR service proof, controlled fixtures, the full live operator matrix,
public live crop matrix, retained bounded-input proof, retained proof-image VLM
resource measurement, and completed WP36 Windows-route evaluation,
production local vision adapter quality beyond
the service proof adapter, service-owned live trigger event producers beyond the
timed cadence
loop and native active-window foreground watcher capture/queue event producer
startup wiring for remaining live producers beyond the service capture/deletion,
retention sweeper deletion-event, and analysis row-ready handoffs, physical
Android parity, physical iOS ReplayKit execution/deletion proof, live macOS
capture proof, Linux root/Wayland portal proof,
browser-trigger artifact closure,
browser/network/mobile/broad block action adapters from screen-derived
decisions, and physical household mesh discovery/runtime proof beyond the
loopback runtime exchange remain.
Browser-trigger proof now covers contract flow into screen evidence and
local-AI context only; live trigger producers, production startup subscriptions
for all live producers, authenticated surfaces, cloud-streamed frames, mobile
parity, and UI remain separate proof gates.

## Checklist

- [x] Parent opt-in setting contract and service runtime gates.
- [x] Capability/status contract.
- [x] Encrypted temporary image queue.
- [x] Local OCR/vision summary proof route.
- [x] Local OCR/vision runtime model aggregates OCR/VLM worker, readiness,
      read-model, and scheduler proof without remote AI or raw retention.
- [x] Image deletion and retention state proof.
- [x] Confidence and unknown handling.
- [x] OCR sensitive text redaction contract proves parent-controlled snippet
      limits, disabled OCR text, credential-like suppression, PII-like
      redaction, and no raw text or remote AI retention.
- [x] OCR redaction read-model/portal intent proof renders redacted snippets
      and redaction notes without raw email, phone, credential, raw image
      retention, or remote AI.
- [x] Service WinRT OCR read model persists bounded OCR snippets and the
      structured redaction-note array shape from real service adapter output.
- [x] Service WinRT OCR redaction proof applies local sensitive-text redaction
      before event persistence and screenshots the real Screen Analysis portal
      route from the service-emitted row.
- [x] Service WinRT OCR redaction proof consumes a persisted parent-selected OCR
      redaction policy file for snippet limit, text retention, credential
      suppression, and PII redaction settings.
- [x] Policy decision references summary evidence.
- [x] Service WinRT OCR row feeds a typed parent policy dry-run without raw
      image retention.
- [x] Portal read-model explanation and audit proof.
- [x] Dedicated Screen Analysis route renders service-backed screen summary
      rows with capability, queue, model, confidence, custody, policy, audit,
      evidence refs, not-claimed enforcement state, and desktop/mobile
      screenshots from the real portal/agent route.
- [x] Parent Settings route renders read-only Screen settings/capability proof.
- [x] Parent Settings route renders writable local screen-summary intent draft
      proof.
- [x] Service-side parent settings persistence proves disabled defaults, local
      JSON-backed strict dry-run setting reload, and rejection of raw retention
      or unsafe policy/capture combinations.
- [x] Service-side parent settings command path proves typed get/replace
      protocol adapter, Rust command/event parity, WebSocket routing into the
      local JSON-backed runtime, strict dry-run persistence across restart, and
      raw-retention rejection before persistence.
- [x] Parent Settings route submits persisted parent screen setting changes
      through the real service command path and renders accepted get/replace
      service responses with local JSON persistence.
- [x] Raw-retention/live-view/remote boundary rejects raw screenshot retention,
      live view, and raw remote upload by default.
- [x] Optional raw-retention/live-view preflight contract requires explicit
      opt-in, custody, audit, deletion/no-retention, and platform-proof gates.
- [x] Optional raw-retention/live-view capability status records disabled,
      manual-required, and blocked child/device readiness rows and rejects
      product readiness without runtime/deletion, live-view platform prompt, and
      production session/UI proof.
- [x] Optional raw screenshot retention runtime accepts only parent-approved
      local short-TTL mode with delete-after-success and delete-after-expiry
      still required; default raw retention, long TTL, raw remote upload, and
      disabled-analysis retention remain rejected.
- [x] Parent Settings renders optional raw-retention/live-view readiness rows
      from the same domain proof through the real portal/agent path without
      enabling raw retention, live view, live transport, relay/cache, remote
      input, or privacy/legal approval.
- [x] Local live-view session transport proof captures one real active-window
      frame, validates view-only loopback delivery with digest/HMAC, and deletes
      the raw temp frame without cache, recording, or remote input.
- [x] Live-view service-session readiness proof consumes the real loopback
      transport/deletion artifact as a non-product-ready row and rejects
      product readiness until service runtime, live-view platform prompt proof,
      no frame cache, no recording, and no remote input are present, while
      carrying parent UI persistence proof.
- [x] Rust agent-service live-view runtime decision proof rejects capture-only
      permission, missing transport/deletion proof, frame cache, session
      recording, and remote input, while preserving a service-ready but
      product-blocked state until platform prompt, production worker,
      relay/cache, physical parity, and privacy/legal gates exist.
- [x] Rust agent-service live-view worker startup gate separates startup
      permission from actual worker execution and refuses permission until
      runtime readiness, real platform prompt artifact, relay/cache when needed,
      physical-device parity, and privacy/legal approval exist.
- [x] Rust agent-service startup owns an env-gated live-view worker runtime hook
      that defaults disabled and starts only after the existing runtime,
      startup, deletion, platform prompt, relay/cache, physical parity,
      privacy/legal, and unsafe-retention/control gates allow it.
- [x] Live-view relay/cache proof uses a real captured frame, encrypted
      forced-relay cache envelope, parent digest verification, raw temp
      deletion, relay-cache deletion, no raw-frame cache, no recording, and no
      remote input.
- [x] Local AI resource scheduler prevents multiple heavy OCR/VLM jobs and
      prioritizes policy-blocking screen analysis.
- [x] Detector-specific prompt packs replace open-ended screen descriptions and
      reject private/raw output fields.
- [x] Legacy household-provider route contract is used before remote/API for
      hard visual cases.
- [x] Legacy household-provider runtime/discovery loopback proof starts a real
      local endpoint, records hello/heartbeat/route evidence, submits a
      redacted-crop job, and preserves no-raw/no-remote custody boundaries.
- [x] Screen intelligence router checks typed evidence and managed-browser
      structured extraction before selecting screenshots.
- [x] Managed-browser CDP screenshot capture is page-scoped, target-tied,
      queued, and deleted.
- [x] Windows WinRT OCR worker analyzes real browser/native captured pixels,
      emits schema-valid screen analysis evidence, feeds policy dry-run, and
      deletes raw temp images.
- [x] Tesseract baseline source/license/runtime extraction, CPU/memory
      measurement, derived failure-mode scenarios, and same-image comparison
      against the isolated local PaddleOCR 2.x fallback are recorded against a
      retained real public Vimeo screenshot artifact; Tesseract is a measured
      fallback while the current Windows service OCR route is WinRT and current
      PP-OCRv5 mobile detector, server detector, and preprocessing variants still
      extract zero text. Production OCR selection remains tracked by the
      PaddleOCR/PP-OCR and Windows route-selection gates, not this baseline row.
- [x] PaddleOCR/PP-OCR candidate readiness records current package availability,
      current PP-OCRv5 mobile-detector and cached server-detector local execution
      with zero extracted text from the retained Vimeo screenshot, zero text from
      deleted preprocessing variants, and a pinned local PaddleOCR 2.x fallback
      that extracts comparable text. The candidate evaluation is complete as an
      explicit non-selection for the current Windows production OCR route; a future
      PaddleOCR production gate requires a new candidate or an explicit
      productionization decision for the pinned 2.x fallback.
- [x] Windows service OCR route selection proof chooses WinRT OCR for the
      current Windows service path from retained real service/redaction evidence
      while leaving Tesseract/PaddleOCR as non-selected fallback candidates.
- [x] Small guided VLM classifier evaluation records local-only handoff,
      template/version, max-pixel, open-ended prompt rejection, deletion,
      provider-command probes, local llama.cpp/Qwen2-VL runtime availability,
      retained controlled local VLM matrix proof, and retained nine-scenario live
      operator matrix proof. The retained resource/crop readiness audit verifies
      bounded VLM input dimensions and managed-browser CDP crop capture/deletion,
      and the retained proof-image runtime measurement records per-sample wall time,
      sampled CPU seconds, peak working set, and normalized local VLM output.
      The public-live crop quality proof verifies real Vimeo, Wikipedia, 2048
      browser game, eBay, and Bluesky public social/feed CDP crops with local
      Qwen2-VL category/text matches and deletion. The current-route model
      selection proof chooses cached local
      llama.cpp/Qwen2-VL for the Windows proof path, and
      the measured fallback gate blocks runtime-missing, oversized-input,
      over-budget, and authenticated-social-unproved states before remote AI.
      This closes the Windows-route evaluation workpack; authenticated-account
      social proof, production model quality, cross-platform parity, and broader
      rollout thresholds across more hardware profiles remain open.
- [x] Service WinRT OCR proof analyzes live public browser pixels through the
      encrypted service queue, records a `localOcr` Activity Screen row, drains
      the queue, and deletes adapter temp image material.
- [x] Screen-derived adapter readiness preserves real owned-process adapter
      proof while keeping broad/browser/network/mobile targets
      manual-required, not-claimed, or unavailable without raw retention or
      claim upgrades. WSL2 Linux host execution is proved separately, while
      native Linux desktop/Wayland/PipeWire product parity remains unclaimed.
- [x] Screen-summary parent explanation/audit context cites local OCR replay
      refs, parent rules, dry-run policy refs, and deleted-image custody without
      remote AI or enforcement claims.
- [x] Screen-summary parent explanation read-model proof preserves refs,
      custody, and deleted-image state without raw screenshot display or portal
      runtime claims.
- [x] Parent explanation refs preserved through the service-backed Activity
      Screen read-model projection.
- [x] Parent explanation refs render on the real Screen Analysis portal route
      from the service-backed Activity Screen read model without raw screenshot
      display or product-complete enforcement claims.
- [x] Final screen-AI product path artifact gate validates retained real-run
      trigger, capture, AI, policy, action/read-model, and deletion/custody
      artifacts without raw screenshot retention, and now requires the portable
      live-operator evidence bundle for remote inspection of redacted retained
      proof artifacts.
- [x] Screen-plan closure audit records current readiness proof artifacts and
      remaining partial platform, live-view, and model-quality product gates so
      screen is not claimed product-complete prematurely.
- [x] Screen eventing runtime proof publishes typed capture, queue, AI-result,
      summary, policy, action dry-run, deletion, and portal-read-model events
      through `ocentra-eventing` without raw-image escape.
- [x] Screen service event bridge proof maps service Activity Screen read-model
      rows into the existing typed screen event chain, rejects raw retention and
      missing policy refs, publishes degraded AI rows without policy/action
      refs, and avoids a duplicate service event bus.
- [x] Screen service event subscription proof consumes typed service row-ready
      events from the service-started subscriber runtime, invokes the existing
      bridge, routes degraded AI rows through the degraded event chain,
      publishes downstream screen runtime events for safe rows, and rejects
      raw-retained rows before downstream publication.
- [x] Screen service analysis row-ready producer proof starts the event
      subscriber runtime from the service analysis loop, publishes
      `screen.service.row.ready`, and gates current analysis rows as
      `MissingPolicyDecision` before downstream policy/action publication.
- [x] Screen service capture event producer proof publishes capture-observed and
      encrypted-queue events from the service cadence and native foreground
      capture loops after encrypted queue handoff.
- [x] Screen service deletion event producer proof publishes deletion-committed
      events from the retention sweeper after expired queue deletion without
      fabricating policy/action refs.
- [x] Screen child disclosure proof defines child-visible disabled, paused,
      active capture, protected-surface, and deleted-summary status while
      rejecting hidden capture/raw screenshot display claims.
- [x] Screen child disclosure proof renders desktop/mobile child-visible
      disclosure screenshots without claiming child-agent runtime deployment.
- [x] Screen enforcement handoff guard proof requires dry-run policy,
      summary/local-AI/audit refs, enabled parent rule, confidence state, and a
      guard audit event while rejecting raw pixels, raw model text, retained
      screenshot, and local-AI authority claims.
- [x] Screen household mesh proof keeps raw screenshots off LAN provider
      payloads, grants one child-owned lease, validates provider results on the
      child agent before policy, and rejects invalid provider results.

Product-complete retention controls, production OCR/VLM quality,
authenticated-account social proof,
externally proved live producer coverage beyond the service capture/queue,
deletion, and analysis row-ready/policy-ref handoffs, child-agent disclosure
runtime deployment, production household mesh transport over physical LAN, broad
adapters remain in the Current Gap section above.

## Next AI Instructions

Never route raw screen images to Ocentra cloud by default. Treat summaries,
confidence, deletion state, and custody labels as required product fields.
