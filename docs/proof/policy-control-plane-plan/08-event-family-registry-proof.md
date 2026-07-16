# WP08 Event Family Registry Proof

## Proves

- `policy-event.event-family-registry`
- `policy-event.causation-correlation-present`

## Evidence

- `packages/policy-domain/tests/unit/policy-event.test.ts`
  - `policyEventFamilyRegistry: keeps the registry and variants aligned with the explicit policy event kinds`
  - `parsePolicyEventEnvelope: keeps causation, correlation, and deterministic metadata aligned`
- `crates/policy-control-core/tests/unit/policy_event.rs`
  - `policy_event_family_registry_lists_all_event_types`
  - `policy_event_keys_and_contract_are_stable_for_delivery_events`

## Result

- The event registry is explicit, typed, and aligned between TS and Rust owner surfaces.
- Causation and correlation metadata are part of the event contract, not optional decoration.

