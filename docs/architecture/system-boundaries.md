<!-- agent-capsule -->

> Agent Capsule
> Doc: System Boundaries
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# System Boundaries

## Runtime Surfaces

| Surface               | First Role                          | Later Role                                                    |
| --------------------- | ----------------------------------- | ------------------------------------------------------------- |
| Windows agent service | Headless local process scaffold     | Capture, local queue, health, enforcement                     |
| Local/LAN control API | Rust-hosted local query/control API | Dev bridge, Rust transport, and future Cloudflare parity layer |
| Portal                | Vite dev/HMR presentation surface   | Tauri/mobile parent apps through HostBridge                   |
| Cloudflare            | Out of v0 scaffold scope            | Auth, billing, relay, notifications, stateless compile status |
| Parent-owned storage  | Out of v0 scaffold scope            | Google Drive, OneDrive, iCloud, Dropbox, NAS, local exports   |
| Notification adapters | Out of v0 scaffold scope            | WhatsApp, push, email, SMS, or provider-specific alerts       |

## Platform Strategy

Current parent architecture is Rust-first. Product flow is TSX UI through
HostBridge into the Rust parent app facade, Rust event bus/domain, Rust read
models, then back through HostBridge to TSX UI. The Vite web portal is dev-only
for Codex/HMR and local visibility; it is not a product target.

The product targets are Tauri desktop plus Android and iOS parent shells. Rust
owns contracts, schemas, route snapshots, action handling, read models,
projections, business logic, and policy/activity/tracking/network/browser/
enforcement/logging shapes. TypeScript owns presentation, generated bridge DTO
consumption, thin host/dev adapters, and minimal local visual state.

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

The product parent UI talks to Rust through HostBridge. Dev web may use
DevWebHostBridge and local dev transport while Codex/HMR work is in progress.
Remove WebSocket from the product `TSX UI <-> parent Rust` path only; Rust-owned
parent/child LAN/WAN transport remains a runtime concern outside this
architecture thread.

```text
TSX UI action
  -> HostBridge
  -> Rust parent app facade
  -> Rust event bus/domain
  -> Rust read model
  -> HostBridge
  -> TSX UI snapshot rendering
```

Dev web may substitute a dev bridge adapter and local dev transport for
HostBridge. That does not make web/Vite a product runtime.

The portal is not an execution boundary. It can request status, author rules, send parent approvals, and display outcomes. The child-device agent validates those requests, runs local AI and policy evaluation, owns timers, and performs enforcement through platform adapters. Browser, mobile, and future desktop portal shells must not run OS commands, capture adapters, child-safety AI, policy evaluators, enforcement logic, or arbitrary scripts.

LAN mode has its own guardrails:

- default dev remains loopback-only
- non-loopback agent binds require an explicit local-network enable flag
- browser origins are allowlisted for HTTP and WebSocket upgrade requests
- managed scripts bind the portal and agent together using the same selected LAN host

Command handlers should be async and nonblocking. If a future platform capability needs blocking OS calls, that work should move behind a bounded adapter so HostBridge, dev transport, WebSocket, and local API surfaces remain responsive in their respective scopes.

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

If two runtimes need to agree on a product value, it belongs in `crates/schema`
or the owning Rust domain/runtime crate before any runtime consumes it.
TypeScript consumes generated bridge DTOs or temporary edge decoders from that
Rust-owned source.
