# 03 Interface Detection

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `03 Interface Detection`
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

Current proof can report local host network inventory and observed LAN neighbor
data, but production interface selection is not yet a full scanner subsystem.
Virtual, VPN, Docker, Hyper-V, WSL, loopback, and link-local-only boundaries
must remain explicit before active discovery expands.

## Where We Want To Be

The scanner selects active household LAN interfaces safely and explainably. It
collects name, description, index, MAC, IPs, gateways, IPv4 subnet, up/down
state, loopback state, and best-effort Wi-Fi/Ethernet classification. The UI can
show a recommended network and advanced overrides without exposing confusing
virtual interfaces by default.

## Requirement Checklist

- [ ] Normalize platform interface data into one contract shape.
- [ ] Exclude loopback, down, disconnected, VPN, Docker, Hyper-V, WSL, and
      link-local-only interfaces by default.
- [ ] Prefer the interface with the default route when multiple candidates
      exist.
- [ ] Support explicit manual interface selection for advanced users.
- [ ] Record the selected interface id on every evidence item.

## Acceptance And Proof

- Unit tests cover each ignored interface type, Wi-Fi, Ethernet, default-route
  preference, and manual override.
- Scanner proof records selected interface, ignored interfaces, and reason
  codes.
- No packet-sending scanner path can run before interface selection succeeds.

## Parallel Ownership Notes

Implementation can be split by platform adapter, but the normalized contract and
selection rules must stay shared.
