# V1 Screen OCR VLM Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `V1 Screen OCR VLM Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Goal

Make screen evidence a shared local visual evidence layer for browser, apps,
native games, browser games, social/video, bypass tools, unknown activity, and
tracking context when needed.

## Capture To AI Flow

```text
approved capture scope
  -> temporary encrypted screen job
  -> screen evidence / redacted crop / screen summary
  -> AI work item queued by child agent
  -> same-device OCR/VLM or trusted household provider route
  -> provider returns typed OCR/VLM/screen result
  -> child agent validates result and custody
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
- Household AI providers may execute bounded OCR/VLM work only through the
  mesh claim/lease/result-validation path.

## Screen Payload Modes

Screen-derived AI jobs default to `raw-image-forbidden`.

Allowed payload modes:

- metadata-only;
- screen-summary-only;
- OCR-text-only;
- redacted-crop;
- encrypted-local-artifact-ref;
- raw-image-forbidden;
- raw-image-explicit-opt-in-only.

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
- Raw screenshot is not sent to LAN providers by default.
- Redacted crop payload is bounded and evidence-cited.
- Screen summary cites image digest and evidence refs.
- Child agent validates deletion/custody before policy.
- Provider cannot retain raw payload unless explicit opt-in contract exists.
- Screen result cannot directly enforce.
- Portal screenshot shows supported, unavailable, permission-required, deleted,
  and degraded states.
