# Workpack 09 - Parent Client Launch Smoke Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `09-parent-client-launch-smoke-matrix`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove launch smoke per artifact without turning smoke into readiness.

## Must prove

- launch smoke exists for web, desktop, Android, and iOS rows
- degraded and unavailable states are visible
- manual-required states are explicit where the platform cannot yet prove parity
- launch smoke does not claim setup completion or child runtime ownership

## Failure conditions

- smoke is treated as enough for readiness
- degraded state is hidden behind a green launch
- launch smoke crosses into setup or child-runtime claims

## Completion status

Status: complete for the bounded WP09 packet.

Proof root:

- `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/00-scope-summary.md`
- `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/01-negative-case-proof.md`
- `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/02-manual-required-gap-register.md`
- `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/03-web-launch-smoke.log`
- `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/04-desktop-launch-smoke.log`
- `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/05-parent-mobile-launch-smoke.log`
- `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/16-validation-commands.log`

Current truth:

- launch smoke coverage exists for web, desktop, Android, and iOS rows
- the current web launch smoke row is explicit and blocked; the exact blocker is recorded in `03-web-launch-smoke.log`
- the current desktop row is explicit and manual-required; dry-run launch anchors and Rust service reachability proof passed without upgrading smoke into readiness
- the current Android and iOS rows are explicit and blocked; the exact parent-mobile runtime blocker is recorded in `05-parent-mobile-launch-smoke.log`
- degraded, unavailable, and manual-required states remain visible instead of hiding behind a green launch
- no row claims setup completion or child-runtime ownership
