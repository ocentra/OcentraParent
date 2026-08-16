# WP07 Conflict Precedence Proof

## Proves

- `policy-conflict.precedence-matrix`
- `policy-conflict.manual-required-output`

## Evidence

- `crates/policy-control-core/tests/unit/policy_conflict.rs`
  - `higher_priority_rule_wins_for_overlapping_target_actions`
  - `equal_priority_overlap_requires_manual_review`
  - `timezone_mismatch_conflict_stays_explicit_and_manual_required`
  - `device_targets_missing_from_household_inventory_are_blocking`
  - `rolled_back_source_conflicts_preserve_rollback_context`
  - `disabled_rule_does_not_create_conflict_noise`
  - `manual_clock_source_stays_explicit_clock_skew_conflict`
  - `child_device_clock_source_does_not_auto_create_clock_skew_conflict`

## Result

- Higher-priority rules win deterministically.
- Equal-priority and manual-clock/timezone/device gaps remain manual-required instead of last-write-wins.
- Rollback context is preserved in conflict reporting.

