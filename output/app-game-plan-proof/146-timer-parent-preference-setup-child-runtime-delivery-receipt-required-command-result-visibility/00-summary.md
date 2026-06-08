# WP146 - Timer parent preference setup child-runtime delivery receipt-required command-result visibility

## Scope

WP146 makes the WP145 service-local child-runtime delivery receipt-required
refs/status visible in the parent portal command-result details for accepted
app/game timer parent preference setup request results.

## Touched Files

- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts`
- `apps/portal/tests/app-game-timer-parent-surface-panel.test.ts`
- `packages/portal-domain/README.md`
- `apps/portal/README.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/README.md`
- `docs/plans/app-game-plan/workpacks/146-timer-parent-preference-setup-child-runtime-delivery-receipt-required-command-result-visibility.md`
- `test-results/app-game-timer-parent-preference-setup-child-runtime-delivery-receipt-required-command-result-visibility/handoff.json`

## Result

- The portal-domain command-result detail builder renders receipt-required
  refs/status beside action-result persistence, mutation receipt, handoff,
  queue, and dispatch details.
- The app portal test asserts the parent-visible receipt-required detail rows.
- Portal/app-game docs record that receipt-required visibility remains
  parent-safe local readiness, not delivery, receipt, provider, outbox, adapter,
  broad-blocking, or platform enforcement proof.
- `docs/product-capability-checklist.md` is intentionally untouched.

## No-Claim Boundaries

- No actual child runtime delivery or receipt is claimed.
- No provider delivery or provider receipt ingestion is claimed.
- No durable production outbox runtime is claimed.
- No adapter dispatch, broad blocking, or platform enforcement is claimed.
- No raw private source rows, raw target values, or private diagnostics are
  exposed.
