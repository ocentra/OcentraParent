<!-- agent-capsule -->

> Agent Capsule
> Doc: Local Screen Evidence Analysis Queue Architecture
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Local Screen Evidence Analysis Queue Architecture

Status: V0.5.3 research/spec. This document defines the local screen evidence
analysis queue before runtime implementation. It does not add feature code,
screen capture adapters, OCR/vision runtime, policy runtime, or portal UI.

## Product Claim

Ocentra Parent may use screen evidence only as an explicit parent-enabled local
analysis layer for ambiguous activity. The child-device agent may capture a
screen or window image on a configured cadence or trigger, store that image only
as an encrypted temporary queue job, analyze it with a local OCR/vision runtime,
store a typed summary with evidence references, and delete the image.

The normal product path is:

```text
parent opt-in setting
  -> child-device capture scheduler
  -> encrypted temporary screen-analysis queue
  -> local OCR/vision worker
  -> schema-valid screen analysis result
  -> encrypted journal and SQLite read model
  -> portal summary, local AI references, and policy dry-run/enforcement handoff
  -> temporary image deletion
```

Ocentra-hosted services are not the default store or processor for screenshots,
screen-analysis images, screen summaries, child activity reports, SQLite
evidence, journals, or parent rules. Any future hosted report or assistant flow
must consume only parent-authorized evidence/report bundles under a separate
data-custody design and must not replace the local child-device safety path.

## Product Boundary

Screen evidence can support parent-visible answers such as:

- visible activity category candidates, for example school, video, chat, game,
  adult content, violence, bypass tool, shopping, productivity, or unknown;
- limited OCR snippets when the parent enables text extraction and the local
  redaction policy allows them;
- risk signals such as possible bypass tool, credential prompt, unsafe visible
  content, self-harm signal, explicit content signal, or unknown;
- confidence, uncertainty reason, source evidence references, image digest, and
  deletion state;
- unavailable, permission-limited, protected-surface, expired, failed, invalid,
  stale, and local-model-unavailable states.

Screen evidence must not support these product claims by default:

- permanent screenshot history;
- cloud/API OCR or vision processing of child screenshots;
- Ocentra-hosted storage of child screen activity;
- password, secure desktop, lock screen, credential prompt, or OS-protected
  surface capture;
- keystrokes, microphone audio, camera video, browser secrets, cookies, tokens,
  decrypted network payloads, or raw browser content capture;
- enforcement from raw AI text, image pixels, or unvalidated model output;
- hidden or undisclosed screen capture.

Unknown is a valid result. The system must show unavailable or low-confidence
states instead of overclaiming what the image or local model did not prove.

## Local-First Custody

Every screen-evidence object needs a source/custody label:

- `live-local-child-agent`: service response over loopback from the child-device
  agent;
- `live-lan-child-agent`: service response over explicit LAN mode from the
  child-device agent;
- `child-device-temp-queue`: encrypted temporary image queue on the child
  device;
- `child-device-journal`: encrypted child-device journal record;
- `child-device-query-store`: local SQLite read model rebuilt from the journal;
- `parent-device-cache`: parent-device cached summary or report;
- `parent-owned-export`: parent-approved export or sync bundle;
- `ocentra-hosted-non-activity`: account, entitlement, pairing, release, or
  notification metadata only;
- `unavailable`: source not configured, unreachable, degraded, expired, or out
  of scope.

No V0.5.3 screen queue path should silently upload child screenshots, local OCR
text, screen summaries, journals, SQLite evidence, generated reports, or parent
rules to Ocentra-hosted storage. The only long-lived evidence from this slice is
the structured local summary, evidence references, model/runtime status
reference, image digest, and deletion/audit state.

## Parent Opt-In Settings

Screen analysis starts disabled. A parent-controlled setting must be present,
validated, journaled or auditable, and visible before capture runs.

Required parent settings:

- `screenAnalysisEnabled`: disabled by default.
- `analysisMode`: `observeOnly`, `policyDryRun`, or `enforcementEligible`.
- `cadenceCaptureEnabled`: whether fixed-interval capture is allowed.
- `cadenceSeconds`: bounded interval, with a conservative default such as five
  minutes and a strict parent-selected option such as one minute.
- `strictModeEnabled`: explicit flag for the shortest supported cadence.
- `triggerCaptureEnabled`: whether event-triggered capture is allowed.
- `enabledTriggers`: foreground app change, managed browser URL change,
  app/game foreground start, unusual network change, policy ambiguity, or manual
  parent test capture.
- `allowedCaptureScope`: full screen, active display, active window, or managed
  browser/window only, depending on platform capability.
- `ocrTextEnabled`: whether OCR snippets may be stored in summaries.
- `ocrTextSnippetLimit`: maximum retained snippet count and length.
- `redactionMode`: local redaction behavior for text snippets and sensitive
  regions.
- `temporaryImageTtlSeconds`: short TTL for queued images.
- `maxRetryCount`: retry budget before expiry/deletion.
- `deleteAfterSuccess`: required true for this milestone.
- `deleteAfterExpiry`: required true for this milestone.
- `retainRawImage`: required false for this milestone unless a later explicit
  feature creates a new custody, retention, and disclosure design.
- `policyUseEnabled`: whether policy may consume screen summaries.
- `changedByParentRef`, `changedAt`, `settingVersion`, and `reason`: audit
  fields for who enabled or changed the setting.

The portal may author these settings as typed parent intent. The Rust agent owns
validation, scheduling, capture, queue storage, analysis handoff, deletion,
journal writes, and policy handoff. The portal must not capture screenshots or
run local OCR/vision.

## Capability And Status States

Screen analysis needs explicit capability state before capture attempts:

- `disabledByParent`: setting is off.
- `unsupportedPlatform`: platform or build cannot capture the requested scope.
- `unsupportedScope`: requested full-screen, display, window, or managed-window
  capture is unavailable.
- `permissionRequired`: OS permission or consent is needed before capture.
- `permissionLimited`: permission exists but does not allow the requested scope.
- `protectedSurface`: secure desktop, lock screen, DRM/protected content,
  password prompt, credential prompt, or OS-protected surface was detected.
- `screenLocked`: device is locked or not in an interactive session.
- `sessionUnavailable`: no supported local desktop/session is active.
- `modelUnavailable`: local OCR/vision runtime is not ready.
- `queueUnavailable`: encrypted temp queue cannot be opened.
- `degraded`: capture or analysis can run with reduced fidelity.
- `adapterError`: capture adapter failed.
- `ready`: capture and local analysis are available within the configured
  boundary.

Capability status is evidence and portal state. It must not be treated as a
successful screen observation unless a capture job and validated result exist.

## Components

Parent setting validator:

- Accepts typed parent intent from local/LAN parent surfaces.
- Enforces disabled-by-default behavior, cadence bounds, TTL bounds, retry
  bounds, disclosure requirements, and retention rules.
- Records who changed the setting and the resulting effective mode.

Capability detector:

- Evaluates platform, session, permissions, capture scope, protected-surface
  state, queue health, and local model/runtime status.
- Emits typed unavailable/degraded states without fabricating analysis results.

Capture scheduler:

- Combines cadence and trigger rules.
- Debounces triggers so foreground, URL, app/game, or network changes do not
  flood the queue.
- Refuses capture when disabled, unavailable, permission-limited, protected, or
  outside the parent-selected schedule.

Screen/window capture adapter:

- Captures only the parent-approved local scope.
- Returns an image buffer to the queue writer, not to portal code.
- Records source ids and related evidence refs for foreground app, browser URL,
  app/game session, and network digest when available.

Encrypted temporary queue:

- Stores images at rest under child-device local custody only.
- Stores metadata needed for scheduling, retry, TTL, source refs, image digest,
  and deletion/audit.
- Does not expose raw image paths in portal copy/debug output.

Local OCR/vision worker:

- Reads only encrypted queue jobs and source evidence refs.
- Runs on the child device.
- Returns structured JSON only.
- Does not scan OS state, browser profiles, files, network traffic, or process
  lists directly.

Result mapper and validator:

- Validates local model output against the screen-analysis result schema.
- Rejects invalid confidence, missing evidence refs, unsupported categories,
  malformed deletion state, or raw unbounded text.
- Emits invalid/failed/expired states that cannot drive enforcement.

Journal and SQLite ingest:

- Writes validated summaries and lifecycle events to the encrypted journal.
- Replays summaries into SQLite read models.
- Keeps SQLite rebuildable from the journal.
- Stores queue image deletion state, not the raw image.

Portal read model provider:

- Shows settings, capability, queue status, summaries, evidence refs,
  source/custody labels, local model status, and deletion state.
- Redacts raw paths and never displays raw screenshots by default.

Policy, AI, and enforcement handoff:

- Provides evidence references and concise typed summaries to local AI and
  deterministic policy.
- Allows policy to act only after schema-valid local summaries and parent rules
  produce a typed policy decision.
- Keeps raw images and raw AI text out of enforcement.

## Queue Contract

The final names belong in the owning domain package, but the queue job concept
should include:

- `queueJobId`.
- `schemaVersion`.
- `createdAt`, `notBefore`, `expiresAt`, and `lastAttemptAt`.
- `captureReason`: cadence, foreground app change, managed URL change,
  app/game foreground start, unusual network change, policy ambiguity, manual
  parent test capture, or retry.
- `captureScope`: full screen, display, active window, managed browser/window,
  or unsupported.
- `sourceId` and `adapterId`.
- `deviceRef` and `localUserRef`.
- `parentSettingRef` and `settingVersion`.
- `relatedEvidenceRefs`: foreground app/window evidence, browser evidence,
  app/game session evidence, network digest evidence, and prior policy/AI
  ambiguity refs where available.
- `encryptedImageRef`: opaque local queue reference, redacted outside the agent.
- `imageDigest`: digest of the captured image bytes for dedupe and audit.
- `imageByteSize` and `imageFormat`, if safe and useful for diagnostics.
- `status`: queued, processing, analyzed, deleting, deleted, expired, failed,
  invalid, unavailable, permissionLimited, protectedSurface, or canceled.
- `attemptCount` and `maxRetryCount`.
- `failureReason` or `unavailableReason` when applicable.
- `deletionRequired`: true.
- `deletedAt`, `deletionStatus`, and `deletionProofRef` where available.
- `custodyState`: child-device-temp-queue.

Queue jobs are not long-term evidence. They are temporary work items whose raw
image payload must be deleted after successful local analysis or TTL expiry.

## TTL, Retry, And Deletion

Temporary images must be bounded:

1. Queue writer encrypts the image before it is durable on disk.
2. Queue metadata includes `expiresAt` and deletion-required state.
3. The worker may retry failed local analysis only within the TTL and retry
   budget.
4. Successful analysis immediately moves the job to deletion.
5. TTL expiry moves the job to expired deletion even if analysis never ran.
6. Delete failures become explicit `deleteFailed` or `deletePending` states with
   retry and parent-visible health status.
7. The journal stores summary/lifecycle/deletion state, not the raw image.
8. The agent startup path scans for expired or delete-pending queue jobs and
   deletes them before normal capture resumes.

Suggested initial TTL behavior:

- default TTL: minutes, not days;
- hard maximum: short enough that queued images cannot become a screenshot
  archive;
- retry budget: small and bounded;
- image deletion: required after success, invalid output, local model failure,
  permission/protected-surface detection after partial capture, or TTL expiry.

The implementation must prove deletion with local tests and manual validation
before policy depends on screen-derived evidence.

## Local OCR/Vision Result Contract

The final Effect Schema contract should include these concepts:

- `screenAnalysisResultId`.
- `schemaVersion`.
- `queueJobId`.
- `analyzedAt`.
- `modelRuntimeRef`, `modelId`, `providerKind`, and `promptOrTemplateVersion`.
- `captureReason`, `captureScope`, and `capabilityStatus`.
- `summary`: concise local text summary, bounded in length.
- `visibleCategoryCandidates`: array of typed categories with confidence in
  `0..1`.
- `primaryCategory`: optional category chosen only when confidence and evidence
  support it.
- `riskSignals`: typed signals with confidence in `0..1` and evidence refs.
- `ocrTextSnippets`: optional bounded snippets with local redaction status when
  parent settings allow snippet storage.
- `redactionNotes`: typed notes such as credential-like text redacted,
  protected region skipped, OCR disabled, or no text extracted.
- `confidence`: overall confidence in `0..1`.
- `uncertaintyReason`: low confidence, ambiguous image, unsupported language,
  protected surface, model unavailable, insufficient pixels, or unknown.
- `sourceEvidenceRefs`: foreground app/window, browser URL/tab, app/game
  session, network digest, queue job, and capability status refs.
- `imageDigest`.
- `rawImageRetained`: false for this milestone.
- `imageDeletionState`: deleted, deletePending, deleteFailed, expiredDeleted, or
  unavailableNoImage.
- `custodyState`: child-device-journal or child-device-query-store for stored
  summaries.
- `policyEligible`: false when invalid, failed, expired, low-confidence below
  parent threshold, protected, unavailable, or missing required evidence refs.

Validation rules:

- Confidence fields must decode only in the inclusive `0..1` range.
- Categories and risk signals must be enum-backed, not free text.
- Source evidence refs are required for policy-eligible results.
- OCR snippets are optional and bounded; disabling OCR text must still allow a
  category-only summary.
- Raw image references must not appear in stored result payloads or portal
  copy/debug output.
- Invalid model output produces a typed invalid result and cannot drive policy
  or enforcement.

## Evidence References

Screen analysis should cite existing evidence instead of replacing it:

- process/window evidence for foreground app and window state;
- browser URL/tab evidence when managed browser state is known;
- app/game session evidence for native app/game context;
- network flow digest evidence for unusual network triggers;
- local model runtime status;
- parent setting version and policy version;
- queue job lifecycle and deletion evidence.

Screen summaries are one evidence layer. They should be interpreted together
with browser, app/game, network, time-window, local AI, and parent-rule evidence.
They must not be the sole proof for exact URL, browser tab, network destination,
or app/game duration.

## Portal Behavior

The portal should show:

- whether screen analysis is enabled, disabled, observe-only, dry-run, or
  enforcement-eligible;
- who enabled or changed it, when, and the current setting version;
- cadence, triggers, strict mode, OCR snippet setting, redaction mode, TTL, and
  deletion behavior;
- capability state and unavailable/permission-limited/protected-surface reason;
- recent summaries with category candidates, confidence, risk signals, source
  evidence refs, custody labels, and deletion state;
- queue health: pending count, expired count, delete-pending/delete-failed
  count, model unavailable, and last successful analysis time;
- policy dry-run or policy decision ids that consumed a screen summary;
- copy/debug output with ids, timestamps, setting version, source ids,
  capability state, custody state, and redacted local references.

The portal must not:

- capture screenshots;
- run OCR/vision or policy evaluation;
- read raw queue files, journals, SQLite files, or model files directly;
- display raw screenshots by default;
- imply that Ocentra cloud is storing or processing child screen activity;
- hide disabled, unavailable, stale, permission-limited, protected, expired,
  invalid, or delete-failed states.

## Local AI And Policy Handoff

Local AI may consume screen-analysis summaries after they are journaled and
schema-valid. It may not consume raw screenshots unless a later local-only model
contract explicitly keeps that processing inside the child-device queue worker.

Policy input should include:

- screen analysis result id;
- source evidence refs;
- visible categories and confidence;
- risk signals and confidence;
- parent rule refs and policy version;
- custody and deletion state;
- degraded/unavailable state;
- model runtime status ref.

Policy rules should support:

- visible activity category targets;
- screen-derived risk signal targets;
- minimum confidence threshold;
- unknown/low-confidence fallback;
- protected/unavailable fallback;
- observe-only and dry-run modes before enforcement;
- evidence-required enforcement rules.

Enforcement can act only after deterministic policy produces a typed decision
such as allow, warn, block, time-limit, ask-parent, or unknown. Raw model output,
OCR text, image pixels, queue status, or category labels alone cannot enforce.

## Hosted And Remote Boundaries

Allowed Ocentra-hosted involvement for this slice:

- account, entitlement, release, update, pairing, and minimal notification route
  metadata;
- hosted docs or download pages;
- future stateless report compilation only after parent-authorized input and
  no-retention behavior are separately specified.

Disallowed by default:

- screenshot upload for OCR/vision;
- screenshot retention;
- storing screen summaries as child activity history;
- storing parent rules as the source of truth;
- storing SQLite evidence, encrypted journals, reports, or local AI decisions;
- using remote/API AI for child-device blocking, timing, or ask-parent
  decisions.

Notifications should carry minimal detail, such as "Review needed on child
device", and link the parent back to authenticated local/LAN, parent-cache, or
parent-owned-storage surfaces for sensitive context.

## Failure Behavior

- Disabled setting: record disabled status and do nothing.
- Permission required or limited: record capability status and show setup/help
  state; do not capture.
- Protected surface: skip capture or delete partial capture and record a
  protected-surface result.
- Queue unavailable: record health failure; do not fall back to unencrypted
  images.
- Encryption failure: fail closed and record degraded queue status.
- Local model unavailable: retry within TTL, then delete image and record
  failed/expired state.
- Invalid JSON: reject the result, delete the image, and record invalid output.
- Low confidence: store summary as low-confidence and make policy eligibility
  depend on parent thresholds.
- Delete failure: surface delete-pending/delete-failed health, retry deletion,
  and prevent the queue from becoming long-term retention.
- Ocentra service unavailable: local capture, storage, deletion, AI, and policy
  continue or degrade locally without uploading child activity.

## Journal And SQLite Flow

Screen evidence follows the local evidence path:

```text
capture setting and capability
  -> queue lifecycle event
  -> encrypted temporary image
  -> local analysis result
  -> queue deletion event
  -> encrypted journal
  -> SQLite screen summary read model
  -> portal, local AI, and policy references
```

SQLite read models should be rebuildable from the journal and expose:

- effective parent screen-analysis settings;
- latest capability and local model status;
- recent screen summaries;
- category and risk-signal rollups;
- queue health and deletion health;
- unavailable, stale, expired, failed, invalid, and protected-surface counts;
- policy and AI evidence references;
- source/custody labels.

Portal and local AI paths must not read raw queue files, raw journal files, raw
SQLite files, screenshots, or OS capture APIs directly.

## Implementation Phases

Phase 0, this spec:

- Add architecture and acceptance plan.
- Update expectation and roadmap links.
- Do not implement runtime feature code.

Phase 1, contracts:

- Add TypeScript Effect Schema contracts for settings, capability, queue job,
  queue lifecycle, local OCR/vision result, deletion state, read models, policy
  references, and custody/source states.
- Add Rust protocol structs only after TypeScript contracts and tests exist.
- Include invalid confidence, missing evidence refs, unavailable states, and
  deletion states in tests.

Phase 2, encrypted temp queue:

- Add a child-device encrypted temporary queue with TTL, retry budget, startup
  cleanup, deletion lifecycle, and redacted diagnostics.
- Prove the queue cannot store plaintext images at rest.

Phase 3, capture scheduler and adapter:

- Add parent setting validation and capability status.
- Add bounded cadence and trigger scheduling.
- Add platform capture adapter only behind disabled-by-default settings and
  protected-surface checks.

Phase 4, local OCR/vision worker:

- Add local model/runtime status.
- Analyze queued jobs locally.
- Validate JSON and delete images after success, failure, invalid output, or TTL
  expiry.

Phase 5, journal and read models:

- Store summaries, evidence refs, image digest, model/runtime ref, and deletion
  state in the encrypted journal.
- Rebuild SQLite read models from journal replay.

Phase 6, portal visibility:

- Show settings, capability, queue health, summaries, confidence, source refs,
  custody, and deletion states from service read models.
- Keep raw images out of the portal by default.

Phase 7, policy dry-run and enforcement handoff:

- Allow policy to consume screen summaries in dry-run.
- Require typed policy decisions before enforcement.
- Keep enforcement disabled unless explicit parent settings and policy allow it.

## Validation Plan

Contract tests:

- Decode valid and invalid parent screen-analysis settings.
- Reject cadence, TTL, retry, snippet length, and confidence values outside
  configured bounds.
- Decode valid queue jobs and lifecycle states.
- Reject queue jobs without deletion-required state.
- Decode valid OCR/vision results with categories, risk signals, confidence,
  evidence refs, custody, and deletion state.
- Reject raw image refs in stored summary payloads.
- Preserve disabled, unavailable, permission-limited, protected-surface,
  expired, failed, invalid, low-confidence, and stale states.

Rust/queue tests:

- Queue images are encrypted before durable storage.
- TTL expiry deletes queued images.
- Successful analysis deletes queued images.
- Invalid model output deletes queued images.
- Startup cleanup deletes expired or delete-pending images.
- Delete failures become typed health states.
- Queue diagnostics redact local paths and image refs.

Rust/adapter tests:

- Capture refuses to run when disabled, unsupported, permission-limited,
  protected, locked, or outside parent schedule.
- Capture uses the requested scope and records capability status.
- Trigger debouncing prevents unbounded queue growth.
- Adapter errors become typed degraded states.

Storage tests:

- Screen summaries and lifecycle events write to the encrypted journal.
- SQLite screen read models rebuild from journal replay.
- Duplicate result ids do not double-count rollups.
- Source/custody labels and deletion states survive replay.

Local AI and policy tests:

- Local AI context builder consumes only schema-valid summaries and evidence
  refs.
- Low-confidence or invalid screen results cannot drive enforcement.
- Parent confidence thresholds and unknown fallbacks are deterministic.
- Enforcement handoff requires a typed policy decision and evidence refs.

Portal tests:

- Current settings, who changed them, cadence, triggers, mode, and deletion
  behavior are visible.
- Unavailable, permission-limited, protected, stale, expired, failed, invalid,
  delete-pending, and delete-failed states are visible.
- Summaries show confidence, categories, risk signals, evidence refs, custody,
  and deletion state.
- Copy/debug output redacts raw image references and local paths.
- Hosted/non-activity metadata is not shown as child screen-activity storage.

Manual Windows validation:

1. Start the agent with screen analysis disabled and confirm no capture occurs.
2. Enable observe-only screen analysis with a short test cadence as a parent
   setting.
3. Confirm a queue job is encrypted locally and carries TTL, setting version,
   source refs, and custody state.
4. Run local OCR/vision analysis and confirm the stored result has categories,
   confidence in `0..1`, evidence refs, image digest, and deletion state.
5. Confirm the temporary image is deleted after success.
6. Force model unavailable or invalid JSON and confirm expiry/deletion plus
   non-policy-eligible state.
7. Force permission-limited or protected-surface state and confirm no raw image
   remains.
8. Confirm journal replay rebuilds SQLite summaries and deletion status.
9. Confirm the portal shows settings, status, summary, custody, and deletion
   state without displaying raw screenshots by default.
10. Confirm no screenshot or screen summary is uploaded to Ocentra-hosted
    services.

## Done Signal

V0.5.3 local screen evidence planning is done when the repo has a clear
architecture and acceptance plan for parent opt-in settings, encrypted temporary
image queueing, TTL and deletion behavior, local OCR/vision result schemas,
confidence in `0..1`, evidence refs, local-first custody, unavailable and
permission-limited states, portal/policy/AI handoff, validation coverage, and no
default Ocentra cloud storage or remote processing of child screen activity.
