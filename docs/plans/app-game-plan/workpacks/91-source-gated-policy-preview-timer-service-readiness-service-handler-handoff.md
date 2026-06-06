# WP91 Source-Gated Policy Preview Timer Service Readiness Service Handler Handoff

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
      `output/app-game-plan-proof/91-source-gated-policy-preview-timer-service-readiness-service-handler-handoff/00-source-snapshot.md`.
- [x] Contracts updated first where behavior changes.
- [x] Rust/service/portal parity not updated because this is a parent-domain
      service-handler handoff proof only.
- [x] Tests/proof listed in this workpack are implemented.
- [x] Security/no-claim negative proof captured: service handler handoff does not
      implement agent-protocol contracts, mirror Rust protocol, register service
      commands, implement service handlers, emit service events, implement read APIs, render
      portal UI, run timers, persist scheduler/audit storage, execute rollback,
      dispatch adapters, deliver child UX, enforce platform controls, or expose
      raw private source rows.
- [x] Feature/expectation/product-checklist/README update decision recorded.
- [x] Known gaps, deferred items, and no-claim boundaries recorded.

## Scope

Create a parent-domain service-handler handoff proof that consumes WP90
protocol command-handoff rows and projects the future service handler and
service read-API proof requirements into service-handler handoff rows.
Eligible native app/native game rows remain service-handler-proof-required;
source and compiler blockers remain blocked before any service handler or read
API implementation claim.

## Evidence

- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff.ts`
- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff-rules.ts`
- `packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff.test.ts`
- `scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff-proof.mjs`
- `test-results/app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff-proof/proof.json`
- `output/app-game-plan-proof/91-source-gated-policy-preview-timer-service-readiness-service-handler-handoff/proof.json`

## No-Claim Boundaries

- No package manifest export.
- No agent-protocol command/event implementation.
- No Rust protocol mirror.
- No service command registration.
- No service handler implementation.
- No service event emission.
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
service-handler handoff without moving app/game feature status.
