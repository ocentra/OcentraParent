# 05 Targeted ARP Checks

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `05 Targeted ARP Checks`
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

Current branch truth: targeted ARP refresh now exists as a bounded Rust scanner
capability for already-known or parent-selected IPv4 hosts. It is gated to the
selected LAN interface and local subnet, records response/no-response/malformed
ARP evidence without deleting device truth, rate-limits repeated refreshes per
host/interface, persists scan-plan metadata through the agent-service scan
history, and has focused packet-IO/deduplicated-reply coverage. It remains
weak network evidence only and does not assign child profile or control
authority.

## Where We Want To Be

The scanner can run bounded, host-specific ARP checks for already-known or
parent-selected IPv4 addresses. The result updates evidence and presence state
without becoming broad scanning.

## Requirement Checklist

- [x] Add a targeted ARP request contract and service command path.
- [x] Restrict checks to selected LAN interfaces and local subnets.
- [x] Store response IP, MAC, interface, timestamp, and source as evidence.
- [x] Treat no-response as stale/presence evidence, not deletion.
- [x] Rate-limit repeated checks per host and interface.

## Acceptance And Proof

- Focused Rust tests cover selected-host targeting, valid response, no
  response, malformed response, off-subnet rejection, ignored-interface
  rejection, repeated refresh throttling, packet-IO reply handling, and
  observation-budget/no-false-no-response behavior.
- Existing device records remain unchanged by targeted ARP alone; the refresh
  path records persisted scan metadata evidence but does not assign child
  profile or control authority.
- Proof note: `output/lan-plan-proof/05-targeted-arp-checks/00-validation-note.md`

## Parallel Ownership Notes

This can be implemented after interface and evidence contracts. Keep it separate
from full sweep work so refresh behavior stays small and testable.
