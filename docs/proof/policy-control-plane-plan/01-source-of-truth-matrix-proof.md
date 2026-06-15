# WP01 Proof: Source Of Truth Matrix

Plan: `policy-control-plane-plan`
Workpack: `01-policy-source-of-truth`

Covered proof IDs:
- `policy-source.source-of-truth-matrix`
- `policy-source.portal-ui-not-truth`
- `policy-source.ai-preview-not-write`
- `policy-source.domain-cache-not-truth`
- `policy-source.audit-ref-required`

Claim:
- The registered `ParentPolicySourceDocument` is the authoritative parent policy source.
- Portal draft/preview, AI preview, compiled artifacts, and domain caches are non-authoritative inputs.
- Delivery, ack, and audit state remain separate artifacts and do not collapse back into source truth.

Evidence references:
- `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
  - records that `policy-control-core` source/compiler/conflict/preview/request/delivery seams compile through the real `policy_source` and `policy_authority` boundaries and that the crate-owned unit plus version-skew suites are green in this checkout.
- `cargo test -p ocentra-policy-control-core --test unit -- --test-threads=1`
  - passed 85 tests.
- `cargo test -p ocentra-policy-control-core --test version_skew -- --test-threads=1`
  - failed 2 assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585.
- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `parent_can_register_versioned_policy_source_of_truth`
  - `ai_preview_and_domain_cache_cannot_become_source_truth`
  - `resolved_policy_states_require_audit_refs`
  - `active_status_requires_acknowledged_delivery_for_every_target`
  - `source_compile_helper_rejects_draft_and_preview_documents`
- `crates/policy-control-core/tests/unit/policy_preview.rs`
  - preview remains a pre-save analysis layer and blocks save on conflicts or invalid states.
- `crates/policy-control-core/tests/unit/policy_authority.rs`
  - enforcement and conflict resolution remain separate from source registration authority.

Open evidence gaps:
- The two failing `version_skew` assertions in `tests/version-skew/policy_compiler.rs` at lines 528 and 585 remain open blockers for closing the WP01 proof set.
