# 66. Notification Parent Surface Intent

## Goal

Combine app/game notification provider-status and preference-status handoff rows
into redacted parent-surface intent rows so future parent notification history
and preference surfaces can render setup and unavailable states without parsing
provider/preference proof internals.

## Scope

- Reuse WP64 `AppGameNotificationProviderStatusHandoffReadModel` rows.
- Reuse WP65 `AppGameNotificationPreferenceStatusHandoffReadModel` rows.
- Pair source rows deterministically by row order after validating the same
  family ref and matching row counts.
- Preserve scheduler, outbox, provider-status, preference-status, quiet-hours,
  drill-in, audit, and manual-proof refs.
- Expose parent history/preference intent rows only as redacted status and setup
  metadata.
- Preserve explicit false claims for rendered parent notification UI, rendered
  parent preference UI, frequency-control UI, parent preference mutation,
  provider delivery, receipt ingestion, credentials, cloud routing, child
  delivery, production runtime, durable production storage, adapter dispatch,
  broad blocking, and platform support.

## Non-Goals

- Portal or mobile notification history/preference UI.
- Parent preference mutation, frequency control, or quiet-hours editor behavior.
- Provider push, email, SMS, WhatsApp, in-app delivery, templates, credentials,
  webhooks, delivery receipts, or receipt ingestion.
- Production retry workers, production quiet-hours timers, durable production
  outbox/history storage, or cloud routing.
- Child-device delivery, policy evaluator execution, adapter dispatch, broad
  app/game blocking, or platform support.

## Proof

- `packages/parent-domain/src/app-game-notification-parent-surface-intent.ts`
- `packages/parent-domain/tests/app-game-notification-parent-surface-intent.test.ts`
- `scripts/test/app-game-notification-parent-surface-intent-proof.mjs`
- `test-results/app-game-notification-parent-surface-intent-proof/proof.json`
- `output/app-game-plan-proof/66-notification-parent-surface-intent/`
- `output/app-plan-proof/66-notification-parent-surface-intent/`

## Validation

- [x] Provider-status and preference-status handoff read models parse before
      mapping.
- [x] Source family refs and row counts must match before parent-surface intent
      rows are produced.
- [x] Manual provider/preference rows become parent-surface
      `manual-action-required` rows with history/preference setup visibility.
- [x] Unavailable provider/preference rows remain visible as unavailable or
      disabled rows without delivery claims.
- [x] Proof pack records no rendered UI, no parent preference mutation, no
      provider delivery, no receipt ingestion, no credentials, no production
      runtime, no child delivery, no adapter dispatch, no broad blocking, and no
      platform support.
- [x] Product checklist unchanged because this parent-surface intent does not
      move feature status and provider/runtime/UI/platform gaps remain.
