# WP104 - Source-Gated Policy Preview Timer Service Event Handoff

## Scope

Add a parent-domain service event handoff that consumes WP103 service read-model handoff rows and records which native app/native game rows still require service event proof before runtime event emission, read API visibility, or parent-surface consumption can be claimed.

## Boundaries

- Keep browser-game work in `browser-plan`.
- Keep native apps and native games as separate row targets on the shared app/game evidence spine.
- Do not add package exports in this slice because another lane owns `packages/parent-domain/package.json`.
- Do not implement service command registration, service handlers, service read-model runtime emission, service event runtime emission, service events, read APIs, response consumers, runtime persistence, parent-surface rendering, portal UI, Rust protocol, timer runtime, scheduler storage, audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, or raw private source rows.

## Implementation Checklist

- [x] Add parent-domain service event handoff schemas, builder, no-claim flags, and state rules.
- [x] Add focused parser/builder tests using the real WP103 proof output.
- [x] Add proof harness and app-game/app proof artifacts.
- [x] Update feature/checklist/README docs with the no-claim decision.
- [x] Leave `docs/product-capability-checklist.md` unchanged because no feature status moved.

## Evidence

- `packages/parent-domain/src/app-game-timer-service-event-handoff.ts`
- `packages/parent-domain/src/app-game-timer-service-event-handoff-rules.ts`
- `packages/parent-domain/tests/app-game-timer-service-event-handoff.test.ts`
- `scripts/test/app-game-timer-service-event-handoff-proof.mjs`
- `output/app-game-plan-proof/104-timer-service-event`
- `test-results/app-game-timer-service-event-handoff-proof/proof.json`

## Known Gaps

Package exports, service command registration, service handler implementation, service read-model runtime emission, service event runtime emission, service events, service read APIs, response consumers, runtime persistence, parent-surface rendering, portal rendering, Rust protocol parity, service runtime, adapter dispatch, child-device delivery, platform enforcement, and raw source row exposure remain unclaimed.
