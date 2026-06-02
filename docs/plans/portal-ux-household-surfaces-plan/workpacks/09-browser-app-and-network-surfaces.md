# 09 Browser, App, And Network Surfaces

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

Browser, app/game, and network evidence each have different trust boundaries.
The UI must keep those distinctions visible.

## Where We Want To Be

Parents can see exact managed browser evidence, unmanaged browser detection,
app/game sessions, and network flow summaries without confusing one for another.

## Requirement Checklist

- [ ] Label exact URL only for managed browser evidence.
- [ ] Label unmanaged browser as process-only/possible bypass.
- [ ] Show app/game session duration from stored evidence.
- [ ] Show network flow as metadata, not page content.
- [ ] Test unknown and degraded states.

## Acceptance And Proof

UI tests fail if weaker evidence is presented as a stronger claim.

## Parallel Ownership Notes

This is a UX truthfulness workpack; runtime evidence producers remain separate.
