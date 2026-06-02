# 05 Policy Authoring Control Center

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

Policy preview and control states exist in pieces. Complete nontechnical policy
authoring remains incomplete.

## Where We Want To Be

Parents can scan, create, preview, and understand rules by child, target,
schedule, action, proof level, and last result.

## Requirement Checklist

- [ ] Use typed intents for rule changes.
- [ ] Show dry-run/observe/enforcement-eligible states.
- [ ] Show conflict and unavailable reasons.
- [ ] Keep policy evaluation out of the portal.
- [ ] Test create/preview UI paths where backed by service state.

## Acceptance And Proof

UI actions produce typed request/preview state and render service response.

## Parallel Ownership Notes

A owns enforcement action truth. C owns the authoring workflow and visual model.
