# WP01 Network Foundation Eventing Contract

## Scope

- `crates/agent-protocol/src/network_flow.rs`
- `crates/agent-protocol/src/network_flow_eventing.rs`
- `crates/agent-protocol/tests/contract/network_flow_tests.rs`

## Proven boundary

`NetworkFlowObservedEvent` is a direct reusable `ocentra-eventing::envelope::DomainEvent` payload. Its reusable-envelope contract is the distinct versioned `network.flow.eventing.observed` type, while the existing runtime stream remains `network.flow.observed`; this prevents a reusable envelope from being routed to runtime-payload subscribers. It produces a device-scoped aggregate key and a length-prefixed device/flow-reference idempotency key, so it can be built, stored, and decoded through `EventEnvelope` without adding a private network bus.

## Proof routing

- `workpack`: `docs/plans/network-plan/workpacks/01-foundation-contracts-and-eventing.md` (WP01 Foundation Contracts And Eventing)
- `checklist_rows`: `docs/plans/network-plan/03-network-implementation-checklist-and-workpacks.md` rows 1-10; this sub-slice updates only the reusable typed-eventing integration obligation, not the other rows.
- `schema_owner`: `crates/agent-protocol` owns this protocol-local payload; no new shared-schema authority is asserted.
- `rust_owner`: `crates/agent-protocol`
- `protocol_owner`: `crates/agent-protocol`
- `eventing_owner`: `crates/ocentra-eventing`
- `eventing_workpack_ref`: `docs/plans/eventing-plan/workpacks/09-network-consumer-event-chain.md`
- `schema_fixture_ref`: `crates/agent-protocol/tests/contract/network_flow_event_fixtures.rs`
- `rust_parity_ref`: `crates/agent-protocol/tests/contract/network_flow_tests.rs` (`network_flow_observed_event_*`)
- `evidence_grade_state`: not proved by this contract-only sub-slice.
- `policy_handoff_state`: not proved by this contract-only sub-slice.
- `enforcement_authority_state`: not claimed; network observations cannot establish enforcement authority here.
- `private_bus_state`: no private bus is added by this direct `DomainEvent` handoff; the broader WP01 private-bus audit remains open.
- `skipped_risk_note`: service delivery, replay beyond envelope storage, evidence-grade, policy, enforcement, capture, portal, and platform proof were intentionally not run because this change is a protocol/eventing contract repair.

## Required negative case

A blank device reference is rejected by the reusable eventing identifier validator before an aggregate key can be emitted. A noncanonical schema version is rejected at the Rust `DomainEvent` boundary, and identical flow references from two devices produce distinct idempotency keys. A regression pair whose hyphenated device and flow components formerly collapsed is now length-prefixed and remains distinct. The tests check these typed failures and independent keys rather than accepting an anonymous aggregate, payload-selected schema, or cross-device deduplication.

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
notes: focused tests cover stored-envelope round trip, distinct eventing route, blank device-reference rejection, noncanonical schema rejection, two-device separation, and hyphen-boundary collision rejection

command: cargo test -p ocentra-parent-agent-protocol --test contract network_flow
exit: 0
result: pass
artifact: target/debug/deps/contract-94a0224961ad0ccc.exe
notes: 15 focused network-flow contract tests passed, including the typed-eventing handoff and all negative cases in this sub-slice

command: npm run lint:architecture -- --files crates/agent-protocol docs/plans/network-plan/workpacks/01-foundation-contracts-and-eventing.md docs/proof/network-plan/01-network-foundation-eventing-contract.md
exit: 0
result: pass
artifact: n/a
notes: scoped architecture validation covers the edited protocol crate and proof-routing documents
```

## No-claim boundary

This proof covers only the Rust protocol-to-reusable-eventing contract and the unambiguous idempotency-key encoding. It does not prove live capture, private content or exact URL truth, evidence grade, policy authority, enforcement execution, service delivery, portal projection, replay semantics beyond the stored-envelope round trip, or platform readiness. WP01 remains open.
