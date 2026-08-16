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

## Code-pass status

The agent-service notification-readiness report now joins the typed WP64
provider boundary and WP65 preference boundary into a redacted parent-surface
intent read model. Matching status-entry cardinality is required; a mismatch
produces no joined intent. Parent-surface rows preserve drill-in, audit, and
manual-proof refs while all delivery, preference-mutation, and runtime claims
remain false. The workpack remains unvalidated with tests/proof/checklist
deferred.

## Proof

- `packages/parent-domain/src/app-game-notification-parent-surface-intent.ts`
- `packages/parent-domain/tests/app-game-notification-parent-surface-intent.test.ts`
- `scripts/test/app-game-notification-parent-surface-intent-proof.mjs`
- `test-results/app-game-notification-parent-surface-intent-proof/proof.json`
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
