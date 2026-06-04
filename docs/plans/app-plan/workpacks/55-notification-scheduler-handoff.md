# 55. Notification Scheduler Handoff

This native app-plan workpack is cross-recorded from shared app/game WP55.

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-notification-outbox-bridge`
- Scope: parent-domain app/game notification scheduler handoff proof.

## Goal

Represent the native app side of app/game notification scheduler readiness by
bridging eligible app/game local outbox records into scheduler-ready local rows
while preserving the shared app/game evidence spine and avoiding provider, UI,
service, timer, policy-evaluator, adapter, broad-blocking, child-delivery, or
platform-support claims.

## Proof

- `scripts/test/app-game-notification-scheduler-handoff-proof.mjs`
- `output/app-plan-proof/55-notification-scheduler-handoff`
- `output/app-game-plan-proof/55-notification-scheduler-handoff`
- `test-results/app-game-notification-scheduler-handoff-proof/proof.json`

## DONE Checklist

- [x] Cross-recorded from shared app/game WP55.
- [x] Native app targets remain app/game notification refs, local outbox refs,
      evidence refs, and policy refs, not provider payload or adapter authority.
- [x] Eligible native app notification bridge rows become `due-local` scheduler
      rows only.
- [x] Manual-required and unavailable native app notification intents remain
      blocked from scheduler rows.
- [x] Provider delivery, receipt ingestion, provider credentials, parent
      notification UI, retry worker execution, quiet-hours timer execution,
      service persistence, child delivery, policy evaluator execution, broad
      blocking, and platform support remain unclaimed.
