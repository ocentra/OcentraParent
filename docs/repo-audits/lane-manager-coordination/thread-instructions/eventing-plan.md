# eventing-plan Instruction

## Verdict

`partial`. Reusable eventing foundation is real. WP10 household-mesh consumer is open and locally actionable.

## Assign first

`eventing-wp10-typed-household-mesh-runtime`:

- wire `crates/agent-core/src/household_mesh_bridge_runtime.rs` to the typed envelope;
- add crate-level tests:
  - `crates/agent-protocol/tests/contract/household_mesh.rs`
  - `crates/agent-core/tests/unit/household_mesh_event_bridge.rs`
  - `crates/agent-core/tests/integration/household_mesh_bridge_runtime.rs`
- update `scripts/test/eventing-household-mesh-consumer-proof.mjs`.

## Then

- generate `output/eventing-plan-proof/12-household-mesh-consumer/` and `test-results/eventing-household-mesh-consumer-proof/proof.json`.
- reconcile WP10 docs and proof manifest.

## Coordinate with

- `lan-plan` for signed-peer/event/assignment authority wording.

## Do not

- Do not wait on remote-access for WP10.
- Do not count inline `src` household-mesh tests as final closure evidence.
- Do not claim physical LAN/provider execution in eventing-plan.
