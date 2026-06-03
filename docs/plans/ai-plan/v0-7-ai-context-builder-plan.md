# V0.7 AI Context Builder Plan

## Goal

Build one local evidence context builder that every AI consumer uses. Capture
slices produce evidence; the context builder selects, normalizes, minimizes, and
validates that evidence before a model or deterministic classifier sees it.

## Inputs

- Stored browser evidence.
- Stored app/game evidence.
- Stored network-flow evidence.
- Stored screen-summary evidence.
- Stored tracking/location evidence.
- Stored LAN/device evidence when relevant to device context.
- Parent rules, schedules, approvals, and policy versions.
- Recent activity window.
- Runtime/provider refs.
- Prompt/template version.
- Evidence-backed memory and graph refs.

## Outputs

- ready context;
- partial context;
- insufficient context;
- unavailable context;
- rejected context.

## Required Rejections

- Missing evidence refs.
- Child activity labeled as Ocentra-hosted non-activity.
- Unsourced memory/graph refs.
- Raw screenshot or raw browser body in default context.
- Process/window/network evidence promoted to exact URL.
- Probabilistic claim without confidence.
- Remote/API route requested for normal child safety.

## Validation

- Build contexts from real stored evidence and parent rules.
- Test every source/custody label.
- Test each degraded/unknown reason.
- Prove model prompt contains only minimized context.
- Prove missing evidence yields insufficient or rejected state.
