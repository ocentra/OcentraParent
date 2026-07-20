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

Independent re-review remains required before treating WP01 as approved.
