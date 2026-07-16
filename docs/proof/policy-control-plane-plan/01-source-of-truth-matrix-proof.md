# WP01 Source Of Truth Matrix Proof

## Proves

- `policy-source.source-of-truth-matrix`
- `policy-source.domain-cache-not-truth`
- `policy-source.portal-ui-not-truth`
- `policy-source.audit-ref-required`
- `policy-source.export-delete-custody`
- `policy-source.policy-version-supersede`

## Owner surfaces

- `packages/policy-domain/src/policy.ts`
- `packages/policy-domain/src/policy-compiler.ts`
- `crates/policy-control-core/src/policy_source.rs`
- `crates/policy-control-core/src/policy_compiler.rs`
- `crates/policy-control-core/src/policy_delivery.rs`

## Evidence

- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `parent_can_register_versioned_policy_source_of_truth`
  - `ai_preview_and_domain_cache_cannot_become_source_truth`
  - `resolved_policy_states_require_audit_refs`
  - `active_status_requires_acknowledged_delivery_for_every_target`
  - `superseded_status_requires_newer_replacement_version_and_new_audit_ref`
  - `rolled_back_status_requires_prior_version_reference_and_new_audit_ref`
  - `source_compile_helper_rejects_draft_and_preview_documents`
- `crates/policy-control-core/tests/unit/policy_delivery.rs`
  - `queued_delivery_preserves_source_lifecycle_metadata_separately_from_delivery_state`
- `crates/policy-control-core/tests/version-skew/policy_delivery.rs`
  - `queued_delivery_serialization_preserves_source_metadata_fields`

## Contract result

- Parent policy source documents remain distinct from preview, compiled artifacts, delivery records, and audit/event artifacts.
- Draft and preview source documents are rejected as compile inputs, so portal preview is not source truth.
- Domain caches and compiled outputs cannot replace the canonical source document.
- Audit references are required on resolved lifecycle states.
- Source retention and custody boundaries are carried explicitly through `PolicyRetentionMetadata` and `evidence_custody_requirements`.
- Supersede and rollback lifecycle references are typed first-class fields, not implicit side effects.

## Does not prove

- Portal authoring UX completion.
- Export/delete runtime execution.
- Device delivery, enforcement, or assistant approval UX.

