# WP105 - Source-Gated Policy Preview Timer Service Read API Handoff

## Scope

Add a parent-domain service read API handoff that consumes WP104 service event handoff rows and records which native app/native game rows still require service read API proof before read API implementation, response shaping, or parent-surface consumption can be claimed.

## Boundaries

- Keep browser-game work in `browser-plan`.
- Keep native apps and native games as separate row targets on the shared app/game evidence spine.
- Do not add package exports in this slice because another lane owns `packages/parent-domain/package.json`.
- Do not implement service command registration, service handlers, service read-model runtime emission, service event runtime emission, service events, read APIs, read API responses, response consumers, runtime persistence, parent-surface rendering, portal UI, Rust protocol, timer runtime, scheduler storage, audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, or raw private source rows.

## Implementation Checklist

- [x] Add parent-domain service read API handoff schemas, builder, no-claim flags, and state rules.
- [x] Add focused parser/builder tests using the real WP104 proof output.
- [x] Add proof harness and app-game/app proof artifacts.
- [x] Update feature/checklist/README docs with the no-claim decision.
- [x] Leave `docs/product-capability-checklist.md` unchanged because no feature status moved.

## Evidence

- `packages/parent-domain/src/app-game-timer-service-read-api-handoff.ts`
- `packages/parent-domain/src/app-game-timer-service-read-api-handoff-rules.ts`
- `packages/parent-domain/tests/app-game-timer-service-read-api-handoff.test.ts`
- `scripts/test/app-game-timer-service-read-api-handoff-proof.mjs`
- `output/app-game-plan-proof/105-timer-service-read-api`
- `test-results/app-game-timer-service-read-api-handoff-proof/proof.json`

## Known Gaps

Package exports, service command registration, service handler implementation, service read-model runtime emission, service event runtime emission, service events, service read APIs, service read API responses, response consumers, runtime persistence, parent-surface rendering, portal rendering, Rust protocol parity, service runtime, adapter dispatch, child-device delivery, platform enforcement, and raw source row exposure remain unclaimed.
