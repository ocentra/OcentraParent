# 13 Service Read Models And API

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
