# V0.7 AI Context Builder Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `V0.7 AI Context Builder Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

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
