# 66. Notification Parent Surface Intent

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `66. Notification Parent Surface Intent`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

- [ ] Provider-status and preference-status handoff read models parse before
      mapping.
- [ ] Source family refs and row counts must match before parent-surface intent
      rows are produced.
- [ ] Manual provider/preference rows become parent-surface
      `manual-action-required` rows with history/preference setup visibility.
- [ ] Unavailable provider/preference rows remain visible as unavailable or
      disabled rows without delivery claims.
- [ ] Proof pack records no rendered UI, no parent preference mutation, no
      provider delivery, no receipt ingestion, no credentials, no production
      runtime, no child delivery, no adapter dispatch, no broad blocking, and no
      platform support.
- [ ] Product checklist unchanged because this parent-surface intent does not
      move feature status and provider/runtime/UI/platform gaps remain.
