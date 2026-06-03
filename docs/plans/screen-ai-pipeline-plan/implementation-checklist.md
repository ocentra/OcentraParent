# Screen AI Pipeline Implementation Checklist

A checkbox may be marked `[x]` only after the matching artifact exists under:

```text
output/screen-ai-pipeline-proof/
```

## Current Stacked Proof Snapshot

These entries are current branch proof status, not broad product-complete
claims.

| Proof                                    | Status                   | Artifact                                                                                          | Non-claim                                                                                                                                                                                          |
| ---------------------------------------- | ------------------------ | ------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Real Windows active-window capture       | P3 proved                | `output/screen-plan-proof/real-capture/manual-parent-test-active-window/proof-summary.json`       | Remaining implementation task: connect the adapter into service/runtime/read-model after crate-lock coordination; this is not a service-wiring claim.                                              |
| Real Windows scope-matrix capture        | P3 proved                | `output/screen-plan-proof/real-capture/scope-matrix/proof-summary.json`                           | Proves adapter scopes only; parent-controlled product setting, scheduler, and disable suppression remain implementation work.                                                                      |
| Desktop adapter path                     | P2 implementation path   | `crates/screen-capture-adapter/src/lib.rs`                                                        | Windows/macOS use `xcap`; Linux uses a real X11 command backend. Live macOS and Linux Wayland/root-display proof still must run before those platform claims are complete.                         |
| Linux WSLg selected-window capture       | P3 local WSLg proved     | `output/screen-plan-proof/linux-wslg/proof-summary.json`                                          | Proves WSLg/X11 selected-window capture only; does not claim WSLg root display, native Wayland portal, or broad Linux compositor parity.                                                           |
| Android MediaProjection emulator capture | P3 local emulator proved | `output/screen-plan-proof/android-mediaprojection/proof-summary.json`                             | Proves explicit OS consent, foreground-service capture, frame digest, and raw-temp deletion on Android API 35 emulator only; physical-device parity and silent background capture are not claimed. |
| Browser-window capture harness           | P3 proved                | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                         | Does not claim browser-plan managed URL trigger integration.                                                                                                                                       |
| Timed two-frame cadence harness          | P3 proved                | `output/screen-plan-proof/real-capture/trigger-matrix/proof-summary.json`                         | Does not claim product scheduler/disable suppression wiring yet.                                                                                                                                   |
| Local VLM analysis of captured screen    | P3 proved                | `output/ai-plan-proof/real-analysis/manual-browser-education-vlm/02-screen-analysis-result.json`  | Uses controlled education fixture; more real social/video/game/app cases remain.                                                                                                                   |
| Local AI safety result                   | P3 contract proved       | `output/ai-plan-proof/real-analysis/manual-browser-education-vlm/04-local-ai-safety-result.json`  | Remaining implementation task: service runtime must consume this result; script proof is not product-complete.                                                                                     |
| Policy dry-run decision                  | P3 contract proved       | `output/ai-plan-proof/real-analysis/manual-browser-education-vlm/06-policy-dry-run-decision.json` | Remaining implementation task: wire the Rust evaluator/action path after crate-lock coordination; this is not a runtime-action claim.                                                              |
| Raw image deletion after analysis        | P3 proved                | `output/ai-plan-proof/real-analysis/manual-browser-education-vlm/03-deletion-after-analysis.json` | Retention/live-view opt-in modes are separate implementation work and are not claimed here.                                                                                                        |

## Prerequisite Gates

- [ ] Screen capture proof PR merged or explicitly stacked.
- [ ] AI analysis proof PR merged or explicitly stacked.
- [ ] Pipeline branch contains both prerequisite implementations.
- [ ] Prerequisite commits recorded in proof artifacts.

## Real Trigger Gates

- [ ] Managed browser social/video trigger.
- [ ] Managed browser education/video trigger.
- [ ] Managed browser social/feed trigger.
- [ ] Managed browser game/cloud-game trigger.
- [ ] Native app foreground trigger.
- [ ] Native game or controlled game-window trigger.
- [ ] Unknown process/app trigger.
- [ ] Timed cadence trigger.
- [ ] Disabled setting suppression.
- [ ] Protected/permission-required skip.

## AI Analysis Gates

- [ ] OCR route proof where visible text is enough.
- [x] Guided VLM route proof where visual classification is needed.
- [ ] Local text model route proof over typed context.
- [ ] Deterministic route proof where structured evidence is enough.
- [ ] Low confidence degrades safely.
- [ ] Invalid output cannot reach policy.

## Policy And Action Gates

- [ ] Observe policy result.
- [x] Allow policy result.
- [ ] Warn policy result.
- [ ] Ask-parent policy result.
- [ ] Time-limit policy result.
- [ ] Block dry-run or real adapter result.
- [ ] Unknown/manual-required result.
- [ ] AI cannot override stricter parent rule.

## Portal And Proof Gates

- [ ] Journal/read model contains trigger, capture, AI, policy, and deletion refs.
- [ ] Parent portal screenshot shows the full chain.
- [ ] Parent explanation cites evidence and rules.
- [x] Raw image deletion proof linked.
- [ ] Remote/cloud screenshot upload disabled.
- [ ] Operator live proof completed before product-complete claim.

## Validation

- [x] Screen focused validation rerun on pipeline branch.
- [x] AI focused validation rerun on pipeline branch.
- [x] Pipeline E2E/proof script run.
- [ ] Playwright screenshot proof run.
- [x] Security negative tests run.
- [x] Performance/cadence proof run.
- [x] Dependency policy rerun after capture dependency narrowing.
- [x] Android emulator MediaProjection proof run.
- [x] `git diff --check`.
- [x] lane/hub guards.
- [ ] `npm run validate` or approved omission.
