# 66. Notification Parent Surface Intent

Cross-recorded from shared app/game WP66.

## Goal

Represent native app notification history/preference parent-surface intent rows
from the shared app/game notification provider-status and preference-status
handoffs without claiming rendered UI or notification delivery.

## Scope

- Reuse the shared WP64 provider-status handoff and WP65 preference-status
  handoff read models.
- Preserve native app scheduler, outbox, provider, preference, quiet-hours,
  drill-in, audit, and manual-proof refs.
- Keep app-specific notification history/preference rows redacted and setup-only
  until a future parent UI/runtime slice renders them.
- Keep provider delivery, receipts, credentials, cloud routing, child delivery,
  production runtime, durable production storage, adapter dispatch, broad app
  blocking, and platform support unclaimed.

## Source/test status

The Rust-owned intent joins the typed WP64 provider boundary and WP65
preference boundary into a redacted parent-surface read model. Matching family
and status-entry cardinality are required; a mismatch is rejected. Real Rust
contract tests cover joined/manual/unavailable rows, preserved drill-in/audit/
manual-proof refs, generated parity, and every delivery, preference-mutation,
runtime, storage, child-delivery, and adapter nonclaim. Bounded source/test
writing is complete; focused execution, proof, and checklist review remain.

## Proof

- `crates/app-game-core/src/app_game_notification_parent_surface_intent.rs`
- `crates/app-game-core/tests/contract/app_game_notification_parent_surface_intent.rs`
- `crates/app-game-core/tests/generated/app-game-notification-parent-surface-intent.ts`
- `output/app-plan-proof/66-notification-parent-surface-intent/`
- `output/app-game-plan-proof/66-notification-parent-surface-intent/`

## Validation

- [ ] Parent-surface rows require matching family and row counts across provider
      and preference status inputs.
- [ ] Redacted native app history/preference intent rows preserve refs for
      future authenticated drill-in.
- [ ] No rendered parent UI, parent preference mutation, provider delivery,
      receipt ingestion, credentials, production runtime, child delivery,
      adapter dispatch, broad app blocking, or platform support is claimed.
- [ ] Product checklist unchanged because this proof does not move feature
      status and provider/runtime/UI/platform gaps remain.
