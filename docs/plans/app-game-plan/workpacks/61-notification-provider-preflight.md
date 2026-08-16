# 61. Notification Provider Preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `61. Notification Provider Preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Bridge app/game notification scheduler rows into an explicit provider-adapter
preflight read model so scheduled alerts expose the provider work still required
before any delivery can be claimed.

## Scope

- Reuse WP59 `AppGameNotificationSchedulerBridgeReadModel` rows.
- Convert scheduled local scheduler rows into `provider-adapter-required`
  preflight rows with scheduler, outbox, decision, provider-channel, and reason
  refs preserved.
- Keep manual-required and unavailable app/game notification rows blocked before
  provider preflight with manual proof requirements.
- Preserve explicit false claims for provider delivery, receipt ingestion,
  credentials, cloud routing, parent notification UI, child delivery,
  retry-worker execution, quiet-hours timer execution, production durable
  outbox storage, and adapter dispatch.

## Non-Goals

- Provider push, email, SMS, WhatsApp, or in-app delivery execution.
- Provider credentials, provider templates, webhooks, delivery receipts, or
  receipt ingestion.
- Production retry workers, production quiet-hours timers, durable production
  outbox storage, or cloud routing.
- Parent notification history/preferences UI.
- Child-device delivery, policy evaluator execution, adapter dispatch, broad
  app/game blocking, or platform support.

## Current Code Audit (2026-08-15)

- `app_game_child_ux_provider_preflight` validates one canonical scheduler row
  against its persisted local-outbox record, rejects identity/evidence/unsafe
  claim mismatches, and maps due/manual/unavailable states without delivery.
- Existing contract tests cover provider-adapter-required, manual,
  unavailable, unpersisted, mismatched, claimed, and missing-requirement cases.
- No current WP61 owner consumes the complete WP59
  `AppGameNotificationSchedulerBridgeReadModel`, generates deterministic
  adapter/credential/smoke requirement refs per scheduled row, or preserves
  WP59 manual/unavailable rows in one preflight read model.
- The advertised `packages/parent-domain` source/test owner and proof harness
  are absent. Current ownership is Rust `app-game-core` plus the canonical
  agent-protocol scheduler schema.

## Proof

- `crates/app-game-core/src/app_game_child_ux_provider_preflight.rs`
- `crates/app-game-core/src/app_game_child_ux_provider_preflight_types.rs`
- `crates/app-game-core/tests/contract/app_game_child_ux_outbox.rs`
- Historical `packages/parent-domain/...` and script harness routes are absent.
- `test-results/app-game-notification-provider-preflight-proof/proof.json`
- `output/app-game-plan-proof/61-notification-provider-preflight/`
- `output/app-plan-proof/61-notification-provider-preflight/`

## Validation

- [ ] Provider preflight parses the WP59 app/game scheduler bridge read model
      before mapping rows.
- [ ] Scheduled local rows become provider-adapter-required preflight rows with
      source scheduler/outbox/decision/provider/reason refs.
- [ ] Manual-required and unavailable rows remain blocked before provider
      preflight.
- [ ] Proof pack records no provider delivery, no receipt ingestion, no
      credentials, no retry-worker/quiet-hours timer runtime, no parent UI, no
      child delivery, no adapter dispatch, and no durable production outbox
      claim.
