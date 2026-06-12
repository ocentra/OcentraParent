# 13 Service Read Models And API

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `13 Service Read Models And API`
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

The service exposes multiple proof/read paths. Parent surfaces need one coherent
product-control state.

## Where We Want To Be

The Rust service returns validated read models for current capability, recent
actions, pending approvals, active timers, degraded states, and manual-required
gaps.

## Requirement Checklist

- [ ] Add or extend protocol contracts before service output.
- [ ] Include capability matrix row, proof level, source evidence, and route.
- [ ] Include active timer and pending approval summaries.
- [ ] Include recent audit/action history.
- [ ] Validate payloads in portal consumers.

## Acceptance And Proof

Real-service tests and proof scripts read the same state that the portal renders.

## Parallel Ownership Notes

This workpack is the bridge before UI. Portal work should not proceed with
hardcoded product-control data.
