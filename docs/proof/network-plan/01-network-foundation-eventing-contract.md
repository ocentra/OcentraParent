# WP01 Network Foundation Eventing Contract

## Scope

- `crates/agent-protocol/src/network_flow.rs`
- `crates/agent-protocol/src/network_flow_eventing.rs`
- `crates/agent-protocol/tests/contract/network_flow_tests.rs`

## Proven boundary

`NetworkFlowObservedEvent` is a direct reusable `ocentra-eventing::envelope::DomainEvent` payload. It produces the versioned `network.flow.observed` contract, a device-scoped aggregate key, and a flow-reference idempotency key, so it can be built, stored, and decoded through `EventEnvelope` without a private network bus.

## Required negative case

A blank device reference is rejected by the reusable eventing identifier validator before an aggregate key can be emitted. The test checks the typed empty-identifier failure, rather than accepting an anonymous network aggregate.

## Validation record

```text
command: cargo fmt --check --package ocentra-parent-agent-protocol
exit: 0
result: pass
artifact: n/a
notes: touched Rust protocol source and contract test are formatted

command: cargo test -p ocentra-parent-agent-protocol --test contract network_flow_observed_event -- --nocapture
exit: 0
result: pass
artifact: target/debug/deps/contract-94a0224961ad0ccc.exe
notes: 2 focused tests passed: stored-envelope round trip and blank device-reference rejection

command: cargo test -p ocentra-parent-agent-protocol --test contract
exit: 0
result: pass
artifact: target/debug/deps/contract-94a0224961ad0ccc.exe
notes: 212 protocol contract tests passed
```

## No-claim boundary

This proof covers only the Rust protocol-to-reusable-eventing contract. It does not prove live capture, private content or exact URL truth, policy authority, enforcement execution, service delivery, portal projection, replay semantics beyond the stored-envelope round trip, or platform readiness. WP01 remains open.
