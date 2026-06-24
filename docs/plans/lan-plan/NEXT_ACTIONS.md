# LAN Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from open workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Open that workpack and exact checklist rows only.
4. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by unchecked boxes

- [16 Read Models And LAN Events](workpacks/16-read-models-and-lan-events.md): 6 open of 13 boxes.
- [01 Contract Boundary And Effect Schemas](workpacks/01-contract-boundary-and-effect-schemas.md): 5 open of 5 boxes.
- [02 Evidence Model And Device Record](workpacks/02-evidence-model-and-device-record.md): 5 open of 5 boxes.
- [04 Neighbor Table Ingestion](workpacks/04-neighbor-table-ingestion.md): 3 open of 5 boxes.
- [05 Targeted ARP Checks](workpacks/05-targeted-arp-checks.md): 5 open of 5 boxes.
- [06 Bounded ARP Sweep](workpacks/06-bounded-arp-sweep.md): 2 open of 5 boxes.
- [07 Passive Discovery Listeners](workpacks/07-passive-discovery-listeners.md): 5 open of 5 boxes.
- [08 mDNS And DNS-SD Discovery](workpacks/08-mdns-dns-sd-discovery.md): 5 open of 5 boxes.
- [09 SSDP And UPnP Discovery](workpacks/09-ssdp-upnp-discovery.md): 5 open of 5 boxes.
- [10 NetBIOS, LLMNR, And Reverse DNS](workpacks/10-netbios-llmnr-reverse-dns.md): 5 open of 5 boxes.
- [11 Light Service Probing](workpacks/11-light-service-probing.md): 5 open of 5 boxes.
- [13 Merge And De-Duplication Engine](workpacks/13-merge-deduplication-engine.md): 5 open of 5 boxes.
- [14 Explainable Classification](workpacks/14-explainable-classification.md): 5 open of 5 boxes.
- [17 Parent And Child mDNS Advertisements](workpacks/17-parent-child-mdns-advertisements.md): 5 open of 5 boxes.
- [18 Signed Child Hello And Heartbeat](workpacks/18-signed-child-hello-heartbeat.md): 5 open of 8 boxes.
- [20 Proof Gates, Fixtures, And Rollout](workpacks/20-proof-gates-fixtures-rollout.md): 5 open of 11 boxes.
- [15 Household Device Store](workpacks/15-household-device-store.md): 3 open of 3 boxes.
- [03 Interface Detection](workpacks/03-interface-detection.md): 2 open of 5 boxes.
- [19 Assignment, Revocation, And Audit](workpacks/19-assignment-revocation-audit.md): 2 open of 9 boxes.

## Active note

- `16 Read Models And LAN Events` now has a real parent-Rust inventory-backed `LanBrowserAddDeviceReadModel` for the product host bridge, forwards Devices-route scan actions into `agent-service`, auto-scans the real Tauri Devices route into visible LAN inventory, and emits typed Tauri host-subscription updates into the portal shell without a UI WebSocket. Remaining `16` work is the broader service/runtime parity path, canonical replay/event proof, explicit relay/cache/source-matrix diagnostics proof, and richer UI/manual proof.
- The latest `03` slice lifts gateway, DNS server, DHCP server, broadcast-address, and IPv6-prefix capture into the shared Rust local-interface shape and persists that interface map into `LanDiscoveryScanPlan` plus scan-history metadata under focused `lan-core` and `agent-service` proof. Remaining `03` work is manual interface override, ignored-interface reason-code proof, and selected-interface propagation on every evidence item.
- The latest `04` slice makes Windows neighbor collection request both IPv4 and IPv6 rows, proves IPv6 neighbor parsing on Windows/Linux, and proves malformed/incomplete/duplicate-row normalization behavior in shared Rust tests. Remaining `04` work is timestamped evidence shape, broader fixture corpus, and macOS/manual live proof.
- The latest `06`/`15` slice keeps passive LAN inventory honest while suppressing costly service-identity probes for durable paired, child-agent-backed, ignored/revoked, and router truth reconstructed from registry JSON plus previous canonical household state. Bounded active refresh now suppresses a stored child/paired IP only when live neighbor state still confirms the same MAC, so stale IP-only truth cannot hide a reused address. Durable truth still feeds passive identity hints for matching neighbors. This is not closure for Workpack `11`; safe-port probing and broader bounded service-proof remain open.
- The latest `02`/`15` slice now persists canonical `knownHouseholdDevices` into trusted-registry JSON, merges evidence `firstSeenAt`/`lastSeenAt` across updates, restores that known-device store into the add-device read model as stale restart truth under explicit runtime proof, and uses it for later scan suppression before falling back to scan-history reconstruction.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
