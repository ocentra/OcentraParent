# 12 Child-Facing Status And Reasons

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Parent-facing proof states exist. Child-facing status remains incomplete as a
product experience.

## Where We Want To Be

When a child is warned, time-limited, blocked, or asked to request permission,
the child device can show a stable reason and next action without portal logic.

## Requirement Checklist

- [ ] Define reason codes and text-token references.
- [ ] Support time limit reached, ask parent, allowed, warning, unavailable, and
      blocked by parent rule.
- [ ] Include target and schedule context where safe.
- [ ] Keep generated/AI text out of enforcement status.
- [ ] Preserve local/offline behavior.

## Acceptance And Proof

Contract and service tests prove child-facing reason state is returned with
action results and survives audit.

## Parallel Ownership Notes

This is runtime/product state, not visual polish. UI lanes may render it after
service state exists.
