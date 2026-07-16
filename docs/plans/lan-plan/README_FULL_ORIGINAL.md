# LAN Plan

This folder is the single working plan location for V0.9 LAN discovery,
household inventory, pairing, and related UI/UX requirements.

- [V0.9 LAN Discovery 20-Step Plan](v0-9-lan-discovery-20-step-plan.md)
- [V0.9 LAN Discovery Test Blueprint](v0-9-lan-discovery-test-blueprint.md)
- [LAN Discovery UI/UX Requirements Guide](ui-ux-requirements-guide.md)
- [LAN Plan Implementation Checklist](implementation-checklist.md)
- [LAN Source Index](source-index.md)
- [Current LAN Snapshot](current-lan-snapshot.md)
- [Pasted Content Coverage Audit](pasted-content-coverage-audit.md)

The rule remains:

```text
LAN scan discovers. Child agent confirms. Parent assigns.
```

## Where We Are

- `origin/main` already has typed V0.9 LAN pairing/control proof, controller
  lease state, trusted registry state, route recovery, local child-agent
  hardware inventory, Windows neighbor-table inventory, service scan summary,
  passive neighbor/router separation, and portal target filtering.
- Branch `codex/v0-9-lan-signed-discovery-relay-spine` adds the next
  protocol/service proof layer for signed discovery, route custody, trusted
  registry safety, relay/cache unavailable states, and manual-required physical
  proof labels. That proof is not the finished parent-facing LAN workflow.
- This working lane owns the full LAN discovery plan end to end: Rust-owned
  contracts, Rust/service/runtime wiring, portal UI/UX presentation surfaces,
  Activity/Network diagnostics, and proof. It still must not claim production
  household LAN readiness.
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
- Playwright fixture-backed UI tests first, then real-backend UI proof later;
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
- Use the [implementation checklist](implementation-checklist.md), [source
  index](source-index.md), [current snapshot](current-lan-snapshot.md), and
  [coverage audit](pasted-content-coverage-audit.md) before splitting work
  across multiple agents.
- Do not create a second source of truth. Durable state belongs in the canonical
  household device registry; read models and portal rows are derived from it.
- Build or repair Rust-owned shared contracts first, Rust protocol/service
  parity second, portal presentation consumption third.
- Tests must live in real organized test folders and crates. Inline
  source-owned tests, empty placeholder directories, fake coverage, and
  mock-only readiness claims do not close LAN workpacks.
- Keep routers and unsupported LAN devices visible but non-enrollable unless a
  real supported child-agent path exists.
- Each worker report must name the workpack, touched paths, validation, product
  doc updates, and manual-required gaps.

## Workpack Checklist

| Step | Workpack                                                                                     | Target State                                                                                                               |
| ---- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| 01   | [Contract boundary and Effect schemas](workpacks/01-contract-boundary-and-effect-schemas.md) | LAN discovery contracts are typed, schema-backed, branded, and owned in the Rust shared-schema/runtime boundary before portal presentation consumes them. |
| 02   | [Evidence model and device record](workpacks/02-evidence-model-and-device-record.md)         | Every visible device has source evidence, first/last seen, confidence, and no IP-only identity.                            |
| 03   | [Interface detection](workpacks/03-interface-detection.md)                                   | Active LAN interfaces are selected safely, with virtual/VPN/link-local interfaces excluded by default.                     |
| 04   | [Neighbor table ingestion](workpacks/04-neighbor-table-ingestion.md)                         | Windows, Linux, and macOS neighbor output normalizes into one evidence shape.                                              |
| 05   | [Targeted ARP checks](workpacks/05-targeted-arp-checks.md)                                   | Individual hosts can be refreshed without sweeping the whole LAN.                                                          |
| 06   | [Bounded ARP sweep](workpacks/06-bounded-arp-sweep.md)                                       | IPv4 subnet sweep is bounded, testable with controlled packet IO, and never becomes broad scanning.                        |
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

## Progress Reconciliation - 2026-06-02

Checked items below mean there is concrete proof in merged `main` or in branch
`codex/v0-9-lan-signed-discovery-relay-spine`. They do not mark a whole
workpack complete unless every requirement in that workpack is complete.

- [ ] Main has the baseline V0.9 household LAN read model: local child-agent
      inventory, passive neighbor/router separation, scan summary, trusted
      registry input, selected-device readiness, route recovery, and portal
      target filtering.
- [ ] Branch `codex/v0-9-lan-signed-discovery-relay-spine` adds typed
      signed-discovery and relay/cache spine contracts in the Rust-owned shared
      schema/protocol boundary. Historical `parent-domain` and
      `agent-protocol-domain` labels do not imply current TS ownership.
- [ ] The same branch adds Rust protocol/service parity for signed discovery
      rows, rejected discovery states, route custody safety, trusted registry
      route checks, and relay/cache unavailable/manual-required states.
- [ ] `scripts/test/v0-9-lan-signed-discovery-relay-spine.mjs` exists as the
      focused proof harness for this branch and keeps physical household proof,
      real relay/cache, and mobile/store/signing claims manual-required unless
      separately proved.
- [ ] Feature docs for remote/LAN/mobile platforms and family setup/device
      roles were updated on this branch to describe the signed discovery relay
      proof and remaining gaps.
- [ ] Parent-facing Devices/LAN UX now consumes the signed discovery relay
      spine from the B read model for selected-device route custody, signed
      proof, relay/cache unavailable, manual-proof, audit, route requirement,
      unproved-claim, and parent-decision labels.
- [ ] Devices/LAN add-to-portal now sends the existing
      `agent.lan-pairing.add-device.request` command when the selected LAN slot
      has a controllable service route, while unsupported/router slots remain
      visible-only.
- [ ] Parent-facing Devices/LAN UX now has first-class action controls for
      add, route select, rename, trust, ignore, restore, and revoke. Add,
      rename/trust/ignore/restore decisions use the existing
      `agent.lan-pairing.add-device.request` household-decision fields; route
      select/revoke use LAN route commands, and the portal transport now routes
      LAN commands to the selected local-network child target.
- [ ] Activity/Network diagnostics now render service-backed LAN read-model
      state for selected/family target, cloud relay, physical LAN state,
      selected route, signed proof, route safety, relay/cache, manual proof,
      unproved claims, route requirements, audit checks, canonical devices,
      evidence records, parent decisions, sources, and latest evidence.
- [ ] Activity/Network diagnostics now include lightweight LAN scan/evidence
      timeline rows, signed adapter/heartbeat proof state, and policy-target
      history in addition to route/evidence/decision state.
- [ ] Branch `codex/v0-9-lan-source-matrix-plan-completion` adds a typed
      `lanDiscoverySourceMatrix` read-model field across the Rust-owned shared
      schema/protocol boundary, Rust protocol, and Rust service state. The
      matrix carries all 25 workpacks and source rows with implemented,
      partial, manual-required, and not-implemented statuses.
- [ ] `scripts/test/v0-9-lan-source-matrix-plan-completion.mjs` writes
      `output/lan-plan-proof/01-lan-b1-proof-regeneration/01-lan-source-matrix-plan-completion-proof.json`.
      Latest local proof shows 13 implemented workpacks, 11 partial workpacks,
      1 manual-required workpack, 14 implemented source rows, 15 partial source
      rows, 2 manual-required source rows, and 4 not-implemented source rows.
      It also proves weak sources cannot confirm child-agent identity or assign
      a child profile.
- [ ] Live browser proof was captured on the B lane dev ports for Devices/LAN,
      Activity/Network diagnostics, and Network policy target binding:
      `output/playwright/lan-source-matrix-plan-completion/devices-lan-source-matrix.png`,
      `output/playwright/lan-source-matrix-plan-completion/activity-network-source-matrix.png`,
      and
      `output/playwright/lan-source-matrix-plan-completion/policy-network-target-binding.png`.
      `output/playwright/lan-source-matrix-plan-completion/browser-proof.json`
      records positive SVG-text checks for Activity/Network source-matrix rows
      and no browser console/page errors. This proves the current
      service-backed surfaces render; it does not prove two-physical-child
      household readiness.
- [ ] Real production household proof still needs a second installed child
      agent, signed hello/heartbeat artifacts, router/firewall reachability,
      and captured manual validation artifacts.
- [ ] Cloud relay/cache, parent-owned storage, Android/iOS child parity,
      package signing, and store-distribution claims remain unavailable or
      manual-required until separate implementation proof exists.
- [ ] `docs/product-capability-checklist.md` has the matching LAN
      source-matrix proof note. The Remote/LAN/mobile and family setup rows
      point at the source-matrix proof script, proof JSON, and screenshot
      artifacts above while keeping physical household proof manual-required.
