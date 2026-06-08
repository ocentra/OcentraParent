# Screen AI Pipeline Implementation Checklist

A checkbox may be marked `[x]` only after the matching artifact exists under:

```text
output/screen-ai-pipeline-proof/
```

## Current Stacked Proof Snapshot

These entries are current branch proof status, not broad product-complete
claims.

| Proof                                       | Status                    | Artifact                                                                                       | Non-claim                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------------------------------------- | ------------------------- | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Real Windows active-window capture          | P3 proved                 | `output/screen-plan-proof/real-capture/manual-parent-test-active-window/proof-summary.json`    | Remaining B implementation task: connect the adapter into service/runtime/read-model after crate-lock coordination; this is not a service-wiring claim.                                                                                                                                                                                                                                                                                                                                                                                          |
| Real Windows scope-matrix capture           | P3 proved                 | `output/screen-plan-proof/real-capture/scope-matrix/proof-summary.json`                        | Proves adapter scopes only; parent-controlled product setting, scheduler, and disable suppression remain implementation work.                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Desktop adapter path                        | P2 implementation path    | `crates/screen-capture-adapter/src/lib.rs`                                                     | Windows/macOS use `xcap`; Linux uses a real X11 command backend. Live macOS and Linux Wayland/root-display proof still must run before those platform claims are complete.                                                                                                                                                                                                                                                                                                                                                                       |
| Linux WSLg selected-window capture          | P3 local WSLg proved      | `output/screen-plan-proof/linux-wslg/proof-summary.json`                                       | Proves WSLg/X11 selected-window capture only; does not claim WSLg root display, native Wayland portal, or broad Linux compositor parity.                                                                                                                                                                                                                                                                                                                                                                                                         |
| Android MediaProjection emulator capture    | P3 local emulator proved  | `output/screen-plan-proof/android-mediaprojection/proof-summary.json`                          | Proves explicit OS consent, foreground-service capture, frame digest, and raw-temp deletion on Android API 35 emulator only; physical-device parity and silent background capture are not claimed.                                                                                                                                                                                                                                                                                                                                               |
| Browser-window scheduler capture            | P3 proved                 | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                      | Scheduler enqueues before real selected-window capture; browser-plan owned managed URL integration remains outside this proof.                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Native app foreground scheduler capture     | P3 proved                 | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                      | Real Windows Notepad foreground capture is proven through the scheduler; service-owned foreground watcher wiring remains.                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Timed two-frame scheduler cadence           | P3 proved                 | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                      | Scheduler-due cadence and two real captures are proven; service timer loop and disabled-setting stop remain.                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Service-owned timed cadence runtime         | P3 local-machine proved   | `output/screen-ai-pipeline-proof/service-cadence/proof-summary.json`                           | Rust service opt-in cadence loop records three real Windows active-window captures, writes encrypted queue metadata, surfaces three Activity Screen rows over the real WebSocket read model, and proves the pending queue cap holds at three frames; it does not claim VLM classification.                                                                                                                                                                                                                                                       |
| Service-owned native foreground runtime     | P3 local-machine proved   | `output/screen-ai-pipeline-proof/service-foreground/proof-summary.json`                        | Rust service opt-in foreground watcher records encrypted active-window captures across real Chromium-to-Notepad foreground activity and surfaces the latest Activity Screen row over the real WebSocket read model; it does not claim browser URL trigger ownership or VLM classification.                                                                                                                                                                                                                                                       |
| Managed browser trigger to local AI context | P3 contract/context proof | `output/screen-ai-pipeline-proof/browser-trigger/proof-summary.json`                           | Proves managed-browser URL and browser-video rows can compose typed browser evidence with screen-analysis evidence into ready local-AI contexts, while social/feed stays manual-required and cloud-game/protected surface stays unavailable. It does not claim authenticated-account social proof, cloud-frame analysis, broad browser enforcement, final policy execution, portal UI, or mobile browser parity.                                                                                                                                 |
| Service-owned local adapter analysis        | P3 local-machine proved   | `output/screen-ai-pipeline-proof/service-analysis/proof-summary.json`                          | Rust service opt-in analysis loop consumes one encrypted queue record, invokes a local adapter command through the service runtime, writes a `localVision` Activity Screen row over the real WebSocket read model, and drains the processed queue; it does not claim production VLM quality.                                                                                                                                                                                                                                                     |
| Service-owned WinRT OCR analysis            | P3 local-machine proved   | `output/screen-ai-pipeline-proof/service-winrt-ocr/proof-summary.json`                         | Rust service opt-in cadence captures a live public Wikipedia Chrome window into the encrypted queue, invokes a local Windows `Windows.Media.Ocr` adapter through the service analysis runtime, writes a `localOcr` `school` Activity Screen row with runtime/model/template metadata over the real WebSocket read model, drains the queue, and deletes adapter temp image material; it does not claim production OCR quality, authenticated/social coverage, enforcement, cross-platform OCR, live view, or raw retention.                       |
| Service WinRT OCR policy dry-run            | P3 local-machine proved   | `output/screen-ai-pipeline-proof/service-winrt-ocr-policy/proof-summary.json`                  | Reruns the real service WinRT OCR proof, consumes that exact `localOcr` Activity Screen row through typed parent-domain policy contracts, writes an allow dry-run decision with activity/journal/query-store evidence refs, and preserves deleted-image/no-raw-retention custody; it does not claim final enforcement, broad adapters, production OCR quality, live view, or raw retention.                                                                                                                                                      |
| Service-owned native game analysis          | P3 local-machine proved   | `output/screen-ai-pipeline-proof/service-native-game-analysis/proof-summary.json`              | Rust service opt-in foreground capture records a controlled native game-like window, then the opt-in analysis loop consumes that encrypted queue record, invokes a local adapter command through the service runtime, writes a `localVision` game Activity Screen row, and drains the processed queue. It does not claim installed commercial-game detection or a dedicated app/game identity producer.                                                                                                                                          |
| Service-owned retention sweeper runtime     | P3 local-machine proved   | `output/screen-ai-pipeline-proof/service-retention-sweeper/proof-summary.json`                 | Rust service opt-in retention sweeper removes one expired encrypted queue record created by the service cadence runtime and surfaces an `expiredDeleted` Activity Screen row for the original queue job over the real WebSocket read model; it does not claim parent retention UI or cloud retention policy.                                                                                                                                                                                                                                     |
| Local VLM analysis of captured screens      | P3 harness proved         | `output/ai-plan-proof/real-analysis/proof-summary.json`                                        | Covers 16 real window captures of controlled video/social/game/shopping/bypass/school/native/cadence fixture content; live external URL/account proof remains before product-complete claims.                                                                                                                                                                                                                                                                                                                                                    |
| Guided VLM worker contract                  | P3 contract proved        | `output/screen-ai-pipeline-proof/screen-vlm-worker-contract-proof/proof-summary.json`          | Proves guided local VLM worker jobs/results are contract-bound before runtime use: source-cited encrypted queue input, bounded local image pixels, schema-bound model output, conversion to screen-analysis evidence, policy eligibility only after deleted-image/query-store custody, and no raw retention or remote AI. It does not claim live model execution, production VLM quality, portal UI, or enforcement.                                                                                                                             |
| VLM execution readiness/status handoff      | P3 contract proved        | `output/screen-ai-pipeline-proof/screen-vlm-execution-readiness-proof/proof-summary.json`      | Proves guided local VLM worker queue jobs can publish accepted readiness handoffs and queued/completed/manual-required status rows with preserved runtime/model/template refs, encrypted temp custody before execution, and deleted query-store custody after completion. It does not claim live model execution, production VLM quality, portal runtime rendering, policy authority, or enforcement.                                                                                                                                            |
| VLM journal/read-model projection           | P3 contract proved        | `output/screen-ai-pipeline-proof/screen-vlm-journal-read-model-proof/proof-summary.json`       | Proves completed VLM readiness status rows can publish encrypted journal lines and Activity Screen read-model rows that preserve local model/runtime/template refs, policy refs, parent-rule refs, explanation refs, deletion refs, journal evidence refs, and no raw image retention. It does not claim live model execution, production VLM quality, portal runtime rendering, policy authority, or enforcement.                                                                                                                               |
| Degraded OCR/VLM portal read-model          | P3 service/portal proved  | `output/ai-plan-proof/activity-screen-ai-degraded-portal-proof/proof-summary.json`             | Starts the real Rust service and parent portal, sends a schema-valid Activity Screen read-model request, and renders stored degraded `localOcr` and `localVision` rows on the Screen Analysis route with model/runtime/template refs, custody, not-reported policy, and not-claimed enforcement state. It does not execute OCR/VLM inference, capture fresh pixels, grant policy authority, dispatch enforcement, or close the final product-complete path.                                                                                      |
| Local AI safety result                      | P3 contract proved        | `output/ai-plan-proof/real-analysis/youtube-ordinary-video/06-ai-result.json`                  | Scenario results validate local AI safety output from captured fixture evidence; service runtime and live-site proof remain separate.                                                                                                                                                                                                                                                                                                                                                                                                            |
| Invalid model output guard                  | P3 proved                 | `output/screen-ai-pipeline-proof/invalid-output/proof-summary.json`                            | Proves malformed local model output cannot become a screen analysis result or policy candidate; this is a contract guard, not a model-quality claim.                                                                                                                                                                                                                                                                                                                                                                                             |
| Stricter parent policy guard                | P3 proved                 | `output/screen-ai-pipeline-proof/stricter-rule/proof-summary.json`                             | Proves local AI recommendations cannot weaken stricter parent policy actions before policy handoff; enforcement adapter execution remains a separate gate.                                                                                                                                                                                                                                                                                                                                                                                       |
| Policy dry-run decision                     | P3 contract proved        | `output/ai-plan-proof/real-analysis/browser-game/07-policy-decision.json`                      | Covers allow, warn, ask-parent, time-limit, and block dry-run actions; real enforcement adapter dispatch is still required in this pipeline scope before product-complete action claims.                                                                                                                                                                                                                                                                                                                                                         |
| Screen-to-action adapter dispatch           | P3 Windows proved         | `output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json`                           | Proves screen-derived native owned-process time-limit decision handoff into the real Windows Rust service adapter path; does not claim browser, network, mobile, or broad block enforcement.                                                                                                                                                                                                                                                                                                                                                     |
| Screen-to-block adapter dispatch            | P3 Windows proved         | `output/screen-ai-pipeline-proof/block-action-dispatch/proof-summary.json`                     | Proves a screen-derived block decision can hand off into the real Windows Rust service owned-process block adapter and terminate a controlled owned process; category/browser/network/mobile/broad block adapters remain separate.                                                                                                                                                                                                                                                                                                               |
| Screen-derived adapter readiness            | P3 readiness proved       | `output/screen-ai-pipeline-proof/adapter-readiness/proof-summary.json`                         | Keeps screen-derived Windows owned-process time-limit/block rows tied to real adapter execution proof, and records broad installed-app, host network/domain, managed exact active-tab, Android/iOS mobile, and Linux host rows as manual-required, not-claimed, or unavailable; it does not implement broad/browser/network/mobile adapters.                                                                                                                                                                                                     |
| Model runtime backpressure                  | P3 contract proved        | `output/screen-ai-pipeline-proof/model-runtime-backpressure/proof-summary.json`                | Proves one heavy local screen AI model job can run per physical child device, queued heavy jobs stay under the configured cap, overflowed cadence/background work degrades as overloaded and cannot become policy eligible, and no remote/API provider or raw image retention is used. It does not claim live model execution, production model quality, portal UI, or enforcement.                                                                                                                                                              |
| Parent portal screen chain                  | P3 proved                 | `output/screen-ai-pipeline-proof/portal-chain/proof-summary.json`                              | Proves real service-to-portal Activity Screen read-model rendering of trigger, capture, AI, policy, deletion, custody, queue, digest, and evidence refs; live external account proof remains.                                                                                                                                                                                                                                                                                                                                                    |
| Live operator proof harness readiness       | P2 executable gate        | `output/screen-ai-pipeline-proof/live-operator/harness-readiness/proof-summary.json`           | Proves the live operator harness exists, covers the required nine scenario ids, and refuses to claim live proof without an operator manifest. It does not claim real YouTube/Vimeo/social/shopping/game/account proof until manifest-run artifacts exist.                                                                                                                                                                                                                                                                                        |
| Live operator full matrix                   | P3 local operator proved  | `output/screen-ai-pipeline-proof/live-operator/proof-summary.json`                             | Proves all nine required operator-supplied rows: ordinary YouTube `video`/`warn`, education YouTube `school`/`allow`, Vimeo `video`/`warn`, Facebook/social `chat`/`warn`, browser game `game`/`time-limit`, shopping `shopping`/`ask-parent`, school/productivity `school`/`allow`, native Notepad `productivity`/`allow`, and protected-surface degraded state with no raw image, AI, or policy claim. Browser rows use selected browser-window capture to avoid foreground contamination; raw image material is deleted after local analysis. |
| Live operator artifact gate                 | P3 artifact gate proved   | `output/screen-ai-pipeline-proof/live-operator-artifact-gate/proof-summary.json`               | Rechecks the existing live operator proof artifacts by command: all nine required rows, redacted live URL/title/text readiness for browser surfaces, local VLM runtime evidence, policy dry-run links, protected-surface non-claims, parent explanation screenshots, and raw image deletion/no-retention custody. It does not rerun the operator capture session or claim managed-browser trigger ownership.                                                                                                                                     |
| Final product path artifact gate            | P3 stacked proof proved   | `output/screen-ai-pipeline-proof/final-product-path/proof-summary.json`                        | Validates retained real-run artifacts across live/operator real triggers, local VLM analysis, policy dry-runs, Windows action handoff proofs, portal/read-model proof, retention/deletion custody, and protected-surface non-claims. It does not rerun the live operator session, claim authenticated-account social proof, own managed-browser trigger producers, or claim broad browser/network/mobile adapters.                                                                                                                               |
| Final adapter dependency audit              | P3 blocker gate proved    | `output/screen-ai-pipeline-proof/final-adapter-dependency-audit/proof-summary.json`            | Machine-checks the final product path and adapter-readiness artifacts, proves Windows owned-process adapter execution remains the only executed screen-derived adapter class, and records broad installed-app, host network/domain, managed exact active-tab, Android/iOS mobile, and Linux host adapter artifacts still required before product-complete action claims. It does not close the open broad/browser/network/mobile adapter row or implement those adapters.                                                                        |
| Adapter blocker ledger proof                | P3 blocker ledger proved  | `output/screen-ai-pipeline-proof/adapter-blocker-ledger/proof-summary.json`                    | Converts the final adapter blocker map into an actionable checked ledger: each missing broad installed-app, host network/domain, managed exact active-tab, Android, iOS, and Linux adapter class names the expected source boundary, required screen-derived apply/rollback/audit custody proof, and checklist rows it unblocks. It verifies the final product path remains valid while final adapter completion stays blocked and does not edit the product checklist while another lane owns it.                                               |
| Upstream adapter prerequisite bridge        | P3 bridge proof proved    | `output/screen-ai-pipeline-proof/upstream-adapter-prerequisite-bridge/proof-summary.json`      | Consumes existing app/game broad-blocking gates, network action-result state proof, managed-browser CDP capture, Android MediaProjection capability, iOS ReplayKit no-overclaim, and Linux capture capability as upstream prerequisites. It keeps execution, rollback, adapter invocation, live host mutation, physical mobile control, native Linux control, and audit custody missing; the final product-complete adapter row remains open.                                                                                                    |
| Linux host adapter custody artifact         | P3 custody artifact       | `output/screen-ai-pipeline-proof/linux-host-adapter-custody/proof-summary.json`                | Converts the screen-derived bypass-tool block decision and Linux capture capability evidence into a concrete Linux host apply/rollback/audit custody artifact. The apply state remains `not-executed-target-unavailable`, rollback remains `not-executed-no-host-apply`, audit custody cites the screen policy, Linux capture, and deletion refs, and final adapter completion stays blocked until native Linux host mutation plus rollback proof exists.                                                                                        |
| Event-driven Screen-AI runtime chain        | P2 runtime proof          | `output/screen-ai-pipeline-proof/event-driven-runtime/proof-summary.json`                      | Proves a fresh uncoupled in-process `ocentra-eventing` chain where screen evidence publishes typed events, AI publishes a typed result event, and policy/action/read-model/deletion phases carry refs from prior events without direct AI-to-policy/action shortcuts. The service bridge/subscriber path now also proves degraded AI rows publish capture/queue/AI/deletion/portal events without policy/action refs. Cross-process/LAN mesh boundaries and broad adapter enforcement remain separate gates.                                     |
| Parent explanation service read model       | P3 service proved         | `output/ai-plan-proof/screen-summary-parent-explanation-service-read-model/proof-summary.json` | Starts the real Rust service against a seeded ActivityStore and requests the Activity Screen read model over WebSocket, proving policy refs, parent rules, local runtime refs, parent explanation refs, deletion reasons, deleted-image state, and child-device custody survive service/query projection. It does not claim production portal rendering, new capture/model inference, remote/API AI, or enforcement.                                                                                                                             |
| Household mesh precursor runtime discovery  | P3 loopback runtime proof | `output/screen-ai-pipeline-proof/family-ai-hub-runtime-discovery/proof-summary.json`           | Starts a real loopback household-provider endpoint, discovers it, records child-agent hello/heartbeat/route evidence, selects the existing screen household-provider route after a child-local degraded attempt, and submits a redacted-crop job with no raw screenshot bytes, no retention, no remote/API provider, and no Ocentra-hosted processing. Physical household LAN, production VLM quality, portal UI, policy authority, cloud relay, and enforcement remain separate gates.                                                          |
| Unknown native process trigger              | P3 local-machine proved   | `output/screen-ai-pipeline-proof/unknown-native-process/03-capture-proof.json`                 | Proves a controlled unknown native process window can trigger selected-window capture, local VLM unknown classification, low-confidence handling, and ask-parent dry-run without retaining raw image.                                                                                                                                                                                                                                                                                                                                            |
| OCR visible-text route                      | P3 contract proved        | `output/screen-ai-pipeline-proof/ocr-route/proof-summary.json`                                 | Proves typed local OCR text evidence can produce schema-valid screen analysis and policy dry-run without a vision model or retained raw image; production OCR adapter remains separate.                                                                                                                                                                                                                                                                                                                                                          |
| Local text typed-context route              | P3 contract proved        | `output/screen-ai-pipeline-proof/local-text-route/proof-summary.json`                          | Proves screen-derived typed activity evidence can enter Local AI safety input/result contracts with local-only runtime status and dry-run policy handoff; live model inference remains separate.                                                                                                                                                                                                                                                                                                                                                 |
| Deterministic structured-evidence route     | P3 contract proved        | `output/screen-ai-pipeline-proof/deterministic-route/proof-summary.json`                       | Proves structured screen-adjacent evidence can produce a no-image deterministic analysis route and dry-run policy decision; live capture and model inference are not claimed.                                                                                                                                                                                                                                                                                                                                                                    |
| Observe-only policy guard                   | P3 contract proved        | `output/screen-ai-pipeline-proof/observe-policy/proof-summary.json`                            | Proves observe-only settings allow analysis but reject policy handoff, leaving `policyEligible:false` and creating no policy decision; enforcement is not claimed.                                                                                                                                                                                                                                                                                                                                                                               |
| Protected-surface skip                      | P3 contract proved        | `output/screen-ai-pipeline-proof/protected-surface/proof-summary.json`                         | Proves protected surfaces are recorded as degraded skips with no raw image, no AI analysis, no policy decision, and policy eligibility rejected; live OS prompt proof remains separate.                                                                                                                                                                                                                                                                                                                                                          |
| Deletion retention custody                  | P2 contract proved        | `output/screen-ai-pipeline-proof/deletion-retention-custody/proof-summary.json`                | Proves successful deletion, expired-image deletion proof, visible delete-failed queue health, bounded retries, and rejection of unsupported raw image retention; service-owned retention sweeper execution is proven separately, while parent retention UI remains separate.                                                                                                                                                                                                                                                                     |
| Windows WinRT OCR worker                    | P3 real OCR proved        | `output/ai-plan-proof/screen-winrt-ocr-worker/proof-summary.json`                              | Proves real selected-window browser/native pixels captured through the Rust adapter can run through Windows WinRT OCR, become schema-valid `ScreenAnalysisResult` evidence, feed allow dry-run policy decisions, and delete raw temp images. It does not claim production OCR quality, cross-platform OCR parity, or final end-to-end pipeline closure.                                                                                                                                                                                          |
| Controlled native game trigger              | P3 local-machine proved   | `output/screen-ai-pipeline-proof/native-game/03-capture-proof.json`                            | Proves a controlled native game foreground trigger can capture a real selected window and classify it as game with local VLM; service-owned foreground watcher remains separate.                                                                                                                                                                                                                                                                                                                                                                 |
| Prerequisite merge record                   | P3 proved                 | `output/screen-ai-pipeline-proof/prerequisite-merge/proof-summary.json`                        | Proves the merged PR258 checkpoint commit is an ancestor and records current head plus screen capture and AI proof artifact paths; it is not a validation or live-site proof.                                                                                                                                                                                                                                                                                                                                                                    |
| Raw image deletion after analysis           | P3 proved                 | `output/ai-plan-proof/real-analysis/proof-summary.json`                                        | Every captured row deletes the raw temp image after analysis; retention/live-view opt-in modes are separate implementation work and are not claimed here.                                                                                                                                                                                                                                                                                                                                                                                        |
| Disabled no-capture no-AI suppression       | P3 local-machine proved   | `output/screen-ai-pipeline-proof/service-disabled-suppression/proof-summary.json`              | Proves the real Rust service honors the parent-disabled setting across cadence capture, foreground capture, and queued analysis processing: no new screen rows, no new queue records, no local vision row, and no pending queue drain while disabled. Product settings UI remains separate.                                                                                                                                                                                                                                                      |

## Planned Household Mesh Proof Rows

These rows are planned only. They do not upgrade the existing household-provider route
or runtime-discovery proofs into full household mesh execution.

| Proof                          | Status           | Artifact                                                                          | Non-claim                                                                                                                                                                                                                                        |
| ------------------------------ | ---------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Household mesh screen AI route | P2 runtime proof | `output/screen-ai-pipeline-proof/household-mesh-screen-ai/proof-summary.json`     | Proves screen-derived child-owned AI work routes through typed local mesh events, claim/lease, provider worker result, child-agent validation, policy-after-acceptance, and no raw screenshot transfer. Physical LAN execution remains separate. |
| No raw screen transfer mesh    | P2 runtime proof | `output/ai-plan-proof/no-raw-screen-transfer-mesh/proof-summary.json`             | Proves raw screenshot payloads are forbidden by default for provider workers and redacted summary/custody refs are used instead. Production bridge transport remains separate.                                                                   |
| Child provider result validity | P2 runtime proof | `output/ai-plan-proof/household-ai-provider-result-validation/proof-summary.json` | Proves duplicate, expired, wrong-provider, wrong-claim, evidence-mismatch, custody-mismatch, raw-transfer, and provider-authority-invalid results are rejected before policy.                                                                    |

## Prerequisite Gates

- [x] Screen capture proof PR merged or explicitly stacked.
- [x] AI analysis proof PR merged or explicitly stacked.
- [x] Pipeline branch contains both prerequisite implementations.
- [x] Prerequisite commits recorded in proof artifacts.
- [x] Pipeline branch proves the screen -> AI -> policy -> action/read-model/
      deletion handoff through `crates/ocentra-eventing`, not direct module
      calls, for the in-process successful runtime chain. Live service
      producer/subscriber wiring remains a follow-up gate.
- [x] Mesh route proof, when household provider execution is used, proves
      provider claim/lease/result-validation and no raw screenshot transfer.

## Real Trigger Gates

- [x] Managed browser social/video trigger. Browser trigger proof composes the
      managed browser/video evidence row with screen-analysis evidence into a
      ready local-AI context; live-operator surface proof remains the live
      external evidence gate, not managed-browser enforcement.
- [x] Managed browser education/video trigger. Browser trigger proof composes a
      managed browser URL evidence row with screen-analysis evidence into a
      ready local-AI context; live-operator school/video rows remain separate
      operator-run evidence.
- [x] Managed browser social/feed trigger. Browser trigger proof carries a
      social/feed row into a partial local-AI context and keeps it
      manual-required; authenticated-account social content and enforcement stay
      unclaimed.
- [x] Managed browser game/cloud-game trigger. Browser trigger proof carries a
      cloud-game/protected-surface row into a partial local-AI context and keeps
      it unavailable; cloud-frame analysis and broad browser-game enforcement
      stay unclaimed.
- [x] Native app foreground trigger. Scheduler proof and service-owned foreground watcher proof exist.
- [x] Native game or controlled game-window trigger. Controlled native window analysis exists, and service-owned foreground capture plus analysis now proves a game-classified `localVision` row; a dedicated installed-game identity producer remains app/game evidence scope.
- [x] Unknown process/app trigger.
- [x] Timed cadence trigger. Scheduler-due proof and service-owned three-frame timer loop proof exist; service pending-queue backpressure is proven, while model-runtime flood control remains.
- [x] Disabled setting suppression.
- [x] Protected/permission-required skip.

## AI Analysis Gates

- [x] OCR route proof where visible text is enough.
- [x] Real Windows WinRT OCR worker proof over captured browser/native pixels.
- [x] Service-owned WinRT OCR local adapter analysis over live public browser
      capture.
- [x] Guided VLM route proof where visual classification is needed.
- [x] Guided VLM worker contract proof before runtime use.
- [x] VLM execution readiness/status handoff proof before runtime queue
      integration.
- [x] Model-runtime flood-control/backpressure proof before high-cadence local
      model execution claims.
- [x] Service-owned local adapter analysis over an encrypted screen queue job.
- [x] AI analysis starts from a typed screen evidence event and publishes a
      typed AI result event for successful and degraded service event-chain
      paths.
- [x] Household provider route, when used, starts from child-owned AI work,
      grants one lease, and returns only a provider result for child validation.
- [x] Local text model route proof over typed context.
- [x] Deterministic route proof where structured evidence is enough.
- [x] Low confidence degrades safely.
- [x] Invalid output cannot reach policy.

## Policy And Action Gates

- [x] Observe policy result.
- [x] Allow policy result.
- [x] Warn policy result.
- [x] Ask-parent policy result.
- [x] Time-limit policy result.
- [x] Block dry-run result.
- [x] Block real adapter result for the owned-process Windows adapter path.
- [x] Unknown/manual-required result.
- [x] AI cannot override stricter parent rule.
- [x] Service WinRT OCR Activity Screen row feeds a typed parent policy dry-run.
- [x] Policy consumes a typed AI result event from the eventing runtime in the
      successful event-chain proof path.
- [x] Policy consumes only child-accepted provider results, never raw provider
      output or provider-originated policy/enforcement events.
- [x] Real Windows owned-process time-limit adapter dispatch, restart recovery, parent cancel, expiry, and process termination proof.
- [x] Screen-derived broad/browser/network/mobile adapter readiness states
      remain manual-required, not-claimed, or unavailable without claim
      upgrades.
- [ ] Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.

## Portal And Proof Gates

- [x] Journal/read model contains trigger, capture, AI, policy, and deletion refs.
- [x] Parent portal screenshot shows the full chain.
- [x] Parent explanation cites evidence and rules.
- [x] Raw image deletion proof linked.
- [x] Remote/cloud screenshot upload disabled.
- [x] Live operator proof harness readiness artifact exists:
      `output/screen-ai-pipeline-proof/live-operator/harness-readiness/proof-summary.json`.
- [x] Full live operator proof completed for all nine required rows:
      `output/screen-ai-pipeline-proof/live-operator/proof-summary.json`.
- [x] Live operator artifact gate validates the retained proof summaries,
      redacted live surface evidence, local VLM rows, policy dry-runs,
      protected-surface non-claims, parent screenshots, and deletion custody:
      `output/screen-ai-pipeline-proof/live-operator-artifact-gate/proof-summary.json`.
- [x] Final product path artifact gate validates the retained live/operator,
      AI, policy, action, portal/read-model, service-backed Activity Screen
      read-model, retention, and deletion custody artifacts:
      `output/screen-ai-pipeline-proof/final-product-path/proof-summary.json`.
- [x] Final adapter dependency audit validates that product-complete
      broad/browser/network/mobile adapter claims remain blocked until their
      own screen-derived execution artifacts exist, and now consumes the
      Linux/Android/iOS custody artifacts while keeping execution claims
      blocked:
      `output/screen-ai-pipeline-proof/final-adapter-dependency-audit/proof-summary.json`.
- [x] Adapter blocker ledger proof names the exact missing adapter source
      boundaries, required apply/rollback/audit custody artifacts, and rows
      unblocked by each artifact without closing the product-complete row:
      `output/screen-ai-pipeline-proof/adapter-blocker-ledger/proof-summary.json`.
- [x] Upstream adapter prerequisite bridge proof consumes existing app/game
      broad-blocking gates, network action-result readiness,
      managed-browser CDP capture, Android MediaProjection capability, iOS
      ReplayKit no-overclaim, and Linux capture capability while keeping
      execution custody missing until the owning adapter lane provides
      screen-derived apply/rollback/audit artifacts:
      `output/screen-ai-pipeline-proof/upstream-adapter-prerequisite-bridge/proof-summary.json`.
- [x] Linux host adapter custody artifact records screen-derived apply,
      rollback, and audit custody for the Linux blocker while preserving
      not-executed target-unavailable state and keeping product-complete Linux
      host control blocked:
      `output/screen-ai-pipeline-proof/linux-host-adapter-custody/proof-summary.json`.
- [x] Android mobile-control custody artifact records screen-derived apply,
      rollback, and audit custody for the Android blocker while preserving
      not-executed manual-required state and keeping Device Owner,
      managed-profile, UsageStats, Accessibility, VPN/DNS, and physical-device
      control claims blocked:
      `output/screen-ai-pipeline-proof/android-mobile-control-custody/proof-summary.json`.
- [x] iOS mobile-control custody artifact records screen-derived apply,
      rollback, and audit custody for the iOS blocker while preserving
      not-executed manual-required state and keeping Family Controls,
      DeviceActivity, Network Extension, ReplayKit physical execution, live iOS
      pixels, deletion proof, and rollback claims blocked:
      `output/screen-ai-pipeline-proof/ios-mobile-control-custody/proof-summary.json`.
- [x] Fresh event-driven runtime path proof proves capture event, AI
      result event, policy event, action/dry-run event, portal/read-model event,
      and deletion/custody event in one in-process runtime chain. Live trigger
      rerun through the service loop remains a follow-up product-complete gate.
- [x] Household mesh screen AI proof, when mesh route is used, records provider
      discovery, provider selection, claim/lease, provider result,
      child-agent validation, policy authority, and custody artifacts.
- [x] Parent explanation refs survive service-backed Activity Screen read-model
      projection:
      `output/ai-plan-proof/screen-summary-parent-explanation-service-read-model/proof-summary.json`.
- [x] Windows WinRT OCR worker proof completed:
      `output/ai-plan-proof/screen-winrt-ocr-worker/proof-summary.json`.
- [x] Guided VLM worker contract proof completed:
      `output/screen-ai-pipeline-proof/screen-vlm-worker-contract-proof/proof-summary.json`.
- [x] VLM execution readiness/status handoff proof completed:
      `output/screen-ai-pipeline-proof/screen-vlm-execution-readiness-proof/proof-summary.json`.
- [x] VLM journal/read-model proof completed:
      `output/screen-ai-pipeline-proof/screen-vlm-journal-read-model-proof/proof-summary.json`.
- [x] Degraded OCR/VLM Activity Screen rows render on the real Screen Analysis
      portal route:
      `output/ai-plan-proof/activity-screen-ai-degraded-portal-proof/proof-summary.json`.
- [x] Service WinRT OCR proof completed:
      `output/screen-ai-pipeline-proof/service-winrt-ocr/proof-summary.json`.
- [x] Service WinRT OCR policy dry-run proof completed:
      `output/screen-ai-pipeline-proof/service-winrt-ocr-policy/proof-summary.json`.
- [x] Screen-derived adapter readiness proof completed:
      `output/screen-ai-pipeline-proof/adapter-readiness/proof-summary.json`.
- [x] Operator live proof completed before product-complete claim.
- [x] Live external URL/surface proof completed for real YouTube/Vimeo/social/shopping/school surfaces; controlled fixture rows are harness proof only.

## Validation

- [x] Screen focused validation rerun on pipeline branch.
- [x] AI focused validation rerun on pipeline branch.
- [x] Pipeline E2E/proof script run.
- [x] Playwright screenshot proof run.
- [x] Security negative tests run.
- [x] Deletion/retention/custody proof run:
      `node --check scripts/test/screen-ai-deletion-retention-custody-proof.mjs` and
      `node scripts/test/screen-ai-deletion-retention-custody-proof.mjs`.
- [x] Performance/cadence proof run.
- [x] Service cadence proof run: `node scripts/test/screen-ai-service-cadence-proof.mjs`.
- [x] Service foreground proof run: `node scripts/test/screen-ai-service-foreground-proof.mjs`.
- [x] Service analysis proof run: `node --check scripts/test/screen-ai-service-analysis-proof.mjs` and
      `node scripts/test/screen-ai-service-analysis-proof.mjs`.
- [x] Service WinRT OCR proof run:
      `node --check scripts/test/screen-ai-service-winrt-ocr-proof.mjs` and
      `node scripts/test/screen-ai-service-winrt-ocr-proof.mjs`.
- [x] Guided VLM worker contract proof run:
      `node --check scripts/test/screen-ai-vlm-worker-contract-proof.mjs` and
      `node scripts/test/screen-ai-vlm-worker-contract-proof.mjs`.
- [x] VLM execution readiness/status handoff proof run:
      `node --check scripts/test/screen-ai-vlm-execution-readiness-proof.mjs`
      and `node scripts/test/screen-ai-vlm-execution-readiness-proof.mjs`.
- [x] VLM journal/read-model proof run:
      `node --check scripts/test/screen-ai-vlm-journal-read-model-proof.mjs`
      and `node scripts/test/screen-ai-vlm-journal-read-model-proof.mjs`.
- [x] Degraded OCR/VLM Activity Screen portal proof run:
      `node --check scripts/test/screen-ai-degraded-portal-proof.mjs` and
      `node scripts/test/screen-ai-degraded-portal-proof.mjs`.
- [x] Service WinRT OCR policy proof run:
      `node --check scripts/test/screen-ai-service-winrt-ocr-policy-proof.mjs` and
      `node scripts/test/screen-ai-service-winrt-ocr-policy-proof.mjs`.
- [x] Screen-derived adapter readiness proof run:
      `node --check scripts/test/screen-ai-adapter-readiness-proof.mjs` and
      `node scripts/test/screen-ai-adapter-readiness-proof.mjs`.
- [x] Screen-AI browser trigger proof run:
      `node scripts/test/screen-ai-browser-trigger-proof.mjs`.
- [x] Model runtime backpressure proof run:
      `node --check scripts/test/screen-ai-model-runtime-backpressure-proof.mjs` and
      `node scripts/test/screen-ai-model-runtime-backpressure-proof.mjs`.
- [x] Service native game analysis proof run:
      `node --check scripts/test/screen-ai-service-native-game-analysis-proof.mjs` and
      `node scripts/test/screen-ai-service-native-game-analysis-proof.mjs`.
- [x] Service disabled suppression proof run:
      `node --check scripts/test/screen-ai-service-disabled-suppression-proof.mjs` and
      `node scripts/test/screen-ai-service-disabled-suppression-proof.mjs`.
- [x] Service retention sweeper proof run:
      `node --check scripts/test/screen-ai-service-retention-sweeper-proof.mjs` and
      `node scripts/test/screen-ai-service-retention-sweeper-proof.mjs`.
- [x] Screen-derived block adapter proof run: `node --check scripts/test/screen-ai-block-action-dispatch-proof.mjs`,
      `OCENTRA_SCREEN_AI_SCENARIOS=bypass-tool node scripts/test/screen-ai-local-vlm-proof.mjs`,
      and `node scripts/test/screen-ai-block-action-dispatch-proof.mjs`.
- [x] Live operator harness readiness run:
      `node --check scripts/test/screen-ai-live-operator-proof.mjs`,
      `node scripts/test/screen-ai-live-operator-proof.mjs --verify-harness`,
      and `node scripts/test/screen-ai-live-operator-proof.mjs --print-template`.
- [x] Live operator artifact gate run:
      `node --check scripts/test/screen-ai-live-operator-artifact-gate.mjs` and
      `node scripts/test/screen-ai-live-operator-artifact-gate.mjs`.
- [x] Final product path artifact gate run:
      `node --check scripts/test/screen-ai-final-product-path-proof.mjs` and
      `node scripts/test/screen-ai-final-product-path-proof.mjs`.
- [x] Final adapter dependency audit run:
      `node --check scripts/test/screen-ai-final-adapter-dependency-audit.mjs` and
      `node scripts/test/screen-ai-final-adapter-dependency-audit.mjs`.
- [x] Adapter blocker ledger proof run:
      `node --check scripts/test/screen-ai-adapter-blocker-ledger-proof.mjs`
      and `node scripts/test/screen-ai-adapter-blocker-ledger-proof.mjs`.
- [x] Upstream adapter prerequisite bridge proof run:
      `node --check scripts/test/screen-ai-upstream-adapter-prerequisite-bridge-proof.mjs`
      and
      `node scripts/test/screen-ai-upstream-adapter-prerequisite-bridge-proof.mjs`.
- [x] Linux host adapter custody proof run:
      `node --check scripts/test/screen-ai-linux-host-adapter-custody-proof.mjs`
      and `node scripts/test/screen-ai-linux-host-adapter-custody-proof.mjs`.
- [x] Android mobile-control custody proof run:
      `node --check scripts/test/screen-ai-android-mobile-control-custody-proof.mjs`
      and
      `node scripts/test/screen-ai-android-mobile-control-custody-proof.mjs`.
- [x] iOS mobile-control custody proof run:
      `node --check scripts/test/screen-ai-ios-mobile-control-custody-proof.mjs`
      and `node scripts/test/screen-ai-ios-mobile-control-custody-proof.mjs`.
- [x] Event-driven Screen-AI runtime chain proof run:
      `node --check scripts/test/screen-ai-event-driven-runtime-proof.mjs` and
      `node scripts/test/screen-ai-event-driven-runtime-proof.mjs`.
- [x] Screen service event bridge proof run:
      `node --check scripts/test/screen-service-event-bridge-proof.mjs` and
      `node scripts/test/screen-service-event-bridge-proof.mjs`.
- [x] Household mesh screen AI route proof run:
      `node --check scripts/test/screen-ai-household-mesh-proof.mjs` and
      `node scripts/test/screen-ai-household-mesh-proof.mjs`.
- [x] No-raw-screen-transfer mesh proof run:
      `node --check scripts/test/screen-ai-household-mesh-proof.mjs` and
      `node scripts/test/screen-ai-household-mesh-proof.mjs`.
- [x] Child-agent provider-result validation proof run:
      `node --check scripts/test/screen-ai-household-mesh-proof.mjs` and
      `node scripts/test/screen-ai-household-mesh-proof.mjs`.
- [x] Dependency policy rerun after capture dependency narrowing.
- [x] Android emulator MediaProjection proof run.
- [x] `git diff --check`.
- [x] lane/hub guards.
- [x] `npm run validate`.
