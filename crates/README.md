# Rust Crates

The Rust workspace is scaffolded for a Windows-first headless agent.

Current crates are placeholders only:

- `agent-core`: future service/runtime core.
- `agent-protocol`: future Rust protocol structs that must match TypeScript domain contracts.
- `agent-service`: future Windows service binary with scaffolded loopback and LAN dev endpoints.

No capture, service installation, startup, tamper-resistance, policy enforcement, or cloud network logic is implemented yet.
