# WP114 Timer Parent-Surface Control Action-Result Visibility

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP114 Timer Parent-Surface Control Action-Result Visibility`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

- Bridge existing app/game SQLite replay control action-result rows into the
  unified app/game timer parent-surface read model.
- Keep native app and native game rows on the same shared parent-surface
  contract.
- Let the parent portal display control action-result counts and result
  references when the service reports replayed rows.
- Keep live scheduling automation, adapter dispatch, child delivery, broad
  blocking, platform enforcement, and raw private source rows unclaimed.

## Implementation

- `crates/agent-service/src/activity_api/app_game_timer_parent_surface_payload.rs`
  now derives `controlActionResultCount` and result-reference ids from the
  existing `approval_action_result_rows` service read model field.
- `crates/agent-protocol/src/app_game_timer_parent_surface_read_model.rs` and
  `packages/agent-protocol-domain/src/app-game-timer-parent-surface-read-model.ts`
  carry the new count and reference fields across Rust and TypeScript
  boundaries.
- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts` renders
  the action-result count and references in the parent-surface summary while
  keeping adapter, child, platform, and raw private source claims unavailable.
- `apps/portal/tests/app-game-timer-parent-surface-panel.test.ts`,
  `packages/agent-protocol-domain/tests/app-game-timer-parent-surface-read-model.test.ts`,
  and Rust service/protocol tests cover the new replay-visible fields.

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
- Product-complete parent/child UX, policy evaluator persistence, and
  cross-platform proof remain follow-up app/game scope.
