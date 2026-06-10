# 18 Parent Mobile Shell Readiness

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

Parent mobile is scaffold/proof-first. The portal should prepare reusable
patterns without claiming mobile child-agent parity.
Installable parent mobile Android/iOS package-preview targets now exist as
separate scaffold apps from child-agent mobile previews.

## Where We Want To Be

Source-state, selected-device, policy, activity, approval, and report patterns
can map to parent mobile shells later.

## Requirement Checklist

- [ ] Keep layouts responsive and touch-friendly where practical.
- [ ] Preserve source/authority labels on small widths.
- [ ] Avoid desktop-only assumptions in reusable components.
- [ ] Label mobile runtime gaps honestly.
- [x] Track parent mobile Android/iOS package-preview CI targets separately
      from child-agent mobile previews.
- [ ] Test narrow widths for key workflows.

## Acceptance And Proof

Responsive portal proof supports future parent mobile UX without claiming child
mobile support.

## Parallel Ownership Notes

D owns parent desktop/package runtime. Mobile runtime proof remains separate.
