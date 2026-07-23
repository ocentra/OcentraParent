# WP01 Contract Boundary And Effect Schemas

- plan: app-plan
- workpack: 01-contract-boundary-and-effect-schemas
- owner: app-core and schema-domain edge decoder
- E2E tier: contract E2E
- source: crates/app-core/src/runtime_decision.rs and crates/app-core/src/runtime_ids.rs
- edge: packages/schema-domain/src/app-runtime-decision.ts
- golden: packages/schema-domain/tests/fixtures/app-runtime-decision-recorded-event.json
- scope: Rust app runtime-decision event serialization plus TypeScript edge decoding only

This slice makes inventory-only classification stay observation-only, exposes the
Rust event contract version/type, requires canonical opaque runtime identifier
prefixes with non-empty suffixes, and validates the same Rust-serialized golden
event at the schema-domain edge. The TypeScript test no longer treats Rust source
text inspection as serialization parity proof.

The edge decoder is transitional validation over Rust-owned values. This slice
does not add platform observation, event-bus publication, journal persistence,
service projection, policy execution, enforcement, or portal behavior.
