# 08 Activity Evidence And Diagnostics

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

Evidence paths exist across browser, app/game, network, and screen summaries.
The UI needs consistent diagnostic and parent-facing patterns.

## Where We Want To Be

Activity surfaces show evidence source, confidence, freshness, custody, and
unknown/degraded state before summaries or assistant actions.

## Requirement Checklist

- [ ] Show source evidence refs where available.
- [ ] Separate confidence from certainty.
- [ ] Label local-only, parent-cache, and unavailable custody.
- [ ] Show stale/degraded/unknown states.
- [ ] Add copy/debug output with redaction.

## Acceptance And Proof

Activity views never invent evidence or collapse unavailable state into normal
data.

## Parallel Ownership Notes

Evidence storage/runtime ownership remains outside C.
