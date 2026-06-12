# 12 Reports, Notifications, And Custody

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `12 Reports, Notifications, And Custody`
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

Reports and notifications are planned/in progress. The portal needs truthful
custody and delivery states before product claims.

## Where We Want To Be

Reports and alerts show source evidence, custody, retention, export/delete,
delivery, quiet-hours, escalation, and authenticated drill-in state where
available.

## Requirement Checklist

- [ ] Label local, parent-owned storage, cache, hosted metadata, and unavailable.
- [ ] Show notification queued/failed/manual-required states.
- [ ] Keep payload detail minimal.
- [ ] Show retention/export/delete status where available.
- [ ] Test report unavailable and degraded states.

## Acceptance And Proof

UI does not imply Ocentra-hosted child activity storage by default.

## Parallel Ownership Notes

Notification providers and sync/export connectors are future/runtime work.
