# 06 Bounded ARP Sweep

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `06 Bounded ARP Sweep`
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

Current proof must not be stretched into a claim that Ocentra can discover a
whole home LAN beyond the bounded active IPv4 sweep and passive follow-on path
already validated here.

Current A-lane proof now adds bounded active IPv4 host stimulation before the
passive neighbor-table read used by browser discovery and add-device scans. The
scan path prioritizes unresolved prior-scan devices before unexplored address
space and still suppresses the live default-gateway/router rows immediately.
Stored paired child/app truth now suppresses bounded active refresh only when
the current neighbor table still confirms the same MAC at that IP, so stale
IP-only history cannot hide a reused address. This is still OS-host
stimulation plus passive neighbor evidence, not raw packet-driver ARP IO.

Current A-lane proof also persists scan-plan metadata in the LAN sidecar JSON:
scan session id, selected interface, local IPv4/CIDR, default gateway, bounded
target counts, target timeout, paired-registry truth count, recent
previous-scan child-truth reuse count, durable household-truth reuse count, and
the suppressed active-target list. That same smart-scan slice now also keeps
costly service-identity probes off devices that are already durably known as
paired, child-agent-backed, revoked/ignored, or network-infrastructure rows
through the persisted registry plus previous canonical household truth.

## Where We Want To Be

The scanner can sweep selected local IPv4 subnets with safe caps. `/24` sized
work is the default. Larger subnets require confirmation, rate limits, or a
configured cap.

## Requirement Checklist

- [x] Introduce a packet IO abstraction so CI does not depend on real packet
      drivers.
- [x] Exclude network and broadcast addresses.
- [x] Bound response windows, repeat count, burst size, and max hosts.
- [x] Deduplicate repeated ARP replies without losing evidence.
- [x] Record sweep session id, selected interface, response window, and skipped
      host counts.

## Acceptance And Proof

- Integration tests use controlled packet IO for host range selection, no-reply,
  malformed replies, duplicate replies, response-window timeout, and
  network/broadcast exclusion.
- Performance proof keeps `/24` packet build under the target window.
- The default CI suite does not require Npcap, pcap permissions, or a real LAN.

Current proof:

- Focused Rust proof:
  `cargo test -p ocentra-lan-core network_inventory`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service scan_history`
- Focused Rust proof:
  `cargo test -p ocentra-parent-agent-service physical_lan_scan`
- Smart scan planning now keeps router/gateway truth out of the active refresh
  target list immediately, requires a live current-MAC confirmation before a
  stored child/paired IP leaves the bounded refresh target list, still
  prioritizes unresolved prior-scan devices first, and reuses durable truth as
  passive identity-hint input for matching neighbors.
- The sidecar snapshot now preserves paired-registry, recent previous-scan
  child-truth, and durable household-truth suppression counts, plus the
  active-target suppression list.
- Costly service-identity probes now also skip durable paired, child-agent,
  ignored/revoked, and router truth reconstructed from the persisted registry
  plus previous canonical household state, while passive neighbor inventory
  still runs.
- Focused packet-IO proof now covers controlled probe execution, duplicate-reply
  collapse, no-attempt handling, and exhausted-observation-budget behavior
  without requiring packet drivers in CI.
- Focused proof note:
  `output/lan-plan-proof/06-bounded-arp-sweep/01-ip-reuse-suppression-fix.md`

## Parallel Ownership Notes

Packet building, packet IO abstraction, and evidence ingestion can be split only
after the shared sweep contract is stable.
