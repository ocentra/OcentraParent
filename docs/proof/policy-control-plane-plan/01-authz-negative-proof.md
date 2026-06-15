# WP01 Proof: Authorization Negative Cases

Plan: `policy-control-plane-plan`
Workpack: `01-policy-source-of-truth`

Covered proof IDs:
- `policy-source.authz-role-matrix`
- `policy-source.wrong-household-rejected`
- `policy-source.revoked-actor-rejected`

Claim:
- Only the correct parent/coparent authority may register source truth.
- Wrong-household, mismatched-actor, mismatched-role, child/support, and revoked-actor writes are rejected.
- AI authority claims remain blocked and do not bypass manual review.

Evidence references:
- `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
  - records the current source lifecycle authority boundary and the green unit/version-skew coverage in this checkout.
- `cargo test -p ocentra-policy-control-core --test unit -- --test-threads=1`
  - passed 85 tests.
- `cargo test -p ocentra-policy-control-core --test version_skew -- --test-threads=1`
  - failed 2 assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585.
- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `coparent_can_write_source_truth_but_child_and_support_cannot`
  - `wrong_household_actor_authority_cannot_register_source_truth`
  - `mismatched_actor_authority_cannot_register_source_truth`
  - `mismatched_role_authority_cannot_register_source_truth`
  - `revoked_actor_authority_cannot_register_source_truth`
- `crates/policy-control-core/tests/unit/policy_authority.rs`
  - `ai_result_claiming_authority_blocks_enforcement_and_requires_review`
  - `missing_evidence_reference_forces_manual_review_conflict_resolution`

Open evidence gaps:
- The two failing `version_skew` assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585 remain open blockers for closing the WP01 proof set.
