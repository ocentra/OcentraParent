# Screen Plan Implementation Checklist

A checkbox may be marked `[x]` only after the matching proof pack exists.

Expected proof pack path:

```text
output/screen-plan-proof/<workpack-id>/
```

## Current Branch Proof Snapshot

These entries summarize proof already produced on `main` or the current
screen-AI stack. They do not claim product-complete UI, authenticated-account
social proof, production VLM quality, broad adapters, or D-lane managed-browser
trigger ownership unless the row explicitly says so.

| Proof                                      | Status                      | Artifact                                                                                           | Non-claim                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------------------ | --------------------------- | -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Windows active-window adapter capture      | P3 proved                   | `output/screen-plan-proof/real-capture/manual-parent-test-active-window/proof-summary.json`        | Windows local host only; macOS, iOS, Android physical parity, and broader Linux compositor proof still need real platform proof.                                                                                                                                                                                                                                                                                          |
| Windows scope-matrix adapter capture       | P3 proved                   | `output/screen-plan-proof/real-capture/scope-matrix/proof-summary.json`                            | Proves active-window, selected-window, and opt-in primary-display adapter scopes; does not claim parent setting wiring.                                                                                                                                                                                                                                                                                                   |
| Desktop adapter path                       | P2 implementation path      | `crates/screen-capture-adapter/src/lib.rs`                                                         | Windows/macOS use `xcap`; Linux uses a real X11 command backend. Live macOS and Linux Wayland/root-display proof still must run before those platform claims are complete.                                                                                                                                                                                                                                                |
| Linux WSLg selected-window capture         | P3 local WSLg proved        | `output/screen-plan-proof/linux-wslg/proof-summary.json`                                           | Proves WSLg/X11 selected-window capture only; does not claim WSLg root display, native Wayland portal, or broad Linux compositor parity.                                                                                                                                                                                                                                                                                  |
| Android MediaProjection emulator capture   | P3 local emulator proved    | `output/screen-plan-proof/android-mediaprojection/proof-summary.json`                              | Proves explicit OS consent, foreground-service capture, frame digest, and raw-temp deletion on Android API 35 emulator only; physical-device parity and silent background capture are not claimed.                                                                                                                                                                                                                        |
| Encrypted temporary queue custody          | P3 proved                   | `output/screen-plan-proof/real-capture/manual-parent-test-active-window/03-encrypted-queue.ndjson` | Remaining implementation task: service scheduler integration; this local harness proof is not that claim.                                                                                                                                                                                                                                                                                                                 |
| Raw image delete-after-success             | P3 proved                   | `output/screen-plan-proof/real-capture/manual-parent-test-active-window/04-deletion-proof.json`    | TTL expiry and delete-failed surfacing remain.                                                                                                                                                                                                                                                                                                                                                                            |
| Browser-window scheduler trigger           | P3 proved                   | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                          | Runs the Rust trigger scheduler before capture; does not claim D-lane managed browser URL-change integration.                                                                                                                                                                                                                                                                                                             |
| Native app foreground scheduler trigger    | P3 proved                   | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                          | Proves a real Windows Notepad foreground-window capture through the scheduler; service-owned foreground watcher wiring remains.                                                                                                                                                                                                                                                                                           |
| Timed two-frame scheduler cadence          | P3 proved                   | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                          | Proves cadence-due scheduler decisions and two real selected-window captures; service timer loop and disabled-setting stop remain.                                                                                                                                                                                                                                                                                        |
| Service disabled-setting suppression       | P3 proved                   | `output/screen-ai-pipeline-proof/service-disabled-suppression/proof-summary.json`                  | Proves no new captures, queue jobs, or local vision rows while disabled; product settings UI remains.                                                                                                                                                                                                                                                                                                                     |
| Service cadence/read-model loop            | P3 proved                   | `output/screen-ai-pipeline-proof/service-cadence/proof-summary.json`                               | Proves three real Windows cadence captures and Activity Screen rows over WebSocket; provider is service metadata, not VLM quality.                                                                                                                                                                                                                                                                                        |
| Service analysis queue drain               | P3 proved                   | `output/screen-ai-pipeline-proof/service-analysis/proof-summary.json`                              | Proves service capture-to-analysis plumbing with a proof adapter; production VLM quality remains.                                                                                                                                                                                                                                                                                                                         |
| Retention sweeper deletion visibility      | P3 proved                   | `output/screen-ai-pipeline-proof/service-retention-sweeper/proof-summary.json`                     | Proves expired queue removal and visible `expiredDeleted` rows; parent retention-duration UI and cloud retention policy remain.                                                                                                                                                                                                                                                                                           |
| Live operator capture-to-AI matrix         | P3 proved                   | `output/screen-ai-pipeline-proof/live-operator/proof-summary.json`                                 | Proves nine real URL/app scenarios through capture, local VLM, schema validation, policy dry-run, and deletion; authenticated-account social proof remains.                                                                                                                                                                                                                                                               |
| Portal screen read-model proof             | P3 proved                   | `output/screen-ai-pipeline-proof/portal-chain/proof-summary.json`                                  | Proves service-backed screen rows render in the portal; product-complete UX and broad adapters remain.                                                                                                                                                                                                                                                                                                                    |
| Settings route screen catalog proof        | P3 proved                   | `output/screen-plan-proof/settings-ui/proof-summary.json`                                          | Proves the real portal Settings route renders read-only Screen settings/capability catalog proof; writable opt-in and retention controls remain.                                                                                                                                                                                                                                                                          |
| Settings route writable intent proof       | P3 proved                   | `output/screen-plan-proof/settings-writable-controls/proof-summary.json`                           | Proves the real portal Settings route controls build schema-valid disabled, observe-only, and strict dry-run local screen-summary drafts; service persistence remains.                                                                                                                                                                                                                                                    |
| Remote/retention/live boundary proof       | P2 contract proved          | `output/screen-plan-proof/remote-retention-boundary/proof-summary.json`                            | Proves raw screenshot retention, live view, and raw remote upload are disabled in the local-summary boundary; no live transport or writable retention UI claimed.                                                                                                                                                                                                                                                         |
| Optional retention/live preflight          | P2 contract proved          | `output/screen-plan-proof/27-28-optional-retention-live-preflight/proof-summary.json`              | Proves separate explicit opt-in raw-retention/live-view modes with approval, audit, custody, TTL/delete/no-retention, platform-proof gates, and no remote input; no runtime transport claimed.                                                                                                                                                                                                                            |
| Detector prompt pack proof                 | P2 contract proved          | `output/screen-plan-proof/40-detector-prompt-packs-and-schema-tests/proof-summary.json`            | Proves guided detector prompt pack/output contracts reject open-ended prompts, raw/private fields, policy authority, and enforcement claims; no production model quality or live inference claimed.                                                                                                                                                                                                                       |
| Local AI resource scheduler proof          | P2 contract + runtime       | `output/screen-plan-proof/local-ai-resource-scheduler/proof-summary.json`                          | Proves screen OCR/VLM priority, singleton heavy-lane admission, timeout/skipped/degraded states, caps, and reuse of the provider scheduler proof; not a final capture-to-policy pipeline claim.                                                                                                                                                                                                                           |
| Household mesh precursor screen route      | P2 contract proved          | `output/screen-plan-proof/37-family-ai-hub-screen-analysis-queue/proof-summary.json`               | Proves hard screen-analysis cases route child-local first, then local household-provider execution with no retention and no remote/API fallback; no decentralized claim/lease mesh, physical LAN runtime, or production model quality claimed.                                                                                                                                                                            |
| Screen eventing consumer boundary          | P2 runtime proof            | `output/screen-plan-proof/screen-eventing-consumer-boundary/proof-summary.json`                    | Proves an ordered typed `ocentra-eventing` runtime chain for capture, encrypted queue, AI request/result, summary, policy, action dry-run, deletion, and portal-read-model events without raw-image escape. Live service producers/subscribers and household mesh execution remain separate gates.                                                                                                                        |
| Screen service event bridge                | P2 service bridge proof     | `output/screen-plan-proof/screen-service-event-bridge/proof-summary.json`                          | Proves service Activity Screen read-model rows map into the existing typed screen event chain, reject raw retention and missing policy refs before publication, and reuse the core event path without a duplicate service event bus. Always-on production subscriptions remain separate.                                                                                                                                  |
| Screen service event subscription          | P2 service subscriber proof | `output/screen-plan-proof/screen-service-event-subscription/proof-summary.json`                    | Proves a service-owned `screen.service.row.ready` subscriber consumes typed Activity Screen rows, invokes the existing bridge, records accepted/rejected dispatch state, publishes downstream screen runtime events for safe rows, and rejects raw-retained rows before downstream publication. Startup wiring remains separate.                                                                                          |
| Screen service analysis row-ready producer | P2 service producer proof   | `output/screen-plan-proof/screen-service-analysis-row-ready/proof-summary.json`                    | Proves the service analysis runtime starts the row-ready event runtime, converts recorded Activity Screen analysis results through the shared row mapper, publishes `screen.service.row.ready`, and gates rows missing policy refs before downstream policy/action publication. Remaining live producers and policy-ref producer wiring remain separate.                                                                  |
| Screen service policy-ref producer         | P2 service producer proof   | `output/screen-plan-proof/screen-service-policy-ref-producer/proof-summary.json`                   | Proves the Rust service event-record producer writes dry-run policy refs for policy-eligible analysis rows before row-ready publication and does not fabricate refs for non-policy-eligible rows. Broad parent-rule compiler coverage, final enforcement, and new live external capture remain separate.                                                                                                                  |
| Screen service capture event producer      | P2 service producer proof   | `output/screen-plan-proof/screen-service-capture-event-producer/proof-summary.json`                | Proves the service cadence and native foreground capture loops publish typed capture-observed and encrypted-queue events after encrypted queue handoff through the existing screen event runtime. Retention sweeper deletion event publication, new live external capture, final enforcement, and model quality remain separate.                                                                                          |
| Screen service deletion event producer     | P2 service producer proof   | `output/screen-plan-proof/screen-service-deletion-event-producer/proof-summary.json`               | Proves the service retention sweeper publishes typed deletion-committed events after expired encrypted queue deletion through the existing screen event runtime without fabricating AI, policy, or action refs. Parent retention UI persistence, final enforcement, and model quality remain separate.                                                                                                                    |
| Screen child disclosure UX                 | P3 rendered UI proof        | `output/screen-plan-proof/screen-child-disclosure/proof-summary.json`                              | Proves child-visible disabled, paused, active capture, protected-surface, and deleted-summary states with calm copy, renders desktop/mobile disclosure screenshots, and rejects hidden capture, raw screenshot display, remote viewer, and policy-authority claims. Child-agent deployment/delivery remains separate.                                                                                                     |
| Screen enforcement handoff guard           | P2 contract proof           | `output/screen-plan-proof/screen-ai-enforcement-handoff-guard/proof-summary.json`                  | Proves screen-derived enforcement handoff payloads require dry-run policy, summary/local-AI/audit refs, enabled parent rule, confidence state, and an audit event while rejecting raw pixels, raw model text, retained screenshot, and local-AI authority claims. Adapter execution and broad/browser/network/mobile enforcement remain separate.                                                                         |
| Screen parent portal summary UI            | P3 real portal UI proof     | `output/screen-plan-proof/screen-parent-portal-summary-ui/proof-summary.json`                      | Proves the dedicated Screen Analysis route consumes service-backed Activity Screen read-model rows from the real portal/agent command path, exposes capability, queue, summary/category, confidence, model/runtime, custody/deletion, policy, audit/evidence, and not-claimed enforcement details, and captures desktop/mobile screenshots under `output/screen-plan-proof/screen-parent-portal-summary-ui/screenshots/`. |
| Router/structured extraction proof         | P2 contract proved          | `output/screen-plan-proof/31-32-screen-router-structured-extraction/proof-summary.json`            | Proves typed screen routing checks existing evidence and managed-browser structured extraction before screenshots; no live producer, portal, policy, or enforcement claim.                                                                                                                                                                                                                                                |
| Managed-browser CDP screenshot capture     | P3 live local proved        | `output/screen-plan-proof/33-managed-browser-cdp-screenshot-capture-path/proof-summary.json`       | Proves real Chromium CDP page/viewport/crop screenshot capture tied to a managed page target with encrypted temp queue handoff and deletion; no URL-trigger ownership, OCR/VLM, policy, enforcement, live view, or raw retention claim.                                                                                                                                                                                   |
| Windows WinRT OCR worker capture analysis  | P3 real OCR proved          | `output/ai-plan-proof/screen-winrt-ocr-worker/proof-summary.json`                                  | Proves real selected-window browser/native captured pixels run through Windows WinRT OCR, become typed screen-analysis evidence, feed allow dry-run policy decisions, and delete raw temp images; no production OCR quality, service runtime, cross-platform, enforcement, live view, or raw retention claim.                                                                                                             |
| Windows service WinRT OCR runtime          | P3 real OCR service proved  | `output/screen-ai-pipeline-proof/service-winrt-ocr/proof-summary.json`                             | Proves timed cadence service capture from a live public Wikipedia Chrome window through the encrypted queue, Windows WinRT OCR, `localOcr` Activity Screen read model, queue drain, and temp image deletion; no production OCR quality, authenticated/social, cross-platform, enforcement, live view, or raw retention claim.                                                                                             |
| Final screen-AI product path gate          | P3 stacked proof proved     | `output/screen-ai-pipeline-proof/final-product-path/proof-summary.json`                            | Validates retained real-run artifacts from live/operator trigger rows through capture, local VLM/OCR analysis, policy dry-run, Windows action handoff, portal/read model, and deletion/custody proof; it does not rerun the live operator session or claim managed-browser trigger ownership/broad adapters.                                                                                                              |

## Planned Household Mesh Screen Custody Rows

These rows are planned only. They do not change the status of the existing
legacy household-provider route or runtime-discovery proofs.

| Proof                                 | Status           | Artifact                                                                      | Non-claim                                                                                                                                                                                                             |
| ------------------------------------- | ---------------- | ----------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Household mesh screen payload custody | P2 runtime proof | `output/ai-plan-proof/no-raw-screen-transfer-mesh/proof-summary.json`         | Proves screen-derived household provider work uses redacted summary/custody refs instead of raw screenshot transfer. Physical LAN provider execution and production mesh transport remain separate.                   |
| Household mesh screen claim/lease     | P2 runtime proof | `output/screen-ai-pipeline-proof/household-mesh-screen-ai/proof-summary.json` | Proves provider claim/lease, child-agent result validation, duplicate/expired/wrong-provider/wrong-claim/evidence/custody/raw-transfer/authority rejection, and custody before policy. Physical LAN remains separate. |

## Main Gates

- [x] Screen analysis starts disabled.
- [x] Parent opt-in setting exists and is auditable.
- [x] MVP scope is capture/routing first, AI model-quality proof second.
- [x] Capture cadence and triggers are parent-controlled.
- [x] Capture scope is parent-controlled and platform-gated.
- [x] Real browser-use trigger proof exists, not only contract tests.
- [x] Real app-use trigger proof exists, not only contract tests.
- [x] Timed cadence proof captures multiple bounded frames and stops after disable.
- [x] Capability status exists before capture.
- [x] Screen evidence is treated as cross-slice, not browser-only.
- [x] Existing browser/app/game/network/session evidence is checked before capture.
- [x] Managed browser structured extraction runs before managed-browser screenshots.
- [x] Managed browser CDP screenshot capture is page-scoped and never becomes desktop capture.
- [x] OCR runs before VLM when text can answer the question.
- [x] Guided detector prompts replace open-ended screen descriptions.
- [x] A capable configured local model may be used after route/cost/privacy proof.
- [x] Protected surfaces are skipped.
- [x] Temporary image queue is encrypted.
- [x] Raw image path is redacted outside child agent.
- [x] Local OCR/vision returns schema-valid JSON only.
- [x] Invalid model output cannot drive policy.
- [x] Summary writes to journal/SQLite.
- [x] Raw image deleted after success.
- [x] Raw image deleted after expiry.
- [x] Deletion state visible.
- [x] Policy consumes summary/evidence refs only.
- [x] Portal does not show raw screenshots by default.
- [x] Screenshot retention is separate opt-in mode.
- [x] Live view is separate opt-in mode.
- [x] Remote/cloud screenshot upload disabled by default.
- [x] Remote/API path accepts only parent-approved redacted summaries by default.
- [x] Local AI resource scheduler prevents multiple heavy jobs on normal PCs.
- [x] Local or trusted household provider route is used before remote/API for hard
      visual cases.
- [x] Screen capture, queue, deletion, and summary lifecycle transitions publish
      typed `ocentra-eventing` events before AI/policy/action consumers run.
- [x] Service Activity Screen rows bridge into the typed screen event chain
      without raw retention, missing policy refs, or duplicate event buses.
- [x] Service analysis rows publish `screen.service.row.ready` into the
      service-owned event subscriber and gate incomplete rows before downstream
      policy/action publication.
- [x] Service policy-eligible analysis rows carry dry-run policy decision,
      action, parent rule, explanation, and deletion proof refs before row-ready
      publication without fabricating refs for non-policy-eligible rows.
- [x] Service cadence and native foreground capture loops publish typed
      capture-observed and encrypted-queue events after encrypted queue handoff.
- [x] Service retention sweeper publishes typed deletion-committed events after
      expired encrypted queue deletion without fabricating policy/action refs.
- [x] Screen child disclosure contract defines child-visible disabled, paused,
      active capture, protected-surface, and deleted-summary status without
      hidden capture or raw screenshot display claims.
- [x] Screen child disclosure renderer produces desktop/mobile child-visible
      screenshots while keeping child-agent deployment/delivery unclaimed.
- [x] Screen enforcement handoff guard requires dry-run policy, summary,
      local-AI result, parent-rule, confidence, and audit refs before any
      action payload, while rejecting raw pixels/raw model text/retained
      screenshot/local-AI-authority claims.
- [x] Screen Analysis route renders a parent summary UI from the real
      portal/agent Activity Screen read-model command path with capability,
      queue, model, confidence, custody, policy, audit, evidence refs,
      not-claimed enforcement details, and desktop/mobile screenshots.
- [x] Screen-derived household provider jobs prove no raw screenshot transfer,
      claim/lease, child-agent result validation, and custody before policy.
- [x] Platform capture proof exists before platform claim.
- [x] Operator live URL/app proof is recorded before product-complete claim.
- [x] Playwright UI proof exists.
- [x] Final product-complete pipeline proof is completed in
      `docs/plans/screen-ai-pipeline-plan` after screen and AI prerequisites are
      merged or explicitly stacked.

## Required Proof Pack

```text
output/screen-plan-proof/<workpack-id>/
  00-source-snapshot.md
  01-contract-proof.log
  02-rust-protocol-proof.log
  03-platform-capability-proof.json
  04-capture-job-proof.json
  05-queue-encryption-proof.json
  06-ai-analysis-proof.json
  07-journal-sqlite-proof.json
  08-deletion-proof.json
  09-policy-dry-run-proof.json
  10-ui-snapshots/
  11-playwright-proof.log
  12-security-negative-proof.log
  13-manual-platform-proof.md
  14-validation-commands.log
  15-real-trigger-proof/
  16-operator-live-proof/
  17-cadence-proof/
```

## Merge-Blocking Failures

```text
screen capture runs while parent setting disabled
raw screenshot stored long-term by default
raw screenshot uploaded remotely by default
portal exposes raw image path
policy consumes raw AI text
policy consumes raw image
invalid AI output drives enforcement
protected surface captured
password/credential prompt captured
deleteAfterSuccess false in V0.5
expired image not deleted
delete failure hidden
live view enabled silently
screenshot retention enabled silently
remote AI receives screenshot
platform claim has no platform proof
screen work claims done without real browser-use trigger proof
screen work claims done without real app-use trigger proof
timed capture does not prove multiple bounded captures
disable setting does not stop future cadence jobs
Android silent background capture claimed
iOS arbitrary other-app background capture claimed
screen evidence treated as browser-only
VLM runs before structured/OCR evidence when not needed
open-ended "describe screen" prompt used
remote API receives raw screenshot
local heavy OCR/VLM jobs run without priority/resource guard
screen capture directly calls AI/policy/action instead of publishing typed
eventing records for consumers
service screen row creates a second event bus instead of reusing the core screen
runtime chain
service row-ready subscriber accepts raw-retained rows or bypasses the existing
bridge before downstream publication
```

## Workpack Status

| Status | Workpack                                           |
| ------ | -------------------------------------------------- |
| [x]    | 01 Source index and doc reconciliation             |
| [x]    | 02 Current screen snapshot and gap map             |
| [x]    | 03 Contract boundary and Effect schemas            |
| [x]    | 04 Parent opt-in settings contract                 |
| [x]    | 05 Capability/status contract                      |
| [x]    | 06 Capture scope model                             |
| [x]    | 07 Capture trigger model                           |
| [x]    | 08 Platform adapter abstraction                    |
| [x]    | 09 Windows capture adapter plan/proof              |
| [~]    | 10 macOS capture adapter plan/proof                |
| [~]    | 11 Linux capture adapter plan/proof                |
| [~]    | 12 Android MediaProjection adapter plan/proof      |
| [ ]    | 13 iOS ReplayKit adapter plan/proof                |
| [x]    | 14 Protected surface detector                      |
| [x]    | 15 Encrypted temporary image queue                 |
| [x]    | 16 Queue scheduler and debouncer                   |
| [~]    | 17 Local OCR/vision runtime model                  |
| [x]    | 18 Screen analysis result schema                   |
| [~]    | 19 Sensitive text and redaction model              |
| [x]    | 20 Result validator and invalid-output handling    |
| [x]    | 21 Journal and SQLite ingest                       |
| [x]    | 22 Deletion and retention proof                    |
| [x]    | 23 Policy compiler for screen-derived evidence     |
| [x]    | 24 Enforcement handoff guard                       |
| [x]    | 25 Parent portal summary UI                        |
| [x]    | 26 Child disclosure UX                             |
| [x]    | 27 Screenshot retention optional mode              |
| [~]    | 28 Live view optional mode                         |
| [x]    | 29 Proof tiers and proof packs                     |
| [~]    | 30 Test suite, Playwright, rollout, PR gate        |
| [x]    | 31 Screen intelligence router                      |
| [x]    | 32 Browser structured extraction before screenshot |
| [x]    | 33 Managed browser CDP screenshot capture path     |
| [~]    | 34 OCR Tesseract baseline                          |
| [~]    | 35 OCR PaddleOCR/PP-OCR evaluation                 |
| [~]    | 36 Small VLM guided classifier evaluation          |
| [x]    | 37 Household mesh screen-analysis queue            |
| [x]    | 38 Local AI resource scheduler/priority queue      |
| [x]    | 39 Redacted summary-only remote boundary           |
| [x]    | 40 Detector prompt packs and schema tests          |
