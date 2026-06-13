# Local API App

Typed local query/control API contract package for routes owned by the Rust agent service.

The current localhost dev API is hosted by the Rust agent service. This package does not implement a second server; it exposes the UI-facing route and command manifest that points back to `crates/agent-service`.

## Ownership

This workspace owns TypeScript contract metadata for local agent-service routes,
WebSocket command transport, and route ownership checks.

## Must Not Own

- Current local WebSocket or HTTP service behavior.
- Child-device evidence custody.
- Policy evaluation or enforcement.
- A second source of truth for endpoint paths or protocol names.

## Connected Docs

- [Endpoint/domain expectations](../../docs/expectations/contracts.md)
- [Data custody expectations](../../docs/expectations/data-custody.md)
- [Product capability checklist](../../docs/product-capability-checklist.md)
