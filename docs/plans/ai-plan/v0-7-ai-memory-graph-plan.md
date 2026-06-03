# V0.7 AI Memory Graph Plan

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
