# WP76 App/Game Source-Gated Policy Preview Read Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP76 App/Game Source-Gated Policy Preview Read Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Add a parent-domain read-model contract that consumes the WP75 source freshness
preview gate and exposes redacted, parent-safe policy preview rows for later
service and portal consumers.

This work remains stacked on WP74/WP75 until those branches land because the
read model must derive from the existing source freshness policy-consumption
and preview-gate contracts rather than duplicating them.

## Owned Proof

- Contract:
  `packages/parent-domain/src/app-game-source-gated-policy-preview-read-model.ts`
- Rules:
  `packages/parent-domain/src/app-game-source-gated-policy-preview-read-model-rules.ts`
- Tests:
  `packages/parent-domain/tests/app-game-source-gated-policy-preview-read-model.test.ts`
- Harness:
  `scripts/test/app-game-source-gated-policy-preview-read-model-proof.mjs`
- Evidence:
  `output/app-game-plan-proof/76-source-gated-policy-preview-read-model` and
  `output/app-plan-proof/76-source-gated-policy-preview-read-model`

## Acceptance

- Read-model rows derive from validated WP75 source freshness preview gate rows.
- Preview-ready rows expose redacted evidence refs and a preview decision ref.
- Source-manual-required rows remain visible but carry no preview decision ref.
- Compiler-manual-required rows stay separate from source freshness failures.
- Counts for native app, native game, preview-ready, source-manual, and
  compiler-manual rows match row state.
- Rows and read models keep service runtime events, portal UI, policy evaluator
  runtime, timers, adapter dispatch, child delivery, platform enforcement, and
  raw private source rows unclaimed.

## Non-Goals

- No service/WebSocket command or event.
- No portal renderer or screenshots beyond explicit N/A proof.
- No package export while `packages/parent-domain/package.json` is owned by
  another lane.
- No runtime policy evaluator, timer, child delivery, adapter dispatch, broad
  blocking, or platform hard-control support.
