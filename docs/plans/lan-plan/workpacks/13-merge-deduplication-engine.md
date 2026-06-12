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
read model rather than keeping a separate duplicate store. Production merge
logic still needs full evidence scoring, forbidden-merge states, possible
duplicate UI state, and explainability across all discovery sources.

## Where We Want To Be

One physical device becomes one canonical household device record. Strong keys
merge. Weak keys do not. Every automatic merge records score, reasons, source
records, and whether the decision was automatic, blocked, forbidden, or
manual-required.

## Requirement Checklist

- [ ] Strong keys: Ocentra device id, install id, pairing id, MAC on same LAN,
      SSDP UDN, and stable mDNS instance id.
- [ ] Weak keys: IP only, hostname only, vendor only, and device type only.
- [ ] Different Ocentra device ids never auto-merge.
- [ ] Different manually assigned child ids never auto-merge.
- [ ] Merge preserves all evidence and parent decisions.

## Acceptance And Proof

- Unit/property tests cover same agent id, install id, same MAC, SSDP UDN, mDNS
  instance, IP-only, hostname-only, randomized MAC, conflicting agent ids, and
  conflicting assigned child ids.
- DHCP renewal keeps one record; DHCP IP reuse by a different device creates a
  separate record.
- UI/read-model proof shows no duplicate local-agent plus IP-only neighbor row.

## Parallel Ownership Notes

This is the central "superior wins" workpack. Do not create alternate canonical
stores or portal-only merge logic.
