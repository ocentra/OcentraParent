# ocentra-parent-agent-protocol

Rust owns this crate's protocol contracts.

It defines command, event, read-model, and constant surfaces consumed by
`agent-service`, `agent-core`, and generated or thin TypeScript adapters.

This crate does not own runtime behavior, policy execution, or UI concerns.

References:

- [Contract expectations](../../docs/expectations/contracts.md)
- [Protocol/domain rules](../../.ocentra-ai/rules/ocentra-parent-protocol-websocket.mdc)
