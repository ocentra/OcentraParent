# V0.5 Screen Test Blueprint

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `V0.5 Screen Test Blueprint`
> Kind: test blueprint reference; read only when local expectations route here.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Test Rule

No fake-green testing. Tests must exercise real contracts, parsers, services, queues, local transports, UI paths, and platform proof where applicable.

Synthetic images are allowed only as image input fixtures for the real local pipeline.

Contract tests are not enough. Product proof must include real trigger runs:
browser use, app use, timed cadence, and disabled-state suppression. The live
proof requirements are detailed in
[V0.5 Real Capture Proof Matrix](v0-5-real-capture-proof-matrix.md).

## Proposed Test Folders

```text
tests/screen/
  unit/
  integration/
  security/
  platform/
  e2e/
playwright/
```

## Unit Tests

- Settings disabled by default.
- `retainRawImage` false for V0.5.
- `deleteAfterSuccess` true.
- `deleteAfterExpiry` true.
- Cadence bounds enforced.
- Strict mode explicit only.
- Capture scope enum valid.
- Trigger enum valid.
- Queue job requires encrypted image ref.
- Queue job requires TTL.
- Queue job requires deletion required true.
- Analysis result requires evidence refs.
- Analysis result confidence is 0..1.
- Invalid category rejected.
- Risk signals are 0..1.
- OCR snippets rejected when disabled.
- Deletion state required.
- Policy cannot consume raw image.
- Screen route returns `no_screen_needed` when structured evidence is enough.
- Screen route uses OCR before VLM when OCR can answer.
- Screen route returns manual-required when VLM is unavailable and remote is not approved.
- Detector prompts reject private-message/name/credential output.

## Integration Tests

- Disabled setting produces no capture jobs.
- Ready capability plus cadence creates queue job.
- Foreground app change trigger creates queue job.
- Managed browser URL change trigger creates queue job.
- Debounce prevents queue flood.
- Queue encrypts image reference.
- TTL expiry deletes image.
- Successful analysis deletes image.
- Failed analysis retries within max retry count.
- Invalid model output becomes invalid state.
- Valid summary writes journal.
- SQLite read model replays from journal.
- Policy dry-run consumes summary evidence.
- Raw image path never appears in portal DTO.
- Managed browser structured extraction runs before screenshot capture.
- CDP screenshot is scoped to managed browser page/window/crop.
- Native game foreground trigger creates active-window route when allowed.
- Unknown process foreground trigger creates manual-required or active-window route by policy.
- Local AI scheduler runs one heavy job at a time.
- Real managed-browser URL change creates a real capture job or real structured-skip proof.
- Real active app foreground change creates a real capture job.
- Real timed cadence creates at least three captures at bounded intervals.
- Disabling screen analysis stops future cadence jobs.

## Security Tests

- Protected surface returns protected-surface state.
- Screen locked returns screen-locked state.
- Credential prompt suppresses capture.
- Password field OCR suppressed.
- Raw image cannot be exported by default.
- Remote AI upload disabled.
- Remote screenshot sync disabled.
- Live view unavailable unless explicit mode.
- Delete-failed state visible.
- Manual-required cannot execute live view.
- Invalid AI output cannot drive enforcement.
- Open-ended screen description prompts are rejected.
- Raw screenshot remote API path is rejected by default.
- Browser-only screen ownership is rejected in docs/status proof.

## E2E Tests

- Parent enables observe-only screen analysis.
- Agent captures approved scope.
- Image enters encrypted queue.
- Local OCR/vision returns summary.
- Summary is schema-valid.
- Raw image is deleted.
- Journal and SQLite store only summary/digest/deletion state.
- Portal shows summary and confidence.
- Policy dry-run sees category but does not enforce.
- Parent disables screen analysis and no capture jobs are created.
- Protected surface is active and capture is skipped.
- Image expires before analysis and is deleted.
- Browser URL/title/DOM answers policy question and no screenshot is captured.
- Native game process starts and screen summary links to game/session evidence.
- Launcher-only state does not count as active game proof.
- Household provider route is selected before remote route for hard local cases,
  and provider output is not policy-authoritative until the child agent accepts
  it.
- Browser-use trigger proof runs against controlled local social/video, education/video, social-feed, browser-game, shopping, and productivity fixtures.
- App-use trigger proof runs against at least one real foreground app and one controlled unknown/game fixture.
- Timed cadence proof uses a short test interval, records actual timestamps, and proves queue debounce/backpressure.
- Operator live proof records YouTube ordinary video, YouTube or Vimeo educational video, Vimeo video, Facebook/social surface, browser game, shopping, school/productivity, and unsupported/protected states before product-complete claim.

## Capture Plus Analysis Pipeline Tests

- Real trigger produces capture job.
- Capture job enters encrypted queue.
- OCR/VLM/text/deterministic route analyzes the captured evidence.
- AI result is schema-valid or degraded explicitly.
- Policy dry-run consumes summary/evidence refs.
- Parent portal shows trigger source, AI summary, policy result, and deletion state.
- Raw image deletion is proved after success or TTL.
- Disabled state produces no capture, no AI analysis, and no screen-derived policy action.

## Playwright UI Tests

- Screen disabled state.
- Permission required state.
- Capability ready state.
- Model unavailable state.
- Queue unavailable state.
- Protected surface state.
- Summary card.
- Confidence badge.
- Unknown result.
- Evidence drawer.
- Deletion proof.
- Retention setting.
- Trigger setting.
- Cadence setting.
- Policy dry-run result.
- Live view unavailable state.
- Screenshot retention separate mode.
- Platform capability matrix.
- Screen intelligence route diagnostics.
- Game/app/browser evidence refs in summary card.
- Remote-redacted-only disabled/default state.

## Platform Manual Proof

Each platform proof run should write artifacts under:

```text
output/screen-plan-proof/<platform>/
```

Required platform artifacts:

- capability/support proof;
- permission/consent proof;
- capture proof;
- protected/unsupported proof;
- local OCR proof;
- queue encryption proof;
- deletion proof;
- no-default-remote-upload proof.

## Real Proof Artifact Root

Real trigger and operator proof artifacts must be written under:

```text
output/screen-plan-proof/real-capture/
```

The minimum folder set is:

```text
managed-browser-url-social-video/
managed-browser-url-education-video/
managed-browser-url-social-feed/
managed-browser-url-game/
native-app-foreground/
native-game-foreground/
unknown-process-foreground/
timed-cadence-active-window/
disable-stops-cadence/
operator-live-url-proof/
```
