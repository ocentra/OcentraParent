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

Production household discovery still lacks a bounded active IPv4 sweep. Current
proof must not be stretched into a claim that Ocentra can discover a whole home
LAN until this exists and is validated.

## Where We Want To Be

The scanner can sweep selected local IPv4 subnets with safe caps. `/24` sized
work is the default. Larger subnets require confirmation, rate limits, or a
configured cap.

## Requirement Checklist

- [ ] Introduce a packet IO abstraction so CI does not depend on real packet
      drivers.
- [ ] Exclude network and broadcast addresses.
- [ ] Bound response windows, repeat count, burst size, and max hosts.
- [ ] Deduplicate repeated ARP replies without losing evidence.
- [ ] Record sweep session id, selected interface, response window, and skipped
      host counts.

## Acceptance And Proof

- Integration tests use controlled packet IO for host range selection, no-reply,
  malformed replies, duplicate replies, response-window timeout, and
  network/broadcast exclusion.
- Performance proof keeps `/24` packet build under the target window.
- The default CI suite does not require Npcap, pcap permissions, or a real LAN.

## Parallel Ownership Notes

Packet building, packet IO abstraction, and evidence ingestion can be split only
after the shared sweep contract is stable.
