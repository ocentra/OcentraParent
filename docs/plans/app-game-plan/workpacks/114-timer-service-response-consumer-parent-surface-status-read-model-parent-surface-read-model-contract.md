# WP114 - Source-Gated Policy Preview Timer Service Response Consumer Parent-Surface Status Read-Model Parent-Surface Read-Model Contract

## Scope

Add a parent-domain parent-surface read-model contract that consumes WP113 handoff rows and emits parent-safe native app/native game rows for future parent-visible consumption.

## Boundaries

- Keep browser-game work in `browser-plan`.
- Keep native apps and native games as separate row targets on the shared app/game evidence spine.
- Do not add package exports in this slice.
- Do not implement service command registration, service handlers, service read-model runtime emission, service event runtime emission, service events, read APIs, read API responses, response consumers, runtime persistence, parent-surface rendering, portal UI, Rust protocol, timer runtime, scheduler storage, audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, or raw private source rows.

## Implementation Checklist

- [x] Add parent-domain parent-surface read-model schemas, builder, no-claim flags, and state rules.
- [x] Add focused parser/builder tests using the real WP113 proof output.
- [x] Add proof harness and app-game/app proof artifacts.
- [x] Update feature/checklist/README docs with the no-claim decision.
- [x] Leave `docs/product-capability-checklist.md` unchanged because no feature status moved.

## Evidence

- `packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model.ts`
- `packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-rules.ts`
- `packages/parent-domain/tests/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model.test.ts`
- `scripts/test/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof.mjs`
- `output/app-game-plan-proof/114-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-contract`
- `test-results/app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof/proof.json`

## Known Gaps

Package exports, service command registration, service handler implementation, service read-model runtime emission, service event runtime emission, service events, service read APIs, service read API responses, response consumers, runtime persistence, parent-surface rendering, portal rendering, Rust protocol parity, service runtime, adapter dispatch, child-device delivery, platform enforcement, and raw source row exposure remain unclaimed.
