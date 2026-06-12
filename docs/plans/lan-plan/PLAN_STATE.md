# LAN Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `lan-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the single working plan location for V0.9 LAN discovery, household inventory, pairing, and related UI/UX requirements.

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Open only the assigned workpack.
5. Use `CHECKLIST_INDEX.md` for exact checklist sections.
6. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- Snapshot: [current-lan-snapshot.md](current-lan-snapshot.md)

## What is already present / proved

- LAN pairing and add-device read-model contracts;
- LAN device parent action contracts;
- LAN production household proof rows;
- signed discovery/relay spine rows;
- LAN discovery source-matrix contracts with all 20 workpack ids;
- source authority, proof state, runtime path, UI surface, and status rows.
- protocol-facing add-device state contracts;
- LAN source-matrix contracts mirrored from parent-domain;
- signed discovery/relay spine contracts;
- challenge/runtime contracts for LAN pairing.

## Open gaps / missing product runtime

- Physical household proof still needs a second installed child agent, router or firewall reachability proof, and generated manual proof artifacts.
- Signed child-agent hello and heartbeat rows exist but are artifact-gated.
- Parent and child mDNS advertisements are not implemented.
- Passive packet listeners are not implemented.
- Targeted ARP refresh, bounded ARP sweep, and light service probing are not implemented.
- mDNS, DNS-SD, SSDP, UPnP, NetBIOS, LLMNR, reverse DNS, service probing, and OUI/vendor rows are represented as weak/manual-required evidence, not identity proof.
- Full canonical household device store and restart proof is partial.
- Full replayable LAN event stream proof is partial.
- Optional relay/cache and parent-owned storage routes remain unavailable or manual-required.
- Android/iOS child-agent parity, signing, store distribution, and mobile entitlements remain manual-required or not implemented.
- Production first-run setup UX is not complete.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checkbox rows detected: 74 total, 9 checked, 65 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks indexed: 20.
- Workpacks with open checkboxes: 20.
- Workpacks with all detected boxes checked: 0.
- Workpacks with no checkbox status: 0.

### Active/open workpacks

- [16 Read Models And LAN Events](workpacks/16-read-models-and-lan-events.md) - 7/13 checked, 6 open.
- [01 Contract Boundary And Effect Schemas](workpacks/01-contract-boundary-and-effect-schemas.md) - 0/5 checked, 5 open.
- [02 Evidence Model And Device Record](workpacks/02-evidence-model-and-device-record.md) - 0/5 checked, 5 open.
- [03 Interface Detection](workpacks/03-interface-detection.md) - 0/5 checked, 5 open.
- [04 Neighbor Table Ingestion](workpacks/04-neighbor-table-ingestion.md) - 0/5 checked, 5 open.
- [05 Targeted ARP Checks](workpacks/05-targeted-arp-checks.md) - 0/5 checked, 5 open.
- [06 Bounded ARP Sweep](workpacks/06-bounded-arp-sweep.md) - 0/5 checked, 5 open.
- [07 Passive Discovery Listeners](workpacks/07-passive-discovery-listeners.md) - 0/5 checked, 5 open.
- [08 mDNS And DNS-SD Discovery](workpacks/08-mdns-dns-sd-discovery.md) - 0/5 checked, 5 open.
- [09 SSDP And UPnP Discovery](workpacks/09-ssdp-upnp-discovery.md) - 0/5 checked, 5 open.
- [10 NetBIOS, LLMNR, And Reverse DNS](workpacks/10-netbios-llmnr-reverse-dns.md) - 0/5 checked, 5 open.
- [11 Light Service Probing](workpacks/11-light-service-probing.md) - 0/5 checked, 5 open.
- [12 OUI And Vendor Lookup](workpacks/12-oui-vendor-lookup.md) - 0/5 checked, 5 open.
- [13 Merge And De-Duplication Engine](workpacks/13-merge-deduplication-engine.md) - 0/5 checked, 5 open.
- [14 Explainable Classification](workpacks/14-explainable-classification.md) - 0/5 checked, 5 open.
- [17 Parent And Child mDNS Advertisements](workpacks/17-parent-child-mdns-advertisements.md) - 0/5 checked, 5 open.
- [18 Signed Child Hello And Heartbeat](workpacks/18-signed-child-hello-heartbeat.md) - 3/8 checked, 5 open.
- [20 Proof Gates, Fixtures, And Rollout](workpacks/20-proof-gates-fixtures-rollout.md) - 6/11 checked, 5 open.
- [15 Household Device Store](workpacks/15-household-device-store.md) - 0/3 checked, 3 open.
- [19 Assignment, Revocation, And Audit](workpacks/19-assignment-revocation-audit.md) - 7/9 checked, 2 open.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/lan-plan/.
- Required proof manifest names:
  - docs/proof/lan-plan/slice-01-\*.md
  - docs/proof/lan-plan/slice-02-\*.md
  - docs/proof/lan-plan/slice-03-\*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
