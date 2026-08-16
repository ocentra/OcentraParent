# WP91 Source-Gated Policy Preview Timer Service Readiness Service Handler Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP91 Source-Gated Policy Preview Timer Service Readiness Service Handler Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## AI Worker Checklist

- [ ] Confirm source docs read: `docs/feature-list.md`,
      `docs/features/app-game-control.md`,
      `docs/expectations/app-game-evidence.md`, and
      `packages/parent-domain/README.md`.
- [ ] Confirm browser-game scope remains in browser-plan.
- [ ] Confirm apps and games share low-level evidence but keep separate product
      meaning.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel app-control or game-control
      truth created.
- [ ] Before-state source snapshot recorded in
      `output/app-game-plan-proof/91-source-gated-policy-preview-timer-service-readiness-service-handler-handoff/00-source-snapshot.md`.
- [ ] Contracts updated first where behavior changes.
- [ ] Rust/service/portal parity not updated because this is a parent-domain
      service-handler handoff proof only.
- [ ] Tests/proof listed in this workpack are implemented.
- [ ] Security/no-claim negative proof captured: service handler handoff does not
      implement agent-protocol contracts, mirror Rust protocol, register service
      commands, implement service handlers, emit service events, implement read APIs, render
      portal UI, run timers, persist scheduler/audit storage, execute rollback,
      dispatch adapters, deliver child UX, enforce platform controls, or expose
      raw private source rows.
- [ ] Feature/expectation/product-checklist/README update decision recorded.
- [ ] Known gaps, deferred items, and no-claim boundaries recorded.

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
