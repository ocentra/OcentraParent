# 65. Notification Preference Status Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `65. Notification Preference Status Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Map app/game notification preference-preflight rows into V3 notification
preference and quiet-hours status entries so parent preference requirements are
visible in the shared notification boundary without claiming parent UI or
delivery.

## Scope

- Reuse WP62 `AppGameNotificationPreferencePreflightReadModel` rows.
- Convert parent-preference-required and manual-required rows into V3
  manual-required notification status entries.
- Convert unavailable rows into V3 disabled/not-sent notification status
  entries.
- Preserve scheduler, outbox, provider-channel, reason, parent preference,
  quiet-hours, and manual proof refs where the source row provides them.
- Preserve explicit false claims for parent preference UI, frequency controls,
  parent notification UI, provider delivery, receipt ingestion, credentials,
  cloud routing, child delivery, retry-worker execution, quiet-hours timer
  execution, production durable outbox storage, adapter dispatch, broad
  blocking, and platform support.

## Non-Goals

- Parent notification preferences UI, frequency controls, or history UI.
- Provider push, email, SMS, WhatsApp, or in-app delivery execution.
- Provider credentials, provider templates, webhooks, delivery receipts, or
  receipt ingestion.
- Production retry workers, production quiet-hours timers, durable production
  outbox storage, or cloud routing.
- Child-device delivery, policy evaluator execution, adapter dispatch, broad
  app/game blocking, or platform support.

## Proof

- `packages/parent-domain/src/app-game-notification-preference-status-handoff.ts`
- `packages/parent-domain/tests/app-game-notification-preference-status-handoff.test.ts`
- `scripts/test/app-game-notification-preference-status-handoff-proof.mjs`
- `test-results/app-game-notification-preference-status-handoff-proof/proof.json`
- `output/app-game-plan-proof/65-notification-preference-status-handoff/`
- `output/app-plan-proof/65-notification-preference-status-handoff/`

## Validation

- [ ] Preference-status handoff parses WP62 app/game preference preflight rows
      before mapping.
- [ ] Parent-preference-required and manual-required rows become V3
      manual-required notification preference/quiet-hours status entries.
- [ ] Unavailable rows become V3 disabled/not-sent status entries.
- [ ] Proof pack records no parent preference UI, no frequency controls, no
      parent notification UI, no provider delivery, no receipt ingestion, no
      credentials, no retry-worker/quiet-hours timer runtime, no child delivery,
      no adapter dispatch, no broad blocking, and no durable production outbox
      claim.
- [ ] Product checklist unchanged because this handoff does not move feature
      status and provider/runtime/UI/platform gaps remain.

## Current production-code pass (2026-08-16)

- The current owner is the agent-service notification report boundary at
  `crates/agent-service/src/activity_api/app_game_notification_readiness_payload.rs`
  and its `logic.rs` plus private `scheduler_runtime.rs` companion.
- The report now loads the persisted WP59 scheduler bridge and service-owned
  scheduler proof store, invokes the Rust WP62 preference-preflight builder,
  and maps only paired verified rows into the preference-status boundary.
- This is a consumer-only service seam: no production writer currently emits
  `scheduler-bridge.json`, so WP59 scheduler production/runtime composition
  and durability remain open rather than being implied by this handoff.
- Missing, malformed, symlinked, or identity-mismatched scheduler evidence
  falls back to explicit invalid/manual-required or unavailable status only. No preference
  mutation, frequency UI, quiet-hours timer, provider delivery, receipt,
  credential, retry, UI, child-delivery, adapter, or enforcement claim is
  produced.
- The historical `packages/parent-domain/...` source/test paths above are not
  current implementation owners. Tests, validation, retained proof, and
  runtime durability remain deferred; this workpack is code-drafted, not DONE.
