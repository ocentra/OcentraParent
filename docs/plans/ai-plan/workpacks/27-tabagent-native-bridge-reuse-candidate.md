# 27 - TabAgent Native Bridge Reuse Candidate

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
