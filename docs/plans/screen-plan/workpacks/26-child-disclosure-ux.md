# 26 Child Disclosure UX

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `26 Child Disclosure UX`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Child-visible/local disclosure, parent-enabled status, calm wording, and no hidden capture are implemented.

## Current State

Disclosure requirement exists in expectations. `screen-child-disclosure-proof`
now defines the child-visible status/copy contract, renders a child-visible
disclosure page for desktop/mobile screenshot inspection, and proves no hidden
capture or raw screenshot display can be represented. Child-agent runtime
deployment/delivery remains open and unclaimed.

## Checklist

- [ ] Define child-visible status.
- [ ] Define local disclosure copy.
- [ ] Define paused/disabled states.
- [ ] Define capture-active state where platform permits.
- [ ] Avoid hidden capture.
- [ ] Add screenshots/proof.

## Proof

- `output/screen-plan-proof/screen-child-disclosure/proof-summary.json`.
- `output/screen-plan-proof/screen-child-disclosure/screenshots/screen-child-disclosure-desktop.png`.
- `output/screen-plan-proof/screen-child-disclosure/screenshots/screen-child-disclosure-mobile.png`.
- Child-agent runtime deployment/delivery remains open.
- Product text reviewed.
