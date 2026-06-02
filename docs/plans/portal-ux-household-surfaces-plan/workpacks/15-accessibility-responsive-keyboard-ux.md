# 15 Accessibility, Responsive, And Keyboard UX

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

Manual browser review has found visual and interaction issues before. C needs a
standing quality gate for responsive and accessible behavior.

## Where We Want To Be

Core routes work with keyboard navigation, stable focus, readable text, and no
layout overlap at common desktop and mobile widths.

## Requirement Checklist

- [ ] Check keyboard focus and visible active states.
- [ ] Check desktop and mobile widths.
- [ ] Avoid text clipping and overlap.
- [ ] Keep button/icon dimensions stable.
- [ ] Use appropriate controls for workflow semantics.

## Acceptance And Proof

Screenshots and Playwright checks support manual visual review.

Current assistant chat proof:

- `apps/portal/e2e/assistant-chat-ui-proof.spec.ts` verifies keyboard
  collapse/expand, keyboard copy, and browser-console cleanliness for the MIA
  chat bubble on `/#/assistant`.
- Desktop and mobile screenshots are captured under
  `output/playwright/assistant-chat-ui-proof/` for visual review.

## Parallel Ownership Notes

This workpack is owned by C/user visual direction.
