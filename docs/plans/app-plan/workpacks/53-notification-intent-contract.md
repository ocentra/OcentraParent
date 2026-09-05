# 53. Notification Intent Contract

This native app-plan workpack is cross-recorded from shared app/game WP53.

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-notification-intent-contract`
- Scope: parent-domain app/game notification intent contract proof.

## Goal

Represent native app notification intent readiness for time-limit, approval
request, suspicious unknown, manual-required, and unavailable states while
preserving the shared app/game evidence spine and avoiding provider, UI,
service, policy-evaluator, adapter, broad-blocking, or platform-support claims.

## Current Source/Test Review (2026-08-29)

The Rust protocol DTO and its nested readiness rows now reject unknown fields,
and the registered contract test covers a valid serde round trip plus top-level
and nested unknown-field rejection. This contract can represent the named
readiness states; it does not authorize the service to manufacture a time-limit
or approval-request intent without an owned runtime signal. Delivery, UI,
scheduler, adapter, and child claims remain false.

## Proof

- `scripts/test/app-game-notification-intent-proof.mjs`
- `output/app-plan-proof/53-notification-intent-contract`
- `output/app-game-plan-proof/53-notification-intent-contract`
- `test-results/app-game-notification-intent-proof/proof.json`

## DONE Checklist

- [ ] Cross-recorded from shared app/game WP53.
- [ ] Native app targets remain app/game evidence refs and policy refs, not
      provider payload or adapter authority.
- [ ] Raw child evidence, URLs/titles, message text, screenshots, and reports
      remain excluded from notification payload boundaries.
- [ ] Provider delivery, receipt ingestion, parent notification UI, service
      persistence, child delivery, policy evaluator execution, broad blocking,
      and platform support remain unclaimed.
