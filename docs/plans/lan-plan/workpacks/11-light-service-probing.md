# 11 Light Service Probing

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `11 Light Service Probing`
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

Service probing is not a current production discovery claim. It must be
introduced as bounded enrichment only, not broad port scanning, generic
platform guessing, or content inspection.

Current branch truth: the Rust LAN scan now keeps service probing on the
already discovered LAN-neighbor path only, suppresses redundant re-probing for
actively paired child devices and known router/infrastructure rows, and reuses
previous-scan hostname/platform continuity from a registry-adjacent JSON sidecar
without upgrading that history into stronger trust. The scan path now also
reuses recent sidecar child-app truth for a short suppression window so repeat
refreshes do not keep probing the same already-observed child host; stale
history falls back to normal probing again. The broader sanitized
HTTP/TLS/banner descriptor catalog remains open.

Focused proof for this slice:

- `cargo test -p ocentra-parent-agent-service lan_pairing_browser_add_device_state`
- `cargo test -p ocentra-parent-agent-protocol lan_pairing_browser_add_device_state`
- `cargo lint-architecture crates/agent-protocol/src/constants/lan_pairing.rs crates/lan-core/src/network_inventory.rs crates/agent-service/src/lan_pairing_browser_add_device_state/physical_lan_scan.rs crates/agent-service/src/lan_pairing_browser_add_device_state/scan_history.rs`

## Where We Want To Be

The scanner can probe safe identity ports only after a host is already
discovered. It collects bounded identity hints such as status code, server
header, title, redirect location, TLS certificate subject, safe banners, and
known descriptor links. It records those hints as low-authority service
evidence and never uses them to confirm a child agent by themselves. It does
not crawl pages.

## Requirement Checklist

- [ ] Probe only discovered hosts on selected interfaces.
- [ ] Limit probes to a curated identity catalog rather than a blind scan.
      Initial TCP candidates may include 21, 22, 23, 25, 53, 80, 81, 82, 88,
      135, 139, 443, 445, 515, 548, 554, 631, 1883, 3000, 3001, 3389, 5000,
      5357, 5555, 5683, 5900, 5985, 5986, 7000, 8001, 8002, 8008, 8009, 8060,
      8080, 8443, 8883, 9000, 9090, 9100, 9443, and 49152-49157.
      Initial UDP candidates may include 53, 67, 68, 123, 137, 138, 161, 1900,
      3702, 5353, 5355, and 5683.
- [ ] Enforce timeout, concurrency cap, and no-link-crawl behavior.
- [ ] Sanitize HTTP title, header, redirect, and certificate values.
- [ ] Keep any WSD or SNMP identity query bounded to already discovered hosts
      and allowed operator settings only.
- [ ] Keep any OS-fingerprint module optional, explainable, and manual-gated.
- [ ] Treat banners, titles, redirects, certificates, and descriptor links as
      weak evidence only.
- [ ] Do not derive platform certainty or installability from MAC vendor or a
      single open port.
- [ ] Store probe results as low-authority service evidence.

## Acceptance And Proof

- Local controlled server tests cover closed port, HTTP title, HTTPS
  certificate, redirect, timeout, max concurrency, safe-port selection, and no
  crawling.
- Security tests cover malicious title, path traversal text, invalid UTF-8,
  oversized responses, and misleading banners that still must not confirm child
  identity.
- Probe evidence cannot mark a device as child-owned or confirmed.
- Probe evidence must remain explainable through read-model reasons and
  confidence.

## Parallel Ownership Notes

This work should wait for evidence, interface, and discovered-host contracts.
Keep it out of initial ARP/mDNS/SSDP discovery if it creates coupling.
