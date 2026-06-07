# WP115 - Source-Gated Policy Preview Timer Service Response Consumer Parent-Surface Status Read-Model Parent-Surface Read-Model Service Handoff

## Scope

Add a parent-domain service handoff contract that consumes WP114 parent-safe
read-model rows and records which native app/native game rows require future
service proof before runtime service registration, handler, read-model emission,
events, or read APIs can be claimed.

## Boundaries

- Keep browser-game work in `browser-plan`.
- Keep native apps and native games as separate row targets on the shared
  app/game evidence spine.
- Do not add package exports in this slice.
- Do not implement service command registration, service handlers, service
  read-model runtime emission, service event runtime emission, service events,
  read APIs, read API responses, response consumers, runtime persistence,
  parent-surface rendering, portal UI, Rust protocol, timer runtime, scheduler
  storage, audit logs, rollback execution, adapter dispatch, child delivery,
  platform enforcement, or raw private source rows.

## Implementation Checklist

- [x] Add parent-domain service handoff schemas, builder, no-claim flags, and
      state rules.
- [x] Add focused parser/builder tests using the real WP114 proof output.
- [x] Add proof harness and app-game/app proof artifacts.
- [x] Update feature/checklist/workpack docs with the no-claim decision.
- [x] Leave `docs/product-capability-checklist.md` unchanged because no feature
      status moved.

## Evidence

- `packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff.ts`
- `packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff-rules.ts`
- `packages/parent-domain/tests/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff.test.ts`
- `scripts/test/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff-proof.mjs`
- `output/app-game-plan-proof/115-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff`
- `test-results/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff-proof/proof.json`

## Known Gaps

Package exports, service command registration, service handler implementation,
service read-model runtime emission, service event runtime emission, service
events, service read APIs, service read API responses, response consumers,
runtime persistence, parent-surface rendering, portal rendering, Rust protocol
parity, service runtime, adapter dispatch, child-device delivery, platform
enforcement, and raw source row exposure remain unclaimed.
