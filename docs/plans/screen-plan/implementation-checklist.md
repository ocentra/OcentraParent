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

| Proof                                    | Status                   | Artifact                                                                                           | Non-claim                                                                                                                                                                                             |
| ---------------------------------------- | ------------------------ | -------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Windows active-window adapter capture    | P3 proved                | `output/screen-plan-proof/real-capture/manual-parent-test-active-window/proof-summary.json`        | Windows local host only; macOS, iOS, Android physical parity, and broader Linux compositor proof still need real platform proof.                                                                      |
| Windows scope-matrix adapter capture     | P3 proved                | `output/screen-plan-proof/real-capture/scope-matrix/proof-summary.json`                            | Proves active-window, selected-window, and opt-in primary-display adapter scopes; does not claim parent setting wiring.                                                                               |
| Desktop adapter path                     | P2 implementation path   | `crates/screen-capture-adapter/src/lib.rs`                                                         | Windows/macOS use `xcap`; Linux uses a real X11 command backend. Live macOS and Linux Wayland/root-display proof still must run before those platform claims are complete.                            |
| Linux WSLg selected-window capture       | P3 local WSLg proved     | `output/screen-plan-proof/linux-wslg/proof-summary.json`                                           | Proves WSLg/X11 selected-window capture only; does not claim WSLg root display, native Wayland portal, or broad Linux compositor parity.                                                              |
| Android MediaProjection emulator capture | P3 local emulator proved | `output/screen-plan-proof/android-mediaprojection/proof-summary.json`                              | Proves explicit OS consent, foreground-service capture, frame digest, and raw-temp deletion on Android API 35 emulator only; physical-device parity and silent background capture are not claimed.    |
| Encrypted temporary queue custody        | P3 proved                | `output/screen-plan-proof/real-capture/manual-parent-test-active-window/03-encrypted-queue.ndjson` | Remaining implementation task: service scheduler integration; this local harness proof is not that claim.                                                                                             |
| Raw image delete-after-success           | P3 proved                | `output/screen-plan-proof/real-capture/manual-parent-test-active-window/04-deletion-proof.json`    | TTL expiry and delete-failed surfacing remain.                                                                                                                                                        |
| Browser-window scheduler trigger         | P3 proved                | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                          | Runs the Rust trigger scheduler before capture; does not claim D-lane managed browser URL-change integration.                                                                                         |
| Native app foreground scheduler trigger  | P3 proved                | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                          | Proves a real Windows Notepad foreground-window capture through the scheduler; service-owned foreground watcher wiring remains.                                                                       |
| Timed two-frame scheduler cadence        | P3 proved                | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                          | Proves cadence-due scheduler decisions and two real selected-window captures; service timer loop and disabled-setting stop remain.                                                                    |
| Service disabled-setting suppression     | P3 proved                | `output/screen-ai-pipeline-proof/service-disabled-suppression/proof-summary.json`                  | Proves no new captures, queue jobs, or local vision rows while disabled; product settings UI remains.                                                                                                 |
| Service cadence/read-model loop          | P3 proved                | `output/screen-ai-pipeline-proof/service-cadence/proof-summary.json`                               | Proves three real Windows cadence captures and Activity Screen rows over WebSocket; provider is service metadata, not VLM quality.                                                                    |
| Service analysis queue drain             | P3 proved                | `output/screen-ai-pipeline-proof/service-analysis/proof-summary.json`                              | Proves service capture-to-analysis plumbing with a proof adapter; production VLM quality remains.                                                                                                     |
| Retention sweeper deletion visibility    | P3 proved                | `output/screen-ai-pipeline-proof/service-retention-sweeper/proof-summary.json`                     | Proves expired queue removal and visible `expiredDeleted` rows; parent retention-duration UI and cloud retention policy remain.                                                                       |
| Live operator capture-to-AI matrix       | P3 proved                | `output/screen-ai-pipeline-proof/live-operator/proof-summary.json`                                 | Proves nine real URL/app scenarios through capture, local VLM, schema validation, policy dry-run, and deletion; authenticated-account social proof remains.                                           |
| Portal screen read-model proof           | P3 proved                | `output/screen-ai-pipeline-proof/portal-chain/proof-summary.json`                                  | Proves service-backed screen rows render in the portal; product-complete UX and broad adapters remain.                                                                                                |
| Settings route screen catalog proof      | P3 proved                | `output/screen-plan-proof/settings-ui/proof-summary.json`                                          | Proves the real portal Settings route renders read-only Screen settings/capability catalog proof; writable opt-in and retention controls remain.                                                      |
| Settings route writable intent proof     | P3 proved                | `output/screen-plan-proof/settings-writable-controls/proof-summary.json`                           | Proves the real portal Settings route controls build schema-valid disabled, observe-only, and strict dry-run local screen-summary drafts; service persistence remains.                                |
| Remote/retention/live boundary proof     | P2 contract proved       | `output/screen-plan-proof/remote-retention-boundary/proof-summary.json`                            | Proves raw screenshot retention, live view, and raw remote upload are disabled in the local-summary boundary; no live transport or writable retention UI claimed.                                     |
| Local AI resource scheduler proof        | P2 contract + runtime    | `output/screen-plan-proof/local-ai-resource-scheduler/proof-summary.json`                          | Proves screen OCR/VLM priority, singleton heavy-lane admission, timeout/skipped/degraded states, caps, and reuse of the provider scheduler proof; not a final capture-to-policy pipeline claim.       |
| Family AI hub screen route proof         | P2 contract proved       | `output/screen-plan-proof/37-family-ai-hub-screen-analysis-queue/proof-summary.json`               | Proves hard screen-analysis cases route child-local first, then local household family hub with no retention and no remote/API fallback; no real LAN hub runtime or production model quality claimed. |

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
- [ ] Existing browser/app/game/network/session evidence is checked before capture.
- [ ] Managed browser structured extraction runs before managed-browser screenshots.
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
- [ ] Screenshot retention is separate opt-in mode.
- [ ] Live view is separate opt-in mode.
- [x] Remote/cloud screenshot upload disabled by default.
- [x] Remote/API path accepts only parent-approved redacted summaries by default.
- [x] Local AI resource scheduler prevents multiple heavy jobs on normal PCs.
- [x] Family AI hub is used before remote/API for hard visual cases.
- [x] Platform capture proof exists before platform claim.
- [x] Operator live URL/app proof is recorded before product-complete claim.
- [x] Playwright UI proof exists.
- [ ] Final product-complete pipeline proof is completed in `docs/plans/screen-ai-pipeline-plan` after screen and AI prerequisites are merged or explicitly stacked.

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
| [~]    | 24 Enforcement handoff guard                       |
| [~]    | 25 Parent portal summary UI                        |
| [ ]    | 26 Child disclosure UX                             |
| [~]    | 27 Screenshot retention optional mode              |
| [~]    | 28 Live view optional mode                         |
| [x]    | 29 Proof tiers and proof packs                     |
| [~]    | 30 Test suite, Playwright, rollout, PR gate        |
| [~]    | 31 Screen intelligence router                      |
| [ ]    | 32 Browser structured extraction before screenshot |
| [ ]    | 33 Managed browser CDP screenshot capture path     |
| [~]    | 34 OCR Tesseract baseline                          |
| [ ]    | 35 OCR PaddleOCR/PP-OCR evaluation                 |
| [~]    | 36 Small VLM guided classifier evaluation          |
| [x]    | 37 Family AI hub screen-analysis queue             |
| [x]    | 38 Local AI resource scheduler/priority queue      |
| [x]    | 39 Redacted summary-only remote boundary           |
| [~]    | 40 Detector prompt packs and schema tests          |
