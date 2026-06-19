<!-- agent-capsule -->

> Agent Capsule
> Doc: V2 Parent-Owned Remote Access And Cloud Relay Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V2 Parent-Owned Remote Access And Cloud Relay Expectations

This is the milestone-specific expectation file for V2 in
`docs/product-roadmap.md`.

Supporting expectation files: [data custody](../expectations/data-custody.md),
[cloud](../expectations/cloud.md), [sync and export](../expectations/sync-export.md),
[LAN pairing](../expectations/lan-pairing.md), and
[static analysis and security](../expectations/static-analysis-security.md).

Supporting architecture notes:

- [RustDesk remote capabilities first pass](../architecture/rustdesk_remote_capabilities_first_pass.md)
- [Remote capability fabric V2 plan](../architecture/remote-capability-fabric-v2-plan.md)

## Outcome

- Parent-away-from-home use cases work without making Ocentra the family-data
  store.
- Ocentra moves beyond local/LAN while keeping child-device agents as the local
  execution authority.
- Remote access is a capability fabric, not only a cloud status page and not
  only a remote desktop clone.
- Remote desktop, live screen view, and remote input/control are real V2+
  product horizons, modeled as explicit capabilities that attach to the same
  route, session, custody, and audit system as rule/query/approval intents.
- Cloud or relay services act as account, control-plane, notification, relay,
  connector-status, and optional stateless compile surfaces.
- Child-device agents validate and execute scoped typed intents locally.
- The parent decides which observation, visibility, and control capabilities are
  enabled for the household.

## RustDesk Research Carry-Forward

RustDesk proves that a related product can combine long-running host services,
device identity, rendezvous, LAN discovery, NAT traversal, relay fallback,
screen transport, input control, session permissions, platform-specific
installation, and self-hostable infrastructure.

Ocentra should borrow ideas, proof shapes, and product lessons, not RustDesk
source code, protocol schemas, UI code, packaging scripts, or vendored
dependencies. The Ocentra implementation remains contract-first and Rust-first.

Borrowed ideas to preserve:

- Direct route first, relay fallback when needed.
- Separate rendezvous/control-plane responsibilities from relay forwarding.
- Parent-visible route state: local, LAN, WAN/direct candidate, relay, queued,
  stale, offline, denied, or unavailable.
- Session-scoped capabilities rather than one all-powerful remote-control mode.
- Controlled-device visible session state, including parent identity, active
  capability, session timer, stop/revoke, and platform permission status.
- Back notifications with exact denial or degraded reasons.
- Forced relay mode and route failure states for deterministic proof harnesses.
- Platform-specific service lifecycle and permission proof.
- Self-host or parent-owned relay as an advanced custody option.

Do not carry forward:

- Unattended permanent password as the default trust model.
- Hidden-screen/privacy-mode behavior as an early product path.
- File transfer, clipboard sync, terminal, restart, or elevation in the first
  remote-control proof.
- Any fallback to weaker or ambiguous route/session authority.
- Any Ocentra-hosted default storage of child activity evidence.

## Parent Outcome

A parent away from home can open the parent app or portal and see:

- selected child and device;
- device health and last heartbeat;
- current route and custody state;
- whether the child agent is reachable directly, through LAN, through relay, or
  not reachable;
- which remote capabilities are available, active, disabled, permission
  required, unavailable, or degraded;
- whether the parent has controller authority or observer-only authority;
- whether a command was accepted, rejected, queued, expired, revoked, or
  degraded;
- how to request remote health, rule updates, approval decisions, reports,
  screen visibility, live view, and later remote input/control.

The parent should not need to understand NAT traversal or relay internals. The
UI should expose product state, not transport trivia.

## Child-Device Outcome

The child-device agent remains the execution authority. It accepts only
schema-valid, paired, scoped remote intents and records route/session/capability
results for audit.

When live visibility or control is active, the child device must have a visible
session surface appropriate to the platform:

- parent identity;
- active capability;
- active route/session state;
- stop/revoke path;
- OS permission state;
- degraded/unavailable reason where relevant.

## Capability Scope

V2 remote capability contracts must cover more than remote status:

- device health;
- route status;
- activity/report query;
- rule update;
- approval decision;
- parent-owned storage connector status;
- sync queue and conflict state;
- screen snapshot request;
- live screen view;
- remote input/control;
- app/browser/game/network control intents;
- assistant action as a typed remote intent;
- advanced support capabilities such as clipboard, file transfer, terminal, and
  restart as later explicit opt-in capabilities.

The first V2 implementation does not need to implement every capability, but
the contract model must not block remote desktop or remote control from being
added cleanly.

## Route And Relay Expectations

The route model must distinguish:

- local cache;
- localhost;
- local network;
- WAN/direct candidate;
- Ocentra relay;
- parent-owned relay;
- queued;
- stale;
- offline;
- denied;
- unavailable.

Ocentra relay is a typed envelope and optional ephemeral stream route. It must
not become the default child-activity data warehouse.

The relay/rendezvous core should be Rust-first:

- TypeScript domain contracts define shared meaning.
- `crates/agent-protocol` mirrors Rust-facing protocol structs and constants.
- `crates/agent-service` validates and executes child-agent behavior.
- A future Rust relay crate or process should own local/dev relay proof before
  hosted complexity is added.
- Cloudflare or another hosted edge can wrap account/control-plane deployment,
  but it must not become the protocol source of truth.

## Portal Expectations

The existing `remote-access` route must become a real product surface.

Expected UI surfaces:

- selected child/device strip;
- route board;
- capability grid;
- active session panel;
- event/audit rail;
- assistant entry point that compiles parent requests into typed actions.

The parent UI must expose:

- active route;
- route fallback;
- reachable/stale/offline state;
- active capability;
- permission-required state;
- denial reason;
- custody label;
- source of any report or visibility result.

## Contract Boundary

Remote capability contracts reuse or extend shared domain packages before
runtime code consumes them.

Expected contract families:

- remote route;
- remote capability;
- remote session;
- remote denial/degraded reason;
- remote audit event;
- remote relay delivery status;
- child-agent response envelope;
- parent intent envelope;
- screen visibility/live-view session descriptor;
- remote input-control session descriptor;
- custody/source label;
- parent-owned relay or storage connector reference.

Initial TypeScript ownership belongs in `@ocentra-parent/parent-domain` unless
the V2 contract set grows enough to justify a later remote/connectivity domain.

Protocol ownership:

- `@ocentra-parent/schema-domain` owns command/event/session/capability
  schemas.
- `@ocentra-parent/agent-protocol-domain` owns protocol adapters and
  transport-facing mapping built from those schemas.
- `crates/agent-protocol` owns Rust-facing mirror structs and constants for the
  same contracts.
- `@ocentra-parent/logging-domain` owns redacted relay/session/audit log shapes
  built from canonical contracts.
- `@ocentra-parent/portal-domain` owns route IDs, DOM IDs, command descriptors,
  and display text tokens.

Worker/cloud/runtime code must consume those contracts instead of inventing
parallel JSON payloads. Matching TypeScript and Rust contracts must keep the
same encoded shape and stay under drift-check coverage.

## Failure Behavior

- Local observation, local policy, local enforcement, local portal operation,
  and local parent cache continue when remote services are unavailable.
- Remote route outages show explicit stale/offline/queued status.
- Remote rule updates and approvals are idempotent and auditable.
- Retries cannot silently apply stale state.
- A device receiving an expired, revoked, malformed, wrong-family,
  wrong-device, wrong-controller, observer-read-only, or unsupported-capability
  command rejects it and records a safe audit event.
- Relay failures do not erase local evidence or parent-owned storage state.
- Screen visibility and remote input return `permission-required` or
  `platform-unsupported` until the real OS permission and platform adapter path
  is implemented.
- Active remote screen/control sessions block unsafe update/restart behavior.

## Expected Deliverables

- Remote capability fabric contract foundation.
- Authenticated parent identity.
- Authenticated device identity.
- Device heartbeat and route status.
- Remote route/session/capability read models.
- Remote access portal surface.
- Rule/query/approval event relay.
- Parent-owned storage connector contracts.
- Stateless report compiler contracts where remote compilation exists.
- Rust-first relay/rendezvous local proof.
- Direct-first and forced-relay proof modes.
- Conflict handling.
- Local-first fallback.
- Family/device authorization model.
- Auditable relay and compiler status.
- Sensitive-detail minimization policy for remote logs.
- Remote desktop capability taxonomy, including view-only live screen and
  separate remote input/control.
- Platform permission states for screen visibility and remote control.

## Acceptance

- Local operation works when remote services are unavailable.
- Parent can see device/account health remotely.
- Parent can see route, session, capability, custody, and source labels.
- Parent can view reports from local cache, reachable child agent, or
  parent-owned storage with source/custody clearly labeled.
- Remote rule updates, queries, approvals, and device events are authenticated,
  scoped, and auditable.
- Child-device agent validates and executes remote intents locally.
- Device state cannot be overwritten silently by stale remote state.
- Ocentra-hosted databases do not store child activity evidence or reports by
  default.
- Parent-owned storage connectors are explicit and parent-visible.
- Remote parent actions are represented as typed intents and executed only by
  the child-device agent.
- Heartbeat and stale-device states are visible to the parent.
- Conflict outcomes are explicit: accepted, rejected as stale, queued,
  superseded, or needs parent review.
- Remote desktop and remote control are represented as explicit capability
  families even before they are fully implemented.
- View-only screen capability and remote input capability remain separate.

## Validation Gates

- Contract tests for identity, route, heartbeat, relay, capability, session,
  connector, compiler, conflict, and audit payloads.
- Rust protocol parity tests for route/session/capability shapes.
- Remote route local proof with parent and child agents in one checkout.
- Local relay proof with forced relay mode and no child evidence retention.
- Child-agent integration tests for accepted remote intent, rejected stale
  intent, rejected wrong-device intent, rejected observer-write intent, queued
  retry, and local-first fallback.
- Portal/app coverage for remote health, route board, capability grid,
  queued/stale state, connector status, report compile status, and explicit
  command result.
- Screen visibility proof that returns permission-required before platform
  permission is wired.
- Future view-only remote desktop proof with route/session/capability state.
- Future remote input proof with view session required and revoke disabling
  input.
- Secret scan, dependency policy, and security review for auth, tokens,
  provider configuration, billing, relay, and logs.

## Non-Goals

- Do not replace local evidence storage with Ocentra cloud storage.
- Do not use Ocentra-hosted infrastructure as the default family-data
  warehouse.
- Do not store raw child activity evidence, generated reports, screenshots, or
  parent rules in Ocentra-hosted infrastructure by default.
- Do not copy RustDesk code, protocol schemas, generated files, UI code, or
  packaging scripts.
- Do not implement remote desktop as a detached utility that bypasses Ocentra
  route/session/capability/audit contracts.
- Do not ship remote input/control before view-only remote visibility,
  permission state, and stop/revoke behavior are proven.
- Do not route production family data through unauthenticated dev endpoints.

## Done Signal

A parent can remotely authenticate, inspect device route/capability/session
state, send scoped rule/query/approval intents, and receive audited results
while the child-device agent remains local-first. Remote desktop is represented
as a staged capability family, with view-only and input-control paths attached
to the same fabric rather than bolted on as a separate product.
