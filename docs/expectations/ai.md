<!-- agent-capsule -->

> Agent Capsule
> Doc: AI Feature Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# AI Feature Expectations

AI safety authority is local to the evidence-owning child agent. The normal
safety evaluator runs against typed evidence and parent rules. AI execution may
run on the same child device or on a trusted paired household AI provider, but
the provider is worker-only and returns results for child-agent validation.
Remote/API AI is not part of the normal child safety path and must not receive
child activity by default. Any future remote assistant or report compiler must
be explicitly parent-authorized, evidence-cited, data-custody reviewed, and
outside blocking, timing, or ask-parent decisions.

The parent portal does not run child-safety AI. It authors rules, approvals, and
questions; the child-device agent validates local context, owns the AI work
ledger, validates provider results, and converts accepted AI output into typed
policy inputs.

AI classification is evidence, not household authority. Ocentra can provide local
models, categories, confidence, and explanations, but parent-authored policy
decides whether a category becomes allow, warn, time-limit, ask-parent, or block.

TabAgent and TabAgentServer are reference systems for the future local AI runtime, browser evidence capture, native bridge, model cache, execution-provider, memory, and knowledge-graph direction. The detailed reuse boundary lives in [Local AI And TabAgent Reuse](../architecture/local-ai-and-tabagent-reuse.md). Reuse must happen behind Ocentra Parent-owned contracts; TabAgent must not redefine the parent portal or child-agent safety boundary.

The V0.6 evidence-context reconciliation for browser, app/game, network, screen,
parent-rule, local-runtime, memory, graph, confidence, degraded-state, and
custody boundaries lives in
[Local AI Evidence Context Builder](../architecture/local-ai-evidence-context-builder.md).

## Roadmap Scope

V0.6 defines contracts before runtime AI affects policy or enforcement.

V0.7 runs a local AI policy evaluator in dry-run and decision-producing modes while enforcement remains disabled by default.

V4 may add parent-owned report assistance or optional remote/API explanation.
V4 cannot replace local child-device safety decisions and cannot create default
Ocentra custody of child activity data.

## Parent Outcome

- Parent can understand why an activity was classified as allowed, warned, blocked, time-limited, ask-parent, or unknown.
- Parent can see the stored evidence, parent rules, local model status, and reason codes behind an AI-assisted decision.
- Parent can ask for richer explanations later through local reports,
  parent-owned storage, or an explicitly authorized remote assistant without
  changing the local safety decision that already happened.
- Parent can distinguish "local evaluator unavailable", "low confidence", "missing evidence", "policy conflict", and "remote assistant unavailable" states.

## Child-Device Outcome

- The child-device agent can evaluate a narrow page, video link, app, domain, or recent activity window locally.
- The local evaluator consumes only typed evidence, typed parent rules, recent local context, and optional evidence-backed memory or graph references.
- The local evaluator reads app/game observations, duration summaries, and
  structured digests produced by the Rust agent; it does not scan processes,
  files, windows, or browser state itself.
- The local evaluator returns a schema-valid result that policy can deterministically consume.
- The child-device agent records the local AI result, its model/runtime status reference, evidence references, and degraded state before policy or enforcement acts on it.
- The local evaluator cannot turn a category label into enforcement without a
  matching parent-authored policy rule.

## Platform Scope

- Windows is first for local model runtime proof because Windows is the first production-grade child-agent platform.
- macOS, Linux, Android, and iOS must not claim local AI parity until model runtime, storage, permissions, and resource behavior are proven on those platforms.
- Web is parent portal only. Web may display AI status and explanations, but it does not run child-device safety models, policy evaluation, timers, or enforcement.

## Data Scope

AI input may include:

- Current typed observation: app, process, window, URL, page, video, domain, category, or network context when those capture slices exist.
- Stored evidence references from the encrypted journal and SQLite query store.
- Agent-generated app/game session summaries and digests, including running
  time, foreground time, evidence ids, category candidates, and unknown states.
- Agent-generated network flow summaries and unusual-traffic digests, including
  process references, destinations, counts, bytes where available, VPN/proxy
  indicators, and unknown/encrypted states.
- Local screen-analysis summaries generated from encrypted temporary screen
  queue jobs, including categories, confidence, risk signals, source evidence
  refs, image digest, and deletion state.
- Parent rule references, policy version, child profile reference, device reference, schedule window, and recent activity summary.
- Local memory references and knowledge-graph references only when those references cite source evidence, policy versions, or parent actions.
- Local model/provider status and prompt/template version.
- Explicit source/custody labels for every evidence, rule, runtime, memory, and
  graph reference used by the context builder.

AI input must not include:

- Raw unbounded browser or OS content by default.
- Direct OS, process, window, browser, launcher, or filesystem scanning by the AI
  runtime.
- Direct packet sniffing, raw packet dumps, decrypted payloads, or network
  content capture by the AI runtime.
- Permanent raw screenshot retention or cloud/API AI upload of screenshots under
  the screen-evidence feature.
- Decrypted HTTPS payloads unless a future explicit legal/product boundary approves a specific capture mode.
- Data uploaded to remote/API AI without explicit parent action, data-custody
  contract, privacy boundary, and no-retention or parent-owned-storage behavior.
- Derived memory or graph claims that cannot point back to source evidence.
- Ocentra-hosted child-activity storage, screenshots, browser history, SQLite
  evidence, reports, journals, or parent rules by default.

## Contract Boundary

V0.6 must define Effect Schema contracts in the owning domain packages before runtime code consumes them. The expected contract families are:

- `LocalAiEvaluationInput`: schema version, request id, child profile reference, device reference, current observation reference, evidence references, parent rule references, recent activity window, optional memory references, optional graph references, model request metadata, and prompt/template version.
- `LocalAiSafetyResult`: schema version, action, confidence, unknown/degraded state, reason codes, explanation token or text reference, evidence references, parent rule references, optional memory/graph references, model runtime reference, prompt/template version, and expiry/timer fields when the action is time-based.
- `LocalModelRuntimeStatus`: provider, model id, local path or opaque model reference, load state, capability flags, resource class, degraded state, last checked time, and unavailable reason.
- `LocalProviderCapability`: available providers, hardware/resource constraints, supported tasks, privacy mode, and fallback order.
- Household provider mesh contracts: provider advertisement, heartbeat,
  capability snapshot, provider selection, AI work item, claim request, claim
  decision, lease, result, result validation, dead letter, and mesh transport
  envelope. These contracts must distinguish execution provider from policy
  authority.
- `LocalMemoryReference` and `LocalGraphReference`: reference id, reference type, source evidence references, source policy version or parent action when applicable, generated time, confidence, and derived-index version.
- `LocalAiEvidenceContextBuildRequest`, `LocalAiEvidenceContext`, and
  `LocalAiEvidenceContextBuildResult`: request scope, evidence refs, parent rule
  refs, local runtime refs, memory/graph refs, confidence validation,
  unknown/degraded states, custody labels, and build-state outcome.
- `RemoteAssistantRequest` and `RemoteAssistantResult` for V4 if needed: parent
  question, parent-approved source, permitted evidence references, data-custody
  boundary, model/prompt version, answer, cited evidence references,
  uncertainty, retention state, and failure state.

Rust protocol parity is required when Rust sends, receives, stores, or journals these shapes. Field names, enum values, schema versions, and reason codes must be tested on both sides before runtime behavior depends on them.

## Local Agent Intelligence

The child-device agent may maintain local, evidence-backed intelligence:

- Evidence memory: immutable observations and AI/policy decision references.
- Recent activity memory: current session and short-window behavior context.
- Policy memory: parent rules, overrides, schedules, and prior parent decisions.
- Semantic memory: local indexes or embeddings for topic, intent, and risk grouping.
- Knowledge graph: typed relationships between child, device, app, site, domain, video, channel, category, policy, decision, evidence, incident, and parent approval.

Memory and graph output are derived indexes, not source truth. They must cite stored evidence, policy versions, or parent actions before they can influence AI, policy, or explanations. If the agent cannot explain which evidence or rule a remembered fact came from, that fact cannot drive blocking. It may only contribute to unknown, ask-parent, warn, or another explicit safe state.

## Local AI Flow

```text
captured page/video/app/domain evidence
  -> encrypted journal and SQLite query store
  -> agent-generated summaries or digests
  -> network flow summaries or unusual-traffic digests
  -> optional local screen-analysis summaries
  -> parent rules and recent context
  -> optional evidence-backed memory and graph references
  -> child-owned AI work item
  -> same-device model or trusted household provider
  -> child-agent result validation
  -> schema-valid child-accepted AI safety result
  -> deterministic policy evaluator
  -> dry-run result, parent approval path, or enforcement adapter
  -> auditable decision event
```

## Remote Assistant Boundary

V4 remote assistance may produce richer explanations, trend summaries, parent
Q&A, and unknown classification suggestions. It must remain outside the normal
blocking path and must not store child activity in Ocentra-hosted infrastructure
by default.

Acceptance for remote assistance:

- Requests cite the stored evidence or parent-owned report bundle the parent is
  allowed to use.
- Responses cite stored evidence references and mark uncertainty.
- Remote failures degrade to local-only explanation, unknown, or ask-parent.
- Remote output cannot override a stricter local parent rule or a typed local
  policy decision.
- Remote model prompts and versions are governed and auditable.
- No child activity leaves the device or parent-owned storage without explicit
  parent action, data-custody contract, and retention behavior.

## Failure Behavior

- Invalid AI input is rejected before model invocation.
- Invalid model output is rejected before policy consumes it.
- Provider results are rejected before policy consumes them unless the claim,
  lease, provider, evidence refs, custody state, prompt/template version,
  runtime refs, and child-agent authority all validate.
- Expired lease, wrong-provider, duplicate, stale-provider, revoked-provider,
  unsupported-capability, and custody-mismatch results are rejected.
- Local model unavailable or overloaded returns an explicit degraded result.
- Low-confidence or contradictory output returns unknown, warn, or ask-parent unless an explicit parent rule gives a stricter deterministic answer.
- Missing evidence prevents AI from claiming content understanding.
- Memory or graph references without source evidence are ignored for decisioning.
- Remote/API assistant outage does not disable local child-device evaluation.
- Model cache corruption or missing model files must not lose captured evidence; the agent keeps writing evidence and reports evaluator unavailable.

## Expected Deliverables

- AI input contract.
- AI output contract.
- Local model/provider adapter boundary.
- Remote/API model/provider adapter boundary only when a feature explicitly needs
  remote AI and satisfies the data-custody expectations.
- Local AI runtime status contract: provider, model id, load state, capability, degraded state, and last checked time.
- Local memory reference contract.
- Local knowledge-graph reference contract.
- Safety context builder that assembles typed evidence, parent rules, recent activity, and relevant memory references.
- Context-builder custody checks that reject Ocentra-hosted non-activity
  metadata as child-activity evidence.
- Prompt/version ownership.
- Evidence references.
- Parent rule references.
- Decision action: allow, warn, block, time-limit, ask-parent, or unknown.
- Timer/expiry fields for temporary block or time-limit decisions.
- Confidence/unknown state.
- Failure/degraded behavior.
- Human override feedback path where relevant.

## Acceptance

- Local AI output is schema-validated before any policy or enforcement code consumes it.
- AI output points to stored evidence.
- AI output points to the parent rules it used.
- Memory and knowledge-graph references point to source evidence, policy versions, or parent actions.
- The safety context builder can be tested from real stored evidence and rules, not handwritten fake context.
- Unknown or failed classification is safe and explicit.
- Policy can explain the local AI decision, the evidence, and the parent rule context.
- Remote/API AI is never required for normal child-device blocking.
- Remote/API AI is never required for time-limit or temporary-block decisions.
- Remote/API AI responses cannot override a stricter local parent rule.
- Tests cover parser behavior and decision integration without mocking provider truth.
- Tests cover replay from stored evidence into AI context when a feature adds memory or graph behavior.

## Validation Gates

- TypeScript schema tests prove valid and invalid AI input/output, runtime
  status, context-builder requests/results, memory references, graph
  references, confidence `0..1` validation, and remote assistant payloads.
- Rust parity tests prove identical field names, enum values, schema versions, and serialization for every Rust-crossing AI shape.
- Stored-evidence integration tests build a safety context from the real encrypted journal and SQLite query store.
- Provider lifecycle tests exercise real local provider status, unavailable, load failure, and degraded states without faking model truth.
- Policy integration tests prove ambiguous AI output cannot override explicit parent rules.
- Remote assistant tests prove remote output remains evidence-cited, respects
  data-custody boundaries, and stays outside the child-device blocking path.
- Validation reports exact commands and failures.

## Non-Goals

- Do not claim AI can see content that was not captured.
- Do not let untyped or untraceable AI output directly enforce blocking.
- Do not hide model/provider calls inside unrelated modules.
- Do not make cloud/remote/API AI mandatory for local child-device safety.
- Do not run the child-safety model in the parent portal.
- Do not upload child activity to remote/API AI or Ocentra-hosted workers without
  explicit parent action, data-custody contract, and retention/deletion behavior.
- Do not let local memory, embeddings, or graph edges become unexplained truth.
- Do not replace the encrypted journal and SQLite ingest source of truth with a memory or graph subsystem.
- Do not copy broad TabAgentServer subsystems unless the Ocentra Parent contract boundary is explicit and tested.

## Done Signal

Local AI can evaluate a narrow evidence-backed case, such as a page URL, video link, app, or domain plus parent rules; return a typed allow/warn/block/time-limit/ask-parent/unknown decision; degrade safely on failure; and keep policy and enforcement decisions auditable.
