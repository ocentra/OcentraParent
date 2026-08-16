# WP07 Time Budget Reset Proof

## Proves

- `policy-schedule.budget-reset`
- `policy-schedule.budget-carryover`
- `policy-schedule.grace-period`
- `policy-schedule.bonus-time-expiry`

## Evidence

- `packages/policy-domain/tests/unit/policy.test.ts`
  - `parsePolicySchedule: rejects capped carryover without an explicit minute cap`
  - `parsePolicySchedule: rejects weekly resets without an explicit reset day`
- `packages/policy-domain/tests/unit/policy-schedule-boundaries.test.ts`
  - `parsePolicyScheduleBoundary: accepts budget status with recovered offline timer state and expiring bonus time`
  - `parsePolicyScheduleBoundary: rejects active bonus time without an explicit expiry timestamp`
  - `parsePolicyScheduleBoundary: rejects active bonus time that omits remaining preview minutes`
  - `resolvePolicyPreviewBudgetBoundaryState: marks active bonus time with shrinking minutes as expiring`
- `packages/policy-domain/tests/unit/policy-approval-override.test.ts`
  - `resolvePolicyApprovalLifecycle: accepts positive bonus-time grants that stay inside approval and schedule context`
  - `resolvePolicyApprovalLifecycle: rejects bonus-time approvals without a positive request amount`
  - `resolvePolicyApprovalLifecycle: rejects bonus-time approvals without schedule budget context`
- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `weekly_reset_requires_day`
  - `daily_and_monthly_resets_reject_unexpected_day`
  - `discard_unused_carryover_rejects_max_minutes`
  - `cap_carryover_requires_positive_max_minutes`
  - `bonus_expiry_minutes_must_be_non_zero`

## Result

- Reset and carryover rules are explicit and validated.
- Bonus-time approvals require schedule budget context and explicit expiry.
- Grace/expiry semantics are represented in the boundary model rather than implied by UI.

