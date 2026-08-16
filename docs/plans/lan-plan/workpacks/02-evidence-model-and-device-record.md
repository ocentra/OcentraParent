# 02 Evidence Model And Device Record

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `02 Evidence Model And Device Record`
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

The current code distinguishes local child-agent evidence, passive LAN neighbor
evidence, router/infrastructure rows, scan summary counts, and canonical
derived household rows. The broader production evidence record still needs a
complete durable shape for all scanner and child-agent observations.

Current A-lane proof now persists canonical household device records inside the
Rust trusted-registry JSON under the same evidence-backed canonical device
shape already used by the LAN read model, instead of inventing a second record
format. Registry merges now preserve evidence `firstSeenAt`, update
`lastSeenAt`, keep source/evidence history attached to the same canonical
device, and allow later scans plus restart recovery to reuse that durable
device-store truth before falling back to weaker scan-history reconstruction.

## Where We Want To Be

Every visible device is backed by evidence. A device record preserves IPs,
MACs, hostnames, services, vendor data, type guess, child-agent identity,
assignment, trust/ignore state, first seen, last seen, last confirmed,
confidence, and source history. IP address is never permanent identity.

## Requirement Checklist

- [x] Model IP, MAC, hostname, service, vendor, protocol, child-agent, and
      parent-manual evidence as separate source-backed entries.
- [x] Preserve first-seen and last-seen timestamps for each evidence value.
- [x] Allow multiple IPs, MACs, names, services, and sources per device.
- [x] Keep manual labels and assignments separate from raw scanner evidence.
- [x] Mark stale, offline, ignored, trusted, revoked, manual-required, and
      unsupported states explicitly.

## Acceptance And Proof

- Focused unit tests prove repeated evidence updates `lastSeen` without moving
  `firstSeen`.
- Focused unit tests prove adding evidence from a second source preserves
  source-backed history instead of collapsing provenance.
- Focused registry and service validation prove router scan-suppression truth
  stays visible while paired/trusted child truth is deduped instead of queued
  twice for scan-hint reuse.

Current proof:

- `cargo test -p ocentra-parent-agent-core trusted_device_registry -- --nocapture`
- `cargo test -p ocentra-parent-agent-service lan_pairing_browser_add_device_state -- --nocapture`
- `cargo lint-architecture crates/agent-core/src/trusted_device_registry.rs crates/agent-core/src/trusted_device_registry/known_household_devices.rs crates/agent-core/tests/unit/trusted_device_registry.rs crates/agent-core/tests/unit/trusted_device_registry_test_fixtures.rs crates/agent-service/src/lan_pairing_browser_add_device_state.rs crates/agent-service/src/lan_pairing_browser_add_device_state/discovery_projection.rs crates/agent-service/tests/unit/lan_pairing_browser_add_device_state.rs`
- The trusted registry persists `knownHouseholdDevices` using the canonical
  household device shape, preserves distinct source-backed evidence rows for the
  same merge key, and reuses durable router/paired-child truth for scan
  suppression without duplicating paired child scan hints.
- Local rerun status on `2026-06-28`: green for the current Rust registry,
  durable known-household restore path, service add-device projection, focused
  unit coverage (`21` targeted `agent-core` tests and `36` targeted
  `agent-service` tests), and scoped Rust architecture validation. Broader
  all-source evidence closure remains an explicit non-claim for this workpack.

## Parallel Ownership Notes

This workpack pairs with workpack 15. Model work can proceed before durable
storage, but storage must persist the same shape rather than introduce a second
record format.
