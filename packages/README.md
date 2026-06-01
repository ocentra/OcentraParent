# Packages

The `packages/` workspace owns shared TypeScript contracts. Runtime apps and
Rust crates consume these packages instead of inventing local strings, routes,
event names, field names, or policy shapes.

```mermaid
flowchart LR
  Schema["schema-domain\nEffect helpers"]
  Endpoint["endpoint-domain\npaths and headers"]
  Text["text-domain\ncopy tokens"]
  Activity["activity-domain\nevidence contracts"]
  Parent["parent-domain\nfamily policy product contracts"]
  Logging["logging-domain\nlog contracts"]
  Portal["portal-domain\nroutes and DOM contracts"]
  Protocol["agent-protocol-domain\ncommands and events"]
  Apps["apps/*"]
  Rust["crates/agent-protocol\ncrates/agent-service"]

  Schema --> Endpoint
  Schema --> Text
  Schema --> Activity
  Schema --> Parent
  Schema --> Logging
  Activity --> Parent
  Parent --> Portal
  Parent --> Protocol
  Activity --> Protocol
  Logging --> Protocol
  Portal --> Apps
  Protocol --> Apps
  Protocol --> Rust
```

## Package Ownership

| Package                 | Owns                                                                                                                                    |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `schema-domain`         | Effect Schema helpers and branded decode helpers.                                                                                       |
| `endpoint-domain`       | Endpoint paths, HTTP/header constants, version constants, and endpoint brands.                                                          |
| `text-domain`           | Schema-backed display text tokens and shared copy values.                                                                               |
| `activity-domain`       | Capture, evidence, journal, query, browser, app/game, network, screen, and activity report contracts.                                   |
| `parent-domain`         | Family, parent, child, policy, enforcement, AI, LAN, mobile, browser/app/game/network/screen control, and product capability contracts. |
| `logging-domain`        | Structured operational logging/redaction contracts.                                                                                     |
| `portal-domain`         | Portal route ids, DOM ids, nav/section contracts, and dev command descriptors.                                                          |
| `agent-protocol-domain` | WebSocket command/event envelopes and shared agent protocol contracts.                                                                  |

## Rules

- Use Effect Schema.
- Do not add Zod.
- Do not create manual string brands.
- Do not place runtime literals in app or crate code when a package should own
  them.
- Add tests with every contract.
- Mirror Rust-crossing contracts into `crates/agent-protocol`.

## Connected Docs

- [Contract expectations](../docs/expectations/contracts.md)
- [Policy expectations](../docs/expectations/policy.md)
- [Product constitution](../docs/product-constitution.md)
- [Product capability checklist](../docs/product-capability-checklist.md)
