# 18 Proof Command And Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `18 Proof Command And Matrix`
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

Several proof commands exist. V0.8 needs one product-control proof view that
prevents accidental claim drift.

## Where We Want To Be

A composed proof command records implemented, report-only, scaffold,
unavailable, degraded, manual-required, and not-implemented states.

## Requirement Checklist

- [x] Include app/game, managed browser, unmanaged browser, network/domain,
      timers, approvals, integrity, and platform rows.
- [x] Write deterministic JSON under `test-results`.
- [x] Include commit SHA, command, platform, proof level, and known gaps.
- [x] Fail or flag claim upgrades without proof.
- [x] Reference the proof in plan proof/checklist routing when status changes.

## Acceptance And Proof

The proof command is repeatable locally and suitable for PR handoff review.

Current proof command:
`node scripts/test/v0-8-enforcement-control-plan-proof.mjs`

Current proof artifacts:
- `test-results/v0-8-enforcement-control-plan-proof/proof.json`
- `output/v0-8-enforcement-control-plan-proof/18-proof-command-and-matrix/`
- `docs/proof/v0-8-enforcement-control-plan/slice-03-proof-command-and-matrix.md`

## Parallel Ownership Notes

This is the final review anchor for A. Do not report product-complete V0.8
without this proof view.
