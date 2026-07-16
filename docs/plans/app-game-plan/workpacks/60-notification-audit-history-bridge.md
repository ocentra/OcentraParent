# 60. Notification Audit-History Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `60. Notification Audit-History Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Bridge app/game notification local outbox handoff rows into the existing
logging-domain notification audit-history entry schema so queued,
manual-required, and unavailable app/game notification states can be audited
without claiming provider delivery, parent history UI, or production runtime.

## Scope

- Reuse WP58 app/game notification local outbox bridge rows as the source.
- Add a logging-domain handoff read model that converts linked local outbox rows
  into queued audit-history entries.
- Keep manual-required and unavailable rows audit-visible with blocked/manual
  refs and without queued provider sends.
- Preserve source audit, evidence, and policy refs in the handoff source rows.
- Keep provider delivery, receipt ingestion, credentials, cloud routing,
  notification history UI, child delivery, retry execution, quiet-hours timer
  execution, production durable outbox storage, adapter dispatch, broad
  blocking, and platform support unclaimed.

## Non-Goals

- Provider push, email, SMS, WhatsApp, or in-app delivery execution.
- Provider credentials, receipt webhooks, receipt ingestion, retry workers, or
  quiet-hours timers.
- Parent notification history/preferences UI.
- Child-device delivery, policy evaluator execution, adapter dispatch, broad
  app/game blocking, or platform support.

## Proof

- `packages/logging-domain/src/notification-audit-history-handoff.ts`
- `packages/logging-domain/tests/notification-audit-history-handoff.test.ts`
- `scripts/test/app-game-notification-audit-history-bridge-proof.mjs`
- `test-results/app-game-notification-audit-history-bridge-proof/proof.json`
- `output/app-game-plan-proof/60-notification-audit-history-bridge/`
- `output/app-plan-proof/60-notification-audit-history-bridge/`

## Validation

- [ ] Handoff parses app/game local outbox bridge rows before audit entry
      creation.
- [ ] Linked local outbox rows become queued audit-history entries.
- [ ] Manual-required and unavailable rows remain blocked/manual and do not
      create provider sends.
- [ ] Source audit, evidence, and policy refs are preserved in the handoff read
      model.
- [ ] Proof pack records no provider delivery, no receipt ingestion, no
      retry-worker/quiet-hours timer runtime, no parent UI, no child delivery,
      no adapter dispatch, no durable production outbox, and no platform claim.
