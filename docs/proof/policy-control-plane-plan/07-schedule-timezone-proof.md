# WP07 Schedule Timezone Proof

## Proves

- `policy-schedule.timezone-model`
- `policy-schedule.local-time-window`
- `policy-schedule.recurrence-rule`
- `policy-schedule.exception-date`
- `policy-schedule.clock-skew`
- `policy-schedule.child-device-clock-source`

## Evidence

- `packages/policy-domain/tests/unit/policy.test.ts`
  - `parseFamilyPolicySet: parses parent-authored rules, schedules, children, and devices with explicit time-budget semantics`
- `packages/policy-domain/tests/unit/policy-schedule-boundaries.test.ts`
  - `parsePolicyScheduleBoundary: accepts active exception windows only while they are still live`
  - `parsePolicyScheduleBoundary: rejects non-expired boundaries evaluated after schedule expiry`
  - `resolvePolicyPreviewBudgetBoundaryState: marks manual clock-source preview boundaries as manual-required`
- `crates/policy-control-core/tests/unit/policy_conflict.rs`
  - `timezone_mismatch_conflict_stays_explicit_and_manual_required`
  - `manual_clock_source_stays_explicit_clock_skew_conflict`
  - `child_device_clock_source_does_not_auto_create_clock_skew_conflict`

## Result

- Schedule boundaries keep timezone and local-time semantics explicit.
- Exception windows and expiry remain typed contract inputs.
- Manual clock sources and timezone mismatches surface as explicit conflicts instead of being auto-normalized away.

