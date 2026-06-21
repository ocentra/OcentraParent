# WP04 Delivery Audit and Rollback Proof

Run id: `019ed32a-fdd2-74b0-bb81-6e152680ac97/2026-06-17T20:17:50Z`

Correlation: `policy-control-plane-plan / WP04 / policy-wp04-delivery-ack-audit / audit-rollback`

## Validation source

- `cargo test -p ocentra-policy-control-core policy_delivery`
- `cargo test -p ocentra-policy-control-core policy_source`
- `npm run test --workspace @ocentra-parent/policy-domain -- tests/unit/policy-compiler.test.ts tests/unit/policy-event.test.ts`

## Proof mapping

| WP04 proof id | Current owner evidence |
| --- | --- |
| `policy-delivery.rollback-audited` | `rejected_and_rolled_back_transitions_require_reason_and_reference_context`, `rolled_back_status_requires_prior_version_reference_and_new_audit_ref`, and `queued_delivery_preserves_source_lifecycle_metadata_separately_from_delivery_state` |
| `policy-delivery.redacted-log-proof` | `queued_delivery_redacts_raw_policy_source_payload_from_structured_and_debug_output` |
| `policy-delivery.ack-required` | `active_status_requires_acknowledged_delivery_for_every_target` and `resolved_policy_states_require_audit_refs` |

## Audit and rollback linkage that is actually proven

- `queued_delivery_preserves_source_lifecycle_metadata_separately_from_delivery_state` proves delivery records preserve source audit refs, source supersede metadata, and source rollback refs without collapsing them into the mutable delivery status.
- `resolved_policy_states_require_audit_refs` proves source-truth states that claim resolution cannot exist without audit references.
- `superseded_status_requires_newer_replacement_version_and_new_audit_ref` proves supersede requires both a newer replacement version and a new audit reference.
- `rolled_back_status_requires_prior_version_reference_and_new_audit_ref` proves rollback requires a prior restored version plus a fresh audit reference.
- `rejected_and_rolled_back_transitions_require_reason_and_reference_context` proves rolled-back delivery state cannot be emitted without explicit reason code and rollback reference state.

## Redaction / no-claim boundary

- The delivery record JSON/debug proof explicitly omits raw child profile arrays, device arrays, rules, schedules, and retention payloads from serialized output.
- The delivery side keeps target identity and source audit lineage explicit without claiming UI rendering or cross-process log transport.
- `packages/schema-domain/src/policy-compiler.ts` and `packages/schema-domain/src/policy-event.ts` keep audit references, rollback refs, and delivery identifiers explicit on the TS contract boundary.

## Honest boundary

This proof covers audit-linkage, rollback-linkage, and redaction on policy-owned delivery surfaces only. It does not claim portal audit presentation or external log pipeline behavior.
