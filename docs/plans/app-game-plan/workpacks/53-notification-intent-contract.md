# 53. Notification Intent Contract

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-notification-intent-contract`
- Scope: parent-domain app/game notification intent contract proof.

## Goal

Represent app/game notification intent readiness for time-limit, approval
request, suspicious unknown, manual-required, and unavailable states without
claiming provider delivery, parent notification UI, service persistence, child
delivery, policy evaluator execution, adapter dispatch, broad blocking, or
platform support.

## In Scope

- Add a parent-domain Effect Schema contract and rule guards for app/game
  notification intents.
- Require evidence, policy, audit, child reason/status, and kind-specific refs
  before an intent parses.
- Require minimal notification payload fields and reject raw child evidence,
  URLs/titles, message text, screenshots, and reports.
- Add focused TypeScript tests and deterministic proof packs under both shared
  app/game and native app proof roots.

## Out Of Scope

- Provider delivery or provider receipt ingestion.
- Parent notification UI, preference UI, or history UI.
- Service persistence, WebSocket protocol, or Rust parity.
- Child app, overlay, push, or local notification delivery.
- Policy evaluator execution, adapter dispatch, broad installed-app blocking,
  or platform support claims.
- `docs/product-capability-checklist.md` edits.

## Proof

- `scripts/test/app-game-notification-intent-proof.mjs`
- `output/app-game-plan-proof/53-notification-intent-contract`
- `output/app-plan-proof/53-notification-intent-contract`
- `test-results/app-game-notification-intent-proof/proof.json`

## DONE Checklist

- [x] Hub lock covers the exact implementation, docs, proof, and validation
      paths.
- [x] App/game child-facing UX, time-budget policy, notification provider, and
      local outbox contract patterns inspected.
- [x] TypeScript contract/parser rejects wrong reason/copy tokens, missing
      kind-specific refs, raw detail leakage, provider delivery/receipt claims,
      false local-outbox claims, and adapter action claims.
- [x] Proof pack records no Rust/service protocol, no UI, no product checklist
      change, no policy execution, no provider delivery, no adapter dispatch,
      and no platform support claim.
