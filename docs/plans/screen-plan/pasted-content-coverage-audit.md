# Screen Plan Coverage Audit

## Covered

- [x] Local screen summaries.
- [x] Parent opt-in.
- [x] Screen capability/status.
- [x] Capture cadence.
- [x] Trigger capture.
- [x] Capture scope.
- [x] Encrypted temporary queue.
- [x] Local OCR/vision.
- [x] Validated structured result.
- [x] Image digest.
- [x] Deletion state.
- [x] Journal/SQLite read model.
- [x] Policy dry-run/enforcement handoff.
- [x] Portal summary UX.
- [x] No raw image by default.
- [x] Optional screenshot mode.
- [x] Optional live-view mode.
- [x] Platform proof.
- [x] Tests/proof packs.
- [x] Tiered screen-intelligence router.
- [x] Managed browser structured extraction before screenshot.
- [x] OCR before VLM.
- [x] Family AI hub before remote/API.
- [x] Detector prompt packs.
- [x] Cross-slice browser/app/game/social/video/bypass/unknown ownership.
- [x] Native game and unknown-process trigger coverage.
- [x] Real browser/app/timed-cadence capture proof matrix.
- [x] Separate post-screen/post-AI pipeline proof plan.

## Not Claimed Until Proof

- [ ] Raw screenshot retention.
- [ ] Remote/cloud screenshot upload.
- [ ] Live view.
- [ ] Mobile background screen capture.
- [ ] iOS other-app background capture.
- [ ] Protected-surface capture.
- [ ] Enforcement from raw pixels.
- [ ] Open-ended VLM screen descriptions.
- [ ] Browser-only screen ownership.
- [ ] Remote raw screenshot API processing.

## Pasted Plan Reconciliation

The pasted plan required this folder shape:

```text
docs/plans/screen-plan/
  README.md
  source-index.md
  current-screen-snapshot.md
  v0-5-screen-evidence-full-scope-plan.md
  v0-5-screen-platform-deep-dive.md
  v0-5-screen-ai-analysis-plan.md
  v0-5-screen-visibility-live-view-plan.md
  v0-5-screen-test-blueprint.md
  ui-ux-requirements-guide.md
  implementation-checklist.md
  pasted-content-coverage-audit.md
  workpacks/
```

This folder now follows that structure and expands the workpacks to the requested 30-step base plan.

The second pasted plan added workpacks 31-40 for the tiered screen-intelligence router, managed browser capture path, OCR/VLM evaluation, resource scheduling, family hub, detector prompts, and redacted-summary-only remote boundary.

The third pasted instruction clarified that screen evidence is cross-slice: browser, apps, native games, browser games, social/video, bypass tools, unknown activity, and tracking/check-in context may consume screen summaries when needed. It also added native game/app/launcher/unknown-process trigger requirements.

The later proof correction adds a hard split between screen capture proof, AI
analysis proof, and the final combined Screen AI Pipeline proof. Screen capture
can be PR-ready with real trigger/capture/queue/deletion artifacts, but
product-complete screen AI requires the post-screen/post-AI pipeline plan.
