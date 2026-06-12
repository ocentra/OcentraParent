# 15 Accessibility, Responsive, And Keyboard UX

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `15 Accessibility, Responsive, And Keyboard UX`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
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
