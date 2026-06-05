# 71. Policy Preview Service Read Model

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-policy-preview-service-read-model`
- Scope: service policy-preview payload bridge plus typed app/game parser.

## Goal

Bridge the existing `agent.policy.preview.read-model.get/reported` service path
into a typed app/game policy-preview read model without adding a duplicate
app/game command, rerunning the evaluator, starting timers, delivering child
notifications, or dispatching adapters.

## In Scope

- Include the full serialized generic policy-preview read model in the existing
  service event payload field.
- Add an agent-protocol-domain app/game parser for that service payload.
- Derive native-app preview rows from existing app/process/window policy
  targets.
- Keep native-game promotion explicitly unclaimed until the service persists the
  WP70 `sourceTargetKind` distinction.
- Reject service rows that are not dry-run or whose enforcement handoff is not
  disabled.
- Record proof packs for app-game and app-plan.

## Out Of Scope

- New Rust command or event constants.
- Portal authoring or preview UI.
- Runtime policy evaluator execution.
- Timer execution, rollback, child notification delivery, adapter dispatch,
  broad installed-app blocking, or platform enforcement.
- Native-game service promotion before source target kind is persisted.
- Product capability checklist movement.

## Proof

- `crates/agent-service/src/policy_preview_payload.rs`
- `crates/agent-service/src/policy_preview_tests.rs`
- `packages/agent-protocol-domain/src/app-game-policy-preview-read-model.ts`
- `packages/agent-protocol-domain/tests/app-game-policy-preview-read-model.test.ts`
- `packages/agent-protocol-domain/package.json`
- `scripts/test/app-game-policy-preview-service-read-model-proof.mjs`
- `test-results/app-game-policy-preview-service-read-model/proof.json`
- `output/app-game-plan-proof/71-policy-preview-service-read-model/`
- `output/app-plan-proof/71-policy-preview-service-read-model/`

## Coordination Notes

- This work reuses the generic policy-preview service command/event so it does
  not conflict with the eventing lane's Rust protocol constant lock.
- Native-game preview rows remain unavailable at this service boundary until a
  later slice persists the app/game source target kind that WP70 has at the
  parent-domain contract boundary.
- `docs/product-capability-checklist.md` is unchanged because no product status
  moved.

## DONE Checklist

- [x] Hub lock covers implementation, test, package export, docs, and proof
      paths.
- [x] Existing service policy-preview payload and WP70 parent-domain handoff
      contract inspected.
- [x] Service event payload exposes the full serialized read model under the
      existing payload field.
- [x] TypeScript parser decodes the service payload and rejects non-dry-run or
      executable handoff rows.
- [x] Native-app preview rows are surfaced from app/process/window policy
      targets.
- [x] Native-game promotion remains false until source target kind is persisted.
- [x] Proof pack records no policy evaluator runtime, timer runtime, child
      delivery, adapter dispatch, broad blocking, or platform enforcement claim.
