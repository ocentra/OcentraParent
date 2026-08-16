# Screen Plan

This folder is the single working plan location for local screen evidence, screenshot capture, OCR/vision summaries, temporary encrypted image queue, deletion proof, screen-derived policy evidence, optional screenshots, optional live view, and screen-related UI/UX.

## Source Docs

- `docs/features/screen-evidence-analysis.md`
- `docs/features/screen-visibility-live-view.md`
- `docs/expectations/screen-evidence.md`
- `docs/architecture/local-screen-evidence-analysis-queue.md`
- `docs/screen-evidence-analysis-schema-proposal.md`
- `docs/screen-evidence-analysis-capability-guide.md`
- `docs/screen-control-settings-inventory.md`

## Core Rule

```text
Screen evidence is opt-in.
Screen capture starts disabled.
Raw images stay local by default.
Temporary images are encrypted while queued.
Local OCR/vision produces typed summaries.
Rust validates AI output before storage/policy use.
Policy consumes summaries and evidence refs, not raw screenshots.
Temporary images are deleted after success or TTL expiry.
Raw screenshot retention and live view are separate opt-in product modes.
Remote/cloud screenshot upload is not default.
AI is evidence, not authority.
Parent policy decides action.
Screen capture does not directly call AI or policy.
Screen capture publishes typed evidence/custody events into the Rust eventing
runtime, and AI/policy/action consumers subscribe through that boundary.
```

## Tiered Screen Intelligence Rule

Do not make VLM the first step.

Use this order:

```text
0. Existing evidence:
   managed URL, app foreground, title, domain, process, network digest.

1. Browser/app structured extraction:
   DOM/title/meta/URL/process/session/platform parser.

2. Targeted OCR:
   only crop/regions or low-res screenshot;
   extract small snippets, not full screen dump.

3. Guided VLM:
   yes/no/multi-label classification from the smallest safe image.

4. Bigger local or household-provider VLM:
   only for hard cases when cheap local evidence is uncertain;
   route through event contracts, not a direct capture-to-model call. Household
   provider execution uses the AI plan's Household Mesh Bridge with child-agent
   claim/lease/result validation and no raw screenshot transfer by default.

5. Remote/API VLM:
   disabled for raw screenshots by default;
   parent-approved redacted summaries only.
```

This does not mean Ocentra should avoid a capable chosen local model. If the parent/device default model, currently expected to be Gemma-family in local AI planning, supports the needed image task with acceptable local performance and privacy proof, it can be used for the guided VLM step. The rule is that capture and cheaper structured/OCR evidence come first when they can answer the question.

## MVP Focus

The first screen-plan MVP is capture and evidence routing:

- prove capture scopes;
- prove managed browser capture path;
- prove active/selected app-window capture path;
- prove native game/app/launcher/unknown-process trigger routing;
- prove encrypted temporary image queue and deletion;
- align AI processing contracts enough that OCR/VLM can plug in later;
- defer full model selection and model-quality proof to the AI plan or a dedicated second screen AI pass.

## Cross-Slice Ownership Rule

Screen evidence is a shared local visual evidence layer for browser, apps, native games, browser games, social/video, bypass tools, unknown activity, and tracking/check-in context when needed.

It must not be owned only by browser-plan.

- Browser-plan may use managed browser page/window capture.
- App/game plan may use active-window or selected app-window capture.
- Social/video/game policies may consume screen summaries.
- Raw screen images remain local, temporary, encrypted, and deleted.

Final architecture:

```text
App/Game/Browser evidence says what surface is active.
Screen evidence helps understand what is visible.
Screen publishes approved capture/evidence events.
AI subscribers summarize only from approved evidence refs.
Policy subscribers decide what to do from validated AI result events.
Action, audit, read-model, and deletion consumers react after policy.
```

## Where We Are

- Feature docs exist for screen evidence and screen visibility/live view.
- Expectation/spec exists for screen evidence.
- Architecture exists for local screen analysis queue.
- Generated/design docs exist for schema, capability, and screen control inventory.
- Code foundation exists in `packages/activity-domain`, `crates/agent-protocol`, `crates/agent-core`, `crates/agent-service`, `packages/portal-domain`, and `apps/portal`.
- Portal/read-model plumbing exists through activity surface and route hooks.

But the product is not complete.

Known gaps:

- Parent opt-in UI.
- Capability/status UI.
- Real platform capture adapters.
- Capture cadence proof.
- OCR/vision runtime quality proof.
- Encrypted queue proof.
- Deletion proof.
- Confidence threshold policy.
- Parent explanation UX.
- Raw screenshot retention mode decision.
- Live view mode decision.
- Platform proof.
- Playwright proof.
- Privacy/legal review.

## Where We Want To Be

Ocentra needs a screen subsystem that can:

- represent parent opt-in settings before any capture runs;
- report capability/status per platform and capture scope;
- capture only approved scope: full screen, display, active window, selected app window, managed browser/window, Android app-window, or unsupported;
- skip protected surfaces such as lock screen, secure desktop, credential prompts, DRM/protected content, password fields, and OS-protected surfaces;
- write temporary image jobs to encrypted local queue;
- run local OCR/vision only on queued jobs;
- validate AI/model output against schema;
- store typed summaries, categories, risk signals, confidence, evidence refs, image digest, model runtime, and deletion state;
- delete raw images after success or TTL expiry;
- expose portal summaries and evidence, not raw screenshots by default;
- allow policy dry-run/enforcement only from validated summaries and parent rules;
- prove retention/delete behavior;
- keep screenshot retention and live view as separate explicit modes;
- keep remote relay/cloud upload disabled by default.
- emit screen capture, queue, deletion, and summary lifecycle transitions as
  typed Rust eventing consumers on `crates/ocentra-eventing`, without creating a
  second Screen-only event bus.
- treat household AI providers as worker-only execution routes for bounded
  screen-derived AI jobs; the child agent still owns evidence truth, result
  validation, policy authority, and deletion/custody proof.

## Plan Files

- [Source Index](source-index.md)
- [Current Screen Snapshot](current-screen-snapshot.md)
- [V0.5 Screen Evidence Full Scope Plan](v0-5-screen-evidence-full-scope-plan.md)
- [V0.5 Screen Platform Deep Dive](v0-5-screen-platform-deep-dive.md)
- [V0.5 Screen Intelligence Router Plan](v0-5-screen-intelligence-router-plan.md)
- [V0.5 Screen AI Analysis Plan](v0-5-screen-ai-analysis-plan.md)
- [V0.5 Real Capture Proof Matrix](v0-5-real-capture-proof-matrix.md)
- [V0.5 Screen Visibility And Live View Plan](v0-5-screen-visibility-live-view-plan.md)
- [V0.5 Screen Test Blueprint](v0-5-screen-test-blueprint.md)
- [UI/UX Requirements Guide](ui-ux-requirements-guide.md)
- [Implementation Checklist](implementation-checklist.md)
- [Pasted Content Coverage Audit](pasted-content-coverage-audit.md)
- Final post-screen/post-AI integration:
  [Screen AI Pipeline Plan](../screen-ai-pipeline-plan/README.md)

## Workpack Checklist

| Step | Workpack                                                                                                           | Target State                                                                                                                                  |
| ---- | ------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 01   | [Source Index And Doc Reconciliation](workpacks/01-source-index-and-doc-reconciliation.md)                         | Existing feature, expectation, architecture, schema, capability, inventory, AI, custody, policy docs are source-indexed.                      |
| 02   | [Current Screen Snapshot And Gap Map](workpacks/02-current-screen-snapshot-and-gap-map.md)                         | Existing code/docs/proof are mapped against missing product work.                                                                             |
| 03   | [Contract Boundary And Effect Schemas](workpacks/03-contract-boundary-and-effect-schemas.md)                       | Settings, capability, queue, capture job, analysis result, model status, deletion, policy target, and read-model contracts are schema-backed. |
| 04   | [Parent Opt-In Settings Contract](workpacks/04-parent-opt-in-settings-contract.md)                                 | Disabled-by-default settings for enablement, cadence, triggers, scope, OCR snippets, redaction, TTL, retention, policy use, and audit fields. |
| 05   | [Capability Status Contract](workpacks/05-capability-status-contract.md)                                           | Disabled, unsupported, permission-required, protected-surface, screen-locked, model-unavailable, queue-unavailable, degraded, ready states.   |
| 06   | [Capture Scope Model](workpacks/06-capture-scope-model.md)                                                         | Full screen, display, active window, selected app window, managed browser/window, Android app-window, unsupported scope.                      |
| 07   | [Capture Trigger Model](workpacks/07-capture-trigger-model.md)                                                     | Cadence, foreground app change, managed browser URL change, app/game foreground start, unusual network, policy ambiguity, manual parent test. |
| 08   | [Platform Adapter Abstraction](workpacks/08-platform-adapter-abstraction.md)                                       | Capture adapter interface for Windows/macOS/Linux/Android/iOS/fake adapters with capability and proof tiers.                                  |
| 09   | [Windows Capture Adapter Plan Proof](workpacks/09-windows-capture-adapter-plan-proof.md)                           | Windows.Graphics.Capture path, picker/consent/border, display/window capture, protected/degraded states.                                      |
| 10   | [MacOS Capture Adapter Plan Proof](workpacks/10-macos-capture-adapter-plan-proof.md)                               | ScreenCaptureKit, Screen Recording permission, display/window capture, PPPC/MDM manual proof.                                                 |
| 11   | [Linux Capture Adapter Plan Proof](workpacks/11-linux-capture-adapter-plan-proof.md)                               | X11 capture, Wayland/PipeWire portal capture, compositor-specific status, manual-required states.                                             |
| 12   | [Android MediaProjection Adapter Plan Proof](workpacks/12-android-mediaprojection-adapter-plan-proof.md)           | Consent, foreground service, app-window/full-display, status chip, onStop callback, Android 14+ constraints.                                  |
| 13   | [iOS ReplayKit Adapter Plan Proof](workpacks/13-ios-replaykit-adapter-plan-proof.md)                               | ReplayKit/broadcast mode, explicit user/session capture, no arbitrary background capture claim.                                               |
| 14   | [Protected Surface Detector](workpacks/14-protected-surface-detector.md)                                           | Lock screen, secure desktop, credential prompt, password field, DRM/protected media, OS-protected surface, unsupported.                       |
| 15   | [Encrypted Temporary Image Queue](workpacks/15-encrypted-temporary-image-queue.md)                                 | Encrypted image refs, TTL, retry, digest, source refs, status, deletion required, redacted paths.                                             |
| 16   | [Queue Scheduler And Debouncer](workpacks/16-queue-scheduler-and-debouncer.md)                                     | Cadence bounds, trigger debounce, strict mode, parent schedule, capability gating, no flood.                                                  |
| 17   | [Local OCR Vision Runtime Model](workpacks/17-local-ocr-vision-runtime-model.md)                                   | Local model runtime status, OCR/vision tasks, no OS/file/network scanning, structured JSON only.                                              |
| 18   | [Screen Analysis Result Schema](workpacks/18-screen-analysis-result-schema.md)                                     | Categories, risk signals, text snippets, redaction notes, confidence, uncertainty, evidence refs, image digest, deletion state.               |
| 19   | [Sensitive Text And Redaction Model](workpacks/19-sensitive-text-and-redaction-model.md)                           | OCR snippet limits, password/credential suppression, PII redaction, parent-controlled text retention.                                         |
| 20   | [Result Validator And Invalid Output Handling](workpacks/20-result-validator-and-invalid-output-handling.md)       | Reject invalid JSON, missing refs, invalid confidence, unsupported categories, raw text overflow, malformed deletion state.                   |
| 21   | [Journal And SQLite Ingest](workpacks/21-journal-and-sqlite-ingest.md)                                             | Store summary/evidence refs/deletion state/model refs, rebuild read model from journal, no raw images.                                        |
| 22   | [Deletion And Retention Proof](workpacks/22-deletion-and-retention-proof.md)                                       | Delete after success, delete after expiry, delete-failed visible state, deletion proof refs, no silent long-term raw image.                   |
| 23   | [Policy Compiler For Screen Derived Evidence](workpacks/23-policy-compiler-for-screen-derived-evidence.md)         | Visible category/risk targets compile only from validated summaries and parent rules.                                                         |
| 24   | [Enforcement Handoff Guard](workpacks/24-enforcement-handoff-guard.md)                                             | No enforcement from raw pixels or raw AI text; dry-run and manual-required guards.                                                            |
| 25   | [Parent Portal Summary UI](workpacks/25-parent-portal-summary-ui.md)                                               | Settings, capability, queue health, summaries, confidence, source refs, deletion state, model status, audit.                                  |
| 26   | [Child Disclosure UX](workpacks/26-child-disclosure-ux.md)                                                         | Child-visible/local disclosure, parent-enabled status, calm wording, no hidden capture.                                                       |
| 27   | [Screenshot Retention Optional Mode](workpacks/27-screenshot-retention-optional-mode.md)                           | Separate opt-in raw screenshot retention design, custody, TTL, disclosure, export/delete proof.                                               |
| 28   | [Live View Optional Mode](workpacks/28-live-view-optional-mode.md)                                                 | Separate live view mode: transport, relay/cache, viewer audit, retention, platform proof, remote access boundary.                             |
| 29   | [Proof Tiers And Proof Packs](workpacks/29-proof-tiers-and-proof-packs.md)                                         | P0-P6 proof tier system, CI/fake/local/physical/authority proof artifact paths.                                                               |
| 30   | [Test Suite Playwright Rollout PR Gate](workpacks/30-test-suite-playwright-rollout-pr-gate.md)                     | Unit/integration/contract/security/E2E/UI/performance/manual tests and merge-blocking gates.                                                  |
| 31   | [Screen Intelligence Router](workpacks/31-screen-intelligence-router.md)                                           | Router chooses the cheapest safe route before capture, OCR, VLM, family hub, or remote redacted-only fallback.                                |
| 32   | [Browser Structured Extraction Before Screenshot](workpacks/32-browser-structured-extraction-before-screenshot.md) | Managed-browser URL/title/meta/DOM/accessibility signals answer first when possible.                                                          |
| 33   | [Managed Browser CDP Screenshot Capture Path](workpacks/33-managed-browser-cdp-screenshot-capture-path.md)         | CDP screenshot capture is scoped to managed browser page/window/crop and never becomes desktop capture.                                       |
| 34   | [OCR Tesseract Baseline](workpacks/34-ocr-tesseract-baseline.md)                                                   | Simple local OCR baseline is evaluated before VLM.                                                                                            |
| 35   | [OCR PaddleOCR PP-OCR Evaluation](workpacks/35-ocr-paddleocr-ppocr-evaluation.md)                                  | Preferred OCR research path evaluates PaddleOCR/PP-OCR packaging, quality, runtime, and local-only proof.                                     |
| 36   | [Small VLM Guided Classifier Evaluation](workpacks/36-small-vlm-guided-classifier-evaluation.md)                   | Small local VLM is used only for guided classification on safe crops when structured/OCR evidence is insufficient.                            |
| 37   | [Household Mesh Screen Analysis Queue](workpacks/37-family-ai-hub-screen-analysis-queue.md)                        | Heavier analysis moves to child-agent-owned local or trusted household provider work before any remote/API path.                              |
| 38   | [Local AI Resource Scheduler Priority Queue](workpacks/38-local-ai-resource-scheduler-priority-queue.md)           | One-heavy-job scheduling, OCR/VLM rate limits, and policy-priority behavior are defined.                                                      |
| 39   | [Redacted Summary Only Remote Boundary](workpacks/39-redacted-summary-only-remote-boundary.md)                     | Remote/API path is disabled for raw screenshots and allowed only for parent-approved redacted summaries.                                      |
| 40   | [Detector Prompt Packs And Schema Tests](workpacks/40-detector-prompt-packs-and-schema-tests.md)                   | Detector-specific JSON prompts replace open-ended screen descriptions.                                                                        |

## Final Quality Bar

Screen is product-credible only when:

```text
Parent can see screen analysis is disabled by default.
Parent can explicitly enable a safe mode.
The agent captures only approved scope.
Real browser-use triggers produce real capture or real structured-skip proof.
Real app-use triggers produce real capture proof.
Timed cadence produces multiple bounded captures and stops when disabled.
Images are encrypted in a temporary queue.
Local OCR/vision produces schema-valid summaries.
Raw images are deleted.
Deletion is proved.
Portal shows summaries, confidence, evidence refs, model status, and deletion state.
Policy consumes summaries only.
Optional screenshot retention and live view are separate explicit product modes.
Platform limitations are visible.
No hidden capture exists.
Screen summaries can be consumed by browser, app, game, social/video, bypass-tool, and unknown-activity policies.
The final Screen AI Pipeline Plan proves trigger -> capture -> analysis -> policy/action after screen and AI prerequisites are merged or explicitly stacked.
```

Final rule:

```text
Screen evidence is not surveillance by default.
It is local, opt-in, temporary, summarized, deleted, audited, and policy-gated.
```
