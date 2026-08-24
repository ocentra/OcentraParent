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

Current proof now covers passive ARP weak hints via OS neighbor collectors,
passive DHCP, mDNS, SSDP, WS-Discovery, LLMNR, and NetBIOS packet ingestion,
plus Ocentra beacon observations recorded through the signed child
hello or heartbeat path. Allowed SNMP response payloads now also feed the same
bounded passive history path through both lan-core and agent-service runtime
tests, and the fixture-backed native UDP/DHCP packet path now preserves
explicit passive `observed_at` timestamps instead of inventing test-time wall
clock values. A real long-running DHCP listener and broader packet/platform
proof are still open.

## Where We Want To Be

Passive listeners refresh device presence as the LAN changes. The scanner uses
startup scans, passive background updates, light ARP refresh every 2 to 5
minutes, manual or 30 to 60 minute full scans, and rescans after network-change
triggers.

## Requirement Checklist

- [x] Listen for ARP, mDNS, SSDP, LLMNR, NetBIOS, and Ocentra agent beacons
      where the platform allows. Current local proof also covers allowed SNMP
      response history bridging on the existing passive path.
- [x] Use passive evidence to update last-seen and candidate identity only.
- [x] Trigger rescan on Wi-Fi SSID change, default gateway change, IP change,
      interface up/down, app resume, and child heartbeat loss.
- [x] Keep passive listeners bounded and stoppable with service lifecycle.
- [x] Record source and trigger reason for passive updates.

## Acceptance And Proof

- Integration tests use fixture packet/responder sources and explicit timestamps.
- Presence tests prove passive return can restore stale/offline state without
  creating duplicate cards.
- Security tests cover malformed and oversized passive payloads.
- Local rerun commands:
  `cargo test -p ocentra-lan-core passive_discovery -- --nocapture`;
  `cargo test -p ocentra-lan-core read_model -- --nocapture`;
  `cargo test -p ocentra-parent-agent-service scan_history -- --nocapture`;
  `cargo lint-architecture crates/lan-core/src/network_inventory/passive_discovery.rs crates/lan-core/tests/unit/network_inventory_passive_discovery.rs`
- Proof note: `output/lan-plan-proof/07-passive-discovery-listeners/01-local-validation.md`

## Expected Test Topology (currently absent)

The current source packet has no retained tests for the repaired boundaries.
Before this workpack can be called `DONE`, add and run the following scoped
test roots against real runtime paths (without test doubles):

- `crates/agent-core/tests/unit/trusted_device_registry.rs` — wire pairing
  proofs remain contract-only; revoked pairing ids cannot be resurrected;
  present malformed optional state fails closed; rejected mutations leave no
  partial route, decision, lease, or replay state; and an accepted intent is
  rejected after restart/replay.
- `crates/agent-service/tests/unit/lan_pairing_runtime_state.rs` — a missing
  registry initializes exactly once, malformed present state stays unavailable,
  controller lease effects and replay acceptance persist atomically, listener
  or reconciliation spawn failure persists unavailable health, and accepted
  intent ids survive reload.
- `crates/agent-service/tests/unit/lan_pairing_browser_runtime.rs` — scan
  cancellation, supersession, listener shutdown, reconciliation drop, and
  blocking-worker cancellation are bounded with owned joins and no
  mutex-held or detached join.
- `crates/agent-service/tests/integration/lan_pairing_runtime.rs` — durable
  restart and atomic intent-consumption behavior through the service route.
- `crates/lan-core/tests/unit/network_inventory_ssdp_upnp.rs` — cancellation,
  response/record bounds, description timeout, and oversized-response handling.
- `crates/lan-core/tests/unit/network_inventory_command.rs` — timeout and
  cancellation concurrently drains bounded output, terminates and reaps the
  owned process tree on the target OS, rejects overflow and descendant-held
  pipes, and resolves only allowlisted executables under canonical protected
  OS roots without ambient `PATH` or current-directory lookup.

These paths are expected-test routing, not completion evidence. The existing
acceptance checkboxes and proof note remain open until the tests, retained
proof, and whole-plan validation are produced.

## Parallel Ownership Notes

Protocol-specific listener work can split by source, but all listeners must feed
the same evidence and event pipeline.
