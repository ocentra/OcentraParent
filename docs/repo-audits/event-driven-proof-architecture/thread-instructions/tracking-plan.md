# tracking-plan Event Architecture Instruction

## Owns

- tracking configuration contracts;
- tracking observation/evidence events;
- expected-place/geofence/location-state decisions;
- tracking retention/read-model contracts;
- tracking notification request contracts, not provider delivery ownership.

## Must not own

- AI routing or model execution;
- policy enforcement actions;
- LAN transport;
- portal final UX truth;
- account/session/device-trust authority;
- notification provider runtime unless explicitly assigned.

## Required chain

```text
portal/dev command or policy owner emits tracking config command
-> tracking owner validates and records config event
-> tracking runtime observes location/geofence state
-> tracking emits tracking evidence / notification-request event
-> orchestrator or service routes AI/policy/notification work through typed contracts
-> read model/journal records result
-> portal renders service-backed state
```

## Logging/proof

Every tracking slice must log: config accepted/rejected, evidence observed, event emitted, downstream request created, provider/runtime result consumed, read model updated. Use one correlation id across the chain.

## Tests

- `tracking-domain`: unit/contract only for contracts and pure decisions.
- Rust tracking/service: crate `tests/` for runtime, replay, read-model, integration.
- Cross-domain tracking -> AI/policy/action proof belongs to app/service/proof-runner or portal dev-command e2e, not `tracking-domain`.

## First architecture slice

Run `S0` and `S1`: fix the schema/import crash, migrate WP33 wrappers from `parent-domain` to `tracking-domain`, then regenerate WP33 notification/provider artifacts. Do not begin WP34-WP39 until event/log proof boundaries are named.
