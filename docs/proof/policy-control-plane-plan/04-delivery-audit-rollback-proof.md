# WP04 Delivery Audit and Rollback Proof

Run id: `019ed32a-fdd2-74b0-bb81-6e152680ac97/2026-06-17T20:17:50Z`

Receipt/provenance refresh: `019f773f-d986-7db2-8a0d-2fba41e42bd2/2026-07-18-policy-receipt-enforcement`

Diagnostic-redaction refresh: `policy-receipt-error-redaction/2026-07-18`

Record-boundary refresh: `policy-wp04-record-boundary/2026-07-18`

Untrusted-Applied refresh: `policy-wp04-record-boundary/2026-07-18-forged-receipt-rejection`

Correlation: `policy-control-plane-plan / WP04 / policy-wp04-delivery-ack-audit / audit-rollback`

## Validation source

- `cargo test -p ocentra-policy-control-core --test unit --test version-skew`
- `cargo test -p ocentra-child-policy-core --test replay_policy_control_delivery_handoff`
- `cargo test -p ocentra-parent-runtime-core --test unit policy_control_`
- `cargo test -p ocentra-child-notification-core --test observability_policy_control_notification`
- `cargo clippy -p ocentra-policy-control-core -p ocentra-child-policy-core -p ocentra-parent-runtime-core -p ocentra-child-notification-core --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- focused `npm run lint:architecture -- --files ...` and `npm run lint:enforcer:source-shape -- --files ...` over every changed Rust file

The former `packages/policy-domain` TypeScript workspace is not present at this
branch head. Current delivery, receipt, rollback, and version-skew authority is
the Rust `ocentra-policy-control-core` crate; no retired-package command is
counted as validation.

## Proof mapping

| WP04 proof id | Current owner evidence |
| --- | --- |
| `policy-delivery.rollback-audited` | `rejected_and_rolled_back_transitions_require_reason_and_reference_context`, `rolled_back_status_requires_prior_version_reference_and_new_audit_ref`, `queued_delivery_preserves_superseded_source_lifecycle_metadata`, `queued_delivery_preserves_rolled_back_source_lifecycle_metadata`, and the `rolled_back_execution_receipt_*` matrix |
| `policy-delivery.redacted-log-proof` | `queued_delivery_redacts_raw_policy_source_payload_from_structured_and_debug_output`, `execution_receipt_debug_redacts_sensitive_provenance`, `formatted_receipt_validation_errors_redact_sensitive_identifiers`, `operational_debug_redacts_identifiers_while_wire_contract_preserves_them`, and `transition_validation_diagnostics_redact_raw_sentinels` |
| `policy-delivery.ack-required` | `active_status_requires_acknowledged_delivery_for_every_target`, `resolved_policy_states_require_audit_refs`, `acknowledged_delivery_requires_an_explicit_execution_receipt`, `applied_delivery_requires_an_explicit_execution_receipt`, `generic_acknowledged_hydration_rejects_matching_public_receipt`, `generic_applied_hydration_rejects_fully_matching_forged_receipt`, `generic_rolled_back_hydration_rejects_matching_public_receipt`, `bare_transition_apis_reject_every_receipt_required_state`, `delivery_handoff_surfaces_receipt_required_states_as_manual_required`, `parent_runtime_policy_control_flow_rejects_receipt_required_child_transitions`, and `applied_state_without_receipt_evidence_fails_closed` |
| `policy-delivery.receipt-provenance` | `ack_applied_and_rolled_back_receipts_require_explicit_adapter_provenance`, `execution_receipt_validation_rejects_provenance_mismatches`, `execution_receipt_validation_rejects_source_document_identity_mismatch`, `execution_receipt_validation_rejects_reason_code_identity_mismatch`, `fully_matching_public_receipt_remains_untrusted_for_applied_hydration`, `generic_acknowledged_hydration_rejects_matching_public_receipt`, `generic_applied_hydration_rejects_fully_matching_forged_receipt`, `generic_rolled_back_hydration_rejects_matching_public_receipt`, `schema_v1_receiptless_applied_is_not_legacy_compatible`, `schema_v1_receiptless_acknowledged_hydrates_as_unverified_manual_required`, and `schema_v1_receiptless_rolled_back_preserves_history_as_unverified` |

## Audit and rollback linkage that is actually proven

- `queued_delivery_preserves_superseded_source_lifecycle_metadata` and `queued_delivery_preserves_rolled_back_source_lifecycle_metadata` prove delivery records preserve source audit refs, source supersede metadata, and source rollback refs without collapsing them into mutable delivery status.
- `resolved_policy_states_require_audit_refs` proves source-truth states that claim resolution cannot exist without audit references.
- `superseded_status_requires_newer_replacement_version_and_new_audit_ref` proves supersede requires both a newer replacement version and a new audit reference.
- `rolled_back_status_requires_prior_version_reference_and_new_audit_ref` proves rollback requires a prior restored version plus a fresh audit reference.
- `rejected_and_rolled_back_transitions_require_reason_and_reference_context` proves rolled-back delivery state cannot be emitted without explicit reason code and rollback reference state.
- The receipt matrix proves acknowledged, applied, and rolled-back evidence is tied to delivery, household, source document, target, attempt, sequence, audit, reason, and rollback provenance. Missing, stale, or mismatched evidence fails closed, but structural matching is not authentication.
- Generic record deserialization rejects every schema-v2 acknowledged, applied, and rolled-back payload even when all caller-supplied record and receipt fields match.
- No production execution-authority entry is exposed. WP04 also defines no authenticated persistence capability for receipt-required states. Trusted initial execution and persisted acknowledged, applied, or rolled-back rehydration are therefore unsupported and not proven.
- Public execution-receipt serde remains a typed evidence/transport shape, not an authenticity credential, and cannot authorize `PolicyDeliveryRecord` hydration.
- The legacy-compatible public transition API and the explicit transition-only API both reject acknowledged, applied, and rolled-back advancement without trusted authority. Child-policy converts acknowledged and applied requests to typed `ManualRequired`; parent-runtime rejects receipt-required promotion; neither seam fabricates receipts.
- Schema-v1 receiptless acknowledged and rolled-back records hydrate only through explicit `LegacySchemaV1Unverified` compatibility, preserve historical audit/rollback/source facts, and surface parent-visible `ManualRequired`; schema-v2 receipt-required records remain strict.

## Redaction / no-claim boundary

- Structured delivery serialization intentionally retains typed delivery, household, source-document, target, attempt, and audit identity required by the wire contract. That serialization is neither a redacted log surface nor proof of receipt authenticity or persisted-`Applied` hydration; it still omits raw policy rules, schedules, retention payloads, and child/device arrays.
- Delivery ID, attempt ID, target, record, transition, apply-outcome, and execution-receipt `Debug` output redact delivery, household, source-document, child, device, domain, attempt, audit-reference, and reason identifiers while retaining non-sensitive state, sequence, counts, and presence flags for diagnostics.
- Receipt identity and hydration `EventingError` values retain stable field/category and expected-versus-reported ownership or sequence state, but do not interpolate delivery, household, source-document, child-profile, device, attempt, audit-reference, or reason-code identifiers. `formatted_receipt_validation_errors_redact_sensitive_identifiers` formats both `Debug` and `Display` across identity, audit, reason, stale, and forged-hydration failures and rejects every sensitive sentinel.
- The delivery side keeps target identity and source audit lineage explicit without claiming UI rendering or cross-process log transport.
- `crates/policy-control-core/src/policy_delivery.rs` and its focused unit/version-skew suites own the current delivery, receipt, rollback, and redaction boundary.

## Honest boundary

This proof covers audit-linkage, rollback-linkage, receipt evidence validation, legacy compatibility, and redaction on policy-owned delivery surfaces only. It does not claim trusted adapter authority, an inspectable execution trace, real enforcement side effects, authenticated persistence, persisted receipt-required rehydration, portal audit presentation, or external log pipeline behavior.
