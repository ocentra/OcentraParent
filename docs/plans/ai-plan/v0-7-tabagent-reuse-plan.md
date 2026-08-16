# V0.7 TabAgent Reuse Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `V0.7 TabAgent Reuse Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Goal

Use TabAgent as a reference for runtime, bridge, cache, providers, memory, and
graph patterns without allowing TabAgent to define Ocentra safety behavior.

## Reuse Candidates

- Native bridge connection lifecycle.
- Request id, response, timeout, queue, reconnect, and status patterns.
- Model load/unload/generation route structure.
- Model cache progress and manifest patterns.
- Provider capability and fallback-order patterns.
- Knowledge graph local persistence and query patterns.

## Non-Reuse

- TabAgent UI.
- TabAgent assistant persona.
- TabAgent remote/API behavior.
- TabAgent stringly route ids.
- TabAgent browser extension as an Ocentra policy authority.
- TabAgent memory graph as source truth.
- TabAgent cache as evidence storage.

## Extraction Gate

Before extracting or vendoring any TabAgent piece:

- map it to an Ocentra TypeScript contract;
- add Rust parity if Rust consumes it;
- remove unused behavior;
- document license and source;
- prove unavailable/degraded states;
- prove no remote child activity upload by default.

## Validation

- Native bridge boundary test.
- Model cache/evidence storage separation test.
- Provider route contract test.
- Graph source-citation test.
- No copied string ids in runtime/app source.
