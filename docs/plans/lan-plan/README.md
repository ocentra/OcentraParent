# LAN Plan

This folder is the single working plan location for V0.9 LAN discovery,
household inventory, pairing, and related UI/UX requirements.

- [V0.9 LAN Discovery 20-Step Plan](v0-9-lan-discovery-20-step-plan.md)
- [V0.9 LAN Discovery Test Blueprint](v0-9-lan-discovery-test-blueprint.md)
- [LAN Discovery UI/UX Requirements Guide](ui-ux-requirements-guide.md)

The rule remains:

```text
LAN scan discovers. Child agent confirms. Parent assigns.
```

## Where We Are

- `origin/main` already has typed V0.9 LAN pairing/control proof, controller
  lease state, trusted registry state, route recovery, local child-agent
  hardware inventory, Windows neighbor-table inventory, service scan summary,
  passive neighbor/router separation, and portal target filtering.
- This working lane owns the full LAN discovery plan end to end: contracts,
  Rust/service wiring, portal UI/UX surfaces, Activity/Network diagnostics, and
  proof. It still must not claim production household LAN readiness.
- Physical household proof is still manual-required until a real second
  child-agent device, signed LAN hello/heartbeat, router/firewall reachability,
  and generated proof artifacts exist.
- The current product docs make codex-b responsible for the household device,
  LAN, pairing, and inventory spine. One physical device should become one
  identity with role badges, not duplicate rows or separate product truths.

## Where We Want To Be

Ocentra Parent needs a production-credible LAN discovery subsystem that:

- discovers household LAN devices from evidence instead of guesses;
- merges evidence into one canonical household device record;
- shows routers, TVs, phones, printers, and unsupported devices honestly without
  implying they are controllable child-agent targets;
- cryptographically confirms child-agent devices through signed hello and
  heartbeat;
- preserves parent assignment, rename, ignore, revocation, stale, and offline
  decisions across rescans and restarts;
- gives Devices, Policy, Activity, Network, Tracking, AI, and Account surfaces
  one shared device source of truth with visible diagnostic evidence, not just
  test-only proof;
- separates CI-proof, local proof, and real physical/manual proof in every
  status claim.

## Coverage Check Against Pasted Source Plans

The pasted source plan, test blueprint, and UI/UX guide were re-read before
this split. The sub-agent audit found several details that must stay visible in
the workpacks:

- exact fixture layout and filenames;
- property-based tests for merge, evidence, parser robustness, events, and
  presence state;
- proof matrix coverage, not only prose acceptance;
- Playwright mocked-backend tests first, then real-backend UI proof later;
- scan cadence and network-change triggers;
- modular Rust LAN crate/service ownership shape;
- platform-specific Android and iOS child-agent limits;
- coverage targets for core model/security and protocol parsers.
- UI/UX separation between discovered, assigned, confirmed, trusted, ignored,
  stale, offline, LAN-seen, and agent-connected states.
- Evidence-first device cards and details that never show guessed owner or child
  identity.

Those items are captured in the workpacks below and the UI/UX guide. The UI/UX
guide is a product requirement guide, not a claim that the portal already has
all screens or that ChatGPT knew the current C-lane UI implementation.

## Parallel Coordination Rules

- Lock the workpack doc and the exact implementation paths before editing.
- Do not create a second source of truth. Durable state belongs in the canonical
  household device registry; read models and portal rows are derived from it.
- Build TypeScript domain contracts first, Rust protocol/service parity second,
  portal consumption third.
- Keep routers and unsupported LAN devices visible but non-enrollable unless a
  real supported child-agent path exists.
- Each worker report must name the workpack, touched paths, validation, product
  doc updates, and manual-required gaps.

## Workpack Checklist

| Step | Workpack                                                                                     | Target State                                                                                                               |
| ---- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| 01   | [Contract boundary and Effect schemas](workpacks/01-contract-boundary-and-effect-schemas.md) | LAN discovery contracts are typed, schema-backed, branded, and owned in domain packages before runtime code consumes them. |
| 02   | [Evidence model and device record](workpacks/02-evidence-model-and-device-record.md)         | Every visible device has source evidence, first/last seen, confidence, and no IP-only identity.                            |
| 03   | [Interface detection](workpacks/03-interface-detection.md)                                   | Active LAN interfaces are selected safely, with virtual/VPN/link-local interfaces excluded by default.                     |
| 04   | [Neighbor table ingestion](workpacks/04-neighbor-table-ingestion.md)                         | Windows, Linux, and macOS neighbor output normalizes into one evidence shape.                                              |
| 05   | [Targeted ARP checks](workpacks/05-targeted-arp-checks.md)                                   | Individual hosts can be refreshed without sweeping the whole LAN.                                                          |
| 06   | [Bounded ARP sweep](workpacks/06-bounded-arp-sweep.md)                                       | IPv4 subnet sweep is bounded, testable with fake packet IO, and never becomes broad scanning.                              |
| 07   | [Passive discovery listeners](workpacks/07-passive-discovery-listeners.md)                   | Passive ARP/mDNS/SSDP/LLMNR/NetBIOS evidence updates last-seen without blasting the LAN.                                   |
| 08   | [mDNS and DNS-SD discovery](workpacks/08-mdns-dns-sd-discovery.md)                           | Service discovery enriches names, services, and Ocentra agent presence without confirming identity by itself.              |
| 09   | [SSDP and UPnP discovery](workpacks/09-ssdp-upnp-discovery.md)                               | UPnP metadata classifies infrastructure and media devices safely.                                                          |
| 10   | [NetBIOS, LLMNR, and reverse DNS](workpacks/10-netbios-llmnr-reverse-dns.md)                 | Name enrichment improves display but never proves owner or child identity.                                                 |
| 11   | [Light service probing](workpacks/11-light-service-probing.md)                               | Bounded probes collect identity hints only from already-discovered hosts.                                                  |
| 12   | [OUI and vendor lookup](workpacks/12-oui-vendor-lookup.md)                                   | MAC vendor data and randomized-MAC suspicion inform confidence without overclaiming.                                       |
| 13   | [Merge and de-duplication engine](workpacks/13-merge-deduplication-engine.md)                | Strong identity keys merge; weak/IP-only evidence never corrupts identity.                                                 |
| 14   | [Explainable classification](workpacks/14-explainable-classification.md)                     | Device type guesses include reasons, confidence, and honest unknown states.                                                |
| 15   | [Household device store](workpacks/15-household-device-store.md)                             | Durable canonical registry preserves evidence and parent decisions across restart.                                         |
| 16   | [Read models and LAN events](workpacks/16-read-models-and-lan-events.md)                     | Portal and service consumers receive replayable events and derived read models.                                            |
| 17   | [Parent and child mDNS advertisements](workpacks/17-parent-child-mdns-advertisements.md)     | Parent and child announce opaque LAN service presence without leaking sensitive child data.                                |
| 18   | [Signed child hello and heartbeat](workpacks/18-signed-child-hello-heartbeat.md)             | Paired child agents cryptographically confirm identity and presence.                                                       |
| 19   | [Assignment, revocation, and audit](workpacks/19-assignment-revocation-audit.md)             | Parent decisions are durable, audited, route-checked, and cannot be overwritten by weak evidence.                          |
| 20   | [Proof gates, fixtures, and rollout](workpacks/20-proof-gates-fixtures-rollout.md)           | CI, fixture, Playwright, performance, and manual proof gates prevent inflated product claims.                              |

## UI/UX Requirement Links

- Main UI rule: discovered, assigned, and confirmed are separate states.
- Device cards must show evidence source, confidence, assignment status, agent
  status, last seen, and next action.
- First-time setup, scan progress, pairing, unknown devices, trust/ignore,
  evidence details, confidence, duplicate merge, offline/stale, interface
  picker, permissions, alerts, empty states, and error states are covered in
  [the UI/UX guide](ui-ux-requirements-guide.md).
- UI implementation remains service-backed. B owns the full LAN plan wiring,
  including the portal UI/UX surfaces needed for this plan. Other lanes are
  coordination boundaries only; they are not a reason to split this LAN work
  into two truths.
