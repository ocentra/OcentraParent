# 55. Notification Scheduler Handoff

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-notification-outbox-bridge`
- Scope: parent-domain app/game notification scheduler handoff proof.

## Goal

Bridge WP54 app/game local outbox records into scheduler-ready local rows without
claiming provider delivery, provider receipt ingestion, parent notification UI,
production retry workers, quiet-hours timer execution, durable service
persistence, child-device delivery, policy evaluator execution, adapter
dispatch, broad blocking, or platform support.

## In Scope

- Add a parent-domain Effect Schema handoff from
  `AppGameNotificationLocalOutboxBridgeProof` rows into
  `NotificationLocalOutboxSchedulerRecord` rows.
- Schedule eligible app/game time-limit, approval request, and suspicious
  unknown bridge rows as `due-local` scheduler rows with parent-owned artifact
  refs.
- Keep manual-required and capability-unavailable app/game notification intents
  blocked from scheduler rows.
- Add focused TypeScript tests and deterministic proof packs under both shared
  app/game and native app proof roots.

## Out Of Scope

- Production scheduler timer loops, retry workers, or quiet-hours execution.
- Provider delivery, provider receipt ingestion, or provider credentials.
- Parent notification UI, preference UI, or history UI.
- Service persistence, WebSocket notification read models, or durable production
  outbox storage.
- Child app, overlay, push, or local notification delivery.
- Policy evaluator execution, adapter dispatch, broad installed-app blocking, or
  platform support claims.
- `docs/product-capability-checklist.md` edits.

## Proof

- `scripts/test/app-game-notification-scheduler-handoff-proof.mjs`
- `output/app-game-plan-proof/55-notification-scheduler-handoff`
- `output/app-plan-proof/55-notification-scheduler-handoff`
- `test-results/app-game-notification-scheduler-handoff-proof/proof.json`

## DONE Checklist

- [x] Hub lock covers the exact implementation, docs, proof, and validation
      paths.
- [x] WP54 local outbox bridge and shared notification scheduler record proof
      patterns inspected.
- [x] TypeScript handoff rejects provider/runtime overclaims and incoherent
      scheduler links.
- [x] Eligible app/game bridge rows create `due-local` scheduler rows with
      parent-owned artifact refs.
- [x] Manual-required and unavailable app/game notification intents remain
      blocked from scheduler rows.
- [x] Proof pack records no Rust/service protocol, no UI, no product checklist
      change, no policy execution, no provider delivery, no adapter dispatch,
      no durable service persistence, and no platform support claim.
