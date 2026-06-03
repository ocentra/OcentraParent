# 16 - Output Parser And Schema Validator

## Target State

No model output can reach policy until it decodes into a schema-valid AI result.

## Where We Are

Contracts exist. The parser layer must make invalid JSON, invalid enum values,
missing refs, low confidence, and unsafe commands explicit degraded states.

## Checklist

- [ ] Define parser input/output.
- [ ] Reject invalid JSON.
- [ ] Reject missing evidence refs.
- [ ] Reject missing parent-rule refs.
- [ ] Reject direct enforcement fields.
- [ ] Degrade low confidence or contradiction.

## Proof

- Parser unit tests.
- Invalid-output integration tests.
- Policy receives only valid result tests.
