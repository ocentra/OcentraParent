# Current AI Snapshot

## Current State

Ocentra Parent has a meaningful AI foundation, but it is spread across feature
docs, architecture docs, domain contracts, service modules, proof scripts, and
portal fragments.

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
- Activity memory graph contracts and read-model proof pieces.
- Policy dry-run evaluator and enforcement policy dispatch proof pieces.

## Current Gaps

Still missing or incomplete:

- Product-grade local model configuration and artifact selection.
- Verified local model artifact download, integrity, cache, and retention flow.
- Product-grade local inference execution path for safety decisions.
- Cross-slice AI job queue and resource scheduler.
- Model quality validation and confidence calibration.
- OCR execution path.
- Guided local VLM execution path.
- Evidence-backed memory/graph minimal product implementation.
- Full TabAgent code reuse audit and extraction plan.
- Parent explanation UI that cites evidence, rules, model/runtime refs, and
  degraded states.
- Real stored-evidence validation for browser, app/game, tracking, LAN, network,
  and screen slices.
- Enforcement handoff proof that consumes policy decisions only.
- Negative security tests proving AI cannot scan directly, enforce directly, or
  upload sensitive data by default.

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

## Target State

The target product state is:

```text
Typed evidence from capture slices
  -> encrypted journal
  -> SQLite read models
  -> evidence context builder
  -> AI queue/provider route
  -> deterministic/text/OCR/VLM worker
  -> schema-valid AI result
  -> deterministic parent policy
  -> audit and parent explanation
  -> later enforcement handoff only from policy
```

## Immediate Priority

The next AI work should not start with a broad assistant. It should prove the
shared spine:

1. Contract completeness.
2. Rust parity.
3. Real stored-evidence context builder.
4. Local model/runtime status plus provider queue.
5. Local text model dry-run execution with invalid-output rejection.
6. Deterministic policy integration.
7. AI result journal and parent explanation read model.
8. Screen OCR/VLM lanes aligned to the same queue and result contracts.

## Non-Claims Until Proven

- No claim that AI blocks directly.
- No claim that remote/API AI participates in child safety.
- No claim that screen AI understands content without OCR/VLM proof.
- No claim that memory/graph truth can drive policy without source evidence.
- No claim that a model artifact is safe, fast, or high quality before model
  quality proof exists.
- No claim that browser/app/game/tracking AI sees raw sources directly.
