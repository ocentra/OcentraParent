# Local API App

Reserved local query/control API package placeholder.

The current localhost dev API is hosted by the Rust agent service. Keep this package empty until there is a TypeScript-specific boundary that should not live in the Rust service.

## Ownership

This workspace should remain empty until there is a concrete TypeScript runtime
boundary that cannot belong to `crates/agent-service` or a platform shell.

Possible future use:

- parent-owned report compile helper that runs outside Ocentra-hosted services;
- local-only bridge that cannot be expressed cleanly in Rust;
- typed API harness for a future packaging target.

## Must Not Own

- Current local WebSocket or HTTP service behavior.
- Child-device evidence custody.
- Policy evaluation or enforcement.
- A second source of truth for endpoint paths or protocol names.

## Connected Docs

- [Endpoint/domain expectations](../../docs/expectations/contracts.md)
- [Data custody expectations](../../docs/expectations/data-custody.md)
- [Product capability checklist](../../docs/product-capability-checklist.md)
