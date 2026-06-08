# WP144 - Timer parent preference setup child-runtime delivery dispatch command-result visibility

## Scope

WP144 makes the WP143 service-local child-runtime delivery dispatch refs/status
visible in the parent portal command-result details for accepted app/game timer
parent preference setup request results.

## Touched Files

- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts`
- `apps/portal/tests/app-game-timer-parent-surface-panel.test.ts`
- `packages/portal-domain/README.md`
- `apps/portal/README.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/README.md`
- `docs/plans/app-game-plan/workpacks/144-timer-parent-preference-setup-child-runtime-delivery-dispatch-command-result-visibility.md`
- `test-results/app-game-timer-parent-preference-setup-child-runtime-delivery-dispatch-command-result-visibility/handoff.json`

## Result

- The portal-domain command-result detail builder renders dispatch refs/status
  beside action-result persistence, mutation receipt, handoff, and queue
  details.
- The app portal test fixture includes the accepted result dispatch fields and
  asserts the parent-visible detail rows.
- Portal/app-game docs record that dispatch readiness remains parent-safe local
  readiness, not delivery, provider, durable outbox, adapter, broad-blocking, or
  platform enforcement proof.
- `docs/product-capability-checklist.md` is intentionally untouched because
  another lane owns central checklist churn.

## No-Claim Boundaries

- No actual child runtime delivery or receipt is claimed.
- No provider delivery or provider receipt ingestion is claimed.
- No durable production outbox runtime is claimed.
- No adapter dispatch, broad blocking, or platform enforcement is claimed.
- No raw private source rows, raw target values, or private diagnostics are
  exposed.
