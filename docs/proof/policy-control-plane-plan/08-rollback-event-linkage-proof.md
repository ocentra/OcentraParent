# WP08 Rollback Event Linkage Proof

## Proves

- `policy-event.rollback-linked`
- `policy-event.dead-letter-manual-required`

## Evidence

- `packages/policy-domain/tests/unit/policy-event.test.ts`
  - `PolicyEventSchema: keeps rollback linkage and dead-letter/manual-required visibility explicit`
- `crates/policy-control-core/tests/unit/policy_event.rs`
  - `policy_event_manual_required_and_dead_letter_payloads_remain_explicit`
- `crates/policy-control-core/tests/unit/policy_source.rs`
  - `rolled_back_status_requires_prior_version_reference_and_new_audit_ref`

## Result

- Rollback events carry typed linkage to prior/restored versions.
- Dead-letter and manual-required visibility is explicit in the event model.

