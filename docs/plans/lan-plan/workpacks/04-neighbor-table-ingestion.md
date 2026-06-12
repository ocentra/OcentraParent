# 04 Neighbor Table Ingestion

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `04 Neighbor Table Ingestion`
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

Windows neighbor-table inventory is already part of the current proof direction.
Linux and macOS parsing are still future production discovery work, and all
platform outputs need normalized evidence fixtures.

## Where We Want To Be

The scanner reads local OS neighbor state before active scans. Windows uses
`GetIpNetTable2`, Linux starts with `/proc/net/arp` and `ip neigh`, and macOS
can begin with `arp -a` before native adapters are added.

## Requirement Checklist

- [ ] Normalize IP, MAC, interface, neighbor state, timestamp, and source.
- [ ] Skip incomplete, malformed, multicast, or all-zero MAC entries.
- [ ] Preserve duplicate rows as evidence candidates until merge rules decide.
- [ ] Keep IPv4 and IPv6 support represented even when IPv4 lands first.
- [ ] Add fixture files for normal, empty, malformed, incomplete, and duplicate
      neighbor outputs.

## Acceptance And Proof

- Parser tests cover Windows neighbor data, Linux `/proc/net/arp`, Linux
  `ip neigh`, macOS `arp -a`, empty tables, malformed rows, duplicate rows, and
  incomplete MACs.
- Integration tests prove neighbor records enter the same evidence pipeline as
  later active/passive discovery.

## Parallel Ownership Notes

Windows, Linux, and macOS parser work can be split across workers after the
shared normalized evidence contract is stable.
