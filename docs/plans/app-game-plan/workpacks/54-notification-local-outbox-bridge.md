# 54. Notification Local Outbox Bridge

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-notification-outbox-bridge`
- Scope: parent-domain app/game notification local outbox bridge proof.

## Goal

Bridge app/game notification intent readiness into parent-owned local outbox
record shapes without claiming provider delivery, parent notification UI, service
persistence, child-device delivery, policy evaluator execution, adapter
dispatch, broad blocking, or platform support.

## In Scope

- Add a parent-domain Effect Schema bridge from local-outbox-eligible app/game
  notification intents to `NotificationLocalOutboxRecord` rows.
- Map app/game time-limit, approval request, and suspicious unknown notification
  reasons into the existing notification reason/channel contract boundary.
- Keep manual-required and capability-unavailable app/game notification intents
  blocked from local outbox records.
- Add focused TypeScript tests and deterministic proof packs under both shared
  app/game and native app proof roots.

## Out Of Scope

- Provider delivery or provider receipt ingestion.
- Parent notification UI, preference UI, or history UI.
- Service persistence, WebSocket notification read models, or durable production
  outbox storage.
- Child app, overlay, push, or local notification delivery.
- Policy evaluator execution, adapter dispatch, broad installed-app blocking, or
  platform support claims.
- `docs/product-capability-checklist.md` edits.

## Proof

- `scripts/test/app-game-notification-local-outbox-bridge-proof.mjs`
- `output/app-game-plan-proof/54-notification-local-outbox-bridge`
- `output/app-plan-proof/54-notification-local-outbox-bridge`
- `test-results/app-game-notification-local-outbox-bridge-proof/proof.json`

## DONE Checklist

- [x] Hub lock covers the exact implementation, docs, proof, and validation
      paths.
- [x] WP53 notification intent and notification local outbox adapter proof
      patterns inspected.
- [x] TypeScript bridge rejects provider/runtime overclaims and incoherent bridge
      records.
- [x] Local-outbox-eligible app/game intents create queued local outbox records
      with minimal ref-only envelopes.
- [x] Manual-required and unavailable app/game notification intents remain
      blocked from local outbox records.
- [x] Proof pack records no Rust/service protocol, no UI, no product checklist
      change, no policy execution, no provider delivery, no adapter dispatch, and
      no platform support claim.
