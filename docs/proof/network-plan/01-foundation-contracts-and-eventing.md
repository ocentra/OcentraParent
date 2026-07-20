# WP01 Foundation Contracts And Eventing Proof

Run context: 2026-07-18, branch `codex/network-wp01-foundation-contract-eventing`, base commit `2973ce0c9` before this packet's commit.

## Automated evidence

| Command | Result | Coverage |
| --- | --- | --- |
| `cargo test -p ocentra-network-evidence` | pass | 252 tests |
| `cargo test -p ocentra-parent-agent-protocol network` | pass | protocol contracts, mutation, version-skew, canonical grade/action payload round trips |
| `cargo test -p ocentra-parent-agent-core network` | pass | production spine handlers, references, no-enforcement, queue idempotency/replay behavior |
| `cargo test -p ocentra-network-core` | pass | 14 tests |
| `cargo check -p ocentra-parent-agent-protocol -p ocentra-parent-agent-core -p ocentra-network-core` | pass | touched Rust owners |
| `cargo clippy -p ocentra-parent-agent-protocol -p ocentra-parent-agent-core -p ocentra-network-core -- -D warnings` | pass | touched Rust owners |
| `cargo fmt --check` and `git diff --check` | pass | formatting and diff integrity |
| `npm run lint:architecture -- --files crates/agent-core crates/agent-protocol` | pass | architecture, source-shape, no re-export gate |
| `cargo fmt --check`; `cargo check -p ocentra-parent-agent-protocol -p ocentra-parent-agent-core -p ocentra-network-core`; `cargo clippy -p ocentra-parent-agent-protocol -p ocentra-parent-agent-core -p ocentra-network-core -- -D warnings` | pass | final scoped formatting, compile, and lint validation using `E:\\OcentraBuild\\network-wp01` |

## WP01 attestation

- `schema_owner`: `crates/agent-protocol/src/network_flow.rs`.
- `rust_owner`: `crates/network-core`, `crates/agent-protocol`, and `crates/agent-core`.
- `protocol_owner`: `crates/agent-protocol`; `NetworkRuntimeEventPayload` is the versioned child-service event boundary.
- `eventing_owner`: reusable `ocentra-eventing`; no private `NetworkEventBus` exists.
- `private_bus_state`: absent; the runtime subscribes and publishes through `ocentra_eventing::bus::EventBus`.
- `evidence_grade_state`: canonical A-D value is carried in the dispatched runtime payload; runtime grade remains explicitly mapped.
- `policy_handoff_state`: canonical policy intent is carried in the dispatched runtime payload; evidence remains non-enforcement input.
- `enforcement_authority_state`: dry-run/manual-required/unavailable only; adapter action remains false in the negative boundary.
- `schema_fixture_ref`: `crates/agent-protocol/tests/contract/network_eventing_contract.rs::payload`.
- `rust_parity_ref`: `crates/agent-protocol/tests/contract/network_eventing_contract.rs` and `crates/agent-core/tests/unit/network_event_runtime_tests.rs`.
- `eventing_workpack_ref`: `docs/plans/eventing-plan` reusable `ocentra-eventing` contract; this packet consumes that bus and does not define an alternate registry, retry queue, or request registry.
- `no_claim`: this proof does not claim live packet capture, host mutation, product enforcement, exact content/URL, or external-platform readiness.

## Obligation Mapping

- `network.contract.schema-unit`: `network_runtime_payload_schema_mutations_fail_closed`.
- `network.contract.schema-fuzz`: `network_contract_schema_fuzz_seeded_mutation_cases_fail_closed` with seed `0x4e57_5030_315f_7632` and five bounded unknown/missing/extra/type/semantic cases.
- `network.rust-protocol.parity`: `network_evidence_grade_wire_values_match_evidence_contract` and `network_runtime_payload_exhaustively_round_trips_canonical_grade_and_policy_values`.
- `network.evidence-grade.boundary-negative`: `network_runtime_payload_rejects_impossible_semantic_tuple_before_dispatch`.
- `network.eventing.integration-contract`: `network_runtime_payload_dispatches_once_through_shared_event_bus` and `invalid_serialized_runtime_payload_is_rejected_before_handler_receipt`.

## Final Scoped Validation

- Branch-budget repair extracted the private evidence-grade tuple map to `crates/agent-protocol/src/network_flow/runtime_semantics.rs`; `NetworkRuntimeEventPayload` remains the unchanged public contract and retains the same fail-closed semantic error.
- `cargo fmt --check`, `cargo test -p ocentra-parent-agent-protocol network` (37), `cargo test -p ocentra-parent-agent-core network` (54), scoped `cargo check`, scoped `cargo clippy -- -D warnings`, and scoped architecture lint passed with `CARGO_TARGET_DIR=E:\\OcentraBuild\\network-wp01`.
- CI service parity was reproduced in `network_bridge_runtime`: three stale assertions still required the retired enforcement phases. The service correctly emitted nine events and zero enforcement commands; the tests now assert audit/portal continuity and `adapter_action_executed: false` without restoring enforcement behavior.
- `cargo test -p ocentra-parent-agent-service --test network_bridge_runtime -- --test-threads=1` passed 38 tests, and the exact CI command `cargo test -p ocentra-parent-agent-service --all-targets` passed. Agent-service `cargo check`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`, and exact-file architecture lint also passed.

Independent re-review remains required before treating WP01 as approved.
