# 64. Notification Provider Status Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `64. Notification Provider Status Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Map app/game notification provider-preflight rows into the existing V0.8
notification provider-status boundary so provider requirements become visible
as manual-required or unavailable status rows without claiming delivery.

## Scope

- Reuse WP61 `AppGameNotificationProviderPreflightReadModel` rows.
- Convert provider-adapter-required and manual-required preflight rows into
  V0.8 provider-status manual-required boundary entries.
- Convert unavailable preflight rows into V0.8 provider-status unavailable
  boundary entries.
- Preserve scheduler, outbox, provider-channel, readiness, and manual proof
  refs where the source row provides them.
- Preserve explicit false claims for provider delivery, receipt ingestion,
  credentials, cloud routing, parent notification UI/history/preferences, child
  delivery, retry-worker execution, quiet-hours timer execution, production
  durable outbox storage, adapter dispatch, broad blocking, and platform
  support.

## Non-Goals

- Provider push, email, SMS, WhatsApp, or in-app delivery execution.
- Provider credentials, provider templates, webhooks, delivery receipts, or
  receipt ingestion.
- Production retry workers, production quiet-hours timers, durable production
  outbox storage, or cloud routing.
- Parent notification history/preferences UI.
- Child-device delivery, policy evaluator execution, adapter dispatch, broad
  app/game blocking, or platform support.

## Proof

- `packages/parent-domain/src/app-game-notification-provider-status-handoff.ts`
- `packages/parent-domain/tests/app-game-notification-provider-status-handoff.test.ts`
- `scripts/test/app-game-notification-provider-status-handoff-proof.mjs`
- `test-results/app-game-notification-provider-status-handoff-proof/proof.json`
- `output/app-game-plan-proof/64-notification-provider-status-handoff/`
- `output/app-plan-proof/64-notification-provider-status-handoff/`

## Validation

- [x] Provider-status handoff parses WP61 app/game provider-preflight rows before
      mapping.
- [x] Provider-adapter-required and manual-required preflight rows become
      manual-required V0.8 provider-status boundary entries.
- [x] Unavailable preflight rows become unavailable V0.8 provider-status
      boundary entries.
- [x] Proof pack records no provider delivery, no receipt ingestion, no
      credentials, no retry-worker/quiet-hours timer runtime, no parent UI, no
      child delivery, no adapter dispatch, no broad blocking, and no durable
      production outbox claim.
- [x] Product checklist unchanged because this handoff does not move feature
      status and provider/runtime/UI/platform gaps remain.
