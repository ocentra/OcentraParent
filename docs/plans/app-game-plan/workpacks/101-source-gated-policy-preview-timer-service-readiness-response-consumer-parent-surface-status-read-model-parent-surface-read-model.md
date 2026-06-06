# WP101 - Source-Gated Policy Preview Timer Service Readiness Response Consumer Parent-Surface Status Read-Model Parent-Surface Read-Model Contract

## Scope

Add a parent-domain parent-surface read-model contract that consumes WP100 handoff rows and projects parent-safe native app/native game rows for future parent-visible read-model consumption.

## Boundaries

- Keep browser-game work in `browser-plan`.
- Keep native apps and native games as separate row targets on the shared app/game evidence spine.
- Do not add package exports in this slice because another lane owns `packages/parent-domain/package.json`.
- Do not implement service commands, service handlers, service events, read APIs, response consumers, read-model runtime persistence, parent-surface rendering, portal UI, Rust protocol, timer runtime, scheduler storage, audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, or raw private source rows.

## Implementation Checklist

- [x] Add parent-domain parent-surface read-model schemas, builder, no-claim flags, and state rules.
- [x] Add focused parser/builder tests using the real WP100 proof output.
- [x] Add proof harness and app-game/app proof artifacts.
- [x] Update feature/checklist/README docs with the no-claim decision.
- [x] Leave `docs/product-capability-checklist.md` unchanged because no feature status moved.

## Evidence

- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model.ts`
- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-rules.ts`
- `packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model.test.ts`
- `scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof.mjs`
- `output/app-game-plan-proof/101-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model`
- `test-results/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof/proof.json`

## Known Gaps

Package exports, service read APIs, response consumers, runtime read-model persistence, parent-surface rendering, portal rendering, Rust protocol parity, service runtime, adapter dispatch, child-device delivery, platform enforcement, and raw source row exposure remain unclaimed.
