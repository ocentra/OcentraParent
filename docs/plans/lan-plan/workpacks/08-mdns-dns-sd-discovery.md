# 08 mDNS And DNS-SD Discovery

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `08 mDNS And DNS-SD Discovery`
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

Current Rust proof covers query construction for `_services._dns-sd._udp.local`
plus the selected service types, packet parsing for PTR, SRV, TXT, A, and AAAA
records, merge of mDNS hints into existing inventory by IP or unique hostname
fallback, hostile-name and TXT sanitization, and shared-contract parent or child
advertisement DTO parsing. The local slice remains hint-only: mDNS can enrich or
suggest Ocentra presence, but it does not confirm child identity without signed
hello proof. Physical packet capture and multi-device household proof are still
open.

## Where We Want To Be

mDNS enriches device records with hostnames, instance names, service types, TXT
fields, A/AAAA records, SRV records, and Ocentra agent service presence.
mDNS can suggest an Ocentra agent but cannot confirm a child device without a
signed hello.

## Requirement Checklist

- [x] Query `_services._dns-sd._udp.local` and selected service types.
- [x] Cover `_workstation`, `_ipp`, `_printer`, `_airplay`, `_raop`,
      `_googlecast`, `_companion-link`, `_ocentra-parent`, and
      `_ocentra-agent`.
- [x] Parse service enumeration, A/AAAA, SRV, and TXT records.
- [x] Merge mDNS names/services into existing records by strong or safe keys.
- [x] Sanitize display names before portal exposure.

## Acceptance And Proof

- Fixture packet tests cover workstation, printer, AirPlay or RAOP, Google
  Cast, companion-link, and Ocentra parent or child advertisement cases.
- Malformed DNS names, hostile display names, control characters, and hostile
  TXT values do not panic and are sanitized before portal exposure.
- mDNS agent presence remains hint-only and never sets agent confirmation
  without signed proof.
- Local rerun commands:
  `cargo test -p ocentra-lan-core mdns_dns_sd -- --nocapture`;
  `cargo test -p ocentra-lan-core read_model -- --nocapture`
- Local rerun status on `2026-06-28`: green for focused `mdns_dns_sd`
  coverage (`13` targeted tests), related `read_model` coverage (`58` library
  tests plus `1` unit test under the selected filter), and scoped Rust
  architecture validation for the mDNS implementation and test surfaces.
- Proof note: `output/lan-plan-proof/08-mdns-dns-sd-discovery/01-local-validation.md`

## Parallel Ownership Notes

Parser work and service query work may split, but both must use the same
evidence source contract.
