# 02 Evidence Model And Device Record

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

The current code distinguishes local child-agent evidence, passive LAN neighbor
evidence, router/infrastructure rows, scan summary counts, and canonical
derived household rows. The broader production evidence record still needs a
complete durable shape for all scanner and child-agent observations.

## Where We Want To Be

Every visible device is backed by evidence. A device record preserves IPs,
MACs, hostnames, services, vendor data, type guess, child-agent identity,
assignment, trust/ignore state, first seen, last seen, last confirmed,
confidence, and source history. IP address is never permanent identity.

## Requirement Checklist

- [ ] Model IP, MAC, hostname, service, vendor, protocol, child-agent, and
      parent-manual evidence as separate source-backed entries.
- [ ] Preserve first-seen and last-seen timestamps for each evidence value.
- [ ] Allow multiple IPs, MACs, names, services, and sources per device.
- [ ] Keep manual labels and assignments separate from raw scanner evidence.
- [ ] Mark stale, offline, ignored, trusted, revoked, manual-required, and
      unsupported states explicitly.

## Acceptance And Proof

- Unit tests prove repeated evidence updates `lastSeen` without moving
  `firstSeen`.
- Property tests prove adding evidence never drops source history.
- Contract/read-model tests reject visible device rows with empty evidence
  summaries.

## Parallel Ownership Notes

This workpack pairs with workpack 15. Model work can proceed before durable
storage, but storage must persist the same shape rather than introduce a second
record format.
