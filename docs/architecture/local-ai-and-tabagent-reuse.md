<!-- agent-capsule -->

> Agent Capsule
> Doc: Local AI And TabAgent Reuse
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Local AI And TabAgent Reuse

Ocentra Parent's child-device agent should become a smart local safety agent, not a thin rule runner. TabAgent and TabAgentServer are reference systems for this direction because they already contain browser capture, native-host bridging, local model runtime, model cache, execution-provider, memory, and knowledge-graph ideas.

This does not mean copying TabAgent wholesale into Ocentra Parent. Ocentra Parent owns the family-safety contracts, privacy boundaries, evidence model, policy model, and enforcement behavior. TabAgent pieces may be reused, extracted, vendored, or co-developed only behind those contracts.

## Product Boundary

- The child-device Rust agent owns capture, memory, local AI, policy evaluation, timers, and enforcement.
- The parent portal owns visibility, rule authoring, approvals, and explanations.
- The parent portal must not run child-safety AI, browser capture, scripts, timers, policy evaluation, or enforcement.
- Remote/API AI may help with parent reports, summaries, and assistant workflows
  later only through explicit parent action and data-custody boundaries. It
  cannot replace the child-device local evaluator in the normal blocking path and
  must not create default Ocentra custody of child activity data.

## Reference Evidence Inspected

The following TabAgent and TabAgentServer files are useful evidence for future reuse planning. They are references, not accepted Ocentra Parent contracts:

| Area                     | Reference files                                                                                                                                                                                                                                                                                                                                                           | Reuse lesson                                                                                                                                                                                   |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Browser-to-native bridge | `E:\Desktop\TabAgent\src\Controllers\NativeHostManager.ts`, `E:\Desktop\TabAgent\src\types\native.ts`, `E:\Desktop\TabAgent\TabAgentServer\Rust\native-messaging\src\router.rs`, `E:\Desktop\TabAgent\TabAgentServer\Rust\native-messaging\src\protocol.rs`                                                                                                               | Persistent native connection, queued messages, reconnect state, route metadata, and request/response envelopes are worth studying for child-device bridge status and typed local API behavior. |
| Model lifecycle          | `E:\Desktop\TabAgent\src\backgroundModelManager.ts`, `E:\Desktop\TabAgent\src\Controllers\services\NativeModelService.ts`, `E:\Desktop\TabAgent\src\Controllers\services\NativeInferenceService.ts`, `E:\Desktop\TabAgent\TabAgentServer\Rust\native-messaging\src\routes\models.rs`, `E:\Desktop\TabAgent\TabAgentServer\Rust\native-messaging\src\routes\generation.rs` | Load, unload, progress, generation, halt, and degraded state need explicit status contracts in Ocentra Parent before policy consumes model output.                                             |
| Model cache              | `E:\Desktop\TabAgent\src\DB\idbModel.ts`, `E:\Desktop\TabAgent\TabAgentServer\Rust\model-cache\README.md`, `E:\Desktop\TabAgent\TabAgentServer\Rust\model-cache\src\lib.rs`                                                                                                                                                                                               | Chunked download, manifests, quant status, progressive callbacks, and cache statistics are useful patterns, but Ocentra Parent must keep evidence storage separate from model cache storage.   |
| Execution providers      | `E:\Desktop\TabAgent\TabAgentServer\Rust\execution-providers\README.md`, `E:\Desktop\TabAgent\TabAgentServer\Rust\execution-providers\src\lib.rs`                                                                                                                                                                                                                         | Provider capability, availability checks, fallback ordering, and hardware-specific adapters should become explicit local provider contracts, not hidden runtime choices.                       |
| Memory and graph         | `E:\Desktop\TabAgent\src\DB\idbKnowledgeGraph.ts`, `E:\Desktop\TabAgent\TabAgentServer\Rust\docs\mia_memory.md`, `E:\Desktop\TabAgent\TabAgentServer\Rust\docs\KnowledgeWeaver.md`, `E:\Desktop\TabAgent\TabAgentServer\Rust\knowledge-graph\src\lib.rs`, `E:\Desktop\TabAgent\TabAgentServer\Rust\storage\src\knowledge.rs`                                              | Derived memory, graph edges, semantic indexes, and asynchronous enrichment are useful only when every derived fact cites source evidence, policy versions, or parent actions.                  |

## TabAgent Pieces To Reuse Or Study

- Browser evidence capture: content scripts, page extraction, video/page metadata extraction, temp-tab fallback, and URL context capture.
- Native bridge: extension-to-native host connection lifecycle, queued messages, reconnect behavior, and status reporting.
- Local model runtime: model load/unload/generate boundaries, provider adapters, hardware detection, model cache, and format-specific loaders.
- Memory system: durable facts, semantic recall, task context, and relationship indexing that can help the child agent reason over more than the latest page.
- Knowledge graph: typed relationships between child, device, app, site, domain, video, channel, category, policy, decision, evidence, and incident.
- Route/test structure: typed route metadata, handler registration, and route-level test cases that make unsupported behavior hard to sneak in.

## What Not To Copy

- Do not copy TabAgent UI, dashboard, assistant persona, or broad agent workflows into Ocentra Parent.
- Do not copy stringly route ids, model ids, provider names, or field names into app/runtime code. Ocentra Parent domain packages and Rust protocol constants own those names.
- Do not copy a memory graph as a source of truth. Ocentra Parent source truth remains encrypted NDJSON journal plus SQLite ingest.
- Do not copy remote/API AI or remote model behavior into the child-device blocking
  path.
- Do not copy broad model runtime subsystems before V0.6 contracts and Rust parity tests exist.
- Do not use TabAgent's browser extension boundary to claim unsupported OS-level capture or enforcement.

## Ocentra Ownership Boundaries

The reusable ideas map to Ocentra-owned boundaries:

| Ocentra boundary                                                    | Owns                                                                                                                               |
| ------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `crates/schema`, `crates/parent-runtime-core`, and Rust domain/runtime crates | Local AI request/result DTOs, route snapshots, actions, read models, and parent-facing runtime state.                              |
| `crates/agent-protocol`                                             | Rust transport/protocol mirror only when the Rust service sends, receives, or journals a transport-specific shape.                 |
| Transitional TS edge/generated DTO surfaces                         | Temporary validation adapters or generated DTO imports; no product authority.                                                      |
| Future local AI runtime crate/module                                | Provider lifecycle, model cache references, local model status, generation status, and degraded states.                            |
| Future policy evaluator crate/module                                | Deterministic policy decisioning over parent rules, evidence, and local AI result.                                                 |
| Future enforcement adapter crate/module                             | Platform-specific execution of typed policy decisions and rollback/unavailable reporting.                                          |

## Child-Agent Memory And Knowledge Graph Expectations

The agent should eventually maintain multiple local memory layers:

- Evidence memory: immutable journaled observations and decision evidence references.
- Recent activity memory: short-window context used for local safety decisions, such as repeated visits, app switching, and time spent.
- Policy memory: parent rules, overrides, schedules, allowances, and previous parent decisions.
- Semantic memory: local embeddings or indexes that help group activity by topic, intent, or risk.
- Knowledge graph: typed entities and typed edges that connect activity, evidence, rules, AI decisions, enforcement actions, and parent approvals.

Derived memory must never become unexplained truth. Every graph edge, summary, semantic match, or remembered pattern that affects a decision must point back to source evidence, a policy version, or a parent action. If the agent cannot cite the facts it used, the decision must degrade to unknown, ask-parent, warn, or another explicit safe state.

## Required Reference Shape

Any local memory or graph reference that influences AI, policy, enforcement, or parent explanation must carry:

- Reference id.
- Reference kind: evidence memory, recent activity, policy memory, semantic memory, graph entity, or graph edge.
- Source evidence references from the encrypted journal/query store.
- Source policy version or parent action reference when applicable.
- Derived index version.
- Generated time.
- Confidence or match score when the reference is probabilistic.
- Expiry or invalidation rule for short-window context.

No derived memory or graph reference may drive a block, timeout, or ask-parent decision unless it can cite source evidence or parent intent.

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

- Start with Rust-owned Ocentra Parent contracts for local AI input, local AI output, memory references, graph references, model status, and provider status.
- Keep the provider/runtime adapter boundary explicit through
  [Local AI Provider Runtime Boundary](local-ai-provider-runtime-boundary.md)
  before model execution exists.
- Add Rust serialization, round-trip, and generated-artifact tests before
  runtime or UI code consumes those contracts.
- Integrate or extract TabAgentServer runtime pieces only after the contract boundary is explicit and tested.
- Prefer a shared crate/workspace strategy for stable runtime pieces if TabAgent and Ocentra Parent will be co-developed.
- Keep storage responsibilities separate: Ocentra Parent's evidence source of truth remains encrypted NDJSON plus SQLite ingest. TabAgent-style memory or graph indexes are derived local indexes, not a replacement for the evidence journal.
- Browser-extension capture belongs on the child device and talks to the child-device agent through a typed native or local API bridge.

## Staged Integration Plan

### Stage 1: V0.6 Contracts

- Define local AI input/result, local model status, provider capability, memory reference, graph reference, policy decision, and enforcement audit event contracts in Rust first.
- Add Rust serialization/round-trip tests and TS edge/generated validation tests before runtime code consumes the shapes.
- Add reason code and degraded-state enums that distinguish unavailable model, invalid output, missing evidence, low confidence, and policy conflict.
- Keep remote/API assistant contracts separate from child-device local AI
  contracts, and make data custody explicit before any remote model call exists.

### Stage 2: V0.7 Dry-Run Evaluator

- Build a safety context builder from real stored evidence, parent rules, and optional evidence-backed memory references.
- Add provider lifecycle status using lessons from TabAgent model load/unload/progress and execution-provider availability.
- Parse local model output into typed safety results.
- Run deterministic policy evaluation in dry-run first.
- Journal AI and policy decision events with evidence references.

### Stage 3: V0.8 Enforcement Handoff

- Keep enforcement disabled by default until dry-run evidence proves expected behavior.
- Allow enforcement adapters to consume only typed policy decisions.
- Add timer, ask-parent, rollback, unavailable, and audit event paths.
- Validate service restart and adapter unavailable behavior.

### Stage 4: V4 Parent Assistant

- Add remote/API AI only as a parent-facing explanation and summary layer after
  parent-owned storage or explicit stateless compile boundaries exist.
- Require evidence-cited remote responses.
- Degrade to local-only explanation or unknown when remote/API AI is unavailable.
- Prevent remote/API AI from overriding local policy decisions or stricter parent
  rules.
- Prevent Ocentra-hosted infrastructure from retaining child activity prompts,
  source bundles, or generated reports by default.

## Security And Privacy Risks

- Model prompts may accidentally include sensitive child activity; keep prompt inputs minimal and evidence-referenced.
- Memory and graph indexes may amplify stale or incorrect derived facts; require source references and invalidation.
- Provider caches may store large model files; keep model cache separate from evidence storage and report cache corruption without losing evidence.
- Native/browser bridges can become control channels; require typed contracts, origin/device checks, request ids, and capability reporting.
- Remote/API AI can leak child activity if introduced too early; require explicit
  parent action, data-custody, privacy, retention, and deletion boundaries before
  remote calls.
- Billing and entitlements must not sit inside the child-device safety decision path.

## Validation Expectations

- Rust serialization and round-trip tests for every local AI, model status, memory reference, graph reference, policy decision, and enforcement event shape.
- TypeScript parser tests only for generated validation or untrusted edge decoders.
- Replay/integration tests that build AI context from real journal and SQLite evidence.
- Provider lifecycle tests that exercise unavailable, loading, loaded, degraded, and failed states.
- Dry-run evaluator tests for allow, warn, block, time-limit, ask-parent, unknown, low-confidence, missing evidence, and conflict cases.
- Memory/graph tests proving derived references cannot influence decisions without source evidence.
- No test doubles for model truth, policy decisions, or local transport behavior.

## Implementation Guardrails

- Do not add a full memory graph before the first narrow safety evaluator works.
- Do not let the graph, embedding index, or model output directly enforce anything.
- Do not store derived memory without source evidence references.
- Do not make model availability a reason to lose raw evidence.
- Do not hide model calls inside capture, portal, or enforcement modules.
- Do not copy broad TabAgentServer subsystems into Ocentra Parent without deleting unused surfaces and proving the Rust-owned contracts.

## First Useful Slice

The first AI slice should prove a narrow, real path:

1. Build Rust-owned local AI decision contracts with optional memory and graph reference fields.
2. Build serializer, round-trip, and generated DTO drift tests.
3. Create a small safety context builder from real stored evidence and parent rules.
4. Run a local provider adapter in dry-run mode, even if the provider is initially a limited local model path.
5. Convert model output into a typed policy decision.
6. Store the AI result and policy decision with evidence references.

Later slices can add richer TabAgentServer-style memory, graph indexes, model cache, execution-provider selection, browser-extension capture, and advanced local agent planning.
