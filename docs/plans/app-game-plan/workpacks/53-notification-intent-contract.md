# 53. Notification Intent Contract

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `53. Notification Intent Contract`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

- [ ] Hub lock covers the exact implementation, docs, proof, and validation
      paths.
- [ ] App/game child-facing UX, time-budget policy, notification provider, and
      local outbox contract patterns inspected.
- [ ] TypeScript contract/parser rejects wrong reason/copy tokens, missing
      kind-specific refs, raw detail leakage, provider delivery/receipt claims,
      false local-outbox claims, and adapter action claims.
- [ ] Proof pack records no Rust/service protocol, no UI, no product checklist
      change, no policy execution, no provider delivery, no adapter dispatch,
      and no platform support claim.
