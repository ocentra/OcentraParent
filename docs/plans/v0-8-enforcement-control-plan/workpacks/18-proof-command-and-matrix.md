# 18 Proof Command And Matrix

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Several proof commands exist. V0.8 needs one product-control proof view that
prevents accidental claim drift.

## Where We Want To Be

A composed proof command records implemented, report-only, scaffold,
unavailable, degraded, manual-required, and not-implemented states.

## Requirement Checklist

- [ ] Include app/game, managed browser, unmanaged browser, network/domain,
      timers, approvals, integrity, and platform rows.
- [ ] Write deterministic JSON under `test-results`.
- [ ] Include commit SHA, command, platform, proof level, and known gaps.
- [ ] Fail or flag claim upgrades without proof.
- [ ] Reference the proof in feature docs/checklist when status changes.

## Acceptance And Proof

The proof command is repeatable locally and suitable for PR handoff review.

## Parallel Ownership Notes

This is the final review anchor for A. Do not report product-complete V0.8
without this proof view.
