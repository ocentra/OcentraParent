# 68. Notification Live Parent Surface Read Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `68. Notification Live Parent Surface Read Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Wire the live app/game notification-readiness service event into the Portal
App/Game Sessions parent-surface panel so real service rows render as
parent-visible manual/unavailable setup state without claiming provider
delivery, preference mutation, scheduler/outbox runtime, child delivery, or
adapter dispatch.

## Scope

- Request the existing
  `agent.activity.app-game.notification-readiness.read-model.get` command in the
  portal overview command set.
- Recognize the existing
  `agent.activity.app-game.notification-readiness.read-model.reported` event as
  a command-result event.
- Parse the live service event through the existing agent-protocol-domain
  readiness parser.
- Project the validated readiness read model into the existing
  parent-domain `AppGameNotificationParentSurfaceIntentReadModel` shape inside
  portal-domain.
- Render the projected rows through the existing WP67 App/Game Sessions route
  panel, including drill-in refs, missing runtime refs, manual proof
  requirements, and no-runtime claim text.

## Non-Goals

- Provider push, email, SMS, WhatsApp, in-app delivery, credentials, webhooks,
  delivery receipts, receipt ingestion, or cloud routing.
- Parent preference mutation, frequency controls, quiet-hours editor behavior,
  or parent notification delivery UI.
- Production retry workers, production quiet-hours timers, durable production
  outbox/history storage, scheduler runtime, or local outbox runtime.
- Child-device delivery, mobile UI, policy evaluator execution, adapter
  dispatch, broad app/game blocking, or platform support.
- `docs/product-capability-checklist.md` update in this slice; E-B owns that
  central checklist lock. This workpack records the product-doc decision and
  keeps the checklist delta for the next available coordinated slot.
- `docs/features/reports-notifications-sync.md` update in this slice; codex-a
  owns that feature doc lock. The app-game owning feature doc was updated.

## Proof

- `packages/portal-domain/src/app-game-notification-parent-surface-live-readiness.ts`
- `packages/portal-domain/src/app-game-notification-parent-surface-panel.ts`
- `packages/portal-domain/src/commands.ts`
- `packages/portal-domain/tests/app-game-notification-parent-surface-panel.test.ts`
- `apps/portal/src/live-activity-state.ts`
- `packages/portal-domain/src/command-results.ts`
- `apps/portal/tests/app-game-notification-parent-surface-panel.test.ts`
- `scripts/test/app-game-notification-live-parent-surface-proof.mjs`
- `test-results/app-game-notification-live-parent-surface-proof/proof.json`
- `output/app-game-plan-proof/68-notification-live-parent-surface-read-model/`
- `output/app-plan-proof/68-notification-live-parent-surface-read-model/`

## Validation

- [ ] Portal overview commands request the existing service notification
      readiness read model.
- [ ] Portal live state derives a parent-surface read model only from the
      validated service readiness event.
- [ ] Portal-domain projection maps manual-required and unavailable readiness
      rows into schema-backed parent-surface setup rows.
- [ ] Scheduler/outbox runtime refs remain `not reported` because the readiness
      event does not prove those runtime outputs.
- [ ] Provider delivery, receipt ingestion, preference mutation, child delivery,
      scheduler/outbox runtime, adapter dispatch, broad blocking, mobile UI, and
      platform support remain explicit non-claims.
