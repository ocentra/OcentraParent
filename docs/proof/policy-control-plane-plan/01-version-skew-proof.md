# WP01 Proof: Version Skew Handling

Plan: `policy-control-plane-plan`
Workpack: `01-policy-source-of-truth`

Covered proof IDs:
- `policy-source.version-skew`
- `policy-source.policy-version-supersede`
- `policy-source.migration-boundary`

Claim:
- Older policy versions are treated as migration-required, not silently accepted as canonical.
- Future schema versions are rejected as unsupported.
- Supersede and rollback paths require the correct version direction and a fresh audit reference.

Evidence references:
- `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
  - records the crate-owned source/compiler/conflict/preview/request/delivery seams as green and notes that the source lifecycle now owns the authority, supersede, and rollback boundaries.
- `cargo test -p ocentra-policy-control-core --test unit -- --test-threads=1`
  - passed 85 tests.
- `cargo test -p ocentra-policy-control-core --test version_skew -- --test-threads=1`
  - failed 2 assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585.
- `crates/policy-control-core/tests/version-skew/policy_source.rs`
  - `stale_policy_version_is_rejected_during_registration`
  - `supersede_rejects_non_newer_replacement_versions`
  - `rollback_rejects_non_older_restored_versions`
- `crates/policy-control-core/tests/version-skew/policy_source_migration.rs`
  - `older_schema_version_is_marked_for_migration`
  - `future_schema_version_is_rejected_as_unsupported`
  - `stale_policy_version_is_marked_for_migration`
- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `superseded_status_requires_newer_replacement_version_and_new_audit_ref`
  - `rolled_back_status_requires_prior_version_reference_and_new_audit_ref`

Open evidence gaps:
- The two failing `version_skew` assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585 remain open blockers for closing the WP01 proof set.
