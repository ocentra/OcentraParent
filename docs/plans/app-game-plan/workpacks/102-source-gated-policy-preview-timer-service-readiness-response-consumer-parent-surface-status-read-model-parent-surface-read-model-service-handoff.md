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

Add a parent-domain service handoff that consumes WP101 parent-safe read-model rows and records which native app/native game rows still require service read-model/event/API proof before any runtime service wiring can expose parent-surface read-model data.

## Boundaries

- Keep browser-game work in `browser-plan`.
- Keep native apps and native games as separate row targets on the shared app/game evidence spine.
- Do not add package exports in this slice because another lane owns `packages/parent-domain/package.json`.
- Do not implement service commands, service handlers, service read-model emission, service events, read APIs, response consumers, runtime persistence, parent-surface rendering, portal UI, Rust protocol, timer runtime, scheduler storage, audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, or raw private source rows.

## Implementation Checklist

- [ ] Add parent-domain service handoff schemas, builder, no-claim flags, and state rules.
- [ ] Add focused parser/builder tests using the real WP101 proof output.
- [ ] Add proof harness and app-game/app proof artifacts.
- [ ] Update feature/checklist/README docs with the no-claim decision.
- [ ] Leave `docs/product-capability-checklist.md` unchanged because no feature status moved.

## Evidence

## Current reviewed topology

The canonical head has no `packages/parent-domain` WP102 service-handoff source
or test root. WP102 does not own any shared app-game helper; the planned
package and test roots remain explicitly absent.

Implementation dependency: WP101 reviewed implementation. This orders a future
source packet only; it does not promote READY/DONE or satisfy tests, proof,
checklist, CI, review, or merge gates.

- `packages/parent-domain/src/app-game-timer-service-handoff.ts`
- `packages/parent-domain/src/app-game-timer-service-handoff-rules.ts`
- `packages/parent-domain/tests/app-game-timer-service-handoff.test.ts`
- `scripts/test/app-game-timer-service-handoff-proof.mjs`
- `output/app-game-plan-proof/102-timer-service-handoff`
- `test-results/app-game-timer-service-handoff-proof/proof.json`

## Known Gaps

Package exports, service read-model emission, service read APIs, response consumers, runtime persistence, parent-surface rendering, portal rendering, Rust protocol parity, service runtime, adapter dispatch, child-device delivery, platform enforcement, and raw source row exposure remain unclaimed.
