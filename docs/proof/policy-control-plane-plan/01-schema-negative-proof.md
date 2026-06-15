# WP01 Proof: Schema Negative Cases

Plan: `policy-control-plane-plan`
Workpack: `01-policy-source-of-truth`

Covered proof IDs:
- `policy-source.schema-negative`
- `policy-source.migration-boundary`

Claim:
- Malformed or incomplete source payloads are rejected before they can become policy truth.
- Required schema blocks, schedule fields, and time-budget fields must be present and valid.
- Invalid schema payloads do not advance into source registration, preview, or compilation.

Evidence references:
- `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
  - records the current policy-control-core source/compiler/conflict/preview/request/delivery seams as green in this checkout.
- `cargo test -p ocentra-policy-control-core --test unit -- --test-threads=1`
  - passed 85 tests.
- `cargo test -p ocentra-policy-control-core --test version_skew -- --test-threads=1`
  - failed 2 assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585.
- `crates/policy-control-core/tests/version-skew/policy_source.rs`
  - `policy_source_serde_rejects_zero_schema_version`
  - `policy_source_serde_rejects_schedule_payload_without_time_budget`
- `crates/policy-control-core/tests/version-skew/policy_source_migration.rs`
  - `compatibility_input_rejects_schedule_payload_without_time_budget`
- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `weekly_reset_requires_day`
  - `daily_and_monthly_resets_reject_unexpected_day`
  - `discard_unused_carryover_rejects_max_minutes`
  - `cap_carryover_requires_positive_max_minutes`
  - `effective_until_must_be_after_effective_from`
  - `bonus_expiry_minutes_must_be_non_zero`

Open evidence gaps:
- The two failing `version_skew` assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585 remain open blockers for closing the WP01 proof set.
