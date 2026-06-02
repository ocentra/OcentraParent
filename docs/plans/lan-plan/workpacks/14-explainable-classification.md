# 14 Explainable Classification

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current proof distinguishes child-agent, passive LAN, and infrastructure/router
categories, but production classification for phones, tablets, laptops,
desktops, printers, TVs, consoles, cameras, NAS, IoT, and unknown devices is
not complete.

## Where We Want To Be

Device type guesses are explainable, confidence-scored, and honest. Ocentra can
say `Likely iPhone/iPad` or `Unknown mobile device`; it cannot say a device
belongs to a child unless parent assignment or child-agent confirmation proves
it.

## Requirement Checklist

- [ ] Classify using vendor, mDNS service type, SSDP type, hostname pattern,
      safe probe hint, child-agent platform, and manual parent label.
- [ ] Include reasons and confidence in read models.
- [ ] Represent unsupported, infrastructure, router, and unknown states.
- [ ] Keep guessed owner/child labels out of scanner-only records.
- [ ] Keep classification changes audit-friendly and test-backed.

## Acceptance And Proof

- Classifier tests cover Apple/AirPlay, Google Cast, IPP printer, port 9100
  printer, SSDP MediaRenderer, router services, signed Ocentra agent, and weak
  unknown evidence.
- UI tests show guessed, unknown, confirmed, router, and unsupported labels
  distinctly.

## Parallel Ownership Notes

Classification can consume merge, vendor, mDNS, SSDP, and service evidence, but
it must not mutate identity or assignment.
