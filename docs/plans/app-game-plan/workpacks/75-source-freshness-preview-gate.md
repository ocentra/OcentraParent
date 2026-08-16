# WP75 App/Game Source Freshness Preview Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP75 App/Game Source Freshness Preview Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Add a parent-domain preview gate that consumes WP74 source freshness readiness
before WP70 policy preview handoff rows are built.

This keeps native app and native game source freshness as a required policy
input instead of allowing stale or manual-required source rows to become
preview-ready policy output. It is stacked on WP74 until that branch lands on
main, because duplicating WP74 contracts on a separate main-based branch would
create parallel truth.

## Owned Proof

- Contract:
  `packages/parent-domain/src/app-game-source-freshness-preview-gate.ts`
- Rules:
  `packages/parent-domain/src/app-game-source-freshness-preview-gate-rules.ts`
- Tests:
  `packages/parent-domain/tests/app-game-source-freshness-preview-gate.test.ts`
- Harness:
  `scripts/test/app-game-source-freshness-preview-gate-proof.mjs`
- Evidence:
  `output/app-game-plan-proof/75-source-freshness-preview-gate` and
  `output/app-plan-proof/75-source-freshness-preview-gate`

## Acceptance

- Policy-ready native app source freshness can produce a read-only policy
  preview row.
- Manual-required source freshness blocks policy preview before any compiled
  decision is accepted.
- Policy-ready native game source freshness can still produce a manual-required
  preview row when compiler proof remains manual-required.
- Native app/native game source target domains must match the compiled preview
  target domain.
- Rows keep policy evaluator runtime, timers, adapter dispatch, child delivery,
  and platform enforcement unclaimed.

## Non-Goals

- No portal authoring or preview UI.
- No service/WebSocket policy evaluation.
- No package export while `packages/parent-domain/package.json` is owned by
  another lane.
- No timers, notification delivery, child-device mutation, adapter dispatch,
  broad blocking, or platform hard-control support.
