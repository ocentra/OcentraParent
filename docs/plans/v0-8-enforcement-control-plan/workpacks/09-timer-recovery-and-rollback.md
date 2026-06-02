# 09 Timer Recovery And Rollback

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Timer and recovery contracts exist. Product behavior needs consistent service
state, restart handling, rollback, and visible failure modes.

## Where We Want To Be

Temporary enforcement has a durable lifecycle: created, active, extended,
expired, cancelled, rollback requested, rollback completed, or recovery-needed.

## Requirement Checklist

- [ ] Journal timer lifecycle events.
- [ ] Recover active timers from durable state on restart.
- [ ] Emit recovery-needed when state cannot be restored safely.
- [ ] Show expiry and next-check state to parent surfaces.
- [ ] Cover rollback success and rollback unavailable.

## Acceptance And Proof

Service tests cover timer create, expire, cancel, restart recover,
recovery-needed, and rollback outcomes.

## Parallel Ownership Notes

Timers must stay in the child-device service path, never portal local state.
