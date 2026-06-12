# 10 Parent Approval And Override

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `10 Parent Approval And Override`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
