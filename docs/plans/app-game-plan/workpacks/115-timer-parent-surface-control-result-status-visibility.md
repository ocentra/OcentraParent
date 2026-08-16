# WP115 Timer Parent-Surface Control Result Status Visibility

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP115 Timer Parent-Surface Control Result Status Visibility`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

- Aggregate parent-safe control action-result statuses from existing app/game
  SQLite replay rows into the unified timer parent-surface read model.
- Keep native app and native game rows on the same shared parent-surface
  contract.
- Let the parent portal display result statuses, capability states, and
  enforcement-result statuses without exposing raw action request or decision
  payloads.
- Keep live scheduling automation, adapter dispatch, child delivery, broad
  blocking, platform enforcement, and raw private source rows unclaimed.

## Implementation

- `crates/agent-service/src/activity_api/app_game_timer_parent_surface_payload.rs`
  now derives unique result statuses, capability states, and enforcement-result
  statuses from `approval_action_result_rows`.
- `crates/agent-protocol/src/app_game_timer_parent_surface_read_model.rs` and
  `packages/agent-protocol-domain/src/app-game-timer-parent-surface-read-model.ts`
  carry those aggregate status arrays across Rust and TypeScript boundaries.
- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts` renders
  the aggregate status arrays in the parent-surface summary while preserving
  no-claim adapter, child, platform, and raw-source fields.
- Rust service/protocol tests, agent-protocol-domain parser tests, and portal
  panel tests cover the status aggregation and no-overclaim behavior.

## Validation

- `cargo fmt --package ocentra-parent-agent-protocol --package ocentra-parent-agent-service`
- `cargo test -p ocentra-parent-agent-protocol app_game_timer_parent_surface_read_model`
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_surface`
- `cmd /c "npm run build --workspace @ocentra-parent/agent-protocol-domain && npm run build --workspace @ocentra-parent/text-domain && npm run build --workspace @ocentra-parent/portal-domain"`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-timer-parent-surface-read-model`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- app-game-timer-parent-surface-panel`
- `cmd /c npx prettier --check ...`
- `cmd /c node scripts/check-no-test-doubles.mjs`
- `cmd /c node scripts/check-source-shape.mjs`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

## Remaining Gaps

- Live scheduling automation remains unimplemented.
- Adapter dispatch, child delivery, broad blocking, platform enforcement, and
  raw private source row access remain unclaimed.
- Central `docs/product-capability-checklist.md` proof wording remains deferred
  while E-B owns that shared checklist lock; this slice updates the owning
  feature doc and app-game checklist instead.
