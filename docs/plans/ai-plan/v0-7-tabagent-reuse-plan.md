# V0.7 TabAgent Reuse Plan

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
