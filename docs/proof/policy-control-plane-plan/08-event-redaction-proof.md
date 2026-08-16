# WP08 Event Redaction Proof

## Proves

- `policy-event.audit-recorded`
- `policy-event.no-sensitive-log-payload`

## Evidence

- `packages/policy-domain/tests/unit/policy-event.test.ts`
  - `policyEventAggregateKey: keeps delivery keys stable and redacted summaries free of private identifiers`
- `crates/policy-control-core/tests/unit/policy_event.rs`
  - `policy_event_redacted_summary_omits_private_identifiers`
- `crates/policy-control-core/tests/unit/policy_delivery.rs`
  - `queued_delivery_redacts_raw_policy_source_payload_from_structured_and_debug_output`

## Result

- Event and delivery debug/summary shapes do not expose raw child identifiers or raw policy payload content.
- Audit/log output stays reference-based and redacted.

