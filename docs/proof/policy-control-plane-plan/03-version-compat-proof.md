# WP03 Version Compatibility Proof

Run id: `019ed32a-fdd2-74b0-bb81-6e152680ac97/2026-06-17T19:43:50Z`

Correlation: `policy-control-plane-plan / WP03 / policy-wp03-compiler-proof-bundle / version-compat`

## Validation source

- `cargo test -p ocentra-policy-control-core`
- `npm run test --workspace @ocentra-parent/policy-domain -- tests/unit/policy-compiler.test.ts tests/unit/policy-schedule-boundaries.test.ts tests/unit/policy-event.test.ts`

## Source-of-truth compatibility proof

| Compatibility rule | Owner proof |
| --- | --- |
| Equal schema and policy versions are compatible | `source_compatibility_reports_equal_schema_and_policy_versions_as_compatible` |
| Older schema/policy versions require migration | `source_compatibility_requires_migration_for_older_schema_and_policy_versions` |
| Newer schema versions are unsupported while equal policy versions stay compatible | `source_compatibility_marks_newer_schema_as_unsupported_and_policy_version_as_compatible` |
| Zero schema versions are rejected | `policy_source_serde_rejects_zero_schema_version` |
| Stale registration versions are rejected | `stale_policy_version_is_rejected_during_registration` |
| Supersede requires a newer replacement version | `supersede_rejects_non_newer_replacement_versions` |
| Rollback requires an older restored version | `rollback_rejects_non_older_restored_versions` |

## Compiler compatibility proof

| Compiler rule | Owner proof |
| --- | --- |
| Compiler schema version is nonzero | `policy_compiler_schema_version_is_nonzero` |
| Consumer version mismatch is rejected before compile | `compiler_rejects_consumer_version_mismatch` |
| Draft and preview source documents cannot compile | `compiler_rejects_draft_and_preview_source_documents_before_release_candidate_stage` |
| Domain-cache source documents cannot compile as canonical truth | `compiler_rejects_domain_cache_source_documents_as_non_canonical_source_truth` |

## Event contract compatibility proof

- `policy_event_schema_version_is_locked_to_one`
- `policy_event_deserialization_rejects_zero_schema_version`

These event checks are owner-local evidence that the compiler-adjacent event surface does not silently drift schema versions while WP03 remains owner-scoped.

## Honest conclusion

Current owner proof shows that source documents, compiled artifacts, and compiler-adjacent events reject version drift, stale registration, and unsupported schema versions before any forbidden shared seam is needed.
