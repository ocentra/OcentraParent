# 62. Notification Preference Preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `62. Notification Preference Preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Bridge app/game notification scheduler rows into an explicit parent-preference
preflight read model so scheduled alerts expose parent preference, frequency,
and quiet-hours proof requirements before any provider delivery can be claimed.

## Scope

- Reuse WP59 `AppGameNotificationSchedulerBridgeReadModel` rows.
- Convert scheduled local scheduler rows into `parent-preference-required`
  preflight rows with scheduler, outbox, provider-channel, and reason refs
  preserved.
- Require parent preference, frequency-control, and quiet-hours policy proof
  refs before delivery.
- Keep manual-required and unavailable app/game notification rows blocked before
  preference preflight with manual proof requirements.
- Preserve explicit false claims for parent preference UI, frequency-control UI,
  quiet-hours timer runtime, provider delivery, receipt ingestion, credentials,
  cloud routing, child delivery, retry-worker execution, durable production
  outbox storage, and adapter dispatch.

## Non-Goals

- Parent notification history, preferences, or frequency-control UI.
- Provider push, email, SMS, WhatsApp, or in-app delivery execution.
- Provider credentials, provider templates, webhooks, delivery receipts, or
  receipt ingestion.
- Production retry workers, production quiet-hours timers, durable production
  outbox storage, or cloud routing.
- Child-device delivery, policy evaluator execution, adapter dispatch, broad
  app/game blocking, or platform support.

## Current Code Audit (2026-08-15)

- `app_game_child_ux_preference_preflight` validates one scheduler row against
  its persisted local-outbox source record and rejects identity, evidence, and
  unsafe-claim mismatches.
- Due-local rows become parent-preference-required only with distinct parent
  preference, notification-frequency, and quiet-hours requirement refs;
  manual and unavailable scheduler states remain blocked.
- Focused Rust contract tests cover due/manual/unavailable, unpersisted,
  mismatched, claimed, and duplicate-requirement paths.
- No current WP62 owner consumes the complete WP59 read model, verifies the
  durable scheduler store, generates deterministic requirements, or retains
  blocked rows as one preflight read model. Historical `packages/parent-domain`
  and proof-harness routes are absent.

## Proof

- `crates/app-game-core/src/app_game_child_ux_preference_preflight.rs`
- `crates/app-game-core/src/app_game_child_ux_preference_preflight_types.rs`
- `crates/app-game-core/tests/contract/app_game_child_ux_outbox.rs`
- Historical `packages/parent-domain/...` and script harness routes are absent.
- `test-results/app-game-notification-preference-preflight-proof/proof.json`
- `output/app-game-plan-proof/62-notification-preference-preflight/`
- `output/app-plan-proof/62-notification-preference-preflight/`

## Validation

- [ ] Preference preflight parses the WP59 app/game scheduler bridge read model
      before mapping rows.
- [ ] Scheduled local rows become parent-preference-required preflight rows with
      source scheduler/outbox/provider/reason refs.
- [ ] Manual-required and unavailable rows remain blocked before preference
      preflight.
- [ ] Proof pack records no parent preference UI, no frequency-control UI, no
      quiet-hours timer runtime, no provider delivery, no receipt ingestion, no
      credentials, no child delivery, no adapter dispatch, and no durable
      production outbox claim.
