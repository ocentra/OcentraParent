# Screen Plan Coverage Audit

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan Coverage Audit`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Covered

- [ ] Local screen summaries.
- [ ] Parent opt-in.
- [ ] Screen capability/status.
- [ ] Capture cadence.
- [ ] Trigger capture.
- [ ] Capture scope.
- [ ] Encrypted temporary queue.
- [ ] Local OCR/vision.
- [ ] Validated structured result.
- [ ] Image digest.
- [ ] Deletion state.
- [ ] Journal/SQLite read model.
- [ ] Policy dry-run/enforcement handoff.
- [ ] Portal summary UX.
- [ ] No raw image by default.
- [ ] Optional screenshot mode.
- [ ] Optional live-view mode.
- [ ] Platform proof.
- [ ] Tests/proof packs.
- [ ] Tiered screen-intelligence router.
- [ ] Managed browser structured extraction before screenshot.
- [ ] OCR before VLM.
- [ ] Household provider route before remote/API.
- [ ] Detector prompt packs.
- [ ] Cross-slice browser/app/game/social/video/bypass/unknown ownership.
- [ ] Native game and unknown-process trigger coverage.
- [ ] Real browser/app/timed-cadence capture proof matrix.
- [ ] Separate post-screen/post-AI pipeline proof plan.

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
