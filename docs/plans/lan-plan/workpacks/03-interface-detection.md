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

Current implemented subset: `lan-core` now owns local host identity selection
for both Windows and Linux instead of assuming a Windows-only shape. Windows
local identity selection prefers an interface with a default gateway and filters
virtual/loopback adapters in Rust. Linux local identity selection now uses
`ip -j route show default` plus `ip -j addr show up`, prefers the default-route
interface when present, skips Docker/WSL/Tailscale/tunnel-style interface names
by default, and rejects link-local-only IPv4 identities. The local-agent device
path can therefore attach a more honest Linux IP/MAC/interface identity even
before the broader scanner interface-selection subsystem is complete. The latest
W03 proof extends that shared Rust identity shape with DNS servers, DHCP server
truth when exposed by the host, computed broadcast address, and non-link-local
IPv6 prefixes, then persists those fields into `LanDiscoveryScanPlan` and the
scan-history sidecar used by later LAN proof paths.

Focused proof for this slice:

- `cargo test -p ocentra-lan-core network_inventory_hardware`
- `cargo test -p ocentra-lan-core network_inventory`
- `cargo test -p ocentra-parent-agent-service scan_history`
- `cargo lint-architecture crates/agent-protocol/src/constants/lan_pairing.rs crates/lan-core/src/network_inventory_hardware.rs crates/lan-core/src/network_inventory.rs crates/agent-service/src/lan_pairing_browser_add_device_state/scan_history.rs`
- Proof note: `output/lan-plan-proof/03-interface-detection/00-interface-map-proof.md`

## Where We Want To Be

The scanner selects active household LAN interfaces safely and explainably. It
collects name, description, index, MAC, IPs, gateways, IPv4 subnet, up/down
state, loopback state, and best-effort Wi-Fi/Ethernet classification. The UI can
show a recommended network and advanced overrides without exposing confusing
virtual interfaces by default.

## Requirement Checklist

- [x] Normalize platform interface data into one contract shape.
- [x] Exclude loopback, down, disconnected, VPN, Docker, Hyper-V, WSL, and
      link-local-only interfaces by default.
- [x] Prefer the interface with the default route when multiple candidates
      exist.
- [x] Support explicit manual interface selection for advanced users.
- [x] Record the selected interface id on every evidence item.

## Acceptance And Proof

- Unit tests now cover ignored virtual/link-local candidates, Wi-Fi/Ethernet
  default-route preference, Windows DNS/DHCP/IPv6 capture, Linux DNS/IPv6
  capture, computed broadcast address, and manual interface selection.
- Scanner proof now records selected interface plus gateway/DNS/DHCP/broadcast
  and IPv6-prefix metadata in the scan-plan sidecar; ignored-interface reason
  codes and selected-interface attribution through discovery evidence are
  locally proven.
- No packet-sending scanner path can run before interface selection succeeds.

## Parallel Ownership Notes

Implementation can be split by platform adapter, but the normalized contract and
selection rules must stay shared.
