# 09 SSDP And UPnP Discovery

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `09 SSDP And UPnP Discovery`
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

Routers and passive infrastructure rows are visible in current proof, but SSDP
and UPnP enrichment for routers, TVs, consoles, printers, and media devices is
not production complete.

## Where We Want To Be

SSDP discovery sends bounded `M-SEARCH` queries, parses responses, fetches safe
device description XML where allowed, and stores friendly name, manufacturer,
model, device type, UDN/UUID, and description URL as evidence.

## Requirement Checklist

- [ ] Bound M-SEARCH timing, MX, response handling, and retry behavior.
- [ ] Parse LOCATION, ST, USN, UDN/UUID, and device type safely.
- [ ] Restrict description fetches to allowed LAN/private targets.
- [ ] Treat routers and infrastructure as visible but non-enrollable.
- [ ] Handle missing LOCATION, bad XML, timeout, and malformed response cases.

## Acceptance And Proof

- Controlled UDP and HTTP responder tests cover TV, router, console, printer, missing
  LOCATION, bad XML, external URL, and timeout fixtures.
- Parser robustness tests reject recursive XML and oversized descriptions.
- Classification uses SSDP evidence with reasons and confidence.

## Parallel Ownership Notes

UDP search, XML parsing, and classification can be separate tasks, but all
external fetch rules must share the same security boundary.
