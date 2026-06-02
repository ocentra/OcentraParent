# 01 Contract Boundary And Effect Schemas

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current V0.9 work has typed LAN pairing, route, registry, selected-device, and
proof-state contracts. The broader production discovery model still needs one
contract boundary for interface evidence, scan evidence, merge decisions,
classification, child-agent hello, heartbeat, assignment, ignore, rename,
revocation, UI state, and proof summaries.

## Where We Want To Be

Every LAN discovery value crossing TypeScript, Rust, service, or portal
boundaries is represented by shared domain contracts first. Runtime code accepts
external values as `unknown`, decodes them with Effect Schema, and uses branded
types or constants instead of raw strings.

## Requirement Checklist

- [ ] Add or extend TypeScript contracts in the correct domain package before
      runtime/service consumption.
- [ ] Use Effect Schema brands and decode helpers for ids, source names, event
      types, route ids, device ids, UI states, and proof ids.
- [ ] Keep API paths, command names, event names, fields, and display tokens out
      of app/runtime source.
- [ ] Define versioned contract families for discovery evidence, device record,
      merge result, child hello, heartbeat, event stream, UI state, and proof
      summary.
- [ ] Add invalid payload coverage for every schema.

## Acceptance And Proof

- Contract tests accept valid payloads and reject malformed, missing-field,
  wrong-version, wrong-family, and future enum payloads safely.
- Rust protocol parity exists before `crates/agent-service` emits or accepts
  the shape.
- `npm run lint:schema-boundaries` passes.

## Parallel Ownership Notes

This workpack blocks most implementation work. Workers may split by contract
family, but they must not create competing id brands or duplicate event names.
