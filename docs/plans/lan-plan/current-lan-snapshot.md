# Current LAN Snapshot - 2026-06-02

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `Current LAN Snapshot - 2026-06-02`
> Kind: current snapshot; read for status/gap claims.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This snapshot records current LAN source, proof, UI, and gap state before the
LAN plan is used as the active execution checklist.

## Product Claim Boundary

Current docs require this boundary:

```text
LAN scan discovers.
Child agent confirms.
Parent assigns.
```

Windows neighbor-table rows, router/infrastructure rows, IP/MAC evidence,
NetBIOS, LLMNR, reverse DNS, mDNS, SSDP, OUI/vendor data, and light probes may
improve discovery or display. They do not prove a child profile, do not confirm
child-agent identity, and do not make a route controllable.

## Contracts That Exist

`packages/parent-domain` already defines:

- LAN pairing and add-device read-model contracts;
- LAN device parent action contracts;
- LAN production household proof rows;
- signed discovery/relay spine rows;
- LAN discovery source-matrix contracts with all 20 workpack ids;
- source authority, proof state, runtime path, UI surface, and status rows.

`packages/agent-protocol-domain` already defines:

- protocol-facing add-device state contracts;
- LAN source-matrix contracts mirrored from parent-domain;
- signed discovery/relay spine contracts;
- challenge/runtime contracts for LAN pairing.

## Feature Routing Snapshot

The owning feature is
`docs/features/remote-lan-mobile-platforms.md`. It owns LAN route custody,
optional relay/cache, physical household proof, mobile child-agent parity, and
platform non-claims.

The owning adjacent feature is
`docs/features/family-setup-device-roles.md`. It owns household setup, child
profile/device role assignment, parent decisions, selected-device state,
revocation/recovery, and parent-readable setup UX.

Adjacent feature docs provide boundaries:

- `network-domain-control.md` owns network/domain evidence and cannot confirm a
  child-agent identity from destination metadata alone.
- `policy-schedules-approvals.md` owns policy target selection and approval UX.
- `evidence-store-query.md` owns durable evidence/query posture where LAN
  evidence becomes product evidence.
- `reports-notifications-sync.md` owns reporting/sync surfaces and must preserve
  LAN custody/source labels.
- `enforcement-integrity-tamper.md` owns enforcement integrity and stale/revoked
  route safety.
- `production-distribution-support.md` owns release/support proof and redaction.

This plan folder centralizes LAN task planning while those shared feature docs
remain in place.

## Rust Runtime That Exists

`crates/agent-protocol` already has:

- LAN pairing protocol structs;
- browser add-device state protocol structs;
- source-matrix protocol structs;
- signed discovery/relay spine structs;
- production household proof structs;
- LAN pairing constants for field names, labels, claims, and non-claims.

`crates/agent-service` already has:

- LAN network inventory helpers;
- LAN pairing runtime state;
- LAN add-device read model;
- household device spine/merge helpers;
- source-matrix rows for all 20 workpacks and known source rows;
- signed discovery/relay spine state;
- production household proof state;
- route selection, parent decision, and audit support paths.

## Portal That Exists

The parent portal currently renders service-backed LAN state through existing
core UI surfaces:

- Devices/LAN read-model surface;
- Activity/Network diagnostics;
- Policy Network target binding;
- add, route select, rename, trust, ignore, restore, and revoke controls for
  supported service-backed LAN slots;
- visible-only unsupported/router/passive rows;
- source-matrix status, authority, evidence, route, signed proof, relay/cache,
  manual-proof, unproved-claim, and audit diagnostics.

## Proof That Exists

Focused LAN source-matrix proof:

```text
node scripts/test/v0-9-lan-source-matrix-plan-completion.mjs
test-results/v0-9-lan-source-matrix-plan-completion/proof.json
```

Current proof counts from that artifact:

- workpacks: 2 implemented, 10 partial, 5 manual-required,
  3 not-implemented;
- sources: 9 implemented, 5 partial, 10 manual-required,
  7 not-implemented;
- weak source fence: 21 weak sources out of 31 total sources, 0 can confirm a
  child agent, and 0 can assign a child profile.

Manual-required workpacks:

- 07 Passive discovery listeners;
- 08 mDNS and DNS-SD discovery;
- 09 SSDP and UPnP discovery;
- 17 Parent and child mDNS advertisements;
- 18 Signed child hello and heartbeat.

Not-implemented workpacks:

- 05 Targeted ARP checks;
- 06 Bounded ARP sweep;
- 11 Light service probing.

Current claims proved:

- LAN read model carries all 20 plan workpacks as typed status rows;
- weak LAN evidence sources are visible but cannot confirm child-agent identity
  or assign child profile;
- signed child-agent hello and heartbeat remain artifact-gated instead of being
  silently marked implemented;
- Devices/LAN and Activity/Network can render the matrix through the
  service-backed add-device read model.

Current claims not proved:

- packet-mode adapters remain manual-required;
- physical household proof still needs a second child-agent device;
- mDNS/SSDP advertisement is not implemented yet.

## UI Proof That Exists

Live B-lane browser proof has been captured under:

```text
output/playwright/lan-source-matrix-plan-completion/
```

Current screenshot artifacts:

- `devices-lan-source-matrix.png`;
- `activity-network-source-matrix.png`;
- `policy-network-target-binding.png`;
- `browser-proof.json`.

These prove the current service-backed surfaces render on the B-lane dev ports.
They do not prove a real two-physical-child household LAN workflow.

## Current Gaps

- Physical household proof still needs a second installed child agent, router or
  firewall reachability proof, and generated manual proof artifacts.
- Signed child-agent hello and heartbeat rows exist but are artifact-gated.
- Parent and child mDNS advertisements are not implemented.
- Passive packet listeners are not implemented.
- Targeted ARP refresh, bounded ARP sweep, and light service probing are not
  implemented.
- mDNS, DNS-SD, SSDP, UPnP, NetBIOS, LLMNR, reverse DNS, service probing, and
  OUI/vendor rows are represented as weak/manual-required evidence, not
  identity proof.
- Full canonical household device store and restart proof is partial.
- Full replayable LAN event stream proof is partial.
- Optional relay/cache and parent-owned storage routes remain unavailable or
  manual-required.
- Android/iOS child-agent parity, signing, store distribution, and mobile
  entitlements remain manual-required or not implemented.
- Production first-run setup UX is not complete.

## Where We Want To Be

The LAN subsystem should become a service-backed product flow from code to UI:

```text
interface selection
-> bounded discovery sources
-> evidence rows
-> canonical household device record
-> signed child-agent confirmation
-> parent assignment and route custody
-> durable registry and restart recovery
-> portal Devices/LAN and Activity/Network diagnostics
-> policy target binding
-> proof artifacts and manual-required gaps
```

Every visible parent claim should answer:

- What source produced this device or evidence row?
- Is the source weak, strong, manual-required, or not implemented?
- Can this source confirm a child agent?
- Can this source assign a child profile?
- Is the route local, LAN, relay, cache, stale, offline, unavailable, or
  revoked?
- What can the parent do now?
- Which actions are visible-only or manual-required?
- What proof artifact backs the claim?

## Next Acceptance Target

The next implementation passes should move specific unchecked workpack rows by
adding real source adapters, signed child-agent artifacts, durable registry
proof, replayable event proof, and physical/manual proof packs. The plan should
not be reported complete until those rows have code, tests, UI screenshots, and
proof artifacts.
