# Workpack 07: Schedule Time Budget Conflict Model

Purpose: define schedule, time budget, recurrence, timezone, DST, clock skew, exception, bonus time, and precedence behavior before policy authoring or delivery claims.

Owns: schedule semantics, time-budget reset and carryover, DST boundaries, ambiguous/nonexistent local time handling, conflict precedence, bonus-time expiry, and offline timer recovery.

## Required proof IDs

- `policy-schedule.timezone-model`
- `policy-schedule.local-time-window`
- `policy-schedule.recurrence-rule`
- `policy-schedule.exception-date`
- `policy-schedule.dst-spring-forward`
- `policy-schedule.dst-fall-back`
- `policy-schedule.ambiguous-local-time`
- `policy-schedule.nonexistent-local-time`
- `policy-schedule.clock-skew`
- `policy-schedule.child-device-clock-source`
- `policy-schedule.budget-reset`
- `policy-schedule.budget-carryover`
- `policy-schedule.grace-period`
- `policy-schedule.bonus-time-expiry`
- `policy-schedule.exception-precedence`
- `policy-schedule.offline-device-timer-recovery`
- `policy-conflict.precedence-matrix`
- `policy-conflict.manual-required-output`

## Required behavior

- Schedules carry timezone, recurrence, exceptions, reset, grace, expiry, and clock-source semantics.
- DST and ambiguous/nonexistent local times are explicit.
- Conflict precedence is deterministic or manualRequired.
- Bonus time and overrides expire.
- Offline timer recovery is defined.

## Canonical code-and-test checkpoint — 2026-08-28

- Canonical `e565bd9dd` hardens the Rust-owned schedule, time-budget,
  conflict, child-request, parent-approval, temporary-override, and policy-source
  contracts. The packet enforces real UTC calendar timestamps, strict temporal
  ordering, positive budget/carryover values, bonus-time/action consistency,
  manual-required handling for unsupported timezone ownership, replay
  validation, and deadline-gated expiry.
- The five mapped Rust test roots now contain focused source for invalid calendar
  dates, zero budget/carryover, cross-midnight conflicts, unsupported timezone
  handling, request/approval window ordering, bonus-time bounds, expiry, and
  replay behavior.
- No test was executed in this code-first phase. The existing proof files were
  not regenerated and do not validate this new packet yet.
- This is not runtime completion. No shipped owner supplies a trusted clock,
  timezone/DST resolver, durable timer journal, restart/offline recovery, or a
  production caller for the crate-private override-expiry transition. Those
  owner/runtime paths and their tests must be written before focused execution
  and proof regeneration.

## Negative cases

```text
DST skip grants wrong time
DST repeat grants double time
clock skew bypasses bedtime
timezone missing
bonus time never expires
offline child loses timer state
conflict silently last-write-wins
manual-required conflict auto-applies
```

## Required proof artifacts

```text
docs/proof/policy-control-plane-plan/07-schedule-timezone-proof.md
docs/proof/policy-control-plane-plan/07-dst-boundary-proof.md
docs/proof/policy-control-plane-plan/07-time-budget-reset-proof.md
docs/proof/policy-control-plane-plan/07-conflict-precedence-proof.md
docs/proof/policy-control-plane-plan/07-offline-timer-recovery-proof.md
```
