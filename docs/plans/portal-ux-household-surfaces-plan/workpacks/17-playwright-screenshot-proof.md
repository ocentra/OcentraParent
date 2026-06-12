# 17 Playwright Screenshot Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `17 Playwright Screenshot Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

Portal E2E checks exist. C needs route-specific screenshot and console evidence
for changed UX.

## Where We Want To Be

Every product-critical C change has Playwright proof against the real service,
browser console/page-error checks, and screenshots for manual review.

## Requirement Checklist

- [ ] Cover changed routes with Playwright.
- [ ] Save screenshots under output paths, not source.
- [ ] Check console and page errors.
- [ ] Include desktop and mobile widths when layout changes.
- [ ] Mention artifacts in `DONE` reports.

## Acceptance And Proof

Primary can review C work without guessing what the user would see.

## Parallel Ownership Notes

Screenshots support review; they do not replace service-backed validation.
