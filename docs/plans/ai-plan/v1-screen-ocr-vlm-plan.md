# V1 Screen OCR VLM Plan

## Goal

Make screen evidence a shared local visual evidence layer for browser, apps,
native games, browser games, social/video, bypass tools, unknown activity, and
tracking context when needed.

## Capture To AI Flow

```text
approved capture scope
  -> temporary encrypted screen job
  -> OCR and/or guided VLM
  -> local screen summary
  -> AI evidence context builder
  -> text model or deterministic policy support
  -> policy decision
  -> deletion proof and audit
```

## Worker Split

- OCR extracts visible text.
- Guided VLM answers scoped visual safety questions.
- Local text LLM reasons over typed OCR/screen-summary JSON.
- Deterministic classifier handles known labels and policy schedules.

## Screen Scope Rules

- Managed browser capture is preferred for browser pages.
- Active app/window capture is preferred for native apps and games.
- Selected app/window capture is parent-scoped.
- Full display capture is strict opt-in.
- Unsupported/protected/permission-required surfaces are honest degraded states.

## Validation

- OCR summary from temporary local screen job.
- Guided VLM summary from approved scope.
- Raw image deletion proof.
- Screenshot never sent remote by default.
- Screen summary cites image digest and evidence refs.
- Screen result cannot directly enforce.
- Portal screenshot shows supported, unavailable, permission-required, deleted,
  and degraded states.
