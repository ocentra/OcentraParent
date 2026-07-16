# 14 Explainable Classification

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `14 Explainable Classification`
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

- [x] Classify using vendor, mDNS service type, SSDP type, hostname pattern,
      safe probe hint, child-agent platform, and manual parent label.
- [x] Include reasons and confidence in read models.
- [x] Represent unsupported, infrastructure, router, and unknown states.
- [x] Keep guessed owner/child labels out of scanner-only records.
- [x] Keep classification changes audit-friendly and test-backed.

## Acceptance And Proof

- Classifier tests cover Apple/AirPlay, Google Cast, IPP printer, port 9100
  printer, SSDP MediaRenderer, router services, signed Ocentra agent, and weak
  unknown evidence.
- UI tests show guessed, unknown, confirmed, router, and unsupported labels
  distinctly.

Current local proof: `output/lan-plan-proof/14-explainable-classification/01-local-validation.md`

Current local closure for this slice:

- The Rust classifier already weights hostname, label, platform, hardware, and
  safe service-probe hints into explicit canonical classifications instead of
  guessing child ownership from scanner-only evidence.
- Focused `lan-core` tests already cover router, unknown, unsupported, printer,
  television, console, camera, NAS, IoT, phone, tablet, laptop, desktop, and
  signed-child-agent cases plus conflicting-hint safety behavior.
- The thin `agent-service` household-device spine remains a Rust-backed
  projection and keeps router rows visible but non-enrollable.
- Current scoped validation in this lane reran the live Rust read-model and
  household-device spine surfaces without requiring source changes for W14.
- Focused portal unit proof reran green for guessed, unknown, confirmed,
  router, infrastructure, revoked, ignored, and manual-required label
  rendering from the real Rust LAN read model, and the refreshed
  source-matrix/browser proof kept the real `/devices` snapshot path honest.

This workpack is locally complete for the current LAN scope. Broader
install-eligibility or physical/manual readiness claims remain owned by later
workpacks instead of classification.

## Parallel Ownership Notes

Classification can consume merge, vendor, mDNS, SSDP, and service evidence, but
it must not mutate identity or assignment.
