# WP87 Source-Gated Policy Preview Timer Service Readiness Read Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP87 Source-Gated Policy Preview Timer Service Readiness Read Model`
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
      `docs/expectations/app-game-evidence.md`,
      `docs/expectations/policy.md`, and `packages/parent-domain/README.md`.
- [ ] Confirm browser-game scope remains in browser-plan.
- [ ] Confirm apps and games share low-level evidence but keep separate product
      meaning.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel app-control or game-control
      truth created.
- [ ] Before-state source snapshot recorded in
      `output/app-game-plan-proof/87-source-gated-policy-preview-timer-service-readiness-read-model/00-source-snapshot.md`.
- [ ] Contracts updated first where behavior changes.
- [ ] Rust/service/portal parity not updated because this is a parent-domain
      service-readiness read-model proof only.
- [ ] Tests/proof listed in this workpack are implemented.
- [ ] Security/no-claim negative proof captured: read-model projection does not
      emit a service event, implement a service read API, render portal UI, run
      the evaluator, start or schedule a timer, persist scheduler
      runtime/state-store rows, write audit runtime rows, write durable audit
      logs, execute rollback, dispatch an adapter, deliver child UX, enforce
      platform controls, or expose raw private source rows.
- [ ] Feature/expectation/product-checklist/README update decision recorded.
- [ ] Known gaps, deferred items, and no-claim boundaries recorded.

## Scope

Create a parent-domain proof that consumes WP86 source-gated policy preview
timer service-readiness handoff rows and projects the same native app/native
game proof requirements into service-readiness read-model rows. Eligible rows
keep service-readiness and service read-API proof refs visible; source and
compiler blockers remain blocked before any service runtime, protocol command,
read API implementation, or portal UI claim.

## Current Rust-first source decision — 2026-08-29

This workpack is still a real source/composition gap. The historical
`packages/parent-domain` files below are retired and are not current owners.
A proposed Rust packet at `bd448a2d1` was reviewed and rejected: its new
read-model type and builder had only a contract-test caller, while WP88 still
accepted a separate manually reconstructed input type. There was no production
consumer and no conversion that let the new output enter the existing chain.

Do not integrate or reproduce that dead DTO. The smallest honest successor
must make the existing Rust owner chain consume one canonical WP87 output
directly in WP88 and must have a real production composition caller. Until
that owner/caller exists, checked boxes and legacy evidence paths are not
implementation completion. Test execution, proof, CI, and DONE also remain
open.

## Evidence

- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-read-model.ts`
- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-read-model-rules.ts`
- `packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-read-model.test.ts`
- `scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof.mjs`
- `test-results/app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof/proof.json`
- `output/app-game-plan-proof/87-source-gated-policy-preview-timer-service-readiness-read-model/proof.json`

## No-Claim Boundaries

- No package manifest export.
- No agent protocol command/event.
- No Rust service runtime event.
- No service read API implementation.
- No portal UI.
- No policy evaluator runtime.
- No timer runtime.
- No timer scheduling.
- No scheduler persistence runtime.
- No durable scheduler state-store rows.
- No audit runtime.
- No durable audit log.
- No rollback runtime.
- No rollback execution.
- No adapter dispatch.
- No child delivery.
- No platform enforcement.
- No raw private source rows.

Product capability checklist remains unchanged because this proof adds
service-readiness read-model classification evidence without moving feature
status.
