# Workpack 08 - Parent Client Update Rollback

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `08-parent-client-update-rollback`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the update channel, checksum/signature check, and rollback/teardown path.

## Must prove

- update available/unavailable/manual-required states are explicit
- rollback available/unavailable/manual-required states are explicit
- teardown or revert evidence exists for negative paths
- update claims do not imply production release on their own

## Failure conditions

- update status hides checksum/signature state
- rollback is not exercised as a real negative case
- child runtime update claims are folded into the parent matrix
