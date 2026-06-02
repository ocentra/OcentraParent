# 08 mDNS And DNS-SD Discovery

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

mDNS and DNS-SD are not yet production-grade enrichment paths. Current LAN proof
must not imply Apple, Chromecast, printer, or Ocentra agent discovery through
mDNS until fixtures and service behavior exist.

## Where We Want To Be

mDNS enriches device records with hostnames, instance names, service types, TXT
fields, A/AAAA records, SRV records, and Ocentra agent service presence.
mDNS can suggest an Ocentra agent but cannot confirm a child device without a
signed hello.

## Requirement Checklist

- [ ] Query `_services._dns-sd._udp.local` and selected service types.
- [ ] Cover `_workstation`, `_ipp`, `_printer`, `_airplay`, `_raop`,
      `_googlecast`, `_companion-link`, `_ocentra-parent`, and
      `_ocentra-agent`.
- [ ] Parse service enumeration, A/AAAA, SRV, and TXT records.
- [ ] Merge mDNS names/services into existing records by strong or safe keys.
- [ ] Sanitize display names before portal exposure.

## Acceptance And Proof

- Fixture-controlled responder tests cover Apple, Android, Chromecast, printer,
  workstation, and Ocentra agent cases.
- Malformed, oversized, invalid UTF-8, and hostile-name fixtures do not panic
  and do not break UI.
- mDNS agent presence never sets `confirmedByAgent` without signed proof.

## Parallel Ownership Notes

Parser work and service query work may split, but both must use the same
evidence source contract.
