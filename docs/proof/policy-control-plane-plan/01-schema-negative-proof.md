# WP01 Schema Negative Proof

## Proves

- `policy-source.schema-negative`
- `policy-source.migration-boundary`

## Evidence

- `packages/policy-domain/tests/unit/policy.test.ts`
  - `parseFamilyPolicySet: parses parent-authored rules, schedules, children, and devices with explicit time-budget semantics`
  - `parsePolicySchedule: rejects capped carryover without an explicit minute cap`
  - `parsePolicySchedule: rejects weekly resets without an explicit reset day`
- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `rules_cannot_reference_unknown_schedule_ids`
  - `weekly_reset_requires_day`
  - `daily_and_monthly_resets_reject_unexpected_day`
  - `discard_unused_carryover_rejects_max_minutes`
  - `cap_carryover_requires_positive_max_minutes`
  - `effective_until_must_be_after_effective_from`
  - `bonus_expiry_minutes_must_be_non_zero`
- `crates/policy-control-core/tests/version-skew/policy_source_migration.rs`
  - `compatibility_input_rejects_schedule_payload_without_time_budget`

## Result

- Invalid schedule/time-budget payloads are rejected at the contract boundary.
- Schema acceptance is explicit, not best-effort.
- Migration-required input is distinguished from invalid input.

