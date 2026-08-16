# V0.7 AI Memory Graph Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `V0.7 AI Memory Graph Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Goal

Create a minimal local memory and graph layer that improves context without
becoming a second source of truth.

## Memory Layers

- Evidence memory: immutable stored observations and decision refs.
- Recent activity memory: short-window local context.
- Policy memory: parent rules, overrides, approvals, and schedules.
- Semantic memory: local derived index for topics, intent, and risk grouping.
- Knowledge graph: typed local edges between child, device, app, site, evidence,
  rule, policy decision, AI result, incident, and parent action.

## Hard Rule

No memory or graph reference may influence AI, policy, enforcement, or parent
explanation unless it cites source evidence, policy version, or parent action.

## Minimal First Graph

- `child -> device`
- `device -> evidence`
- `evidence -> app`
- `evidence -> site`
- `evidence -> category candidate`
- `policy rule -> target`
- `ai result -> evidence`
- `policy decision -> ai result`
- `policy decision -> parent rule`
- `parent action -> policy decision`

## Validation

- Unsourced memory rejected.
- Unsourced graph edge rejected.
- Expired recent memory ignored.
- Stale semantic memory degraded.
- Graph rebuild from journal/read models is possible.
- Parent explanation cites graph source refs.
