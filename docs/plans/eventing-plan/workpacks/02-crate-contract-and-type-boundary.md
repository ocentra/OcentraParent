# WP02 Crate Contract And Type Boundary

Scope: define the typed public contract for event kinds, IDs, envelopes, event sources, runtime roles, custody, and serialization boundaries.

Source rows: `05-implementation-workpacks.md` rows 6-10.

Read next:

- `../02-crate-api-and-code-shape.md`
- `../04-tests-proof-and-validation.md`
- `../TEST_PROOF_EXPECTATIONS.md`
- `../implementation-checklist.md` only for the matching core contract rows

Expected outcome:

- Event type grammar, duplicate detection, namespace rules, and reserved families are explicit.
- Event, correlation, subscriber, request, aggregate, idempotency, source, path, name, handler, and runtime IDs are strongly typed.
- Live typed envelope and stored serialized envelope have different responsibilities.
- Event source, runtime role, custody, and target-handler semantics are present and versioned.
- Serialization compatibility and schema/version skew behavior are specified and proved.

Expected tests/proof:

- `eventing.event-type.grammar.unit`
- `eventing.duplicate-registry.negative`
- `eventing.strong-id.validation.property`
- `eventing.live-vs-stored-envelope.serialization`
- `eventing.version-skew.roundtrip`
- Proof artifact includes fixture names, rejected malformed inputs, and compatibility notes.

Failure conditions:

- No naked domain strings for event types, IDs, role, custody, or source values.
- No proof based only on compile success.
- No product-specific payload contract belongs here unless it is routed through WP07.
