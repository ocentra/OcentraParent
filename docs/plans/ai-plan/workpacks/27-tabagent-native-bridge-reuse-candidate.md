# 27 - TabAgent Native Bridge Reuse Candidate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `27 - TabAgent Native Bridge Reuse Candidate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Native bridge lessons become Ocentra local service route/status behavior with
typed request ids, queueing, reconnect, timeout, and unavailable states.

## Where We Are

TabAgent's `NativeHostManager.ts`, `types/native.ts`, Rust router, and protocol
files show useful connection and dispatch patterns.

## Checklist

- [ ] Translate connection status to Ocentra route status.
- [ ] Translate queued message handling to Ocentra local command queue rules.
- [ ] Translate timeout/reconnect events into typed degraded states.
- [ ] Keep route ids in domain/protocol packages.
- [ ] Add local transport tests without test doubles.

## Proof

- Bridge boundary tests.
- Timeout/reconnect tests.
- String id guard passes.
