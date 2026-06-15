# Workpack 03 - Parent Desktop Shell Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `03-parent-desktop-shell-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the desktop shell/package boundary, including local-service bridge and launch smoke, without claiming product readiness.

## Must prove

- `dev:desktop` and `dev:desktop:lan` are honest launch anchors
- the shell reaches service state or degrades honestly
- signing/update/rollback remain explicit artifact claims
- launch smoke does not imply child runtime authority

## Failure conditions

- launch smoke becomes product readiness
- stale local-service state is treated as healthy
- desktop proof is used to claim mobile parity
