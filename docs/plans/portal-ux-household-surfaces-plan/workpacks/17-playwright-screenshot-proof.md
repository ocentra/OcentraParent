# 17 Playwright Screenshot Proof

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
