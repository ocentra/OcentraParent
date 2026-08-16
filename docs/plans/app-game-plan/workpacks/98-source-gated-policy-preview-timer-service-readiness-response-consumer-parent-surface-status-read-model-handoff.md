# WP98 - Source-Gated Policy Preview Timer Service Readiness Response Consumer Parent-Surface Status Read-Model Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP98 - Source-Gated Policy Preview Timer Service Readiness Response Consumer Parent-Surface Status Read-Model Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Add a parent-domain-only handoff that consumes WP97 response-consumer parent-surface status rows and records which native app/native game rows still require future parent-surface status read-model proof before parent-visible status read-model consumption can exist.

## Boundaries

- Keep browser-game work in `browser-plan`.
- Keep native apps and native games as separate row targets on the shared app/game evidence spine.
- Do not add package exports in this slice.
- Do not implement service commands, service handlers, service events, read APIs, response consumers, parent-surface status/read-model runtime, portal UI, Rust protocol, timer runtime, scheduler storage, audit logs, rollback execution, adapter dispatch, child delivery, platform enforcement, or raw private source rows.

## Implementation Checklist

- [ ] Add parent-domain status read-model handoff schemas, builder, no-claim flags, and state rules.
- [ ] Add focused parser/builder tests using the real WP97 proof output.
- [ ] Add proof harness and app-game/app proof artifacts.
- [ ] Update feature/checklist/README docs with the no-claim decision.
- [ ] Leave `docs/product-capability-checklist.md` unchanged because no feature status moved.

## Evidence

- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff.ts`
- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff-rules.ts`
- `packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff.test.ts`
- `scripts/test/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff-proof.mjs`
- `output/app-game-plan-proof/98-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff`
- `test-results/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff-proof/proof.json`

## Known Gaps

Parent-surface status read-model implementation, service read APIs, portal rendering, Rust protocol parity, service runtime, adapter dispatch, child-device delivery, platform enforcement, package exports, and raw source row exposure remain unclaimed.
