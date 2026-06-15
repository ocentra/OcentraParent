# WP01 Proof: Duplicate Truth Rejection

Plan: `policy-control-plane-plan`
Workpack: `01-policy-source-of-truth`

Covered proof IDs:
- `policy-source.duplicate-truth-rejected`
- `policy-source.domain-cache-not-truth`

Claim:
- Competing source documents for the same household and policy version are rejected.
- There is no second authoritative parent policy truth for the same version.
- Non-authoritative surfaces such as domain cache are not allowed to stand in as replacement truth.

Evidence references:
- `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
  - records that policy truth is centralized in the policy control plane contract and not delegated to domain caches or ad hoc local copies.
- `cargo test -p ocentra-policy-control-core --test unit -- --test-threads=1`
  - passed 85 tests.
- `cargo test -p ocentra-policy-control-core --test version_skew -- --test-threads=1`
  - failed 2 assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585.
- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `duplicate_household_truth_for_same_version_is_rejected`
  - `ai_preview_and_domain_cache_cannot_become_source_truth`
  - `stale_policy_version_is_rejected_during_registration`
- `crates/policy-control-core/tests/version-skew/policy_source.rs`
  - `stale_policy_version_is_rejected_during_registration`

Open evidence gaps:
- The two failing `version_skew` assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585 remain open blockers for closing the WP01 proof set.
