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

## Completion status

Status: complete for the bounded WP08 packet.

Proof root:

- `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/00-scope-summary.md`
- `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/01-negative-case-proof.md`
- `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/02-manual-required-gap-register.md`
- `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/16-validation-commands.log`

Current truth:

- update available/unavailable/manual-required states are explicit by channel
- rollback available/unavailable/manual-required states are explicit at the contract surface, with current proved parent-desktop rows limited to unavailable or manual-required
- checksum and signature truth are explicit per update channel
- teardown or revert evidence is recorded for the negative scaffold and unsigned-preview paths
- update claims stay separated from production release claims
- child runtime update claims remain excluded
- SBOM stays an explicit blocker/manual-required gap instead of a fake proof claim
