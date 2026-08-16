<!-- agent-capsule -->

> Agent Capsule
> Doc: RustDesk Remote Capabilities First Pass
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# RustDesk Remote Capabilities First Pass

Status: first-pass research and product direction note.

This document records what Ocentra Parent should learn from RustDesk without
copying RustDesk source. The goal is not to shrink the product ambition because
remote desktop, NAT traversal, multi-platform services, or remote control are
hard. The goal is to use a working product as a reference so Ocentra can build a
cleaner, contract-first, Rust-first remote capability system.

RustDesk is a strong reference because it already solves a related class of
problems: device identity, rendezvous, LAN discovery, NAT traversal, relay
fallback, remote screen transport, input control, session permissions,
platform-specific service installation, and self-hostable infrastructure.
Ocentra Parent is not trying to become a generic remote desktop utility, but
remote desktop and remote control are real capabilities that belong on the
product horizon.

## Research Baseline

Local research checkouts used for this pass:

- `C:\Users\sujan\.codex\research\rustdesk`
  at `fa369365a576cc3a86e4643ba7224f65b48bdf9c`.
- `C:\Users\sujan\.codex\research\rustdesk-server`
  at `815c728837b8a091c9feeeabb423d543be3a7f8d`.
- `C:\Users\sujan\.codex\research\rustdesk-server-demo`
  at `4165a07271700356822ae5ad94696033e882d718`.

Primary external references:

- RustDesk product site: <https://rustdesk.com/>
- RustDesk client: <https://github.com/rustdesk/rustdesk>
- RustDesk server: <https://github.com/rustdesk/rustdesk-server>
- RustDesk self-host docs:
  <https://rustdesk.com/docs/en/self-host/rustdesk-server-oss/>
- RustDesk client configuration docs:
  <https://rustdesk.com/docs/en/self-host/client-configuration/>

License note: RustDesk client and server are AGPL-3.0. Treat the repositories as
reference material. Do not vendor, copy, port line-by-line, or reuse protocol
schemas. Borrow architecture, proof ideas, and implementation lessons through a
clean Ocentra design.

## Product Lens

Ocentra Parent is a parent-controlled capability platform for a real modern
problem: parents need visibility, controls, assistance, and peace of mind across
the devices and media surfaces their children use. The product should not
decide a household's rulebook. It should provide powerful, understandable
tools, let the parent choose what to enable, and make the system usable both by
technical parents and by non-technical family members through the assistant.

That means remote capability is bigger than "remote desktop":

- remote child-device health;
- remote route status;
- remote rule updates;
- remote approvals;
- remote activity/report queries;
- remote screen visibility;
- remote guided support;
- remote input control;
- remote app/game/social/media control;
- remote location or device-presence visibility where platforms allow it;
- remote evidence sync to parent-owned storage;
- optional remote desktop when the parent explicitly enables it.

RustDesk is useful because it proves much of the transport and platform
machinery is not hypothetical. Ocentra should not tone this down to a weak
LAN-only product. The right move is to separate ambition from sequencing: build
the remote fabric first, then attach increasingly powerful capabilities.

## What RustDesk Proves

RustDesk proves these product and engineering claims:

1. A local device can be a long-running host service, not just a UI app.
2. A second device can discover or reach that host through direct LAN, NAT
   traversal, or relay.
3. Rendezvous and relay can be separated so the control plane does not need to
   own the whole data stream.
4. A self-hosted deployment can work with a small server pair.
5. Session capabilities can be negotiated and toggled.
6. Screen capture and input control can be normalized behind platform adapters.
7. Platform installers must own service lifecycle directly.
8. Android remote-control style capabilities require foreground service,
   permissions, MediaProjection, Accessibility, and visible OS state.
9. User-facing session UI matters as much as transport: the controlled device
   needs to show who is connected, what is allowed, and how to stop it.

RustDesk's own README names the useful split:

- `libs/hbb_common`: video codec, config, TCP/UDP wrapper, protobuf, file
  transfer helpers, and other utilities.
- `libs/scrap`: screen capture.
- `libs/enigo`: platform keyboard/mouse control.
- `src/server`: audio, clipboard, input, video, and network host services.
- `src/client.rs`: peer connection startup.
- `src/rendezvous_mediator.rs`: server communication and direct or relayed
  connection setup.
- `src/platform`: platform-specific code.
- `flutter`: desktop and mobile UI.

Ocentra should not copy that structure exactly. It should keep Rust-first
runtime/service code and Rust-owned contracts, with TypeScript limited to
presentation, generated DTO consumption, thin adapters, and edge validation
where needed. The responsibility split is valuable.

## Core Borrowable Ideas

### 1. Remote Fabric, Not One Remote Feature

RustDesk has a remote session fabric. Remote desktop is only one thing carried
over that fabric. Ocentra should model the same way:

- device identity;
- pairing and trust;
- route negotiation;
- direct route attempt;
- relay fallback;
- capability-scoped session;
- session audit;
- child-agent execution;
- parent surface status;
- failure and denial reasons.

For Ocentra, the session payload is not only pixels and input. It can be rule
changes, approval decisions, report queries, app control commands, screen
visibility requests, assistant actions, or later a desktop stream.

### 2. Rendezvous And Relay Split

RustDesk server uses `hbbs` as the ID/rendezvous server and `hbbr` as the relay.
The server README describes the binaries this way, and the server `Cargo.toml`
defines separate `hbbs` and `hbbr` binaries.

In code, the rendezvous server starts TCP, UDP, NAT-test, and websocket
listeners from one selected port family. It decides whether a peer is online,
whether peers are in the same intranet, whether direct punch-hole setup should
be attempted, and which relay server to return.

RustDesk evidence:

- `rustdesk-server/src/rendezvous_server.rs`: server start initializes UDP,
  TCP, NAT-test, websocket, relay lists, and an `ALWAYS_USE_RELAY` mode.
- `rustdesk-server/src/rendezvous_server.rs`: `handle_punch_hole_request`
  rejects invalid keys, marks offline peers, checks LAN/same-intranet state,
  returns local address fetch, punch-hole, or relay direction.
- `rustdesk-server/src/relay_server.rs`: relay pairs clients by relay request
  UUID and then copies bytes between them.

Ocentra direction:

- V0.9 should keep hardening local/LAN route selection.
- V2 should add an Ocentra remote control plane and relay route.
- The relay must be a transport for typed parent-child intents, not a place
  where Ocentra stores child activity evidence by default.
- A parent-owned or self-hosted relay should remain a product option.

### 3. Direct First, Relay When Needed

RustDesk client chooses a rendezvous server, optionally performs UDP NAT tests,
builds a punch-hole request, sends several attempts, races direct connection
methods, then falls back to relay if direct fails or force-relay is enabled.

RustDesk evidence:

- `rustdesk/src/client.rs`: selects rendezvous server and starts optional UDP
  NAT test.
- `rustdesk/src/client.rs`: sends `PunchHoleRequest` with peer ID, token,
  connection type, NAT type, UDP port, force-relay, and IPv6 socket.
- `rustdesk/src/client.rs`: races TCP, UDP, and IPv6 attempts, then requests
  relay when direct connection fails.

Ocentra direction:

- Parent surfaces should show the chosen route: loopback, LAN-direct,
  WAN-direct, relay, queued, offline, stale, or denied.
- Remote capability should not be blocked by "same LAN only" once V2 starts.
- Route choice should be observable and testable, not hidden inside transport
  code.
- Every route decision should be owned by Rust domain/schema contracts before
  Rust service implementation.

### 4. Capability-Scoped Sessions

RustDesk has a `ControlPermissions` protocol message with individual bits for
keyboard, printer, clipboard, file, audio, camera, terminal, tunnel, restart,
recording, block input, remote modify, and privacy mode.

Ocentra should borrow the idea but not the exact permissions or schema. Ocentra
capability families should map to parent product choices:

- view device health;
- view current active app/window/site;
- view recent report;
- request a live screenshot;
- request live screen view;
- send rule update;
- approve or deny a child request;
- close or pause an app where the platform allows it;
- start guided support session;
- allow remote input control;
- allow clipboard/file/terminal only as explicit advanced support tools.

Important product decision: remote desktop should be a capability family, not
the authorization model. A paired parent should not automatically get every
powerful control. The parent chooses capabilities, the session asks for the
specific capability, and the child agent records the decision and route.

### 5. Back Notifications And Exact Failure Reasons

RustDesk sends back notifications and status changes when a requested state is
unavailable or denied, such as privacy mode or block-input failures.

Ocentra needs this pattern everywhere:

- OS permission missing;
- parent role lacks authority;
- controller lease is held elsewhere;
- child device offline;
- route unavailable;
- relay unavailable;
- family/device mismatch;
- session revoked;
- child-side stop button pressed;
- app control unavailable on the platform;
- screen capture permission denied;
- mobile OS does not allow the requested action.

This should become a typed denial/result vocabulary, not UI-only text.

### 6. Visible Controlled-Device Surface

RustDesk controlled-device UI shows requester, session status, timer,
permissions, and stop/disconnect controls. Android uses foreground service and
notifications for remote service state.

Ocentra should use this, but product-shaped:

- "Parent is connected" status on the child device.
- Active capability labels, such as "Viewing health", "Viewing screen",
  "Controlling app", or "Helping remotely".
- Session timer and route state.
- Stop/revoke control.
- Clear parent identity.
- Platform permission state.
- Audit-friendly event log.

This is not moral positioning. It is product clarity and debugging clarity.

### 7. Platform Service Lifecycle

RustDesk does not treat desktop/mobile as one generic packaging problem.

Observed RustDesk patterns:

- Windows routes `--install-service`, `--uninstall-service`, `--service`, and
  `--server` before UI launch.
- Windows MSI custom actions install services and firewall rules.
- macOS uses LaunchDaemon plus LaunchAgent.
- Linux uses systemd service lifecycle.
- Android declares foreground service, boot receiver, overlay, accessibility,
  MediaProjection, notifications, and wake permissions.
- Updates avoid active sessions and re-check state before applying.

Ocentra should mirror the principle, not the code:

- child-agent service and parent UI are separate runtime roles;
- platform packages own install/start/stop/uninstall;
- service state is provable after reboot;
- updater cannot break an active parent-child session;
- Android remote capability requires explicit permission and foreground state;
- route/service names, args, events, and logs live in Ocentra domain/protocol
  boundaries, not inline app strings.

### 8. Self-Host And Parent-Owned Routes

RustDesk's self-host model matters for Ocentra because Ocentra's product thesis
is local-first and parent-owned custody.

Ocentra should consider three deployment modes:

1. Local/LAN only: parent and child devices communicate on household network.
2. Ocentra relay/control plane: account/device route metadata, entitlement,
   notification, and stateless relay of typed intents.
3. Parent-owned relay: advanced families or technical users can run their own
   rendezvous/relay or storage connector.

The product should not require every parent to understand this. The assistant
can configure the default. But the architecture should not block self-host or
parent-owned custody.

## Where Ocentra Should Be Cleaner

RustDesk is a mature product, but it carries a lot of history and many
technologies: Rust, Flutter, Kotlin, Java, C++, Objective-C, CMake, vcpkg,
system packaging, protobufs, and native platform glue. Ocentra does not need to
inherit that surface.

Cleaner Ocentra choices:

- Rust service first for runtime behavior.
- Rust-owned schema/runtime crates for product contracts, parser/serde tests,
  route snapshots, and actions.
- Rust protocol crate mirrors only for transport-specific service boundaries.
- Tauri/HostBridge for product parent UI and Vite only for dev/HMR.
- Effect Schema only for untrusted TS edges or generated validation edges.
- No copied protobuf schema.
- No generic remote-desktop permission model as the first product model.
- No direct child evidence custody in Ocentra-hosted cloud by default.
- Remote streams should be ephemeral unless parent explicitly records or stores
  to a parent-owned target.
- Assistant actions should compile to typed intents and audited outcomes.

## Remote Desktop Horizon

Remote desktop is valid for this product. The question is how to build it into
Ocentra without making it a detached utility.

Recommended horizon:

### Phase 0: Remote Route Contracts

Define route/session/device contracts before streaming:

- device route advertisement;
- parent role and controller lease;
- paired device registry;
- remote session request;
- capability request;
- route result;
- relay result;
- denial reason;
- session audit event;
- heartbeat.

### Phase 1: Remote Status And Intent Relay

Support parent-away-from-home basics:

- device health;
- online/stale/offline;
- rule update;
- approval decision;
- report query;
- selected-device state;
- queue when child is temporarily unreachable;
- local-first fallback when cloud is unavailable.

### Phase 2: Remote Visibility

Add parent-requested visibility that is lighter than full desktop:

- current app/site/window where platform allows it;
- recent activity report;
- live screenshot request;
- short live screen sample;
- screen evidence with clear source/custody label;
- permission-required and unavailable states.

### Phase 3: View-Only Remote Desktop

Add live screen view:

- explicit parent capability;
- child-agent OS permission gate;
- visible child-device session state;
- route quality indicator;
- pause/stop/revoke;
- no clipboard/file/input by default;
- no cloud retention by default.

### Phase 4: Remote Control

Add input/control only after view-only path is proven:

- input capability separate from view capability;
- app/window scoped control where possible;
- emergency stop;
- audit log;
- platform-specific capability labels;
- assistant can explain what is available on this device.

### Phase 5: Advanced Support Tools

Optional advanced tools:

- clipboard sync;
- file transfer;
- terminal;
- remote restart;
- privacy/block-input equivalents.

These should be advanced, explicit, and probably not part of the first child
remote-control slice. They are useful support tools, but they can also destroy
trust and create difficult platform/security problems if mixed into the first
remote capability proof.

## Ocentra Package And Crate Ownership

Start with existing boundaries:

- `crates/schema`
  owns cross-boundary remote route, session, heartbeat, capability, custody,
  selected-device, conflict, action/result, and generated UI DTO shapes.
- `crates/parent-runtime-core`
  owns the parent facade, HostBridge action handling, route snapshots, and
  parent-facing read models.
- `crates/agent-protocol`
  mirrors transport-specific Rust command/event contracts for parent/child
  remote intents only where Rust transport needs them.
- Rust endpoint/logging/activity/domain crates
  own product endpoint policy, proof log shape, activity/report query behavior,
  and transport/runtime decisions.
- TypeScript packages
  keep only generated DTO imports, TS edge decoders, pure presentation labels,
  DOM IDs, thin adapters, or dev/proof helpers.
- `crates/agent-service`
  owns runtime validation, route handling, local service behavior, and any
  Rust-owned parent/child transport execution.
- `crates/agent-core`
  can own reusable route registry, queue, and persistence helpers if they become
  cross-service primitives.

A future `@ocentra-parent/remote-domain` or `connectivity-domain` can make sense
when V2 grows beyond `parent-domain`, but it should not be created just because
RustDesk has many modules.

## Proof And Harness Ideas To Borrow

RustDesk is useful as proof inspiration. Ocentra should build smaller,
product-specific harnesses:

1. Two local child agents and one parent portal route selector.
2. LAN parent-to-child route proof across two machines.
3. Direct route preferred over relay when both are available.
4. Forced relay mode for deterministic testing.
5. Stale heartbeat produces visible stale state.
6. Wrong family/device/session token is rejected.
7. Revoked pairing cannot open a remote route.
8. Controller lease prevents competing write authority.
9. Observer route remains read-only.
10. Remote rule update travels through typed relay and executes on child agent.
11. Remote approval travels through typed relay and records audit result.
12. View-only screen request returns permission-required until platform
    permission is actually granted.
13. Android foreground service and permission state are visible in proof.
14. Active remote session blocks unsafe update/restart.
15. Cloud unavailable still leaves LAN/local path working.

This is the right kind of moat: not one flashy remote desktop demo, but a
remote capability system whose route, authority, custody, and proof states are
visible end to end.

## Risks Without Reducing Ambition

Security, abuse resistance, and privacy are not reasons to avoid remote
capability. They are reasons to model authority and proof from the beginning.
For the first pass, the important point is not to over-index on security review
so early that nothing powerful gets built. The important point is to avoid
building a transport that cannot later support security, custody, revocation,
and audit.

Design now so later hardening has places to attach:

- typed identity;
- typed session authority;
- typed capability grants;
- expiration;
- revocation;
- audit events;
- route labels;
- custody labels;
- denial reasons;
- no silent fallback to weaker modes;
- no hidden product behavior.

## Recommended Next Document

The next useful document should be a concrete V2 shape, not another broad
research note:

`docs/architecture/remote-capability-fabric-v2-plan.md`

It should define:

- remote route state model;
- capability taxonomy;
- session lifecycle;
- rendezvous/control-plane responsibilities;
- relay responsibilities;
- child-agent responsibilities;
- parent-portal responsibilities;
- proof harness list;
- non-goals for the first V2 slice;
- how view-only and full remote desktop attach later.

## First Implementation Bias

When this moves from research to implementation, avoid a weak "remote status
only" slice that cannot grow. The first slice should be narrow but structurally
real:

- typed Rust-owned remote route/session/capability contracts;
- real Rust parser/validation;
- real local two-service proof;
- deterministic relay simulation or local relay process;
- visible parent route state;
- child-agent acceptance/rejection;
- audit result.

That gives Ocentra the base for remote desktop, remote control, remote
approvals, remote reports, and assistant-driven actions without turning the
first branch into a giant cross-platform screen-streaming project.
