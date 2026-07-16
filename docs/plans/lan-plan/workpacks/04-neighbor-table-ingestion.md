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
Linux `/proc/net/arp` and `ip neigh` ingestion are now wired into the shared
`lan-core` inventory path, and their evidence sources survive through the
browser add-device read model instead of collapsing to a fake Windows-only
origin. Windows now explicitly collects both IPv4 and IPv6 neighbor rows from
the local neighbor table, the shared Rust parser accepts IPv6 rows on both
Windows and Linux, and duplicate/malformed neighbor-row behavior has focused
unit proof. macOS `arp -a` parsing is now covered through the shared Rust path,
all three platform-normalization paths have broader normal/empty/incomplete/
malformed/duplicate fixture proof where applicable, and normalized neighbor
evidence now carries a per-row observation timestamp through the downstream
discovered-device/read-model seam.

As of `2026-06-28`, this workpack is locally code-complete in the owned Rust
surface. The honest non-claim is live macOS/manual platform proof, not a
remaining local neighbor-ingestion implementation gap.

### Current implemented subset

- Windows neighbor rows still feed the existing normalized inventory path.
- Windows neighbor collection now requests both IPv4 and IPv6 rows instead of
  hard-filtering the product path to IPv4 only.
- Linux neighbor rows now ingest from both `/proc/net/arp` and `ip neigh`,
  merge by MAC, keep private-address filtering, and preserve source truth as
  `windows-neighbor`, `linux-proc-net-arp`, or `linux-ip-neigh`.
- Linux and Windows parser proof now covers IPv6 neighbor rows, incomplete
  rows, malformed/all-zero MAC rejection, and duplicate-row merge behavior
  that preserves source truth while preferring a private IPv4 identity when
  both IPv6 and IPv4 rows describe the same MAC.
- Linux neighbor merge now preserves the earliest normalized `observed_at`
  timestamp when multiple neighbor rows collapse into one MAC-backed device.
- macOS `arp -a` parsing now flows through the normalized neighbor-observation
  path with fixture proof for normal, empty, incomplete, malformed, and
  duplicate rows.
- Passive neighbor collection still runs so the read model can report current
  reachability, but routers and already-paired/trusted devices skip the heavier
  service-identity probe path.
- Previous trusted/known device truth continues to win over weaker historical
  hints, and previous-scan continuity remains a weak hint instead of becoming
  canonical identity truth.
- Canonical LAN evidence records and scan summaries now surface the real
  neighbor source rather than branding every passive record as Windows.
- Discovered-device projection now preserves a normalized neighbor row's
  `observed_at` value instead of always rewriting it to the scan generation
  time.

Validated in this packet:

- `cargo test -p ocentra-lan-core network_inventory -- --nocapture`
- `cargo test -p ocentra-parent-agent-service lan_pairing_browser_add_device_state -- --nocapture`
- `cargo lint-architecture crates/lan-core/src/network_inventory.rs crates/lan-core/src/network_inventory/windows_neighbors.rs crates/lan-core/src/network_inventory/linux_neighbors.rs crates/lan-core/src/network_inventory/macos_neighbors.rs`
- Proof note: `output/lan-plan-proof/04-neighbor-table-ingestion/00-neighbor-normalization-proof.md`

## Where We Want To Be

The scanner reads local OS neighbor state before active scans. Windows uses
`GetIpNetTable2`, Linux starts with `/proc/net/arp` and `ip neigh`, and macOS
can begin with `arp -a` before native adapters are added.

## Requirement Checklist

- [x] Normalize IP, MAC, interface, neighbor state, timestamp, and source.
- [x] Skip incomplete, malformed, multicast, or all-zero MAC entries.
- [x] Preserve duplicate rows as evidence candidates until merge rules decide.
- [x] Keep IPv4 and IPv6 support represented even when IPv4 lands first.
- [x] Add fixture files for normal, empty, malformed, incomplete, and duplicate
      neighbor outputs.

## Acceptance And Proof

- Parser tests now cover Windows neighbor data, Linux `/proc/net/arp`, Linux
  `ip neigh`, and macOS `arp -a`, including IPv6 neighbor rows,
  malformed/all-zero MAC rejection, incomplete rows, duplicate-row merge
  behavior, and normalized `observed_at` timestamps.
- Service-backed LAN/read-model tests now prove the normalized neighbor path
  still feeds the browser add-device state and preserves neighbor-derived
  discovery timestamps instead of breaking downstream state.
- Local WP04 closure does not depend on a remaining Rust code patch in this
  surface; the remaining non-claim is manual/live platform proof only.

## Parallel Ownership Notes

Windows, Linux, and macOS parser work can be split across workers after the
shared normalized evidence contract is stable.
