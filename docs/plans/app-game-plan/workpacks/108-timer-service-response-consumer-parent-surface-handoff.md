# WP108 - Source-Gated Policy Preview Timer Service Response Consumer Parent-Surface Handoff

## Scope

Add a parent-domain service response-consumer parent-surface handoff that consumes WP107 service read API response consumer handoff rows and records which native app/native game rows still require parent-surface proof before parent-visible consumption can be claimed.

## Boundaries

- Keep browser-game work in `browser-plan`.
- Keep native apps and native games as separate row targets on the shared app/game evidence spine.
- Do not add package exports in this slice because another lane owns `packages/parent-domain/package.json`.
- Do not implement service command registration, service handlers, service read-model runtime emission, service event runtime emission, service events, read APIs, read API responses, response consumers, runtime persistence, parent-surface rendering, portal UI, Rust protocol, timer runtime, scheduler storage, audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, or raw private source rows.

## Implementation Checklist

- [x] Add parent-domain service response-consumer parent-surface handoff schemas, builder, no-claim flags, and state rules.
- [x] Add focused parser/builder tests using the real WP107 proof output.
- [x] Add proof harness and app-game/app proof artifacts.
- [x] Update feature/checklist/README docs with the no-claim decision.
- [x] Leave `docs/product-capability-checklist.md` unchanged because no feature status moved.

## Evidence

- `packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-handoff.ts`
- `packages/parent-domain/src/app-game-timer-service-response-consumer-parent-surface-handoff-rules.ts`
- `packages/parent-domain/tests/app-game-timer-service-response-consumer-parent-surface-handoff.test.ts`
- `scripts/test/app-game-timer-service-response-consumer-parent-surface-handoff-proof.mjs`
- `output/app-game-plan-proof/108-timer-service-response-consumer-parent-surface`
- `test-results/app-game-timer-service-response-consumer-parent-surface-handoff-proof/proof.json`

## Known Gaps

Package exports, service command registration, service handler implementation, service read-model runtime emission, service event runtime emission, service events, service read APIs, service read API responses, response consumers, runtime persistence, parent-surface rendering, portal rendering, Rust protocol parity, service runtime, adapter dispatch, child-device delivery, platform enforcement, and raw source row exposure remain unclaimed.
