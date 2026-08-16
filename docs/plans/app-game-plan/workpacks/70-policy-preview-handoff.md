# 70. Policy Preview Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `70. Policy Preview Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-policy-preview-handoff`
- Scope: parent-domain read-only app/game policy preview handoff.

## Goal

Map already-compiled app/game dry-run policy decisions into parent-visible
preview handoff rows so later authoring or portal surfaces can show the policy
result without re-running the evaluator, starting timers, delivering child
notifications, or dispatching adapters.

## In Scope

- Add a parent-domain Effect Schema read model for app/game policy preview
  handoff rows.
- Parse source rows through the existing
  `AppGamePolicyCompiledDecisionSchema`.
- Preserve native app versus native game target meaning.
- Preserve evidence refs, policy rule refs, capability refs, authority refs,
  audit refs, dry-run state, and disabled enforcement handoff.
- Reject rows that claim policy evaluator runtime, timer runtime, adapter
  dispatch, child delivery, or platform enforcement.

## Out Of Scope

- Portal authoring or preview UI.
- Runtime policy evaluator execution.
- Rust protocol or WebSocket service persistence.
- Timer execution, rollback, child notification delivery, adapter dispatch,
  broad installed-app blocking, or platform enforcement.
- Product capability checklist movement.

## Proof

- `packages/parent-domain/src/app-game-policy-preview-handoff.ts`
- `packages/parent-domain/src/app-game-policy-preview-handoff-rules.ts`
- `packages/parent-domain/package.json`
- `packages/parent-domain/README.md`
- `packages/parent-domain/tests/app-game-policy-preview-handoff.test.ts`
- `packages/parent-domain/tests/app-game-policy-preview-handoff-fixtures.ts`
- `scripts/test/app-game-policy-preview-handoff-proof.mjs`
- `test-results/app-game-policy-preview-handoff-proof/proof.json`
- `output/app-game-plan-proof/70-policy-preview-handoff/`
- `output/app-plan-proof/70-policy-preview-handoff/`

## Coordination Notes

- `packages/parent-domain/package.json` exports
  `./app-game-policy-preview-handoff`, preserving the merged PR364 app-install
  child-device delivery runtime writer proof export.
- `docs/product-capability-checklist.md` is unchanged because this is a
  contract handoff proof and does not move feature status.

## DONE Checklist

- [ ] Hub lock covers implementation, test, package export, README, docs, and
      proof paths.
- [ ] Existing app/game policy compiler and parent-domain policy primitives
      inspected.
- [ ] TypeScript contract parses compiled decisions before building preview
      rows.
- [ ] Native app and native game rows remain separate product meanings while
      sharing the compiler evidence spine.
- [ ] Manual-required block-launch remains manual-required and
      not-dispatched.
- [ ] Proof pack records no policy evaluator runtime, timer runtime, child
      delivery, adapter dispatch, broad blocking, or platform enforcement
      claim.
