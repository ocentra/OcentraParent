# 58. Notification Local Outbox Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `58. Notification Local Outbox Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-notification-local-outbox-bridge`
- Scope: parent-domain app/game notification intent to local outbox bridge.

## Goal

Prove that validated app/game notification intents can be bridged into the
existing parent-owned local notification outbox record schema and deterministic
JSONL artifact without creating a second outbox truth or claiming provider
delivery/runtime support.

## In Scope

- Add a parent-domain bridge from `AppGameNotificationIntent` to the existing
  `NotificationLocalOutboxRecord` schema.
- Write and reread deterministic JSONL records for local-outbox-eligible
  time-limit and suspicious-unknown app/game notification intents.
- Keep manual-required and unavailable app/game intents visible in the bridge
  read model without queueing delivery records.
- Preserve explicit false claims for provider delivery, receipts, scheduler
  runtime, cloud routing, parent UI, child delivery, adapter dispatch, broad
  blocking, and platform support.
- Add focused contract tests and proof packs under both shared app/game and
  native app proof roots.

## Out Of Scope

- Durable production local outbox storage.
- Provider delivery, credentials, webhook receipts, or receipt ingestion.
- Quiet-hours timer execution, retry workers, or production scheduler runtime.
- Parent notification UI, preference UI, or history UI.
- Child app, overlay, push, or local notification delivery.
- Policy evaluator execution, adapter dispatch, broad installed-app blocking,
  or platform support claims.
- `packages/parent-domain/package.json` and `packages/parent-domain/README.md`
  updates, because E-B owns those locks during this slice.

## Current Code Audit (2026-08-15)

- The Rust notification-readiness contract and projection expose eligible
  intent rows, but there is no production `AppGameNotificationIntent` to
  `NotificationLocalOutboxRecord` append boundary.
- `build_activity_app_game_notification_readiness_report` currently derives
  `local_outbox_runtime_claimed` from
  `setup_outbox_has_records(activity_db_path)`. That helper reads the separate
  WP125 parent-preference setup outbox, not an app/game notification outbox.
  Any non-empty setup record can therefore create a false WP58 runtime claim.
- The focused service regression proves only that the unrelated setup-outbox
  file flips the Boolean. It does not prove eligible intent mapping, typed
  append, reopen, idempotency/conflict handling, manual/unavailable exclusion,
  or dead-letter behavior.
- The named proof script, generated proof root, and test-results artifact are
  absent from the tracked checkout. They cannot support the historical DONE
  wording.
- WP121 supplies a typed atomic child-UX local-outbox store and canonical
  `NotificationLocalOutboxRecord` use, but it does not consume the general
  WP53/WP56 notification-readiness rows. WP58 must reuse that canonical storage
  truth or a shared owner rather than create a second JSONL format.

## Proof

- `scripts/test/app-game-notification-local-outbox-bridge-proof.mjs`
- `output/app-game-plan-proof/58-notification-local-outbox-bridge`
- `output/app-plan-proof/58-notification-local-outbox-bridge`
- `test-results/app-game-notification-local-outbox-bridge-proof/proof.json`

## DONE Checklist

- [ ] Hub lock covers bridge source/test, proof harness, proof roots, product
      docs, and workpack docs.
- [x] Existing app/game notification intent contract and notification local
      outbox adapter proof inspected and reused.
- [ ] Eligible app/game notification intents become existing local outbox
      records and round-trip through JSONL parsing.
- [ ] Manual-required and unavailable app/game notification intents do not
      queue delivery records.
- [ ] Proof pack records no provider delivery, no receipt ingestion, no
      scheduler runtime, no parent UI, no child delivery, no policy execution,
      no adapter dispatch, no broad blocking, and no platform support claim.
