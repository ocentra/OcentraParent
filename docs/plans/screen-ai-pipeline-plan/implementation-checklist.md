# Screen AI Pipeline Implementation Checklist

A checkbox may be marked `[x]` only after the matching artifact exists under:

```text
output/screen-ai-pipeline-proof/
```

## Current Stacked Proof Snapshot

These entries are current branch proof status, not broad product-complete
claims.

| Proof                                    | Status                   | Artifact                                                                                    | Non-claim                                                                                                                                                                                                                                                                                    |
| ---------------------------------------- | ------------------------ | ------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Real Windows active-window capture       | P3 proved                | `output/screen-plan-proof/real-capture/manual-parent-test-active-window/proof-summary.json` | Remaining B implementation task: connect the adapter into service/runtime/read-model after crate-lock coordination; this is not a service-wiring claim.                                                                                                                                      |
| Real Windows scope-matrix capture        | P3 proved                | `output/screen-plan-proof/real-capture/scope-matrix/proof-summary.json`                     | Proves adapter scopes only; parent-controlled product setting, scheduler, and disable suppression remain implementation work.                                                                                                                                                                |
| Desktop adapter path                     | P2 implementation path   | `crates/screen-capture-adapter/src/lib.rs`                                                  | Windows/macOS use `xcap`; Linux uses a real X11 command backend. Live macOS and Linux Wayland/root-display proof still must run before those platform claims are complete.                                                                                                                   |
| Linux WSLg selected-window capture       | P3 local WSLg proved     | `output/screen-plan-proof/linux-wslg/proof-summary.json`                                    | Proves WSLg/X11 selected-window capture only; does not claim WSLg root display, native Wayland portal, or broad Linux compositor parity.                                                                                                                                                     |
| Android MediaProjection emulator capture | P3 local emulator proved | `output/screen-plan-proof/android-mediaprojection/proof-summary.json`                       | Proves explicit OS consent, foreground-service capture, frame digest, and raw-temp deletion on Android API 35 emulator only; physical-device parity and silent background capture are not claimed.                                                                                           |
| Browser-window scheduler capture         | P3 proved                | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                   | Scheduler enqueues before real selected-window capture; browser-plan owned managed URL integration remains outside this proof.                                                                                                                                                               |
| Native app foreground scheduler capture  | P3 proved                | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                   | Real Windows Notepad foreground capture is proven through the scheduler; service-owned foreground watcher wiring remains.                                                                                                                                                                    |
| Timed two-frame scheduler cadence        | P3 proved                | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                   | Scheduler-due cadence and two real captures are proven; service timer loop and disabled-setting stop remain.                                                                                                                                                                                 |
| Service-owned timed cadence runtime      | P3 local-machine proved  | `output/screen-ai-pipeline-proof/service-cadence/proof-summary.json`                        | Rust service opt-in cadence loop records three real Windows active-window captures, writes encrypted queue metadata, surfaces three Activity Screen rows over the real WebSocket read model, and proves the pending queue cap holds at three frames; it does not claim VLM classification.   |
| Service-owned native foreground runtime  | P3 local-machine proved  | `output/screen-ai-pipeline-proof/service-foreground/proof-summary.json`                     | Rust service opt-in foreground watcher records encrypted active-window captures across real Chromium-to-Notepad foreground activity and surfaces the latest Activity Screen row over the real WebSocket read model; it does not claim browser URL trigger ownership or VLM classification.   |
| Service-owned local adapter analysis     | P3 local-machine proved  | `output/screen-ai-pipeline-proof/service-analysis/proof-summary.json`                       | Rust service opt-in analysis loop consumes one encrypted queue record, invokes a local adapter command through the service runtime, writes a `localVision` Activity Screen row over the real WebSocket read model, and drains the processed queue; it does not claim production VLM quality. |
| Local VLM analysis of captured screens   | P3 harness proved        | `output/ai-plan-proof/real-analysis/proof-summary.json`                                     | Covers 16 real window captures of controlled video/social/game/shopping/bypass/school/native/cadence fixture content; live external URL/account proof remains before product-complete claims.                                                                                                |
| Local AI safety result                   | P3 contract proved       | `output/ai-plan-proof/real-analysis/youtube-ordinary-video/06-ai-result.json`               | Scenario results validate local AI safety output from captured fixture evidence; service runtime and live-site proof remain separate.                                                                                                                                                        |
| Invalid model output guard               | P3 proved                | `output/screen-ai-pipeline-proof/invalid-output/proof-summary.json`                         | Proves malformed local model output cannot become a screen analysis result or policy candidate; this is a contract guard, not a model-quality claim.                                                                                                                                         |
| Stricter parent policy guard             | P3 proved                | `output/screen-ai-pipeline-proof/stricter-rule/proof-summary.json`                          | Proves local AI recommendations cannot weaken stricter parent policy actions before policy handoff; enforcement adapter execution remains a separate gate.                                                                                                                                   |
| Policy dry-run decision                  | P3 contract proved       | `output/ai-plan-proof/real-analysis/browser-game/07-policy-decision.json`                   | Covers allow, warn, ask-parent, time-limit, and block dry-run actions; real enforcement adapter dispatch is still required in this pipeline scope before product-complete action claims.                                                                                                     |
| Screen-to-action adapter dispatch        | P3 Windows proved        | `output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json`                        | Proves screen-derived native owned-process time-limit decision handoff into the real Windows Rust service adapter path; does not claim browser, network, mobile, or broad block enforcement.                                                                                                 |
| Parent portal screen chain               | P3 proved                | `output/screen-ai-pipeline-proof/portal-chain/proof-summary.json`                           | Proves real service-to-portal Activity Screen read-model rendering of trigger, capture, AI, policy, deletion, custody, queue, digest, and evidence refs; live external account proof remains.                                                                                                |
| Live operator proof harness readiness    | P2 executable gate       | `output/screen-ai-pipeline-proof/live-operator/harness-readiness/proof-summary.json`        | Proves the live operator harness exists, covers the required nine scenario ids, and refuses to claim live proof without an operator manifest. It does not claim real YouTube/Vimeo/social/shopping/game/account proof until manifest-run artifacts exist.                                    |
| Unknown native process trigger           | P3 local-machine proved  | `output/screen-ai-pipeline-proof/unknown-native-process/03-capture-proof.json`              | Proves a controlled unknown native process window can trigger selected-window capture, local VLM unknown classification, low-confidence handling, and ask-parent dry-run without retaining raw image.                                                                                        |
| OCR visible-text route                   | P3 contract proved       | `output/screen-ai-pipeline-proof/ocr-route/proof-summary.json`                              | Proves typed local OCR text evidence can produce schema-valid screen analysis and policy dry-run without a vision model or retained raw image; production OCR adapter remains separate.                                                                                                      |
| Local text typed-context route           | P3 contract proved       | `output/screen-ai-pipeline-proof/local-text-route/proof-summary.json`                       | Proves screen-derived typed activity evidence can enter Local AI safety input/result contracts with local-only runtime status and dry-run policy handoff; live model inference remains separate.                                                                                             |
| Deterministic structured-evidence route  | P3 contract proved       | `output/screen-ai-pipeline-proof/deterministic-route/proof-summary.json`                    | Proves structured screen-adjacent evidence can produce a no-image deterministic analysis route and dry-run policy decision; live capture and model inference are not claimed.                                                                                                                |
| Observe-only policy guard                | P3 contract proved       | `output/screen-ai-pipeline-proof/observe-policy/proof-summary.json`                         | Proves observe-only settings allow analysis but reject policy handoff, leaving `policyEligible:false` and creating no policy decision; enforcement is not claimed.                                                                                                                           |
| Protected-surface skip                   | P3 contract proved       | `output/screen-ai-pipeline-proof/protected-surface/proof-summary.json`                      | Proves protected surfaces are recorded as degraded skips with no raw image, no AI analysis, no policy decision, and policy eligibility rejected; live OS prompt proof remains separate.                                                                                                      |
| Controlled native game trigger           | P3 local-machine proved  | `output/screen-ai-pipeline-proof/native-game/03-capture-proof.json`                         | Proves a controlled native game foreground trigger can capture a real selected window and classify it as game with local VLM; service-owned foreground watcher remains separate.                                                                                                             |
| Prerequisite merge record                | P3 proved                | `output/screen-ai-pipeline-proof/prerequisite-merge/proof-summary.json`                     | Proves the merged PR258 checkpoint commit is an ancestor and records current head plus screen capture and AI proof artifact paths; it is not a validation or live-site proof.                                                                                                                |
| Raw image deletion after analysis        | P3 proved                | `output/ai-plan-proof/real-analysis/proof-summary.json`                                     | Every captured row deletes the raw temp image after analysis; retention/live-view opt-in modes are separate implementation work and are not claimed here.                                                                                                                                    |
| Disabled no-capture no-AI suppression    | P3 proved                | `output/ai-plan-proof/real-analysis/disabled-no-capture-no-ai/01-source-evidence.json`      | Harness proof only; service-owned disabled suppression wiring remains.                                                                                                                                                                                                                       |

## Prerequisite Gates

- [x] Screen capture proof PR merged or explicitly stacked.
- [x] AI analysis proof PR merged or explicitly stacked.
- [x] Pipeline branch contains both prerequisite implementations.
- [x] Prerequisite commits recorded in proof artifacts.

## Real Trigger Gates

- [ ] Managed browser social/video trigger. AI matrix rows exist; managed-browser URL integration remains D/browser-owned.
- [ ] Managed browser education/video trigger. AI matrix rows exist; managed-browser URL integration remains D/browser-owned.
- [ ] Managed browser social/feed trigger. AI matrix rows exist; managed-browser URL integration remains D/browser-owned.
- [ ] Managed browser game/cloud-game trigger. AI matrix rows exist; managed-browser URL integration remains D/browser-owned.
- [x] Native app foreground trigger. Scheduler proof and service-owned foreground watcher proof exist.
- [x] Native game or controlled game-window trigger. Controlled native window analysis exists; game-specific service classification remains.
- [x] Unknown process/app trigger.
- [x] Timed cadence trigger. Scheduler-due proof and service-owned three-frame timer loop proof exist; service pending-queue backpressure is proven, while model-runtime flood control remains.
- [x] Disabled setting suppression.
- [x] Protected/permission-required skip.

## AI Analysis Gates

- [x] OCR route proof where visible text is enough.
- [x] Guided VLM route proof where visual classification is needed.
- [x] Service-owned local adapter analysis over an encrypted screen queue job.
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
- [ ] Block real adapter result.
- [x] Unknown/manual-required result.
- [x] AI cannot override stricter parent rule.
- [x] Real Windows owned-process time-limit adapter dispatch, restart recovery, parent cancel, expiry, and process termination proof.
- [ ] Browser, network, mobile, and broad block adapters proven from screen-derived decisions before product-complete action claims.

## Portal And Proof Gates

- [x] Journal/read model contains trigger, capture, AI, policy, and deletion refs.
- [x] Parent portal screenshot shows the full chain.
- [x] Parent explanation cites evidence and rules.
- [x] Raw image deletion proof linked.
- [x] Remote/cloud screenshot upload disabled.
- [x] Live operator proof harness readiness artifact exists:
      `output/screen-ai-pipeline-proof/live-operator/harness-readiness/proof-summary.json`.
- [ ] Operator live proof completed before product-complete claim.
- [ ] Live external URL/account proof completed for real YouTube/Vimeo/social/shopping/browser-game surfaces; controlled fixture rows are harness proof only.

## Validation

- [x] Screen focused validation rerun on pipeline branch.
- [x] AI focused validation rerun on pipeline branch.
- [x] Pipeline E2E/proof script run.
- [x] Playwright screenshot proof run.
- [x] Security negative tests run.
- [x] Performance/cadence proof run.
- [x] Service cadence proof run: `node scripts/test/screen-ai-service-cadence-proof.mjs`.
- [x] Service foreground proof run: `node scripts/test/screen-ai-service-foreground-proof.mjs`.
- [x] Service analysis proof run: `node --check scripts/test/screen-ai-service-analysis-proof.mjs` and
      `node scripts/test/screen-ai-service-analysis-proof.mjs`.
- [x] Live operator harness readiness run:
      `node --check scripts/test/screen-ai-live-operator-proof.mjs`,
      `node scripts/test/screen-ai-live-operator-proof.mjs --verify-harness`,
      and `node scripts/test/screen-ai-live-operator-proof.mjs --print-template`.
- [x] Dependency policy rerun after capture dependency narrowing.
- [x] Android emulator MediaProjection proof run.
- [x] `git diff --check`.
- [x] lane/hub guards.
- [ ] `npm run validate` or approved omission.
