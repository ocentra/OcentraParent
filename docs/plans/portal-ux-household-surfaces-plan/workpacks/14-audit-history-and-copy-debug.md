# 14 Audit History And Copy/Debug

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

Copy/debug affordances exist in places. Product handoff needs consistent,
redacted, useful diagnostic output.

## Where We Want To Be

Parents and developers can copy connection, source, event, request, and recent
state details without exposing secrets or raw private content.

## Requirement Checklist

- [ ] Include current route, agent URL, connection state, timestamps, ids, and
      concise read-model rows.
- [ ] Redact secrets and unnecessary local private paths.
- [ ] Use timeline/table patterns for history.
- [ ] Show copy success/failure state.
- [ ] Test copy/debug on touched routes.

## Acceptance And Proof

Copied output is useful for support/handoff and does not leak forbidden data.

## Parallel Ownership Notes

Debug output must not become a data export feature.
