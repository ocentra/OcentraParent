# Screen AI Pipeline Implementation Checklist

A checkbox may be marked `[x]` only after the matching artifact exists under:

```text
output/screen-ai-pipeline-proof/
```

## Current Stacked Proof Snapshot

These entries are current branch proof status, not broad product-complete
claims.

| Proof                                    | Status                   | Artifact                                                                                    | Non-claim                                                                                                                                                                                          |
| ---------------------------------------- | ------------------------ | ------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Real Windows active-window capture       | P3 proved                | `output/screen-plan-proof/real-capture/manual-parent-test-active-window/proof-summary.json` | Remaining implementation task: connect the adapter into service/runtime/read-model after crate-lock coordination; this is not a service-wiring claim.                                              |
| Real Windows scope-matrix capture        | P3 proved                | `output/screen-plan-proof/real-capture/scope-matrix/proof-summary.json`                     | Proves adapter scopes only; parent-controlled product setting, scheduler, and disable suppression remain implementation work.                                                                      |
| Desktop adapter path                     | P2 implementation path   | `crates/screen-capture-adapter/src/lib.rs`                                                  | Windows/macOS use `xcap`; Linux uses a real X11 command backend. Live macOS and Linux Wayland/root-display proof still must run before those platform claims are complete.                         |
| Linux WSLg selected-window capture       | P3 local WSLg proved     | `output/screen-plan-proof/linux-wslg/proof-summary.json`                                    | Proves WSLg/X11 selected-window capture only; does not claim WSLg root display, native Wayland portal, or broad Linux compositor parity.                                                           |
| Android MediaProjection emulator capture | P3 local emulator proved | `output/screen-plan-proof/android-mediaprojection/proof-summary.json`                       | Proves explicit OS consent, foreground-service capture, frame digest, and raw-temp deletion on Android API 35 emulator only; physical-device parity and silent background capture are not claimed. |
| Browser-window scheduler capture         | P3 proved                | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                   | Scheduler enqueues before real selected-window capture; browser-plan owned managed URL integration remains outside this proof.                                                                     |
| Native app foreground scheduler capture  | P3 proved                | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                   | Real Windows Notepad foreground capture is proven through the scheduler; service-owned foreground watcher wiring remains.                                                                          |
| Timed two-frame scheduler cadence        | P3 proved                | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                   | Scheduler-due cadence and two real captures are proven; service timer loop and disabled-setting stop remain.                                                                                       |
| Local VLM analysis of captured screens   | P3 proved                | `output/ai-plan-proof/real-analysis/proof-summary.json`                                     | Covers 15 real window captures across controlled video/social/game/shopping/bypass/school/native/cadence rows; live external URL/account proof remains.                                            |
| Local AI safety result                   | P3 contract proved       | `output/ai-plan-proof/real-analysis/youtube-ordinary-video/06-ai-result.json`               | Scenario results validate local AI safety output; service runtime must still consume this result.                                                                                                  |
| Policy dry-run decision                  | P3 contract proved       | `output/ai-plan-proof/real-analysis/browser-game/07-policy-decision.json`                   | Covers allow, warn, ask-parent, time-limit, and block dry-run actions; real enforcement adapter dispatch is still required in this pipeline scope before product-complete action claims.           |
| Raw image deletion after analysis        | P3 proved                | `output/ai-plan-proof/real-analysis/proof-summary.json`                                     | Every captured row deletes the raw temp image after analysis; retention/live-view opt-in modes are separate implementation work and are not claimed here.                                          |
| Disabled no-capture no-AI suppression    | P3 proved                | `output/ai-plan-proof/real-analysis/disabled-no-capture-no-ai/01-source-evidence.json`      | Harness proof only; service-owned disabled suppression wiring remains.                                                                                                                             |

## Prerequisite Gates

- [ ] Screen capture proof PR merged or explicitly stacked.
- [ ] AI analysis proof PR merged or explicitly stacked.
- [ ] Pipeline branch contains both prerequisite implementations.
- [ ] Prerequisite commits recorded in proof artifacts.

## Real Trigger Gates

- [ ] Managed browser social/video trigger. AI matrix rows exist; managed-browser URL integration remains D/browser-owned.
- [ ] Managed browser education/video trigger. AI matrix rows exist; managed-browser URL integration remains D/browser-owned.
- [ ] Managed browser social/feed trigger. AI matrix rows exist; managed-browser URL integration remains D/browser-owned.
- [ ] Managed browser game/cloud-game trigger. AI matrix rows exist; managed-browser URL integration remains D/browser-owned.
- [x] Native app foreground trigger.
- [ ] Native game or controlled game-window trigger. Controlled native window analysis exists; service-owned foreground watcher remains.
- [ ] Unknown process/app trigger.
- [ ] Timed cadence trigger. Scheduler-due proof exists; service timer and disabled-stop proof remain.
- [x] Disabled setting suppression.
- [ ] Protected/permission-required skip.

## AI Analysis Gates

- [ ] OCR route proof where visible text is enough.
- [x] Guided VLM route proof where visual classification is needed.
- [ ] Local text model route proof over typed context.
- [ ] Deterministic route proof where structured evidence is enough.
- [x] Low confidence degrades safely.
- [ ] Invalid output cannot reach policy.

## Policy And Action Gates

- [ ] Observe policy result.
- [x] Allow policy result.
- [x] Warn policy result.
- [x] Ask-parent policy result.
- [x] Time-limit policy result.
- [x] Block dry-run or real adapter result.
- [x] Unknown/manual-required result.
- [ ] AI cannot override stricter parent rule.
- [ ] Real action adapter dispatch and rollback proof, owned by the screen+AI pipeline before product-complete action claims.

## Portal And Proof Gates

- [x] Journal/read model contains trigger, capture, AI, policy, and deletion refs.
- [ ] Parent portal screenshot shows the full chain.
- [x] Parent explanation cites evidence and rules.
- [x] Raw image deletion proof linked.
- [x] Remote/cloud screenshot upload disabled.
- [ ] Operator live proof completed before product-complete claim.

## Validation

- [x] Screen focused validation rerun on pipeline branch.
- [x] AI focused validation rerun on pipeline branch.
- [x] Pipeline E2E/proof script run.
- [x] Playwright screenshot proof run.
- [x] Security negative tests run.
- [x] Performance/cadence proof run.
- [x] Dependency policy rerun after capture dependency narrowing.
- [x] Android emulator MediaProjection proof run.
- [x] `git diff --check`.
- [x] lane/hub guards.
- [ ] `npm run validate` or approved omission.
