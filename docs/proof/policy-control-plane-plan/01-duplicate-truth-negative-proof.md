# WP01 Duplicate Truth Negative Proof

## Proves

- `policy-source.duplicate-truth-rejected`
- `policy-source.domain-cache-not-truth`

## Evidence

- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `duplicate_household_truth_for_same_version_is_rejected`
  - `ai_preview_and_domain_cache_cannot_become_source_truth`
- `crates/policy-control-core/tests/unit/policy_compiler.rs`
  - `compiler_rejects_domain_cache_source_documents_as_non_canonical_source_truth`

## Result

- The same household/version cannot register two independent source-of-truth documents.
- Domain-cache and compiled-artifact paths stay downstream from source truth.
- Negative cases stay negative; no fake-green fallback accepts duplicate truth.

