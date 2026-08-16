# AI UI/UX Requirements Guide

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI UI/UX Requirements Guide`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Goal

The parent must be able to see what AI is doing, what it cannot do, and why a
decision or explanation exists. AI UI must not hide uncertainty.

## Required Surfaces

### AI Runtime

- Local runtime configured/unconfigured.
- Model artifact status.
- Provider availability.
- Hardware fit.
- Current load state.
- Generation state.
- Queue/backpressure state.
- Last checked time.
- Degraded/unavailable reason.

### AI Jobs

- Active jobs.
- Pending jobs.
- Cancelled jobs.
- Timed-out jobs.
- Failed/degraded jobs.
- Source evidence refs.
- Task scope.
- Provider route.
- Claim owner.
- Claim id.
- Lease expiry.
- Claim rejection reason.
- Requeue/dead-letter state.
- Result validation state.
- Child-agent authority state.
- Payload/custody mode.
- Result ref.

### Household AI Providers

- Discovered AI-capable devices.
- Device role: child-agent, parent-controller, parent-observer, ai-provider.
- Provider class: desktop, laptop, mobile-dormant, or mobile-fallback.
- Provider trust and pairing state.
- Provider capability flags.
- Supported job kinds.
- Queue depth.
- Current job state.
- Last heartbeat and reachability.
- Battery, thermal, and resource degraded state.
- Whether provider is eligible for child-safety work.
- Whether provider is worker-only and not policy authority.

### AI Decisions

- Decision action from policy, not model.
- AI evidence result.
- Parent rule refs.
- Evidence refs.
- Model/runtime ref.
- Prompt/template version.
- Confidence and degraded state.
- Human-readable reason.

### Memory And Graph

- Recent activity memory state.
- Semantic memory state.
- Graph index state.
- Source evidence refs.
- Expiry/invalidation state.
- Rebuild state.
- Unsourced memory rejected state.

### Remote Boundary

- Remote disabled by default.
- Parent authorization state.
- Data custody/retention state.
- Redaction state.
- Remote unavailable/degraded state.

## UI Rules

- Do not show raw screenshots by default.
- Do not show unbounded child activity in assistant prompts.
- Do not imply AI is the policy authority.
- Show unavailable, unsupported, low-confidence, and manual-required states as
  first-class product states.
- Keep detailed diagnostic data available for parent/developer proof, but keep
  the default UI readable.
- Every explanation must be traceable to evidence and parent rules.

## Snapshot Requirements

Before claiming UI done, capture screenshots for:

- runtime unconfigured;
- runtime configured but unavailable;
- local model ready;
- job queued/running;
- no provider available;
- desktop provider available;
- mobile provider dormant;
- job claimed by provider;
- competing claim rejected;
- lease expired and requeued;
- provider result accepted;
- provider result rejected;
- child-agent authority shown;
- raw screenshot transfer disabled/default-forbidden;
- timeout/degraded;
- AI result with explanation;
- memory source-citation state;
- remote disabled state;
- screen OCR/VLM permission-required state if screen UI changes.
