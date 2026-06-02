# 04 Owned-Process Time Limit

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Owned-process terminate/time-limit proof exists in narrow form. It must remain
clearly separate from broad app blocking.

## Where We Want To Be

Scoped process control is safe, explicit, auditable, restart-aware, and
parent-visible as one narrow adapter capability.

## Requirement Checklist

- [ ] Require pid and process identity checks before action.
- [ ] Record mismatch, already-exited, unavailable, and success outcomes.
- [ ] Tie limits to policy decisions and app/game evidence refs.
- [ ] Add rollback/recovery where the adapter supports it.
- [ ] Keep broad app blocking manual-required.

## Acceptance And Proof

Focused service proof covers success, no-op, mismatch, unavailable, and
manual-required broad target states.

## Parallel Ownership Notes

Do not generalize this workpack into all installed-app blocking until a separate
adapter and proof path exists.
