# 19 Playwright And UI Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `19 Playwright And UI Proof`
> Kind: proof reference; read only when validating matching claim.
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

Portal proof exists for several routes. V0.8 control state needs browser-visible
evidence once service read models are wired.

## Where We Want To Be

Playwright exercises parent-visible controls through the real Rust service,
validates returned states, and captures degraded/manual-required UI behavior.

## Requirement Checklist

- [ ] Test dry-run, observe-only, active timer, ask-parent pending,
      unavailable, degraded, and manual-required states.
- [ ] Fail on console errors and page errors.
- [ ] Check desktop and mobile widths when layouts change.
- [ ] Save screenshots only as review artifacts, not source truth.
- [ ] Keep tests free of mocks and fake service handlers.

## Acceptance And Proof

UI proof can be reviewed alongside the proof JSON and service tests.

## Parallel Ownership Notes

C can improve visual quality, but A must ensure the rendered states are true to
service/product-control contracts.
