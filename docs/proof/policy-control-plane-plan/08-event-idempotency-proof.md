# WP08 Event Idempotency Proof

## Proves

- `policy-event.aggregate-key-stable`
- `policy-event.idempotency-key-stable`

## Evidence

- `packages/policy-domain/tests/unit/policy-event.test.ts`
  - `policyEventAggregateKey: keeps delivery keys stable and redacted summaries free of private identifiers`
- `crates/policy-control-core/tests/unit/policy_event.rs`
  - `policy_event_keys_and_contract_are_stable_for_delivery_events`
- `crates/policy-control-core/tests/version-skew/policy_event.rs`
  - `policy_event_schema_version_is_locked_to_one`
  - `policy_event_deserialization_rejects_zero_schema_version`

## Result

- Aggregate and idempotency keys are stable across owner implementations.
- Schema versioning is locked and rejects zero/invalid payloads.

