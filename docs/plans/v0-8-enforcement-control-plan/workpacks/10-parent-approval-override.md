# 10 Parent Approval And Override

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Approval and override audit-reference proof exists. Product flow still needs
typed request, response, expiry, bonus-time, and child-agent validation.

## Where We Want To Be

A child request can become a parent-approved or denied action through typed,
audited, expiry-aware intents validated by the child-device agent.

## Requirement Checklist

- [ ] Add request, approval, denial, expiry, duplicate, and override states.
- [ ] Validate child, device, route, actor, policy version, and target.
- [ ] Record audit events for every transition.
- [ ] Show pending and expired state in parent surfaces.
- [ ] Keep notification delivery separate unless a provider is proved.

## Acceptance And Proof

Tests cover granted, denied, expired, wrong-device, duplicate, and override
outcomes.

## Parallel Ownership Notes

Notification work is a dependent future slice. This workpack can expose queued
or manual-required delivery state without claiming push delivery.
