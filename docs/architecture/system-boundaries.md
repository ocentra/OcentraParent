# System Boundaries

## Runtime Surfaces

| Surface               | First Role                          | Later Role                                              |
| --------------------- | ----------------------------------- | ------------------------------------------------------- |
| Windows agent service | Headless local process scaffold     | Capture, local queue, health, enforcement               |
| Local/LAN control API | Rust-hosted local query/control API | Parent portal bridge and future Cloudflare parity layer |
| Portal                | Web-first parent-facing scaffold    | Reports, rules, devices, alerts, mobile/desktop shells  |
| Cloudflare            | Out of v0 scaffold scope            | Remote control plane, sync, auth, device fleet          |
| Notification adapters | Out of v0 scaffold scope            | WhatsApp, push, email, SMS, or provider-specific alerts |

## Platform Strategy

The v0 implementation target is Windows desktop service plus local portal. The Rust service owns the first API and WebSocket endpoint so the portal can command and observe the agent before real capture, policy, or storage work begins.

Shared domains and contracts must still be platform-neutral. They should leave room for:

- desktop service agents: Windows first, then macOS and Linux
- mobile service agents: Android and iOS where platform rules allow it
- portal shells: web first, then mobile and desktop wrappers
- remote access: Cloudflare later for the parent-away-from-home use case
- outbound notifications: provider adapters later, not core service logic

## Local Command Channel

The local portal talks to the Rust service through WebSocket command/event envelopes. Loopback mode is private by default; LAN mode is explicit and exposes the same dev protocol to another device on the local network.

```text
portal command button
  -> AgentCommandEnvelope
  -> ws://127.0.0.1:4477/api/dev/ws or ws://<lan-ip>:4477/api/dev/ws
  -> Rust command dispatcher
  -> AgentEventEnvelope
  -> portal event log and read model
```

This keeps the long-term shape compatible with multiple devices. Loopback and LAN are the first routes; later Cloudflare can relay the same command/event envelope to a remote device service.

LAN mode has its own guardrails:

- default dev remains loopback-only
- non-loopback agent binds require an explicit local-network enable flag
- browser origins are allowlisted for HTTP and WebSocket upgrade requests
- managed scripts bind the portal and agent together using the same selected LAN host

Command handlers should be async and nonblocking. If a future platform capability needs blocking OS calls, that work should move behind a bounded adapter so the WebSocket and local API surfaces remain responsive.

## Data Flow Target

```text
agent service
  -> normalized local event
  -> NDJSON journal
  -> SQLite ingest
  -> local AI safety evaluation
  -> typed policy decision
  -> local API
  -> parent portal
```

The scaffold does not implement the capture or local AI safety-evaluation flow yet. It only reserves the boundaries where the flow will live. The product target is local-first: child-device safety decisions should run on the child device by default, while API AI is reserved for later parent assistant, reporting, or cloud-supported workflows with explicit privacy boundaries.

## Contract Rule

If two runtimes need to agree on a value, it belongs in a domain package before any runtime consumes it.
