# WP102 - Source-Gated Policy Preview Timer Service Readiness Response Consumer Parent-Surface Status Read-Model Parent-Surface Read-Model Service Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP102 - Source-Gated Policy Preview Timer Service Readiness Response Consumer Parent-Surface Status Read-Model Parent-Surface Read-Model Service Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Record the reviewed ownership decision for the proposed service handoff after
WP101. The intermediate `parent-domain` packet is retired as redundant because
WP103 already owns the real Rust protocol and agent-service parent-surface
read-model boundary.

## Boundaries

- Keep browser-game work in `browser-plan`.
- Keep native apps and native games as separate row targets on the shared app/game evidence spine.
- Do not recreate the retired `packages/parent-domain` business owner or add a
  duplicate test-only builder.
- Do not implement service commands, service handlers, service read-model emission, service events, read APIs, response consumers, runtime persistence, parent-surface rendering, portal UI, Rust protocol, timer runtime, scheduler storage, audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, or raw private source rows.

## Implementation Checklist

- [x] Record that no new WP102 product source or test is required.
- [x] Route the real service boundary to WP103's Rust protocol, agent-service
      implementation, and checked-in tests.
- [ ] Add proof harness and app-game/app proof artifacts.
- [x] Update this route with the no-code supersession decision.
- [x] Leave `docs/product-capability-checklist.md` unchanged because no feature
      status moved.

## Evidence

## Current reviewed topology

The canonical head intentionally has no `packages/parent-domain` WP102
service-handoff source or test root. Those paths describe a removed business
owner and would add a dead intermediate builder with no production caller.

WP103 already owns the intended executable boundary:

- `crates/agent-protocol/src/app_game_timer_parent_surface_read_model.rs`
  defines the serialized parent-safe timer surface contract.
- `crates/agent-service/src/activity_api/app_game_timer_parent_surface_payload.rs`
  builds the real service payload from the current app/game service read model.
- `crates/agent-protocol/tests/contract/app_game_timer_parent_surface_read_model_tests.rs`
  and
  `crates/agent-service/tests/unit/app_game_timer_parent_surface_payload_tests.rs`
  are the checked-in contract and service tests.

WP102 is therefore a reviewed no-code-required supersession packet. It does not
claim WP103 focused execution/proof or a live caller from the bounded WP101
builder chain.

Dependencies: WP101's reviewed bounded contract and WP103's reviewed Rust
implementation. These dependencies validate the ownership decision only; they
do not promote READY/DONE or satisfy focused execution, proof, CI, review, or
merge gates.

- `output/app-game-plan-proof/102-timer-service-handoff`
- `test-results/app-game-timer-service-handoff-proof/proof.json`

## Known Gaps

Focused execution, proof artifacts, the live linkage from WP101 into product
runtime, runtime persistence, parent-surface rendering, portal rendering,
adapter dispatch, child-device delivery, platform enforcement, and raw source
row exposure remain unclaimed.
