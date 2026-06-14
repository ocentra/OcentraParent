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
