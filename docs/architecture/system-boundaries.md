# System Boundaries

## Runtime Surfaces

| Surface               | First Role                          | Later Role                                                    |
| --------------------- | ----------------------------------- | ------------------------------------------------------------- |
| Windows agent service | Headless local process scaffold     | Capture, local queue, health, enforcement                     |
| Local/LAN control API | Rust-hosted local query/control API | Parent portal bridge and future Cloudflare parity layer       |
| Portal                | Web-first dev scaffold              | Tauri/mobile parent apps, reports, rules, devices, alerts     |
| Cloudflare            | Out of v0 scaffold scope            | Auth, billing, relay, notifications, stateless compile status |
| Parent-owned storage  | Out of v0 scaffold scope            | Google Drive, OneDrive, iCloud, Dropbox, NAS, local exports   |
| Notification adapters | Out of v0 scaffold scope            | WhatsApp, push, email, SMS, or provider-specific alerts       |

## Platform Strategy

The v0 implementation target is Windows desktop service plus local portal. The Rust service owns the first API and WebSocket endpoint so the portal can query, configure, and observe the agent before real capture, policy, or storage work begins.

Shared domains and contracts must still be platform-neutral. They should leave room for:

- desktop service agents: Windows first, then macOS and Linux
- mobile service agents: Android and iOS where platform rules allow it
- parent portal shells: packaged desktop/mobile first for product use; the Vite
  web portal remains a dev scaffold until a packaged app exists
- remote access: parent-owned storage, authenticated relay, and minimal
  notification routing for the parent-away-from-home use case
- outbound notifications: provider adapters later, not core service logic

The public website at `family.ocentra.ca` is a download, account, subscription,
documentation, and optional stateless report-compile surface. It must not become
the default custody layer for child activity data.

## Local Command Channel

The local portal talks to the Rust service through WebSocket intent/event envelopes. Loopback mode is private by default; LAN mode is explicit and exposes the same dev protocol to another device on the local network.

```text
portal query or rule intent
  -> AgentCommandEnvelope
  -> ws://127.0.0.1:4477/api/dev/ws or ws://<lan-ip>:4477/api/dev/ws
  -> Rust agent validator and dispatcher
  -> AgentEventEnvelope
  -> portal event log and read model
```

This keeps the long-term shape compatible with multiple devices. Loopback and
LAN are the first routes; later Cloudflare can relay typed query, rule, approval,
and event envelopes to a remote device service without storing child evidence as
the default product model.

The portal is not an execution boundary. It can request status, author rules, send parent approvals, and display outcomes. The child-device agent validates those requests, runs local AI and policy evaluation, owns timers, and performs enforcement through platform adapters. Browser, mobile, and future desktop portal shells must not run OS commands, capture adapters, child-safety AI, policy evaluators, enforcement logic, or arbitrary scripts.

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
  -> local enforcement adapter when action is required
  -> local API
  -> parent portal
```

The scaffold does not implement the capture or local AI safety-evaluation flow
yet. It only reserves the boundaries where the flow will live. The product
target is local-first: child-device safety decisions run on the child device,
while any remote assistant, report compilation, or cloud-supported workflow is
reserved for later parent-authorized data-custody boundaries.

## Data Custody Target

```text
child evidence -> child device journal and SQLite
parent settings -> child or parent device, then optional parent-owned storage
parent reports -> parent device cache or parent-owned storage
Ocentra services -> account, billing, downloads, update metadata, relay, minimal notifications
```

Ocentra-hosted services should not store raw journals, SQLite evidence stores,
screen images, browser history, generated reports, or parent rules by default.
If a future feature needs Ocentra-hosted family data custody, that feature needs
its own explicit product, security, privacy, retention, deletion, and validation
design before implementation.

## Contract Rule

If two runtimes need to agree on a value, it belongs in a domain package before any runtime consumes it.
