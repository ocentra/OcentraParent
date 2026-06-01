# ocentra-parent-agent-protocol

Rust protocol parity crate for data that crosses the TypeScript/Rust boundary.

## Owns

- Serde structs and enums for Rust-crossing command/event payloads.
- Protocol constants used by Rust service/core code.
- Exact field names and enum values mirrored from TypeScript contracts.

## Must Not Own

- Runtime behavior.
- Policy decisions.
- UI labels or portal layout.
- Local string literals that should be constants.

## Flow

```mermaid
flowchart LR
  TSDomain["packages/*-domain"]
  RustProtocol["agent-protocol"]
  Service["agent-service"]
  Core["agent-core"]
  TSDomain --> RustProtocol
  RustProtocol --> Service
  RustProtocol --> Core
```

## Connected Docs

- [Contract expectations](../../docs/expectations/contracts.md)
- [Protocol/domain rules](../../.ocentra-ai/rules/ocentra-parent-protocol-websocket.mdc)

## Gaps To Fill

- Add parity tests for every new Rust-crossing shape.
- Keep constants granular so service/core code does not invent strings.
