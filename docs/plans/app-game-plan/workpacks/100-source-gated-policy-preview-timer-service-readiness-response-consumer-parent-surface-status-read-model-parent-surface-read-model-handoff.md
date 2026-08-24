# WP100 - Source-Gated Policy Preview Timer Service Readiness Response Consumer Parent-Surface Status Read-Model Parent-Surface Read-Model Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP100 - Source-Gated Policy Preview Timer Service Readiness Response Consumer Parent-Surface Status Read-Model Parent-Surface Read-Model Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Own the Rust app/game handoff that consumes WP99 response-consumer
parent-surface rows and records which native app/native game rows still require
future parent-surface read-model proof before any parent-visible surface
read-model can be implemented or rendered.

## Boundaries

- Keep browser-game work in `browser-plan`.
- Keep native apps and native games as separate row targets on the shared app/game evidence spine.
- Do not recreate the retired `packages/parent-domain` business owner or add a
  TypeScript package export in this slice.
- Do not implement service commands, service handlers, service events, read APIs, response consumers, parent-surface read-model runtime, parent-surface rendering, portal UI, Rust protocol, timer runtime, scheduler storage, audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, or raw private source rows.

## Implementation Checklist

- [x] Keep the parent-surface read-model handoff schemas, builder, no-claim
      flags, and state rules in the Rust `app-game-core` owner.
- [x] Keep focused contract coverage over the real WP99 builder output.
- [ ] Add proof harness and app-game/app proof artifacts.
- [x] Update this route with the Rust-first no-claim decision.
- [x] Leave `docs/product-capability-checklist.md` unchanged because no feature
      status moved.

## Evidence

## Current reviewed topology

The canonical head intentionally has no `packages/parent-domain` WP99 handoff
source or test root because that removed package is not a current business
owner. The bounded WP100 handoff is already Rust-owned:

- `crates/app-game-core/src/app_game_source_gated_policy_preview_timer_followthrough/parent_surface_status.rs`
  owns the WP100 option, row, and result shapes.
- `crates/app-game-core/src/app_game_source_gated_policy_preview_timer_followthrough/tail.rs`
  owns the builder that consumes the WP99 handoff.
- `crates/app-game-core/tests/contract/app_game_source_gated_policy_preview_timer_followthrough.rs`
  exercises proof-required versus blocked rows through the real Rust builder
  chain.

This is complete for bounded Phase 1 source/test writing only. The focused test
was not rerun in this truth-sync packet, and no proof, runtime reachability,
parent-visible rendering, READY, or DONE claim follows.

Implementation dependency: WP99 reviewed implementation. This orders the
bounded Rust handoff only; it does not promote normal READY/DONE or satisfy
focused execution, proof, CI, review, or merge gates.

- `output/app-game-plan-proof/100-timer-parent-read-model`
- `test-results/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof/proof.json`

## Known Gaps

Focused execution, proof artifacts, parent-surface runtime/persistence,
parent-surface rendering, service read APIs, portal rendering, Rust protocol
parity, service runtime, adapter dispatch, child-device delivery, platform
enforcement, and raw source row exposure remain unclaimed.
