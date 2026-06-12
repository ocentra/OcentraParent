# WP116 Timer Parent-Surface Child Reason/Status Refs

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP116 Timer Parent-Surface Child Reason/Status Refs`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

- Carry child-facing reason and status reference ids from existing app/game
  approval action-result rows into the unified timer parent-surface read model.
- Keep native app and native game rows on the same shared parent-surface
  contract.
- Let the parent portal display child-safe reason/status refs for future child
  warning/request UX auditability.
- Keep child delivery, notifications, adapter dispatch, broad blocking,
  platform enforcement, diagnostics, and raw private source rows unclaimed.

## Implementation

- `crates/agent-service/src/activity_api/app_game_timer_parent_surface_action_results.rs`
  now derives unique child reason/status refs from replayed approval
  action-result requests.
- `crates/agent-protocol/src/app_game_timer_parent_surface_read_model.rs` and
  `packages/agent-protocol-domain/src/app-game-timer-parent-surface-read-model.ts`
  carry those child-facing ref arrays across Rust and TypeScript boundaries.
- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts` renders
  the refs while keeping child delivery and adapter execution not claimed.
- Rust service/protocol tests, agent-protocol-domain parser tests, and portal
  panel tests cover the child-facing refs and no-overclaim behavior.

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

- Live child UI and notification delivery remain unimplemented.
- Adapter dispatch, broad blocking, platform enforcement, diagnostics, and raw
  private source row access remain unclaimed.
- Central `docs/product-capability-checklist.md` proof wording remains deferred
  while another lane owns that shared checklist lock; this slice updates the
  owning feature doc and app-game checklist/workpack.
