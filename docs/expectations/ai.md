# AI Feature Expectations

AI is a core child-device product layer. The safety evaluator for child activity runs locally on the child device against typed evidence and parent rules. API AI is secondary and may assist with parent reports, unknown classification, and remote summaries after privacy boundaries are explicit, but it is not the normal decision maker for blocking, timing, or asking the parent.

The parent portal does not run the child-safety model. It authors rules and shows explanations; the child-device agent runs the local model and converts model output into typed decisions.

TabAgent and TabAgentServer are reference systems for the future local AI runtime, browser evidence capture, native bridge, model cache, execution-provider, memory, and knowledge-graph direction. The detailed reuse boundary lives in [Local AI And TabAgent Reuse](../architecture/local-ai-and-tabagent-reuse.md). Reuse must happen behind Ocentra Parent-owned contracts; TabAgent must not redefine the parent portal or child-agent safety boundary.

## Expected Deliverables

- AI input contract.
- AI output contract.
- Local model/provider adapter boundary.
- API model/provider adapter boundary only when a feature explicitly needs remote AI.
- Local AI runtime status contract: provider, model id, load state, capability, degraded state, and last checked time.
- Local memory reference contract.
- Local knowledge-graph reference contract.
- Safety context builder that assembles typed evidence, parent rules, recent activity, and relevant memory references.
- Prompt/version ownership.
- Evidence references.
- Parent rule references.
- Decision action: allow, warn, block, time-limit, ask-parent, or unknown.
- Timer/expiry fields for temporary block or time-limit decisions.
- Confidence/unknown state.
- Failure/degraded behavior.
- Human override feedback path where relevant.

The local flow is:

```text
captured page/video/app/domain evidence
  -> parent rules and recent context
  -> local memory and knowledge-graph context
  -> child-device local model
  -> typed allow/warn/block/time-limit/ask-parent/unknown decision
  -> local enforcement adapter or parent approval path
```

## Local Agent Intelligence

The child-device agent should become smarter over time by maintaining local, evidence-backed memory:

- Evidence memory: immutable observations and AI/policy decision references.
- Recent activity memory: current session and short-window behavior context.
- Policy memory: parent rules, overrides, schedules, and prior parent decisions.
- Semantic memory: local indexes or embeddings for topic, intent, and risk grouping.
- Knowledge graph: typed relationships between child, device, app, site, domain, video, channel, category, policy, decision, evidence, incident, and parent approval.

Memory and graph output are derived indexes, not source truth. They must cite stored evidence, policy versions, or parent actions before they can influence AI, policy, or explanations. If the agent cannot explain which evidence or rule a remembered fact came from, that fact cannot drive blocking.

## Acceptance

- Local AI output is schema-validated before any policy or enforcement code consumes it.
- AI output points to stored evidence.
- AI output points to the parent rules it used.
- Memory and knowledge-graph references point to source evidence, policy versions, or parent actions.
- The safety context builder can be tested from real stored evidence and rules, not handwritten fake context.
- Unknown or failed classification is safe and explicit.
- Policy can explain the local AI decision, the evidence, and the parent rule context.
- API AI is never required for normal child-device blocking.
- API AI is never required for time-limit or temporary-block decisions.
- API AI responses cannot override a stricter local parent rule.
- Tests cover parser behavior and decision integration without mocking provider truth.
- Tests cover replay from stored evidence into AI context when a feature adds memory or graph behavior.

## Non-Goals

- Do not claim AI can see content that was not captured.
- Do not let untyped or untraceable AI output directly enforce blocking.
- Do not hide model/provider calls inside unrelated modules.
- Do not make cloud/API AI mandatory for local child-device safety.
- Do not run the child-safety model in the parent portal.
- Do not upload child activity to API AI without explicit privacy and parent-control boundaries.
- Do not let local memory, embeddings, or graph edges become unexplained truth.
- Do not replace the encrypted journal and SQLite ingest source of truth with a memory or graph subsystem.
- Do not copy broad TabAgentServer subsystems unless the Ocentra Parent contract boundary is explicit and tested.

## Done Signal

Local AI can evaluate a narrow evidence-backed case, such as a page URL, video link, app, or domain plus parent rules; return a typed allow/warn/block/time-limit/ask-parent/unknown decision; degrade safely on failure; and keep policy and enforcement decisions auditable.
