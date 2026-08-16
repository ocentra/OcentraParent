# Screen Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `screen-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the single working plan location for local screen evidence, screenshot capture, OCR/vision summaries, temporary encrypted image queue, deletion proof, screen-derived policy evidence, optional screenshots, optional live view, and screen-related UI/UX.

## Current ownership interpretation

```text
screen-plan:
  Local screen capture/evidence/custody/settings/live-view-boundary owner and screen proof router.

screen-domain:
  Public screen capture, evidence, OCR, VLM, disclosure, settings, screen-intelligence-router, and handoff contract package.

screen-ai-pipeline-plan:
  Screen -> AI -> policy/action product-path integration proof and live-operator proof.

ai-plan/schema-domain:
  AI context/result/provider/degradation contracts and model/runtime behavior when selected.

policy-control-plane-plan:
  Policy authority, parent-rule precedence, and deterministic policy decision semantics.

v0-8-enforcement-control-plan:
  Adapter execution, rollback, and supported runtime proof.

data-custody-storage-plan:
  Retention/export/delete/privacy/custody policy and parent-owned storage semantics.

portal-ux-household-surfaces-plan:
  Rendered parent UX, screenshots, route proof, and no-fake-data presentation.

remote-access-plan:
  Remote live-access capability, relay/session authority, standing grants, and remote product proof.

browser/app-game/network/tracking plans:
  Source-trigger/source-truth behavior for their domains.
```

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
5. Open only the assigned workpack.
6. Use `CHECKLIST_INDEX.md` for exact checklist sections.
7. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- Snapshot: [current-screen-snapshot.md](current-screen-snapshot.md).

## What is already present / proved

The current snapshot records retained proof for these screen-owned or screen-adjacent slices. These are real proof references but each row keeps its own non-claim.

```text
Windows active/selected/parent opt-in capture: real Windows pixels enter adapter path, encrypted queue metadata is produced, and raw image material is deleted; no macOS/Linux/physical Android/iOS parity claim.
Trigger scheduler capture: managed-browser/native/cadence inputs enqueue selected-window captures and delete raw images; browser URL ownership and service-owned producers remain separate.
Linux WSLg selected-window capture: real X11 selected-window capture with encrypted custody/deletion; no native Wayland/root claim.
Android MediaProjection emulator capture: explicit OS-consent emulator capture; no silent background capture or physical Android parity claim.
Disabled setting suppression: service path creates no new capture/queue/local-vision rows while disabled; service-persisted product settings remain separate.
Service cadence/read-model loop: Rust service cadence and WebSocket read rows; no VLM quality claim.
Service analysis loop: local adapter command and localVision row; proof adapter, not production VLM quality.
Retention sweeper: expired encrypted queue record removed and visible as expiredDeleted row; no parent retention UI/cloud policy claim. WP22 additionally has a real Windows raw-temp deletion proof, focused queue/sweeper/protocol gates, and exact-spec Playwright desktop/mobile portal proof showing deleted custody without raw screenshot rendering.
Live operator matrix: nine real operator-supplied URL/app scenarios through real capture/local VLM/schema/policy dry-run/raw deletion; no authenticated-social or managed-browser trigger ownership claim.
Portal rendering: service-backed screen read model renders custody/evidence refs; no product-complete background watcher or broad adapter claim.
Settings and optional-visibility proofs: catalog, writable intent, service command/persistence, optional raw-retention/live-view status; no raw retention/live view/raw remote upload enablement claim.
Remote/retention/live boundary: raw screenshot retention, live view, and raw remote upload disabled for local summaries; no live-view transport/writable retention/privacy/legal claim.
Live-view gates: platform-permission, loopback/session/runtime, UI-persistence, worker-startup, relay/cache harnesses; no product live-view, platform prompt screenshot, physical parity, hosted relay, or privacy/legal claim.
OCR/VLM proofs: Tesseract/PaddleOCR/WinRT candidate selection and small VLM readiness; final broad OCR/VLM quality and cross-platform parity remain open.
Closure/external gates: remaining platform/live-view/model-quality gaps are enumerated; external evidence intake rejects fixture/static/raw-private artifacts but does not replace missing physical/external proof.
```

## Open gaps / missing product runtime

- Product-complete parent retention controls and service runtime enablement for optional raw retention.
- Production live capability/runtime UI beyond readiness rendering and parent UI persistence carry-forward proof.
- Production OCR/VLM quality beyond controlled fixtures, proof adapters, the public/live operator matrix, public live crop matrix, and current WinRT OCR route evidence.
- Authenticated-account social proof beyond public/live surface proof.
- Externally proved browser/live trigger producer coverage beyond consumed scheduler inputs and service-started row-ready subscriber wiring.
- macOS live capture proof, Linux Wayland/root proof, physical Android proof, and iOS ReplayKit proof.
- Writable raw screenshot retention mode UI and privacy/legal approval.
- Actual production live-view worker start/relay-cache transport, real platform live-view prompt screenshots, physical-device parity, and product live-view UI/runtime completion.
- Current PP-OCRv5 quality/resource resolution, cross-platform OCR parity beyond Windows WinRT selection, and live VLM provider/runtime quality measurement beyond retained bounded-input and retained proof-image resource measurement.
- Browser/network/mobile/broad block action adapters from screen-derived decisions.
- Production parent explanation UX.
- Privacy/legal review.

## Production reachability audit (2026-08-16)

This is a production-code audit only. A checked workpack, proof adapter,
contract, read model, or portal panel does not establish a shipped capture or
retention path. The current non-AI screen path has real platform primitives and
queue/custody code, but its shipped service callers remain coupled to the
screen-AI service runtime and several platform owners are absent.

| Workpack | Real production caller/effect | Code-pass truth and remaining gap |
| --- | --- | --- |
| 01 Source Index And Doc Reconciliation | Plan/source inventory only | Documentation work; no runtime entrypoint or product effect. |
| 02 Current Screen Snapshot And Gap Map | Snapshot/read-only inventory | Documentation work; no runtime entrypoint or product effect. |
| 03 Contract Boundary And Effect Schemas | `agent-protocol`/`schema` DTOs | Contract foundation only; no shipped capture caller or durable effect. |
| 04 Parent Opt-In Settings Contract | `agent-service` settings command/store and portal command path | Settings persist through the service; no proof that settings start a non-AI capture provider. |
| 05 Capability Status Contract | `activity_surface_read_models::screen` and portal status cards | Read-model projection only; provider permission/protected/ready transitions are not owned by a shipped caller. |
| 06 Capture Scope Model | `screen-capture-adapter` scope API | Scope is modeled and platform-gated; only AI runtime callers consume it, with no standalone child/service capture path. |
| 07 Capture Trigger Model | `screen-capture-adapter::trigger_scheduler` | Scheduler is called by screen-AI cadence/foreground loops only; no non-AI trigger ingress reaches capture. |
| 08 Platform Adapter Abstraction | Desktop XCap/X11 functions and Android Java capture classes | Real primitives exist, but platform ownership/child composition is incomplete; fake/dev proof cannot close it. |
| 09 Windows Capture Adapter Plan Proof | XCap Windows capture is called from `screen_ai_cadence_runtime`/`screen_ai_foreground_runtime` | Real Windows primitive, AI-coupled caller; picker/consent/production proof and non-AI service ownership remain open. |
| 10 MacOS Capture Adapter Plan Proof | XCap macOS conditional primitive | No shipped macOS capture caller or platform proof; Apple permission/packaging remains external. |
| 11 Linux Capture Adapter Plan Proof | X11 primitive in `screen-capture-adapter::linux_x11` | X11 path exists; no Wayland/root/provider caller and no non-AI service path. |
| 12 Android MediaProjection Adapter Plan Proof | Parent-package MediaProjection activity/service | Explicit-consent proof adapter only; child runtime capture ingress and physical/device authority remain absent. |
| 13 iOS ReplayKit Adapter Plan Proof | Parent Swift status/app shell | No ReplayKit extension/capture runtime or physical-device caller; Apple entitlement/signing authority remains external. |
| 14 Protected Surface Detector | `screen-ai-core` router/redaction adapter | AI/router classification only; no provider-owned protected-surface signal in non-AI capture path. |
| 15 Encrypted Temporary Image Queue | `agent-core` encrypted queue, leases, outbox, sweep/remove | Durable local custody code is reachable through AI capture loops; standalone capture ingress, bounded production operation, and complete custody proof remain open. |
| 16 Queue Scheduler And Debouncer | AI cadence/foreground loops plus scheduler | Real scheduler caller is AI-owned; no independent screen capture service caller. |
| 17 Local OCR Vision Runtime Model | `screen_ai_analysis_runtime`/local AI runtime | AI-only runtime and evaluation; outside this non-AI pass and cannot prove capture product effect. |
| 18 Screen Analysis Result Schema | AI event-record parsing/read-model fields | DTO/event shape only; no independent capture or retention effect. |
| 19 Sensitive Text And Redaction Model | AI adapter redaction and portal projection | AI redaction path only; raw-capture provider boundary and independent retention effect remain open. |
| 20 Result Validator And Invalid Output Handling | AI adapter process/parser | Model-output validation only; no non-AI capture/provider effect. |
| 21 Journal And SQLite Ingest | `activity_store_screen_evidence` plus `ScreenAiServiceEventRuntime` row-ready bridge | Summary/read-model persistence is real and raw blobs are excluded; downstream screen event spine is in-memory/manual-required and not a standalone capture service. |
| 22 Deletion And Retention Proof | Queue remove/sweep/outbox and retention sweeper caller | Local deletion/custody code is real; product retention controls, atomic end-to-end runtime evidence, and custody/legal ownership remain open. |
| 23 Policy Compiler For Screen Derived Evidence | AI policy refs and policy-control handoff | Policy handoff shape only; policy authority is another plan and no screen capture effect is established. |
| 24 Enforcement Handoff Guard | Screen action/read-model guard | Guard/readiness only; no enforcement adapter execution owned by screen plan. |
| 25 Parent Portal Summary UI | Portal renders Rust screen read model | Presentation/read-model effect only; no capture source or retention authority. |
| 26 Child Disclosure UX | No mapped implementation roots | UX/status work only; no shipped child disclosure runtime caller. |
| 27 Screenshot Retention Optional Mode | Settings/preflight schemas and writable settings UI | Default remains disabled; no production opt-in custody/export/delete implementation. |
| 28 Live View Optional Mode | Rust worker/readiness gate and local proof paths | Worker/transport remains blocked; no live frame provider, session, relay, or product effect. |
| 29 Proof Tiers And Proof Packs | Proof routing only | Validation/proof work; no runtime implementation. |
| 30 Test Suite Playwright Rollout PR Gate | Tests/scripts only | Validation-only; deferred from code pass. |
| 31 Screen Intelligence Router | `screen-ai-core` routing modules | AI routing only; no non-AI capture/provider caller. |
| 32 Browser Structured Extraction Before Screenshot | AI router with browser source handoff | Browser/AI source handoff only; browser owner and non-AI capture path remain separate. |
| 33 Managed Browser CDP Screenshot Capture Path | No mapped production implementation | Proof/test roots are absent; no shipped CDP capture caller. |
| 34 OCR Tesseract Baseline | Tests/proof only | Validation-only; no runtime implementation. |
| 35 OCR PaddleOCR PP-OCR Evaluation | Tests/proof only | Validation-only; no runtime implementation. |
| 36 Small VLM Guided Classifier Evaluation | AI analysis/runtime modules | AI evaluation/runtime only; outside this non-AI pass. |
| 37 Household Mesh Screen Analysis Queue | Child mesh + screen-AI routing | AI/mesh handoff only; no independent capture or retention effect. |
| 38 Local AI Resource Scheduler Priority Queue | Local AI provider scheduler | AI resource scheduling only; outside capture/provider ownership. |
| 39 Redacted Summary Only Remote Boundary | Protocol/live-view/mesh boundary types | Boundary contract only; remote transport/custody owners remain external and raw upload stays disabled. |
| 40 Detector Prompt Packs And Schema Tests | Tests/proof only | Validation-only; no runtime implementation. |
| Screen Control Settings Inventory | Generated documentation | Inventory only; no runtime entrypoint or product effect. |
| Screen Evidence Analysis Capability Guide | Capability documentation | Guidance only; no runtime entrypoint or product effect. |
| Screen Evidence Analysis Schema Proposal | Proposal documentation | Proposal only; not source/runtime authority. |

The one production correction in this pass is in
`crates/agent-core/src/screen_event_runtime.rs`: degraded service rows no
longer emit synthetic AI-completed events. They publish only the capture,
queue, deletion, and portal phases represented by the degraded row; AI quality,
policy, enforcement, live view, platform parity, and retention-product claims
remain open. No tests, builds, proof, or CI were run in this code phase.

## Current coupling risks

```text
- 100/100 implementation checklist status is not whole-plan completion while 22 workpacks remain open.
- Checked workpack proof cannot close unrelated open workpacks.
- Local capture proof is not screen-AI pipeline completion.
- Screen summary/evidence proof is not policy authority.
- Policy dry-run proof is not enforcement adapter execution.
- Live-view preflight/loopback/relay-cache/worker-startup proof is not product live-view readiness.
- Redacted summary export proof is not raw screenshot remote upload proof.
- Portal screenshot proof is not runtime capture proof.
- Mock screenshot or fixture-only proof is not product proof.
```

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checkbox rows detected: 100 total, 100 checked, 0 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).
- Status rule: checklist rows alone do not prove whole-plan completion while `WORKPACK_INDEX.md` still lists open workpacks.

## Workpack summary

- Workpacks indexed: 40.
- Workpacks with open checkboxes: 22.
- Workpacks with all detected boxes checked: 18.
- Workpacks with no checkbox status: 0.

### Active/open workpacks

- [03 Contract Boundary And Effect Schemas](workpacks/03-contract-boundary-and-effect-schemas.md) - 0/8 checked, 8 open.
- [05 Capability Status Contract](workpacks/05-capability-status-contract.md) - 0/8 checked, 8 open.
- [09 Windows Capture Adapter Plan Proof](workpacks/09-windows-capture-adapter-plan-proof.md) - 0/7 checked, 7 open.
- [18 Screen Analysis Result Schema](workpacks/18-screen-analysis-result-schema.md) - 0/7 checked, 7 open.
- [20 Result Validator And Invalid Output Handling](workpacks/20-result-validator-and-invalid-output-handling.md) - 0/7 checked, 7 open.
- [01 Source Index And Doc Reconciliation](workpacks/01-source-index-and-doc-reconciliation.md) - 0/6 checked, 6 open.
- [02 Current Screen Snapshot And Gap Map](workpacks/02-current-screen-snapshot-and-gap-map.md) - 0/6 checked, 6 open.
- [06 Capture Scope Model](workpacks/06-capture-scope-model.md) - 0/6 checked, 6 open.
- [07 Capture Trigger Model](workpacks/07-capture-trigger-model.md) - 0/6 checked, 6 open.
- [08 Platform Adapter Abstraction](workpacks/08-platform-adapter-abstraction.md) - 0/6 checked, 6 open.
- [14 Protected Surface Detector](workpacks/14-protected-surface-detector.md) - 0/6 checked, 6 open.
- [15 Encrypted Temporary Image Queue](workpacks/15-encrypted-temporary-image-queue.md) - 0/6 checked, 6 open.
- [16 Queue Scheduler And Debouncer](workpacks/16-queue-scheduler-and-debouncer.md) - 0/6 checked, 6 open.
- [21 Journal And SQLite Ingest](workpacks/21-journal-and-sqlite-ingest.md) - 0/6 checked, 6 open.
- [22 Deletion And Retention Proof](workpacks/22-deletion-and-retention-proof.md) - 0/6 checked, 6 open (independent production-path review rejected prior proof).
- [23 Policy Compiler For Screen Derived Evidence](workpacks/23-policy-compiler-for-screen-derived-evidence.md) - 0/6 checked, 6 open.
- [29 Proof Tiers And Proof Packs](workpacks/29-proof-tiers-and-proof-packs.md) - 0/6 checked, 6 open.
- [12 Android MediaProjection Adapter Plan Proof](workpacks/12-android-mediaprojection-adapter-plan-proof.md) - 7/9 checked, 2 open.
- [13 iOS ReplayKit Adapter Plan Proof](workpacks/13-ios-replaykit-adapter-plan-proof.md) - 5/7 checked, 2 open.
- [28 Live View Optional Mode](workpacks/28-live-view-optional-mode.md) - 12/14 checked, 2 open.
- [30 Test Suite Playwright Rollout PR Gate](workpacks/30-test-suite-playwright-rollout-pr-gate.md) - 15/17 checked, 2 open.
- [39 Redacted Summary Only Remote Boundary](workpacks/39-redacted-summary-only-remote-boundary.md) - 5/6 checked, 1 open.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- `WORKPACK_FAMILIES.md` unless selected workpack owner/proof family is unclear.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log or explicit known blocker from the assigned implementation boundary,
  - proof under `output/screen-plan-proof/<workpack-file-stem>/` or the exact named historical proof directory,
  - proof manifest notes under docs/proof/screen-plan/ when a slice claims closure.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, rollback/teardown, deletion/custody, redaction, platform/manual-required, and no-claim proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
