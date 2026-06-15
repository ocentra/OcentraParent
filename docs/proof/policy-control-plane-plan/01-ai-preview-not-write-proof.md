# WP01 Proof: AI Preview Is Not Write

Plan: `policy-control-plane-plan`
Workpack: `01-policy-source-of-truth`

Covered proof IDs:
- `policy-source.ai-preview-not-write`
- `policy-source.portal-ui-not-truth`

Claim:
- AI preview can draft or summarize policy intent, but it cannot replace the registered source document.
- Preview is a pre-save analysis step, not a mutation path.
- Preview remains blocked until acknowledged and continues to surface conflicts instead of writing truth.

Evidence references:
- `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
  - records that the plan owns the parent policy control plane contract while parent-facing UI remains non-authoritative.
- `cargo test -p ocentra-policy-control-core --test unit -- --test-threads=1`
  - passed 85 tests.
- `cargo test -p ocentra-policy-control-core --test version_skew -- --test-threads=1`
  - failed 2 assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585.
- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `ai_preview_and_domain_cache_cannot_become_source_truth`
  - `source_compile_helper_rejects_draft_and_preview_documents`
- `crates/policy-control-core/tests/unit/policy_preview.rs`
  - `preview_must_be_acknowledged_before_save_is_ready`
  - `overlapping_rules_are_reported_as_visible_conflicts`
  - `timezone_boundary_conflict_is_visible_before_save`
  - `unsupported_target_state_is_visible_and_blocks_save`
  - `manual_required_target_state_stays_visible_in_preview`
  - `offline_target_state_stays_visible_and_blocks_save`
  - `stale_target_state_stays_visible_in_preview`
- `crates/policy-control-core/tests/unit/policy_authority.rs`
  - `ai_result_claiming_authority_blocks_enforcement_and_requires_review`

Open evidence gaps:
- The two failing `version_skew` assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585 remain open blockers for closing the WP01 proof set.
