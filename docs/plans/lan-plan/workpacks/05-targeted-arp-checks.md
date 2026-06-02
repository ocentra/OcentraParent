# 05 Targeted ARP Checks

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current proof can observe neighbor data, but targeted host refresh is not yet a
first-class scanner capability. The system needs a way to refresh known hosts
without a full subnet sweep.

## Where We Want To Be

The scanner can run bounded, host-specific ARP checks for already-known or
parent-selected IPv4 addresses. The result updates evidence and presence state
without becoming broad scanning.

## Requirement Checklist

- [ ] Add a targeted ARP request contract and service command path.
- [ ] Restrict checks to selected LAN interfaces and local subnets.
- [ ] Store response IP, MAC, interface, timestamp, and source as evidence.
- [ ] Treat no-response as stale/presence evidence, not deletion.
- [ ] Rate-limit repeated checks per host and interface.

## Acceptance And Proof

- Tests cover valid response, no response, malformed response, off-subnet
  rejection, ignored interface rejection, and repeated refresh throttling.
- Existing device records update `lastSeen` when the same strong identity
  returns.
- No targeted ARP result can assign a child profile by itself.

## Parallel Ownership Notes

This can be implemented after interface and evidence contracts. Keep it separate
from full sweep work so refresh behavior stays small and testable.
