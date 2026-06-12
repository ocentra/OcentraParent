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
introduced as bounded enrichment only, not broad port scanning or content
inspection.

## Where We Want To Be

The scanner can probe safe identity ports only after a host is already
discovered. It collects status code, server header, title, redirect location,
and TLS certificate subject. It does not crawl pages.

## Requirement Checklist

- [ ] Probe only discovered hosts on selected interfaces.
- [ ] Limit ports to safe identity hints such as 22, 53, 80, 443, 445, 548,
      631, 8008, 8009, 8080, 8443, and 9100.
- [ ] Enforce timeout, concurrency cap, and no-link-crawl behavior.
- [ ] Sanitize HTTP title, header, redirect, and certificate values.
- [ ] Store probe results as low-authority service evidence.

## Acceptance And Proof

- Local controlled server tests cover closed port, HTTP title, HTTPS certificate,
  redirect, timeout, max concurrency, and no crawling.
- Security tests cover malicious title, path traversal text, invalid UTF-8, and
  oversized responses.
- Probe evidence cannot mark a device as child-owned or confirmed.

## Parallel Ownership Notes

This work should wait for evidence, interface, and discovered-host contracts.
Keep it out of initial ARP/mDNS/SSDP discovery if it creates coupling.
