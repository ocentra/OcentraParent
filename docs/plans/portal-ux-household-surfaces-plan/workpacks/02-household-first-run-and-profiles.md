# 02 Household First-Run And Profiles

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

First-run setup is not product-complete. Parents still need an understandable
path for household, child profile, device, and role state.

## Where We Want To Be

A nontechnical parent can see setup progress, what is missing, and which states
are live, unavailable, or manual-required.

## Requirement Checklist

- [ ] Show household and child profile state.
- [ ] Show parent role and observer/co-parent distinction.
- [ ] Show setup incomplete and recovery-needed states.
- [ ] Keep child-device authority separate from account membership.
- [ ] Add tests for empty and partially configured households.

## Acceptance And Proof

The route renders setup state from contracts/read models or labels fixture state
explicitly.

## Parallel Ownership Notes

Backend family setup contracts or service changes must be owned outside C unless
explicitly assigned.
