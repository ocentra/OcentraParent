# WP89 Source-Gated Policy Preview Timer Service Readiness Protocol Read Model

## AI Worker Checklist

- [x] Confirm source docs read: `docs/feature-list.md`,
      `docs/features/app-game-control.md`,
      `docs/expectations/app-game-evidence.md`, and
      `packages/parent-domain/README.md`.
- [x] Confirm browser-game scope remains in browser-plan.
- [x] Confirm apps and games share low-level evidence but keep separate product
      meaning.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel app-control or game-control
      truth created.
- [x] Before-state source snapshot recorded in
      `output/app-game-plan-proof/89-source-gated-policy-preview-timer-service-readiness-protocol-read-model/00-source-snapshot.md`.
- [x] Contracts updated first where behavior changes.
- [x] Rust/service/portal parity not updated because this is a parent-domain
      protocol-readiness read-model proof only.
- [x] Tests/proof listed in this workpack are implemented.
- [x] Security/no-claim negative proof captured: protocol read model does not
      implement agent-protocol contracts, mirror Rust protocol, register service
      commands, emit service/read-model events, implement read APIs, render
      portal UI, run timers, persist scheduler/audit storage, execute rollback,
      dispatch adapters, deliver child UX, enforce platform controls, or expose
      raw private source rows.
- [x] Feature/expectation/product-checklist/README update decision recorded.
- [x] Known gaps, deferred items, and no-claim boundaries recorded.

## Scope

Create a parent-domain read-model proof that consumes WP88 protocol handoff rows
and projects the future protocol command/event, Rust mirror, service handler,
and service read-API proof requirements into protocol-readiness read-model rows.
Eligible native app/native game rows remain protocol-read-model-proof-required;
source and compiler blockers remain blocked before any protocol/service event
claim.

## Evidence

- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model.ts`
- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-rules.ts`
- `packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model.test.ts`
- `scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof.mjs`
- `test-results/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof/proof.json`
- `output/app-game-plan-proof/89-source-gated-policy-preview-timer-service-readiness-protocol-read-model/proof.json`

## No-Claim Boundaries

- No package manifest export.
- No agent-protocol command/event implementation.
- No Rust protocol mirror.
- No service command registration.
- No service event or read-model event emission.
- No service read API implementation.
- No portal UI.
- No policy evaluator runtime.
- No timer runtime or scheduling.
- No scheduler persistence runtime or durable scheduler state-store.
- No audit runtime or durable audit log.
- No rollback runtime or execution.
- No adapter dispatch.
- No child delivery.
- No platform enforcement.
- No raw private source rows.

Product capability checklist remains unchanged because this proof adds a
protocol-readiness read model without moving app/game feature status.
