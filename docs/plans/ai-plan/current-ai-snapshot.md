# Current AI Snapshot

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `Current AI Snapshot`
> Kind: current snapshot; read for status/gap claims.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Current State

Ocentra Parent has a meaningful AI foundation, but it is spread across feature
docs, architecture docs, domain contracts, service modules, proof scripts, and
portal fragments.

The 2026-08-15 source-first audit is retained in [CODE_AUDIT.md](CODE_AUDIT.md):
11 of 48 workpacks have their bounded Phase 1 production code and expected test
code written, while 37 remain partial or missing. All 48 now have reviewed graph
topology. This count does not claim that tests, Enforcer, proof, CI, or the whole
workpack are complete.

Already present:

- AI expectation doc and local-first custody boundary.
- Local AI and TabAgent reuse architecture.
- Local AI provider runtime boundary.
- Local AI evidence context builder spec.
- Local AI safety evaluator feature doc.
- Parent assistant feature doc and provider routing proof.
- Browser URL/video, social/feed, and browser-game AI planning inside
  `browser-plan`.
- Screen capture and screen-intelligence planning inside `screen-plan`.
- Runtime/provider status, local AI chat generation, provider scheduler, and
  parent assistant proof scripts.
- Reusable Rust eventing infrastructure in `crates/ocentra-eventing`, with
  existing network and parent/child consumer examples that AI work should
  follow instead of adding direct capture-to-AI coupling.
- LAN AI job submit/status and legacy screen family-hub proof primitives exist,
  but the target architecture is the Household AI Provider Mesh. Those existing
  primitives are precursors only: they do not yet prove decentralized provider
  discovery, claim/lease, child-agent result validation, duplicate prevention,
  retry/dead-letter, mobile dormant/fallback policy, child-agent-only policy
  authority, or physical household LAN execution.
- Activity memory graph contracts and read-model proof pieces.
- Policy dry-run evaluator and enforcement policy dispatch proof pieces.
- A real configured `llama.cpp` process execution boundary, typed timeout and
  unavailable states, and a singleton per-device scheduler.
- Household LAN AI claim/lease/idempotency/requeue/dead-letter paths with
  child-owned validation and raw-screen-transfer rejection tests.
- A live portal AI runtime surface for latest runtime, LAN job, memory graph,
  remote-boundary, and degraded/unavailable projections.

## Current Gaps

Still missing or incomplete:

- Product-grade local model configuration and artifact selection.
- Verified local model artifact download, integrity, cache, and retention flow.
- Product-grade local inference execution path for safety decisions.
- Cross-slice AI job queue and resource scheduler.
- A neutral durable general AI work-item lifecycle; the implemented scheduler
  and LAN lease state are narrower precursors.
- Household AI provider mesh contracts and runtime proof. Existing local
  provider scheduler proof does not prove cross-device provider discovery,
  claim/lease, idempotency, result validation, two-device LAN execution, no raw
  screenshot transfer, or child-agent policy authority.
- Model quality validation and confidence calibration.
- OCR execution path.
- Guided local VLM execution path.
- Evidence-backed memory/graph minimal product implementation.
- Semantic memory, expiry/invalidation, and the policy/result/action graph edge
  families; the current graph is an Activity SQLite projection.
- Full TabAgent code reuse audit and extraction plan.
- Parent explanation UI that cites evidence, rules, model/runtime refs, and
  degraded states.
- Real stored-evidence validation for browser, app/game, tracking, LAN, network,
  and screen slices.
- Enforcement handoff proof that consumes policy decisions only.
- Negative security tests proving AI cannot scan directly, enforce directly, or
  upload sensitive data by default.
- A canonical AI-result journal with SQLite ingest/replay and a unified parent
  decision-explanation read model.
- A verified model download/extract/checksum/license/corruption/resume pipeline;
  current code selects assets and paths but does not install or verify them.

## Current Product Risk

The biggest risk is not lack of AI code. The risk is a loose AI boundary where
each slice creates a local helper, prompt, cache, or explanation path with its
own truth. That would create duplicate policy authority and make validation
weak.

This plan prevents that by making AI a shared local safety subsystem with:

- one contract family;
- one context-builder boundary;
- one job/provider routing boundary;
- one result journal shape;
- one memory/graph source-citation rule;
- one portal explanation model;
- one remote/API boundary.
- one event-driven AI consumer boundary on top of `crates/ocentra-eventing`,
  so capture, AI, policy, action, audit, read-model, and deletion steps remain
  uncoupled.
- one Household Mesh Bridge boundary for cross-device AI execution, keeping
  `ocentra-eventing` local to each runtime and translating only selected,
  validated local events into authenticated LAN messages.

## Target State

The target product state is:

```text
Typed evidence from capture slices
  -> encrypted custody/journal/read-model records
  -> typed `ocentra-eventing` evidence event
  -> AI work item and provider route
  -> same-device deterministic/text/OCR/VLM worker or trusted household provider
  -> child-agent result validation
  -> schema-valid AI result event
  -> deterministic parent policy event
  -> audit, parent explanation, and deletion/read-model events
  -> later enforcement handoff only from policy events
```

## Immediate Priority

The next AI work should not start with a broad assistant. It should prove the
shared spine:

1. Contract completeness.
2. Rust parity.
3. Real stored-evidence context builder.
4. Local model/runtime status plus provider queue.
5. Event-driven AI consumer contracts on `crates/ocentra-eventing`.
6. Household AI provider mesh contracts: provider advertisement/heartbeat,
   work item, claim/lease, result validation, custody, and child-agent authority.
7. Local text model dry-run execution with invalid-output rejection.
8. Deterministic policy integration.
9. AI result journal and parent explanation read model.
10. Screen OCR/VLM lanes aligned to the same queue and result contracts.

## Non-Claims Until Proven

- No claim that AI blocks directly.
- No claim that remote/API AI participates in child safety.
- No claim that screen AI understands content without OCR/VLM proof.
- No claim that memory/graph truth can drive policy without source evidence.
- No claim that a model artifact is safe, fast, or high quality before model
  quality proof exists.
- No claim that browser/app/game/tracking AI sees raw sources directly.
- No claim that `ocentra-eventing` is one shared LAN-wide event bus.
- No claim that household mesh execution works until provider claim/lease,
  result-validation, topology, no-raw-screen-transfer, and child-agent authority
  proofs exist.
- No claim that mobile providers process heavy work by default.
