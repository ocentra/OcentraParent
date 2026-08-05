# WP01 Network Foundation Eventing Contract

## Scope

- `crates/agent-protocol/src/network_flow.rs`
- `crates/agent-protocol/src/network_flow_eventing.rs`
- `crates/agent-protocol/tests/contract/network_flow_tests.rs`

## Proven boundary

`NetworkFlowObservedEvent` is a direct reusable `ocentra-eventing::envelope::DomainEvent` payload. Its reusable-envelope contract is the distinct versioned `network.flow.eventing.observed` type, while the existing runtime stream remains `network.flow.observed`; this prevents a reusable envelope from being routed to runtime-payload subscribers. It produces a device-scoped aggregate key and device-scoped flow-reference idempotency key, so it can be built, stored, and decoded through `EventEnvelope` without a private network bus.

## Required negative case

A blank device reference is rejected by the reusable eventing identifier validator before an aggregate key can be emitted. A noncanonical schema version is rejected at the Rust `DomainEvent` boundary, and identical flow references from two devices produce distinct idempotency keys. The tests check these typed failures and independent keys rather than accepting an anonymous aggregate, payload-selected schema, or cross-device deduplication.

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
notes: focused tests cover stored-envelope round trip, distinct eventing route, blank device-reference rejection, noncanonical schema rejection, and two-device idempotency separation

command: cargo test -p ocentra-parent-agent-protocol --test contract
exit: 0
result: pass
artifact: target/debug/deps/contract-94a0224961ad0ccc.exe
notes: 214 protocol contract tests passed

command: npm run lint:architecture -- --base origin/main --head HEAD
exit: 0
result: pass
artifact: n/a
notes: full PR diff architecture validation covers every changed Rust source and contract test file
```

## No-claim boundary

This proof covers only the Rust protocol-to-reusable-eventing contract. It does not prove live capture, private content or exact URL truth, policy authority, enforcement execution, service delivery, portal projection, replay semantics beyond the stored-envelope round trip, or platform readiness. WP01 remains open.
