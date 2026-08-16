# WP01 Version Skew Proof

## Proves

- `policy-source.version-skew`
- `policy-source.migration-boundary`

## Evidence

- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `source_compatibility_reports_equal_schema_and_policy_versions_as_compatible`
  - `source_compatibility_requires_migration_for_older_schema_and_policy_versions`
  - `source_compatibility_marks_newer_schema_as_unsupported_and_policy_version_as_compatible`
- `crates/policy-control-core/tests/version-skew/policy_source_migration.rs`
  - `older_schema_version_is_marked_for_migration`
  - `future_schema_version_is_rejected_as_unsupported`
  - `stale_policy_version_is_marked_for_migration`

## Result

- Equal source/schema versions are accepted.
- Older supported inputs are marked `migration-required`, not silently accepted as current.
- Newer unsupported schema versions are rejected as unsupported.
- Version skew is explicit state, not hidden parser behavior.

