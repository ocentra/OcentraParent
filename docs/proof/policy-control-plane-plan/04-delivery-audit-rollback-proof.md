# WP04 Delivery Audit and Rollback Proof

Run id: `019ed32a-fdd2-74b0-bb81-6e152680ac97/2026-06-17T20:17:50Z`

Receipt/provenance refresh: `019f773f-d986-7db2-8a0d-2fba41e42bd2/2026-07-18-policy-receipt-enforcement`

Correlation: `policy-control-plane-plan / WP04 / policy-wp04-delivery-ack-audit / audit-rollback`

## Validation source

- `cargo test -p ocentra-policy-control-core --test unit --test version-skew`
- `cargo test -p ocentra-child-policy-core --test replay_policy_control_delivery_handoff`
- `cargo test -p ocentra-parent-runtime-core --test unit policy_control_update_flow`
- `cargo clippy -p ocentra-policy-control-core -p ocentra-child-policy-core -p ocentra-parent-runtime-core --all-targets -- -D warnings`
- `cargo fmt -p ocentra-policy-control-core -- --check`
- `npm run lint:architecture -- --files crates/policy-control-core`

The former `packages/policy-domain` TypeScript workspace is not present at this
branch head. Current delivery, receipt, rollback, and version-skew authority is
the Rust `ocentra-policy-control-core` crate; no retired-package command is
counted as validation.

## Proof mapping

| WP04 proof id | Current owner evidence |
| --- | --- |
| `policy-delivery.rollback-audited` | `rejected_and_rolled_back_transitions_require_reason_and_reference_context`, `rolled_back_status_requires_prior_version_reference_and_new_audit_ref`, `queued_delivery_preserves_superseded_source_lifecycle_metadata`, `queued_delivery_preserves_rolled_back_source_lifecycle_metadata`, and the `rolled_back_execution_receipt_*` matrix |
| `policy-delivery.redacted-log-proof` | `queued_delivery_redacts_raw_policy_source_payload_from_structured_and_debug_output` and `execution_receipt_and_adapter_debug_redact_sensitive_provenance` |
| `policy-delivery.ack-required` | `active_status_requires_acknowledged_delivery_for_every_target`, `resolved_policy_states_require_audit_refs`, `acknowledged_delivery_requires_an_explicit_execution_receipt`, `applied_delivery_requires_an_explicit_execution_receipt`, `bare_transition_apis_reject_every_receipt_required_state`, `delivery_handoff_rejects_receipt_required_states_without_receipts`, and `parent_runtime_policy_control_flow_rejects_receipt_required_child_transitions` |
| `policy-delivery.receipt-provenance` | `ack_applied_and_rolled_back_receipts_require_explicit_adapter_provenance`, `execution_receipt_validation_rejects_provenance_mismatches`, `execution_receipt_validation_rejects_source_document_identity_mismatch`, and `execution_receipt_validation_rejects_reason_code_identity_mismatch` |

## Audit and rollback linkage that is actually proven

- `queued_delivery_preserves_superseded_source_lifecycle_metadata` and `queued_delivery_preserves_rolled_back_source_lifecycle_metadata` prove delivery records preserve source audit refs, source supersede metadata, and source rollback refs without collapsing them into mutable delivery status.
- `resolved_policy_states_require_audit_refs` proves source-truth states that claim resolution cannot exist without audit references.
- `superseded_status_requires_newer_replacement_version_and_new_audit_ref` proves supersede requires both a newer replacement version and a new audit reference.
- `rolled_back_status_requires_prior_version_reference_and_new_audit_ref` proves rollback requires a prior restored version plus a fresh audit reference.
- `rejected_and_rolled_back_transitions_require_reason_and_reference_context` proves rolled-back delivery state cannot be emitted without explicit reason code and rollback reference state.
- The receipt matrix proves acknowledged, applied, and rolled-back adapter results require an execution receipt tied to delivery, household, source document, target, attempt, sequence, audit, reason, and rollback provenance; missing, stale, duplicate, or mismatched receipts fail closed.
- The legacy-compatible public transition API and the explicit transition-only API both reject acknowledged, applied, and rolled-back advancement without a receipt. Child-policy and parent-runtime transition-only seams call that fail-closed API and never fabricate receipts.

## Redaction / no-claim boundary

- The delivery record JSON/debug proof explicitly omits raw child profile arrays, device arrays, rules, schedules, and retention payloads from serialized output.
- Execution-receipt and adapter `Debug` output redact delivery, household, source-document, child, device, domain, attempt, audit-reference, and reason identifiers while retaining non-sensitive state, sequence, counts, and presence flags for diagnostics.
- The delivery side keeps target identity and source audit lineage explicit without claiming UI rendering or cross-process log transport.
- `crates/policy-control-core/src/policy_delivery.rs` and its focused unit/version-skew suites own the current delivery, receipt, rollback, and redaction boundary.

## Honest boundary

This proof covers audit-linkage, rollback-linkage, and redaction on policy-owned delivery surfaces only. It does not claim portal audit presentation or external log pipeline behavior.
