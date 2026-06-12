# 07 Passive Discovery Listeners

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `07 Passive Discovery Listeners`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current proof separates passive LAN neighbors from child-agent targets, but
production passive listening for ARP, mDNS, SSDP, LLMNR, NetBIOS, and Ocentra
announcements is not complete.

## Where We Want To Be

Passive listeners refresh device presence as the LAN changes. The scanner uses
startup scans, passive background updates, light ARP refresh every 2 to 5
minutes, manual or 30 to 60 minute full scans, and rescans after network-change
triggers.

## Requirement Checklist

- [ ] Listen for ARP, mDNS, SSDP, LLMNR, NetBIOS, and Ocentra agent beacons
      where the platform allows.
- [ ] Use passive evidence to update last-seen and candidate identity only.
- [ ] Trigger rescan on Wi-Fi SSID change, default gateway change, IP change,
      interface up/down, app resume, and child heartbeat loss.
- [ ] Keep passive listeners bounded and stoppable with service lifecycle.
- [ ] Record source and trigger reason for passive updates.

## Acceptance And Proof

- Integration tests use fixture packet/responder sources and explicit timestamps.
- Presence tests prove passive return can restore stale/offline state without
  creating duplicate cards.
- Security tests cover malformed and oversized passive payloads.

## Parallel Ownership Notes

Protocol-specific listener work can split by source, but all listeners must feed
the same evidence and event pipeline.
