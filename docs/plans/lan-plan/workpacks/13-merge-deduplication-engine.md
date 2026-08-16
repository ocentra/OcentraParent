# 13 Merge And De-Duplication Engine

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `13 Merge And De-Duplication Engine`
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

The working lane now derives canonical household rows from the LAN add-device
read model rather than keeping a separate duplicate store. The current Rust
merge path already emits explicit dedupe explainability through
`dedupe-decision=<state> score=<value> reasons=<labels>` notes attached to the
matching canonical evidence records, and the focused LAN-core tests cover the
strong-key merge, weak-key non-merge, forbidden-merge, evidence-preservation,
and read-model duplicate-proof cases for this workpack.

## Where We Want To Be

One physical device becomes one canonical household device record. Strong keys
merge. Weak keys do not. Every automatic merge records score, reasons, source
records, and whether the decision was automatic, blocked, forbidden, or
manual-required.

## Requirement Checklist

- [x] Strong keys: Ocentra device id, install id, pairing id, MAC on same LAN,
      SSDP UDN, and stable mDNS instance id.
- [x] Weak keys: IP only, hostname only, vendor only, and device type only.
- [x] Different Ocentra device ids never auto-merge.
- [x] Different manually assigned child ids never auto-merge.
- [x] Merge preserves all evidence and parent decisions.

## Acceptance And Proof

- Unit/property tests cover same agent id, install id, same MAC, SSDP UDN, mDNS
  instance, IP-only, hostname-only, randomized MAC, conflicting agent ids, and
  conflicting assigned child ids.
- DHCP renewal keeps one record; DHCP IP reuse by a different device creates a
  separate record.
- UI/read-model proof shows no duplicate local-agent plus IP-only neighbor row.

## Follow-Up Status

- 2026-06-28 blocker fix: `LanPairingDeviceRef` now carries explicit
  `install_id`, `LanBrowserAddDeviceDiscoveryDevice` now carries explicit
  `pairing_id`, and the canonical household evidence spine records both as
  strong merge evidence instead of relying on `agent_peer_id`.
- 2026-06-28 truth-sync: the canonical household merge path already emits the
  explicit score/reasons/decision-state output this workpack requires through
  `dedupe-decision=<automatic|manual-required|forbidden> score=<value> reasons=<labels>`
  notes on the matching evidence rows, and the focused LAN-core read-model
  tests already assert those outputs across strong-key, weak-key, forbidden,
  and evidence-preservation cases.
- Focused local proof is green for the Rust-owned W13 slice under the exact
  scoped rerun recorded in
  `output/lan-plan-proof/13-merge-deduplication-engine/01-local-validation.md`.
- No remaining local Rust/test blocker is left inside the W13-owned merge and
  dedupe slice.

## Parallel Ownership Notes

This is the central "superior wins" workpack. Do not create alternate canonical
stores or portal-only merge logic.
