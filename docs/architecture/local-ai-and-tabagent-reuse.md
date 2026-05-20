# Local AI And TabAgent Reuse

Ocentra Parent's child-device agent should become a smart local safety agent, not a thin rule runner. TabAgent and TabAgentServer are reference systems for this direction because they already contain browser capture, native-host bridging, local model runtime, model cache, execution-provider, memory, and knowledge-graph ideas.

This does not mean copying TabAgent wholesale into Ocentra Parent. Ocentra Parent owns the family-safety contracts, privacy boundaries, evidence model, policy model, and enforcement behavior. TabAgent pieces may be reused, extracted, vendored, or co-developed only behind those contracts.

## Product Boundary

- The child-device Rust agent owns capture, memory, local AI, policy evaluation, timers, and enforcement.
- The parent portal owns visibility, rule authoring, approvals, and explanations.
- The parent portal must not run child-safety AI, browser capture, scripts, timers, policy evaluation, or enforcement.
- API AI can help with parent reports, summaries, and assistant workflows later, but it cannot replace the child-device local evaluator in the normal blocking path.

## TabAgent Pieces To Reuse Or Study

- Browser evidence capture: content scripts, page extraction, video/page metadata extraction, temp-tab fallback, and URL context capture.
- Native bridge: extension-to-native host connection lifecycle, queued messages, reconnect behavior, and status reporting.
- Local model runtime: model load/unload/generate boundaries, provider adapters, hardware detection, model cache, and format-specific loaders.
- Memory system: durable facts, semantic recall, task context, and relationship indexing that can help the child agent reason over more than the latest page.
- Knowledge graph: typed relationships between child, device, app, site, domain, video, channel, category, policy, decision, evidence, and incident.
- Route/test structure: typed route metadata, handler registration, and route-level test cases that make unsupported behavior hard to sneak in.

## Child-Agent Memory And Knowledge Graph Expectations

The agent should eventually maintain multiple local memory layers:

- Evidence memory: immutable journaled observations and decision evidence references.
- Recent activity memory: short-window context used for local safety decisions, such as repeated visits, app switching, and time spent.
- Policy memory: parent rules, overrides, schedules, allowances, and previous parent decisions.
- Semantic memory: local embeddings or indexes that help group activity by topic, intent, or risk.
- Knowledge graph: typed entities and typed edges that connect activity, evidence, rules, AI decisions, enforcement actions, and parent approvals.

Derived memory must never become unexplained truth. Every graph edge, summary, semantic match, or remembered pattern that affects a decision must point back to source evidence, a policy version, or a parent action. If the agent cannot cite the facts it used, the decision must degrade to unknown, ask-parent, warn, or another explicit safe state.

## Target Local AI Flow

```text
browser/process/network/mobile observation
  -> normalized typed evidence
  -> encrypted NDJSON journal
  -> SQLite query store
  -> local memory and knowledge-graph update
  -> safety context builder
  -> child-device local model/provider
  -> typed AI safety result
  -> deterministic policy evaluator
  -> enforcement adapter or parent approval path
  -> auditable decision event
```

The local model should not receive raw unbounded data by default. The safety context builder should assemble the smallest relevant typed context: current URL/page/video/app/domain, parent rules, recent activity, known site/app relationships, prior decisions, and evidence references.

## Reuse Strategy

- Start with Ocentra Parent-owned contracts for local AI input, local AI output, memory references, graph references, model status, and provider status.
- Add Rust parity structs before runtime code consumes those contracts.
- Integrate or extract TabAgentServer runtime pieces only after the contract boundary is explicit and tested.
- Prefer a shared crate/workspace strategy for stable runtime pieces if TabAgent and Ocentra Parent will be co-developed.
- Keep storage responsibilities separate: Ocentra Parent's evidence source of truth remains encrypted NDJSON plus SQLite ingest. TabAgent-style memory or graph indexes are derived local indexes, not a replacement for the evidence journal.
- Browser-extension capture belongs on the child device and talks to the child-device agent through a typed native or local API bridge.

## Implementation Guardrails

- Do not add a full memory graph before the first narrow safety evaluator works.
- Do not let the graph, embedding index, or model output directly enforce anything.
- Do not store derived memory without source evidence references.
- Do not make model availability a reason to lose raw evidence.
- Do not hide model calls inside capture, portal, or enforcement modules.
- Do not copy broad TabAgentServer subsystems into Ocentra Parent without deleting unused surfaces and proving the contracts.

## First Useful Slice

The first AI slice should prove a narrow, real path:

1. Build local AI decision contracts with optional memory and graph reference fields.
2. Build Rust parity structs and serializer tests.
3. Create a small safety context builder from real stored evidence and parent rules.
4. Run a local provider adapter in dry-run mode, even if the provider is initially a limited local model path.
5. Convert model output into a typed policy decision.
6. Store the AI result and policy decision with evidence references.

Later slices can add richer TabAgentServer-style memory, graph indexes, model cache, execution-provider selection, browser-extension capture, and advanced local agent planning.
